# API design

How this crate's surface is derived from the Gladia API, and where it deliberately
departs from the official [TypeScript][sdk-ts] and [Python][sdk-py] SDKs. It describes
the crate as built; the reasoning is kept so the departures are auditable rather than
folklore.

The short version: the *structure* of the official SDKs is worth copying, meaning a
root client that hands out per-domain sub-clients and one convenience method that
collapses the common flow. Their *live transcription model* is not: both use event-emitter
callbacks, which exist because neither language had a better option at the time. Rust
does, and it changes the shape of that API considerably.

## Reference

`docs/openapi.json`, which is OpenAPI 3.1 with 133 schemas, is vendored from
`https://api.gladia.io/openapi.json`, and is what the types are generated from.
`cargo xtask fetch-spec` refreshes it.

It is vendored rather than downloaded during codegen because **the served document is
not byte-stable**: the API stamps the current time into the `date`, `before_date` and
`after_date` query-parameter examples, so two fetches minutes apart differ while every
schema is identical. A codegen that downloaded would make `--check` fail at random.
`fetch-spec` pins those 34 examples to a fixed instant, which is what makes repeated
refreshes idempotent. Vendoring also keeps builds offline and reproducible, and turns a
Gladia-side API change into a reviewable diff instead of a silent change in generated
types.

The live WebSocket protocol is documented separately, as AsyncAPI 3.0 at
`https://docs.gladia.io/asyncapi.yaml`. Nothing is generated from it, since the live
message types are hand-written in `src/live/message.rs`, so it is not vendored. It is
the reference for what those types must cover.

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
  is already filled: `serde_json::Value` implements `Serialize`, so a caller who
  needs an unmodelled field can pass one to the same typed method. A second set of
  methods would double the surface for nothing.
- **The event-emitter live session.** Discussed below.

## The design

### Sub-clients, borrowed not owned

```rust
let gladia = Client::builder().with_api_key(key).build()?;

gladia.prerecorded()   // -> PreRecorded<'_>
gladia.live()           // -> Live<'_>
```

`Client` is already `Arc`-backed, so a sub-client is a borrow of shared state, not a
new connection pool. Making them borrow rather than own means `prerecorded()` in a
method chain costs nothing and there is no second lifetime to reason about.

### Pre-recorded: a builder for the request, a job handle for the result

The init request is the awkward part of this API. `InitTranscriptionRequest` has 27
optional fields, most in `feature: bool` + `feature_config: Object` pairs such as
`diarization` / `diarization_config` and `translation` / `translation_config`, nine
features in all. A faithful translation is a 27-field struct where 26 fields are
`Option` and half of them are a bool that must agree with a sibling.

Rust can do better than faithful. The bool is redundant with the presence of the
config, so the builder owns that invariant:

```rust
let request = TranscriptionRequest::new("https://example.com/meeting.wav")
    .with_diarization(
        DiarizationConfigDto::builder().max_speakers(Some(4u64)).try_into()?,
    )
    .with_sentences()
    .build();
```

`with_diarization` writes `diarization: true` *and* `diarization_config: {...}` on the
wire. The caller cannot set one without the other, so the state that makes no sense
cannot be constructed. Unset features are absent from the body entirely rather than
sent as `false`, so the API's own defaults apply.

The nine pairs are not uniform, and the builder reflects that. Four configs carry
required fields (`translation` needs target languages, `custom_vocabulary` a
vocabulary, `custom_spelling` a dictionary, `audio_to_llm` prompts), so those features
cannot be enabled bare. The other five are entirely optional and get a `*_default`
form, so `with_subtitles_default()` enables subtitles without inventing a config.

For callers who want the raw request, the ones the `Untyped` methods exist for, the
generated type stays public and `serde`-serializable, reachable through `build()`.

### Three levels, not one

The official SDKs expose `create`, `poll`, `createAndPoll`, and `transcribe` as four
peers on one client. The relationship between them is only in the docs. Making the
layering explicit means each level is a complete API and the level above is a thin
convenience:

```rust
// 1. one call, the common case: upload, init, poll, return
let job = gladia
    .prerecorded()
    .transcribe("meeting.wav", audio, |request| request.with_sentences())
    .await?;

// 2. submit and poll separately, for a queue or a progress bar
let handle = gladia.prerecorded().submit(&request).await?;
let job = handle.wait_with(interval, Some(deadline)).await?;

// 3. the endpoints themselves
let upload = gladia.prerecorded().upload("meeting.wav", audio).await?;
let submitted = gladia.prerecorded().init(&request).await?;
let job = gladia.prerecorded().get(submitted.id).await?;
```

