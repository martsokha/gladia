# API design

How this crate's surface is derived from the Gladia API, and where it deliberately
departs from the official [TypeScript][sdk-ts] and [Python][sdk-py] SDKs.

The short version: the *structure* of the official SDKs is worth copying — a root
client that hands out per-domain sub-clients, one convenience method that collapses
the common flow. Their *live transcription model* is not: both use event-emitter
callbacks, which exist because neither language had a better option at the time. Rust
does, and it changes the shape of that API considerably.

## Reference

The specs are vendored under `docs/` so the types can be checked against something
authoritative rather than prose:

- `docs/openapi.json` — OpenAPI 3.1, 133 schemas, from `https://api.gladia.io/openapi.json`
- `docs/asyncapi.yaml` — AsyncAPI 3.0, the live WebSocket protocol, from `https://docs.gladia.io/asyncapi.yaml`

**The narrative docs disagree with the spec on three paths.** The spec wins; it is
what the server serves:

| `docs.gladia.io` says | The spec says |
| --- | --- |
| `POST /v2/upload/audio-file` | `POST /v2/upload` |
| `GET /v2/pre-recorded/list` | `GET /v2/pre-recorded` |
| `GET /v2/pre-recorded/:id/audio` | `GET /v2/pre-recorded/{id}/file` |

Auth is `x-gladia-key`, a header API key (`components.securitySchemes.x_gladia_key`).

## What the official SDKs do

Both SDKs are the same design in two languages, which makes the shared parts a strong
signal and the divergences mostly incidental.

```
GladiaClient
├── preRecordedV2()  ──▶  uploadFile · create · get · delete · getFile
│                         poll · createAndPoll · transcribe
└── liveV2()         ──▶  startSession · connectSession · get · delete · getFile
                          └── LiveV2Session: sendAudio · stopRecording · endSession
                                             on(event, cb) / once / off
```

Worth keeping:

- **A root client that vends sub-clients per domain.** Pre-recorded and live share
  credentials and transport but have disjoint operations. Nesting them keeps
  completion lists short and makes the two flows hard to confuse.
- **`transcribe()` as the headline method.** The raw flow is upload → init → poll →
  read. Most callers want exactly that, once. Both SDKs collapse it into one call, and
  it is the first thing in both READMEs.
- **Versioned names (`preRecordedV2`).** The API has already deprecated
  `/v2/transcription` in favour of `/v2/pre-recorded`. Version in the type name buys
  room to add `v3` without a breaking change.

Not worth keeping:

- **`createUntyped` / `transcribeUntyped`.** These exist because a generated type can
  lag the API, so the SDKs offer a `Record<string, unknown>` bypass. In Rust that role
  is already filled — `serde_json::Value` implements `Serialize`, so a caller who
  needs an unmodelled field can pass one to the same typed method. A second set of
  methods would double the surface for nothing.
- **The event-emitter live session.** Discussed below.

## The design

### Sub-clients, borrowed not owned

```rust
let gladia = Client::builder().with_api_key(key).build()?;

gladia.pre_recorded()   // -> PreRecorded<'_>
gladia.live()           // -> Live<'_>
```

`Client` is already `Arc`-backed, so a sub-client is a borrow of shared state, not a
new connection pool. Making them borrow rather than own means `pre_recorded()` in a
method chain costs nothing and there is no second lifetime to reason about.

### Pre-recorded: a builder for the request, a job handle for the result

The init request is the awkward part of this API. `InitTranscriptionRequest` has 27
optional fields, most in `feature: bool` + `feature_config: Object` pairs —
`diarization` / `diarization_config`, `translation` / `translation_config`, and so on
for nine features. A faithful translation is a 27-field struct where 26 fields are
`Option` and half of them are a bool that must agree with a sibling.

Rust can do better than faithful. The bool is redundant with the presence of the
config, so the builder owns that invariant:

```rust
let job = gladia.pre_recorded()
    .transcribe("meeting.wav")                    // path, URL, or bytes
    .language(Language::En)
    .diarization(Diarization::new().max_speakers(4))   // sets the flag too
    .await?;

println!("{}", job.transcript());
```

`diarization(..)` writes `diarization: true` *and* `diarization_config: {...}` on the
wire. The caller cannot set one without the other, so the state that makes no sense
cannot be constructed. Each feature config gets its own small builder rather than one
27-field god-struct.

For callers who want the raw request — the ones the `Untyped` methods exist for — the
request type stays public and `serde`-serializable, so it can be built directly or
deserialized from JSON.

### Three levels, not one

The official SDKs expose `create`, `poll`, `createAndPoll`, and `transcribe` as four
peers on one client. The relationship between them is only in the docs. Making the
layering explicit means each level is a complete API and the level above is a thin
convenience:

