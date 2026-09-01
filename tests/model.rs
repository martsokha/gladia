//! Guards for the decisions `cargo xtask codegen` makes about the spec.
//!
//! These are not a survey of the generated types, and they do not test `serde`. Each
//! one pins a spec quirk the codegen works around, so a regenerated model that loses
//! the workaround fails here rather than in a caller.

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
