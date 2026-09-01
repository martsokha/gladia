//! Round-trip tests for the generated wire types.
//!
//! These guard the two places the spec is known to be awkward, so a regenerated model
//! that breaks either one fails here rather than in a caller's deserialization:
//!
//! - The spec declares OpenAPI 3.1 but uses 3.0's `nullable: true` in 58 places. If a
//!   future generator ignores it (3.1 has no such keyword), nullable fields become
//!   non-optional and every queued job fails to deserialize.
//! - Most request toggles carry `default: false`. If the codegen's default-stripping
//!   stops working, requests serialize a dozen-odd explicit `false`s.

use gladia::model::*;

/// The shape returned immediately after init: queued, with every result field null.
#[test]
fn a_queued_job_deserializes_with_null_results() {
    let body = serde_json::json!({
        "id": "6f2c0e1a-0000-4000-8000-000000000001",
        "request_id": "req_1",
        "version": 2,
        "status": "queued",
        "created_at": "2026-09-01T00:00:00.000Z",
        "kind": "pre-recorded",
        "post_session_metadata": {},
        "result": null,
        "file": null,
        "request_params": null
    });

    let job: PreRecordedResponse = serde_json::from_value(body).unwrap();

    assert!(job.result.is_none());
    assert!(job.file.is_none());
    assert!(job.completed_at.is_none());
    assert_eq!(job.status, PreRecordedResponseStatus::Queued);
}

/// A terminal job carries a result; the status enum covers every documented state.
#[test]
fn job_statuses_deserialize() {
    let states = [
        ("queued", PreRecordedResponseStatus::Queued),
        ("processing", PreRecordedResponseStatus::Processing),
        ("done", PreRecordedResponseStatus::Done),
        ("error", PreRecordedResponseStatus::Error),
    ];

    for (wire, expected) in states {
        let parsed: PreRecordedResponseStatus =
            serde_json::from_value(serde_json::json!(wire)).unwrap();
        assert_eq!(parsed, expected, "status {wire}");
    }
}

/// The init response is what a submission returns: an id and a URL to poll.
#[test]
fn the_init_response_deserializes() {
    let body = serde_json::json!({
        "id": "6f2c0e1a-0000-4000-8000-000000000001",
        "result_url": "https://api.gladia.io/v2/pre-recorded/6f2c0e1a-0000-4000-8000-000000000001"
    });

    let response: InitPreRecordedTranscriptionResponse = serde_json::from_value(body).unwrap();

    assert_eq!(
        response.result_url.to_string(),
        "https://api.gladia.io/v2/pre-recorded/6f2c0e1a-0000-4000-8000-000000000001"
    );
}

/// A request must serialize only the fields the caller set. The spec gives the feature
/// toggles `default: false`; if those defaults survive into the generated types, every
/// request carries them explicitly.
#[test]
fn a_request_serializes_only_what_was_set() {
    let request: InitTranscriptionRequest = InitTranscriptionRequest::builder()
        .audio_url("https://files.gladia.io/example/audio.wav")
        .try_into()
        .unwrap();

    let value = serde_json::to_value(&request).unwrap();
    let fields = value.as_object().unwrap();

    assert_eq!(
        fields.keys().collect::<Vec<_>>(),
        vec!["audio_url"],
        "unset fields leaked onto the wire"
    );
}

/// A feature toggle and its config serialize together, and nothing else comes along.
#[test]
fn a_request_carries_a_feature_config() {
    let request: InitTranscriptionRequest = InitTranscriptionRequest::builder()
        .audio_url("https://files.gladia.io/example/audio.wav")
        .diarization(Some(true))
        .diarization_config(Some(
            DiarizationConfigDto::builder()
                .max_speakers(Some(4u64))
                .try_into()
                .unwrap(),
        ))
        .try_into()
        .unwrap();

    let value = serde_json::to_value(&request).unwrap();

    assert_eq!(value["diarization"], serde_json::json!(true));
    assert_eq!(
        value["diarization_config"]["max_speakers"],
        serde_json::json!(4)
    );
    assert_eq!(value.as_object().unwrap().len(), 3);
}

/// Enum values that are not valid Rust identifiers keep their wire spelling.
#[test]
fn awkward_enum_values_round_trip() {
    let encoding: StreamingSupportedEncodingEnum =
        serde_json::from_value(serde_json::json!("wav/pcm")).unwrap();
    assert_eq!(encoding, StreamingSupportedEncodingEnum::WavPcm);
    assert_eq!(
        serde_json::to_value(encoding).unwrap(),
        serde_json::json!("wav/pcm")
    );

    let model: TranscriptionSupportedModels =
        serde_json::from_value(serde_json::json!("solaria-1")).unwrap();
    assert_eq!(
        serde_json::to_value(model).unwrap(),
        serde_json::json!("solaria-1")
    );
}

/// `AudioUploadMetadataDTO.extension` is declared `format: uuid` in the upstream spec,
/// contradicted by its own example (`"wav"`). The codegen patches that out; without the
/// patch every upload response fails to deserialize.
#[test]
fn an_upload_response_deserializes_despite_the_spec_bug() {
    let body = serde_json::json!({
        "audio_url": "https://api.gladia.io/file/abc",
        "audio_metadata": {
            "id": "7a1b0c2d-0000-4000-8000-0000000000ff",
            "filename": "meeting.wav",
            "extension": "wav",
            "size": 365702,
            "audio_duration": 4.145782,
            "number_of_channels": 1
        }
    });

    let upload: AudioUploadResponse = serde_json::from_value(body).unwrap();

    assert_eq!(upload.audio_metadata.extension, "wav");
    assert_eq!(upload.audio_metadata.number_of_channels, 1);
}
