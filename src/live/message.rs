//! Messages the server sends over a live session.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A message from the server during a live session.
///
/// Every variant carries an [`Envelope`] with the session id and the message's
/// timestamp, plus whatever payload that message type defines. The payloads are
/// [`serde_json::Value`] rather than generated structs: the live schemas are not in
/// Gladia's OpenAPI document, so typing them here would mean hand-written mirrors that
/// drift silently. The envelope is typed, since that is what the session state machine
/// and most callers act on.
///
/// Non-exhaustive: Gladia adds message types, and an unknown one arrives as
/// [`Unknown`] rather than failing the stream.
///
/// [`Unknown`]: Message::Unknown
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Message {
    /// The session is open and ready for audio.
    StartSession(Envelope),
    /// The server began recording.
    StartRecording(Envelope),
    /// An interim or final transcript segment.
    Transcript(Payload),
    /// Speech was detected.
    SpeechStart(Payload),
    /// A stretch of speech ended.
    SpeechEnd(Payload),
    /// A translation of a transcript segment.
    Translation(Payload),
    /// Named entities found in a segment.
    NamedEntityRecognition(Payload),
    /// Sentiment for a segment.
    SentimentAnalysis(Payload),
    /// The full transcript, sent after recording stops.
    PostTranscript(Payload),
    /// The final transcript, after post-processing.
    PostFinalTranscript(Payload),
    /// Chapters derived from the transcript.
    PostChapterization(Payload),
    /// A summary of the transcript.
    PostSummarization(Payload),
    /// The server acknowledged an audio chunk.
    AudioChunkAck(Ack),
    /// The server acknowledged the stop-recording request.
    StopRecordingAck(Ack),
    /// Recording ended.
    EndRecording(Payload),
    /// The session closed. No further messages follow.
    EndSession(Envelope),
    /// A message type this crate does not model yet.
    ///
    /// Carries the raw JSON so a caller can handle a new message type without waiting
    /// for a release.
    Unknown {
        /// The message's `type` field.
        kind: String,
        /// The whole message, as received.
        raw: serde_json::Value,
    },
}

/// The fields every live message carries.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Envelope {
    /// The live session this message belongs to.
    pub session_id: String,
    /// When the server produced the message.
    pub created_at: DateTime<Utc>,
}

/// A message carrying a data payload.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Payload {
    /// The session id and timestamp.
    pub envelope: Envelope,
    /// The message's `data` field, if it carried one.
    pub data: Option<serde_json::Value>,
    /// The message's `error` field, for the message types that report per-feature
    /// failures (translation, summarization, and the other add-ons) without ending
    /// the session.
    pub error: Option<serde_json::Value>,
}

/// An acknowledgement of something the client sent.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Ack {
    /// The session id and timestamp.
    pub envelope: Envelope,
    /// Whether the server accepted the message being acknowledged.
    pub acknowledged: bool,
    /// Why the message was rejected, when `acknowledged` is false.
    pub error: Option<serde_json::Value>,
    /// Any data the acknowledgement carried.
    pub data: Option<serde_json::Value>,
}

impl Message {
    /// The session id, for every variant that carries one.
    ///
    /// [`Unknown`] has none, since its shape is by definition unknown.
    ///
    /// [`Unknown`]: Message::Unknown
    pub fn session_id(&self) -> Option<&str> {
        self.envelope().map(|envelope| envelope.session_id.as_str())
    }

    /// The message's envelope, for every variant that carries one.
    pub fn envelope(&self) -> Option<&Envelope> {
        match self {
            Self::StartSession(envelope)
            | Self::StartRecording(envelope)
            | Self::EndSession(envelope) => Some(envelope),

            Self::Transcript(payload)
            | Self::SpeechStart(payload)
            | Self::SpeechEnd(payload)
            | Self::Translation(payload)
            | Self::NamedEntityRecognition(payload)
            | Self::SentimentAnalysis(payload)
            | Self::PostTranscript(payload)
            | Self::PostFinalTranscript(payload)
            | Self::PostChapterization(payload)
            | Self::PostSummarization(payload)
            | Self::EndRecording(payload) => Some(&payload.envelope),

            Self::AudioChunkAck(ack) | Self::StopRecordingAck(ack) => Some(&ack.envelope),

            Self::Unknown { .. } => None,
        }
    }