```rust
// 1. one call, the common case — upload, init, poll, return
let job = gladia.pre_recorded().transcribe("meeting.wav").await?;

// 2. submit and poll separately, for a queue or a progress bar
let handle = gladia.pre_recorded().submit(request).await?;
let job = handle.wait().await?;

// 3. the endpoints themselves
let upload = gladia.pre_recorded().upload("meeting.wav").await?;
let handle = gladia.pre_recorded().init(&request).await?;
let job = gladia.pre_recorded().get(handle.id()).await?;
```

`submit` returns a `JobHandle` carrying the id and the client, so `wait()` is a method
on the thing being waited for rather than a free function taking an id. A handle is
cheap and `Clone`, and it can be reconstructed from a stored id — which matters,
because transcription jobs outlive processes and a caller who persists an id needs to
resume from it.

Polling stays explicit about time: `wait()` uses a default interval, `wait_with` takes
an interval and a deadline. The crate does not pick a runtime — the sleep is supplied
by the caller, the way `wait_until_ready` already does in the client.

### Live: a `Stream`, not callbacks

This is the significant departure, so it is worth being precise about why.

Both official SDKs model a live session as an event emitter — `session.on('message',
cb)`. In JavaScript that is the only real option. Python's SDK does the same even in
its `async` client, which is more surprising: `LiveV2AsyncSession` has no `__aiter__`,
so `async for` does not work on it.

The protocol underneath is not event-shaped. It is a single WebSocket carrying a
tagged union — every server message has a `type` field (`transcript`, `speech_start`,
`speech_end`, `translation`, `sentiment_analysis`, `post_final_transcript`,
`post_summarization`, and so on, per `asyncapi.yaml`). That is a `Stream` of an
`enum`, and Rust models it directly:

```rust
let mut session = gladia.live().start(config).await?;

// send and receive are independent halves
let audio = session.sender();
tokio::spawn(async move {
    while let Some(chunk) = mic.next().await {
        audio.send(chunk).await?;
    }
    audio.finish().await          // stop_recording, then end_session
});

while let Some(message) = session.next().await {
    match message? {
        Message::Transcript(t) if t.is_final => println!("{}", t.text),
        Message::SpeechStart(_) => print!("…"),
        _ => {}
    }
}
```

What this buys over callbacks, concretely:

- **Exhaustiveness.** A `match` on the message enum is checked. A missed `on('...')`
  string is not — and the channel carries 19 message types, 13 of them server→client,
  which is more than anyone reliably remembers to handle.
- **Errors travel with the messages.** The stream yields `Result<Message>`, so a
  protocol error surfaces at the `?` where the caller already handles errors, rather
  than in a separate `on('error')` channel that is easy to leave unregistered.
- **Backpressure.** A slow consumer of a `Stream` stops pulling. A slow callback just
  falls behind, and the buffer grows without anything noticing.
- **It composes.** `StreamExt` gives `filter`, `take_while`, `timeout`, `chunks` for
  free. Everything on an emitter is bespoke.

Splitting the session into a sender half and a receiver half is what lets audio be
pumped from one task while transcripts are consumed in another — the normal shape for
live transcription, and awkward to express when both directions hang off one object.

The message enum uses `#[serde(tag = "type")]`, which maps one-to-one onto the
protocol, and is `#[non_exhaustive]` so Gladia can add a message type without it being
a breaking change here.

### Types

133 OpenAPI schemas and ~2,400 lines of generated types per official SDK is more than
is worth hand-writing, and hand-written types drift from the spec silently.

The plan is to generate the leaf types (enums, config structs, response bodies) from
`docs/openapi.json`, check the output in, and hand-write only the surface a caller
touches — the builders, the handles, the session. Generated code stays in its own
module so the boundary is obvious, and regenerating is a diff, not a rewrite.

Language and model codes become enums with a `#[serde(other)]` catch-all variant:
Gladia adds languages regularly, and an unknown one should deserialize, not fail the
whole response.

### Feature flags

`live` returns, this time with a module behind it. WebSocket support pulls in
`tokio-tungstenite` and a runtime dependency that HTTP-only callers should not pay
for.

## Plan

Each lands as its own PR:

1. **Generated types** — codegen from `docs/openapi.json`, committed output, no public
   API yet.
2. **Pre-recorded endpoints** — `upload`, `init`, `get`, `list`, `delete`, `file`; the
   raw layer, level 3 above.
3. **`JobHandle` and polling** — `submit` / `wait`, level 2.
4. **The request builder and `transcribe`** — level 1, and the README's headline
   example becomes real.
5. **Live sessions** — behind the `live` feature: `start`, the split halves, the
   message `Stream`.

Order matters: each level is built on the one below, and each is usable on its own, so
the crate is never in a state where the only way to do something is to wait for the
next PR.

[sdk-ts]: https://www.npmjs.com/package/@gladiaio/sdk
[sdk-py]: https://pypi.org/project/gladiaio-sdk/