`submit` returns a `JobHandle` carrying the id and the client, so `wait()` is a method
on the thing being waited for rather than a free function taking an id. A handle is
cheap and `Clone`, and it can be reconstructed from a stored id. That matters because
transcription jobs outlive processes, and a caller who persists an id needs to resume
from it.

Polling stays explicit about time: `wait()` uses a default interval, `wait_with` takes
an interval and an optional deadline. The deadline is checked after each poll rather
than before, so a job that completes on its final attempt is returned instead of
reported as a timeout, and the sleep is clamped so it never overshoots.

`wait_with` sleeps on `tokio::time::sleep` rather than taking a sleep closure to stay
runtime-agnostic. That portability would be illusory: `reqwest` depends on `tokio`
through `hyper`, so every user of this crate already has it compiled.

### Live: a `Stream`, not callbacks

This is the significant departure, so it is worth being precise about why.

Both official SDKs model a live session as an event emitter, `session.on('message',
cb)`. In JavaScript that is the only real option. Python's SDK does the same even in
its `async` client, which is more surprising: `LiveV2AsyncSession` has no `__aiter__`,
so `async for` does not work on it.

The protocol underneath is not event-shaped. It is a single WebSocket carrying a
tagged union, since every server message has a `type` field (`transcript`, `speech_start`,
`speech_end`, `translation`, `sentiment_analysis`, `post_final_transcript`,
`post_summarization`, and so on, per the AsyncAPI document). That is a `Stream` of an
`enum`, and Rust models it directly:

```rust
let mut session = gladia.live().start(&request).await?;

// send and receive are independent halves
let mut audio = session.sender();
tokio::spawn(async move {
    while let Some(chunk) = mic.next().await {
        audio.send(chunk).await?;
    }
    audio.finish().await          // stop_recording, then close
});

while let Some(message) = session.next().await {
    match message? {
        Message::Transcript(transcript) => println!("{:?}", transcript.data),
        Message::SpeechStart(_) => print!("…"),
        _ => {}
    }
}
```

The variants are typed but their payloads are `serde_json::Value`: the live schemas
are absent from the OpenAPI document, so typing them would mean hand-written mirrors
drifting silently against a protocol nothing checks them on. The envelope every message
shares, the session id and timestamp, is typed, since that is what the session state
machine and most callers act on.

What this buys over callbacks, concretely:

- **Exhaustiveness.** A `match` on the message enum is checked. A missed `on('...')`
  string is not, and the channel carries 19 message types, 16 of them server→client,
  which is more than anyone reliably remembers to handle.
- **Errors travel with the messages.** The stream yields `Result<Message>`, so a
  protocol error surfaces at the `?` where the caller already handles errors, rather
  than in a separate `on('error')` channel that is easy to leave unregistered.
- **Backpressure.** A slow consumer of a `Stream` stops pulling. A slow callback just
  falls behind, and the buffer grows without anything noticing.
- **It composes.** `StreamExt` gives `filter`, `take_while`, `timeout`, `chunks` for
  free. Everything on an emitter is bespoke.

Splitting the session into a sender half and a receiver half is what lets audio be
pumped from one task while transcripts are consumed in another. That is the normal
shape for live transcription, and awkward to express when both directions hang off one
object.

The message enum uses `#[serde(tag = "type")]`, which maps one-to-one onto the
protocol, and is `#[non_exhaustive]` so Gladia can add a message type without it being
a breaking change here.

### Types, and how they get generated

133 OpenAPI schemas and ~2,400 lines of generated types per official SDK is more than
is worth hand-writing, and hand-written types drift from the spec silently. So: generate
every wire type (requests, responses, configs, enums) and hand-write only the surface
a caller touches, which is the ergonomic layer over them: the feature builders, the job
handle, the session.

The question is what does the generating. Both candidates were run against the real
spec rather than chosen from their READMEs.

**[`progenitor`], rejected.** It generates a whole client, not just types, which is
the closer fit on paper. In practice it fails twice. It rejects the spec outright
(`invalid version: 3.1.0`, since it supports 3.0.x only), and relabelling the version to
`3.0.3` gets a panic instead:

```
not yet implemented: more media types than expected for FileController_upload_v2: 2
```

That is `POST /v2/upload`, which accepts both `multipart/form-data` and
`application/json`. Removing the endpoint just moves the failure to
`unexpected content type: multipart/form-data`. Progenitor cannot express file upload,
which is the one operation this API is built around.

