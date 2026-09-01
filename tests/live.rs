//! Live-session tests against a real WebSocket server run in-process.
//!
//! `wiremock` speaks HTTP only, so these spin up a `tokio-tungstenite` server on a
//! loopback port and drive a genuine socket: the handshake, binary audio frames, the
//! JSON control messages, and the close. Nothing here touches the network.

#![cfg(feature = "live")]

use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use gladia::live::{Message, Session};
use gladia::{Client, Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// What the fake server received from the client.
#[derive(Debug)]
enum Received {
    /// A binary audio frame, with its length.
    Audio(usize),
    /// A JSON control message's `type`.
    Control(String),
}

/// Starts a WebSocket server that sends `script` and records what it receives.
///
/// Returns the address to connect to and a channel of received frames.
async fn serve(script: Vec<serde_json::Value>) -> (SocketAddr, mpsc::UnboundedReceiver<Received>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        handle(stream, script, sender).await;
    });

    (address, receiver)
}

/// Accepts one connection, sends the scripted messages, and reports what arrives.
async fn handle(
    stream: TcpStream,
    script: Vec<serde_json::Value>,
    sender: mpsc::UnboundedSender<Received>,
) {
    let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();

    for message in script {
        let frame = WsMessage::Text(message.to_string().into());
        if socket.send(frame).await.is_err() {
            return;
        }
    }

    while let Some(Ok(frame)) = socket.next().await {
        let received = match frame {
            WsMessage::Binary(bytes) => Received::Audio(bytes.len()),
            WsMessage::Text(text) => {
                let value: serde_json::Value = serde_json::from_str(&text).unwrap();
                Received::Control(value["type"].as_str().unwrap_or_default().to_owned())
            }
            WsMessage::Close(_) => break,
            _ => continue,
        };

        if sender.send(received).is_err() {
            break;
        }
    }
}

/// Builds a client whose `POST /v2/live` hands back `socket_address`.
async fn client_for(socket_address: SocketAddr) -> (Client, MockServer) {
    let api = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/live"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "3d9a1f6c-0000-4000-8000-000000000002",
            "created_at": "2026-09-01T00:00:00.000Z",
            "url": format!("ws://{socket_address}/ws")
        })))
        .mount(&api)
        .await;

    let client = Client::builder()
        .with_api_key("test-key")
        .with_base_url(api.uri())
        .with_max_retries(0u32)
        .build()
        .unwrap();

    // The mock server is returned so it outlives the client that points at it.
    (client, api)
}

/// Opens a session against a server that will send `script`.
async fn session_with(
    script: Vec<serde_json::Value>,
) -> (Session, mpsc::UnboundedReceiver<Received>, MockServer) {
    let (address, received) = serve(script).await;
    let (client, api) = client_for(address).await;

    let request = serde_json::from_value(serde_json::json!({})).unwrap();
    let session = client.live().start(&request).await.unwrap();

    (session, received, api)
}

/// A server message with the standard envelope.
fn message(kind: &str, extra: serde_json::Value) -> serde_json::Value {
    let mut value = serde_json::json!({
        "type": kind,
        "session_id": "sess_1",
        "created_at": "2026-09-01T00:00:00.000Z"
    });

    if let Some(fields) = extra.as_object() {
        for (key, field) in fields {
            value[key] = field.clone();
        }
    }

    value
}

#[tokio::test]
async fn a_session_streams_messages_until_it_ends() {
    let (mut session, _received, _api) = session_with(vec![
        message("start_session", serde_json::json!({})),
        message(
            "transcript",
            serde_json::json!({ "data": { "utterance": { "text": "hello" } } }),
        ),
        message("end_session", serde_json::json!({})),
    ])
    .await;

    assert_eq!(session.id(), "3d9a1f6c-0000-4000-8000-000000000002");

    let messages: Vec<Message> = (&mut session)
        .map(|message| message.unwrap())
        .collect()
        .await;

    assert!(matches!(messages[0], Message::StartSession(_)));
    assert!(matches!(messages[2], Message::EndSession(_)));
    assert_eq!(messages.len(), 3, "the stream must end after end_session");

    let Message::Transcript(transcript) = &messages[1] else {
        panic!("expected a transcript, got {:?}", messages[1]);
    };
    assert_eq!(
        transcript.data.as_ref().unwrap()["utterance"]["text"],
        "hello"
    );
}

#[tokio::test]
async fn audio_is_sent_as_binary_frames() {
    let (mut session, mut received, _api) =
        session_with(vec![message("start_session", serde_json::json!({}))]).await;

    let mut audio = session.sender();
    audio.send(vec![0u8; 3200]).await.unwrap();
    audio.send(vec![0u8; 1600]).await.unwrap();
    audio.finish().await.unwrap();

    // The two chunks arrive as binary frames, then `finish` sends stop_recording.
    assert!(matches!(received.recv().await, Some(Received::Audio(3200))));
    assert!(matches!(received.recv().await, Some(Received::Audio(1600))));

    let Some(Received::Control(kind)) = received.recv().await else {
        panic!("expected a control message after the audio");
    };
    assert_eq!(kind, "stop_recording");
}