    /// Whether this message ends the session, after which no more arrive.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::EndSession(_))
    }

    /// Parses a message from the JSON the server sent.
    ///
    /// Unrecognized `type` values become [`Unknown`] rather than an error, so a new
    /// server message does not break an existing client.
    ///
    /// [`Unknown`]: Message::Unknown
    pub(crate) fn from_json(raw: serde_json::Value) -> Result<Self, serde_json::Error> {
        let kind = raw
            .get("type")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned();

        // `audio_chunk` and `stop_recording` name both a client message and its
        // server-sent acknowledgement. Only the acknowledgement carries
        // `acknowledged`, which is what tells the two apart on the wire.
        let acknowledged = raw.get("acknowledged").is_some();

        let message = match (kind.as_str(), acknowledged) {
            ("start_session", _) => Self::StartSession(envelope(&raw)?),
            ("start_recording", _) => Self::StartRecording(envelope(&raw)?),
            ("end_session", _) => Self::EndSession(envelope(&raw)?),

            ("transcript", _) => Self::Transcript(payload(&raw)?),
            ("speech_start", _) => Self::SpeechStart(payload(&raw)?),
            ("speech_end", _) => Self::SpeechEnd(payload(&raw)?),
            ("translation", _) => Self::Translation(payload(&raw)?),
            ("named_entity_recognition", _) => Self::NamedEntityRecognition(payload(&raw)?),
            ("sentiment_analysis", _) => Self::SentimentAnalysis(payload(&raw)?),
            ("post_transcript", _) => Self::PostTranscript(payload(&raw)?),
            ("post_final_transcript", _) => Self::PostFinalTranscript(payload(&raw)?),
            ("post_chapterization", _) => Self::PostChapterization(payload(&raw)?),
            ("post_summarization", _) => Self::PostSummarization(payload(&raw)?),
            ("end_recording", _) => Self::EndRecording(payload(&raw)?),

            ("audio_chunk", true) => Self::AudioChunkAck(ack(&raw)?),
            ("stop_recording", true) => Self::StopRecordingAck(ack(&raw)?),

            _ => Self::Unknown { kind, raw },
        };

        Ok(message)
    }
}

/// Deserializes the shared envelope out of a raw message.
fn envelope(raw: &serde_json::Value) -> Result<Envelope, serde_json::Error> {
    serde_json::from_value(raw.clone())
}

/// Deserializes a data-carrying message.
fn payload(raw: &serde_json::Value) -> Result<Payload, serde_json::Error> {
    Ok(Payload {
        envelope: envelope(raw)?,
        data: raw.get("data").cloned().filter(|d| !d.is_null()),
        error: raw.get("error").cloned().filter(|e| !e.is_null()),
    })
}

/// Deserializes an acknowledgement.
fn ack(raw: &serde_json::Value) -> Result<Ack, serde_json::Error> {
    Ok(Ack {
        envelope: envelope(raw)?,
        acknowledged: raw
            .get("acknowledged")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or_default(),
        error: raw.get("error").cloned().filter(|e| !e.is_null()),
        data: raw.get("data").cloned().filter(|d| !d.is_null()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: serde_json::Value) -> Message {
        Message::from_json(value).unwrap()
    }

    #[test]
    fn a_transcript_carries_its_envelope_and_data() {
        let message = parse(serde_json::json!({
            "type": "transcript",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z",
            "data": { "utterance": { "text": "hello" }, "is_final": true }
        }));

        let Message::Transcript(payload) = &message else {
            panic!("expected a transcript, got {message:?}");
        };
        assert_eq!(payload.envelope.session_id, "sess_1");
        assert_eq!(payload.data.as_ref().unwrap()["utterance"]["text"], "hello");
        assert_eq!(message.session_id(), Some("sess_1"));
    }

    #[test]
    fn an_ack_is_distinguished_from_the_client_message_of_the_same_type() {
        // Both the client's audio chunk and the server's ack use `audio_chunk`; only
        // the ack carries `acknowledged`.
        let message = parse(serde_json::json!({
            "type": "audio_chunk",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z",
            "acknowledged": true
        }));

        let Message::AudioChunkAck(ack) = &message else {
            panic!("expected an ack, got {message:?}");
        };
        assert!(ack.acknowledged);
    }

    #[test]
    fn a_rejected_chunk_keeps_its_error() {
        let message = parse(serde_json::json!({
            "type": "audio_chunk",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z",
            "acknowledged": false,
            "error": { "message": "invalid encoding" }
        }));

        let Message::AudioChunkAck(ack) = &message else {
            panic!("expected an ack, got {message:?}");
        };
        assert!(!ack.acknowledged);
        assert_eq!(ack.error.as_ref().unwrap()["message"], "invalid encoding");
    }

    #[test]
    fn an_unknown_type_is_preserved_rather_than_failing() {
        let message = parse(serde_json::json!({
            "type": "some_future_message",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z",
            "data": { "field": 1 }
        }));

        let Message::Unknown { kind, raw } = &message else {
            panic!("expected an unknown message, got {message:?}");
        };
        assert_eq!(kind, "some_future_message");
        assert_eq!(raw["data"]["field"], 1);
        assert_eq!(message.session_id(), None);
    }

    #[test]
    fn end_session_is_terminal() {
        let end = parse(serde_json::json!({
            "type": "end_session",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z"
        }));
        assert!(end.is_terminal());

        let start = parse(serde_json::json!({
            "type": "start_session",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z"
        }));
        assert!(!start.is_terminal());
    }

    #[test]
    fn a_null_data_field_becomes_none() {
        let message = parse(serde_json::json!({
            "type": "end_recording",
            "session_id": "sess_1",
            "created_at": "2026-09-01T00:00:00.000Z",
            "data": null
        }));

        let Message::EndRecording(payload) = &message else {
            panic!("expected end_recording, got {message:?}");
        };
        assert!(payload.data.is_none());
    }
}