**[`typify`], chosen.** Progenitor's schema half, used on its own. Fed
`components.schemas` as a JSON Schema document, it generates all 133 types, compiles
clean with no warnings, and gets the two things right that were expected to break:

- `wav/pcm`, `solaria-1` and the integer enums become valid idents with a `#[serde(rename)]`
  (`WavPcm`, `Solaria1`), rather than mangled names or a bare `String`.
- `result`, the field callers actually read, becomes `Option<TranscriptionResultDto>`.
  This one matters more than it looks: the spec *declares* OpenAPI 3.1 but uses 3.0's
  `nullable: true` in 58 places, and wraps a single `$ref` in `allOf` 143 times. Under
  strict 3.1, `nullable` is not a keyword and is ignored, so a conformant generator would
  emit a non-optional `TranscriptionResultDto` and then fail to deserialize every job
  that is still queued. Typify honours it.

Round-tripping real payloads through the output confirms both directions of the
response path work: a queued job with `"result": null, "file": null` deserializes, and
so does the init response.

**Defaults have to be stripped from the spec first.** Out of the box, serializing a
request leaks every unset field:

```
unset fields leaked: ["audio_to_llm", "callback", "custom_spelling", "custom_vocabulary",
 "diarization", "model", "named_entity_recognition", "pii_redaction", ...]
```

The spec marks those booleans `default: false`, so typify emits `bool` with a `Default`
rather than `Option<bool>` with `skip_serializing_if`, and every request would carry
fifteen explicit `false`s, which is noise and pins defaults the server is free to change.

The fix is in the input, not the output. Deleting `default` from any property that is
not in its schema's `required` list (83 of them) makes typify emit `Option<T>` with
`skip_serializing_if`, and the request serializes to exactly the fields that were set.
That is an eight-line preprocessing step in the xtask, and it means **requests are
generated too**, with no hand-written mirror of the wire types and no drift between
them and the spec.

What stays hand-written is only the thin layer above: `PreRecordedRequest` wrapping the
generated `InitTranscriptionRequest` so that `diarization(Diarization::new().max_speakers(4))`
sets both the flag and the config. The generated builder can express that, just
verbosely, as `.diarization(Some(true)).diarization_config(Some(...))`, and it cannot
enforce that the two agree. That is a wrapper over one generated type, not a parallel
set of types.

**Mechanics.** Generation is a committed artifact, not a build script: a small
`xtask` reads `docs/openapi.json` and writes `src/model/generated.rs`, and the output
is checked in. Reasons: `docs.rs` builds offline, contributors get no extra toolchain,
`cargo build` stays fast, and a spec change shows up as a reviewable diff rather than
silently altering the API. CI re-runs it and fails if the committed output is stale.

Generated types carry `chrono::DateTime<Utc>` and `uuid::Uuid`, which is what typify
emits for `format: date-time` and `format: uuid`. The scaffold's `jiff` dependency is
dropped in favour of `chrono` rather than post-processing 133 types to convert it.
`jiff` had no callers yet, and fighting the generator to save one dependency is not
worth the xtask complexity.

Language codes get `#[serde(other)]` catch-alls where the enum permits. Gladia adds
languages regularly, and an unknown one should deserialize rather than fail the whole
response.

[`progenitor`]: https://github.com/oxidecomputer/progenitor
[`typify`]: https://github.com/oxidecomputer/typify

### Feature flags

`live` gates the WebSocket module, so HTTP-only callers do not pay for
`reqwest-websocket` and its transitive dependencies.

The socket is an upgraded `reqwest` request rather than a connection from a standalone
WebSocket stack. That is what carries the client's TLS backend, proxy settings, default
headers, and middleware onto the handshake, including the API key, without which the
session would not authenticate at all.

## Layout

| Module | Contents |
| --- | --- |
| `client` | `Client`, `ClientBuilder`, the header accumulator |
| `model` | generated wire types, re-exported from `model::generated` |
| `prerecorded` | the endpoints, `TranscriptionRequest`, `JobHandle`, `ListQuery` |
| `live` | `Live`, `Session`, `AudioSender`, `Message` (feature `live`) |
| `error` | `Error`, `Result` |

Tests are layered to match: unit tests beside the code for URL and query building,
`tests/model.rs` for round-tripping real payloads through the generated types,
`tests/prerecorded.rs` against an in-process HTTP server, and `tests/live.rs` against
an in-process WebSocket server.

[sdk-ts]: https://www.npmjs.com/package/@gladiaio/sdk
[sdk-py]: https://pypi.org/project/gladiaio-sdk/