#[tokio::test]
async fn stop_recording_leaves_the_session_open() {
    let (mut session, mut received, _api) = session_with(vec![
        message("start_session", serde_json::json!({})),
        // Post-processing arrives after recording stops, so the stream must not end
        // when the client stops sending audio.
        message("post_final_transcript", serde_json::json!({ "data": {} })),
        message("end_session", serde_json::json!({})),
    ])
    .await;

    let mut audio = session.sender();
    audio.stop_recording().await.unwrap();

    let Some(Received::Control(kind)) = received.recv().await else {
        panic!("expected stop_recording");
    };
    assert_eq!(kind, "stop_recording");

    let kinds: Vec<&'static str> = (&mut session)
        .map(|message| match message.unwrap() {
            Message::StartSession(_) => "start",
            Message::PostFinalTranscript(_) => "post_final",
            Message::EndSession(_) => "end",
            _ => "other",
        })
        .collect()
        .await;

    assert_eq!(kinds, ["start", "post_final", "end"]);
}

#[tokio::test]
async fn an_unknown_message_does_not_break_the_stream() {
    let (mut session, _received, _api) = session_with(vec![
        message("some_future_message", serde_json::json!({ "data": {} })),
        message("transcript", serde_json::json!({ "data": {} })),
        message("end_session", serde_json::json!({})),
    ])
    .await;

    let messages: Vec<Message> = (&mut session)
        .map(|message| message.unwrap())
        .collect()
        .await;

    assert!(
        matches!(&messages[0], Message::Unknown { kind, .. } if kind == "some_future_message"),
        "{:?}",
        messages[0]
    );
    // The stream carries on, so one new server message does not blind a client.
    assert!(matches!(messages[1], Message::Transcript(_)));
}

#[tokio::test]
async fn a_non_json_frame_reports_a_decode_error() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        socket
            .send(WsMessage::Text("this is not json".into()))
            .await
            .unwrap();
        // Hold the socket open so the client sees the frame, not a close.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    });

    let (client, _api) = client_for(address).await;
    let request = serde_json::from_value(serde_json::json!({})).unwrap();
    let mut session = client.live().start(&request).await.unwrap();

    let error = session.next().await.unwrap().unwrap_err();

    let Error::Decode { message, .. } = &error else {
        panic!("expected a decode error, got {error:?}");
    };
    assert!(message.contains("this is not json"), "{message}");
}

#[tokio::test]
async fn taking_the_sender_twice_panics() {
    let (mut session, _received, _api) =
        session_with(vec![message("start_session", serde_json::json!({}))]).await;

    let _first = session.sender();

    // Two senders on one socket would interleave frames, so the second take is a bug
    // in the caller rather than a runtime condition to handle.
    let second = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| session.sender()));
    assert!(second.is_err());
}

#[tokio::test]
async fn the_handshake_carries_the_clients_configured_headers() {
    // The client's default headers, the API key among them, must reach the WebSocket
    // handshake: the session is authenticated by the same credential as every request.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, mut headers) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        // The callback's error type is tungstenite's own; its size is not ours to fix.
        #[allow(clippy::result_large_err)]
        let socket = tokio_tungstenite::accept_hdr_async(
            stream,
            |request: &tokio_tungstenite::tungstenite::handshake::server::Request, response| {
                let seen: Vec<(String, String)> = request
                    .headers()
                    .iter()
                    .map(|(name, value)| {
                        (
                            name.as_str().to_owned(),
                            value.to_str().unwrap_or_default().to_owned(),
                        )
                    })
                    .collect();
                let _ = sender.send(seen);
                Ok(response)
            },
        )
        .await
        .unwrap();

        drop(socket);
    });

    let api = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2/live"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "3d9a1f6c-0000-4000-8000-000000000002",
            "created_at": "2026-09-01T00:00:00.000Z",
            "url": format!("ws://{address}/ws")
        })))
        .mount(&api)
        .await;

    let client = Client::builder()
        .with_api_key("test-key")
        .with_base_url(api.uri())
        .with_header("x-trace-id", "abc123")
        .with_max_retries(0u32)
        .build()
        .unwrap();

    let request = serde_json::from_value(serde_json::json!({})).unwrap();
    let _session = client.live().start(&request).await.unwrap();

    let seen = headers.recv().await.expect("the handshake headers");
    let find = |name: &str| {
        seen.iter()
            .find(|(header, _)| header == name)
            .map(|(_, value)| value.clone())
    };

    assert_eq!(find("x-gladia-key").as_deref(), Some("test-key"));
    assert_eq!(find("x-trace-id").as_deref(), Some("abc123"));
}
