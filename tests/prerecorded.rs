//! Endpoint tests for pre-recorded transcription, against an in-process HTTP server.
//!
//! These check what actually goes on the wire: method, route, query string, headers,
//! and body. The unit tests cannot, since they only see URLs. Every test points the
//! client at a [`MockServer`] rather than `api.gladia.io`, so nothing here touches the
//! network.

use std::time::Duration;

use gladia::model::{DiarizationConfigDto, InitTranscriptionRequest, PreRecordedResponseStatus};
use gladia::prerecorded::ListQuery;
use gladia::{Client, Error};
use uuid::Uuid;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A job id used across the tests.
const JOB_ID: &str = "6f2c0e1a-0000-4000-8000-000000000001";

/// Builds a client pointed at `server`.
fn client(server: &MockServer) -> Client {
    Client::builder()
        .with_api_key("test-key")
        .with_base_url(server.uri())
        // Retries would replay each request against the mock and slow failures down.
        .with_max_retries(0u32)
        .build()
        .unwrap()
}

/// A minimal job body in the given status.
fn job_body(status: &str) -> serde_json::Value {
    serde_json::json!({
        "id": JOB_ID,
        "request_id": "req_1",
        "version": 2,
        "status": status,
        "created_at": "2026-09-01T00:00:00.000Z",
        "kind": "pre-recorded",
        "post_session_metadata": {},
        "result": null,
        "file": null,
        "request_params": null
    })
}

#[tokio::test]
async fn upload_posts_multipart_with_the_api_key() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/upload"))
        .and(header("x-gladia-key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "audio_url": "https://api.gladia.io/file/abc",
            "audio_metadata": {
                "id": "7a1b0c2d-0000-4000-8000-0000000000ff",
                "filename": "meeting.wav",
                "extension": "wav",
                "size": 3,
                "audio_duration": 1.5,
                "number_of_channels": 1
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client(&server)
        .prerecorded()
        .upload_file("meeting.wav", &b"abc"[..])
        .await
        .unwrap();

    assert_eq!(response.audio_url, "https://api.gladia.io/file/abc");
    assert_eq!(response.audio_metadata.filename, "meeting.wav");

    // The multipart body must carry the file under the `audio` field, with the
    // filename the API reads the container type from.
    let request = &server.received_requests().await.unwrap()[0];
    let body = String::from_utf8_lossy(&request.body);
    assert!(body.contains(r#"name="audio""#), "field name: {body}");
    assert!(
        body.contains(r#"filename="meeting.wav""#),
        "filename: {body}"
    );

    let content_type = request
        .headers
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(
        content_type.starts_with("multipart/form-data"),
        "{content_type}"
    );
}

#[tokio::test]
async fn upload_url_posts_json() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/upload"))
        .and(body_json(serde_json::json!({
            "audio_url": "https://example.com/meeting.wav"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "audio_url": "https://api.gladia.io/file/abc",
            "audio_metadata": {
                "id": "7a1b0c2d-0000-4000-8000-0000000000ff",
                "filename": "meeting.wav",
                "extension": "wav",
                "size": 3,
                "audio_duration": 1.5,
                "number_of_channels": 1
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .prerecorded()
        .upload_url("https://example.com/meeting.wav")
        .await
        .unwrap();
}

#[tokio::test]
async fn init_posts_the_request_and_returns_the_job_id() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/pre-recorded"))
        .and(body_json(serde_json::json!({
            "audio_url": "https://api.gladia.io/file/abc"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": JOB_ID,
            "result_url": format!("https://api.gladia.io/v2/pre-recorded/{JOB_ID}")
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request: InitTranscriptionRequest = InitTranscriptionRequest::builder()
        .audio_url("https://api.gladia.io/file/abc")
        .try_into()
        .unwrap();

    let response = client(&server).prerecorded().init(&request).await.unwrap();

    assert_eq!(response.id, Uuid::parse_str(JOB_ID).unwrap());
}

#[tokio::test]
async fn get_fetches_a_job_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("processing")))
        .expect(1)
        .mount(&server)
        .await;

    let job = client(&server)
        .prerecorded()
        .get(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Processing);
    assert!(job.result.is_none());
}

#[tokio::test]
async fn list_sends_its_filters_as_query_parameters() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/pre-recorded"))
        .and(query_param("limit", "10"))
        .and(query_param("status", "done"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "first": "/v2/pre-recorded?offset=0",
            "current": "/v2/pre-recorded?offset=0",
            "next": "/v2/pre-recorded?offset=10",
            "items": [job_body("done")]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let query = ListQuery::default()
        .with_limit(10)
        .with_status([PreRecordedResponseStatus::Done]);

    let page = client(&server).prerecorded().list(&query).await.unwrap();

    assert_eq!(page.items.len(), 1);
}

#[tokio::test]
async fn delete_accepts_an_empty_202() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .prerecorded()
        .delete(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn file_returns_the_raw_bytes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}/file")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"RIFF....WAVE".to_vec()))
        .expect(1)
        .mount(&server)
        .await;

    let audio = client(&server)
        .prerecorded()
        .file(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap();

    assert_eq!(&audio[..4], b"RIFF");
}

#[tokio::test]
async fn an_api_error_carries_the_status_and_message() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "message": "job not found",
            "error": "not_found"
        })))
        .mount(&server)
        .await;

    let error = client(&server)
        .prerecorded()
        .get(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(404));
    let Error::Api { message, kind, .. } = &error else {
        panic!("expected an api error, got {error:?}");
    };
    assert_eq!(message, "job not found");
    assert_eq!(kind.as_deref(), Some("not_found"));
}

#[tokio::test]
async fn a_malformed_body_is_a_decode_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;

    let error = client(&server)
        .prerecorded()
        .get(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap_err();

    let Error::Decode { message, .. } = &error else {
        panic!("expected a decode error, got {error:?}");
    };
    // The body is echoed back, so a surprising payload is visible without a re-run.
    assert!(message.contains("not json"), "{message}");
}

#[tokio::test]
async fn submit_returns_a_handle_that_waits_for_completion() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/pre-recorded"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": JOB_ID,
            "result_url": format!("https://api.gladia.io/v2/pre-recorded/{JOB_ID}")
        })))
        .expect(1)
        .mount(&server)
        .await;

    // The job is queued on the first poll and done on the second, so `wait` has to
    // loop at least once rather than returning the first response it sees.
    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("queued")))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("done")))
        .expect(1)
        .mount(&server)
        .await;

    let request: InitTranscriptionRequest = InitTranscriptionRequest::builder()
        .audio_url("https://api.gladia.io/file/abc")
        .try_into()
        .unwrap();

    let handle = client(&server)
        .prerecorded()
        .submit(&request)
        .await
        .unwrap();
    assert_eq!(handle.id(), Uuid::parse_str(JOB_ID).unwrap());

    let job = handle
        .wait_with(Duration::from_millis(1), None)
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Done);
}

#[tokio::test]
async fn wait_returns_a_failed_job_rather_than_erroring() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("error")))
        .mount(&server)
        .await;

    // `error` is terminal: the request succeeded, so `wait` yields the job and lets
    // the caller decide what a failed transcription means.
    let job = client(&server)
        .prerecorded()
        .job(Uuid::parse_str(JOB_ID).unwrap())
        .wait()
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Error);
}

#[tokio::test]
async fn into_result_turns_a_failed_job_into_an_error() {
    let server = MockServer::start().await;

    let mut body = job_body("error");
    body["error_code"] = serde_json::json!(500);

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let error = client(&server)
        .prerecorded()
        .job(Uuid::parse_str(JOB_ID).unwrap())
        .into_result()
        .await
        .unwrap_err();

    let Error::Job { id, error_code } = &error else {
        panic!("expected a job error, got {error:?}");
    };
    assert_eq!(*id, Uuid::parse_str(JOB_ID).unwrap());
    assert_eq!(*error_code, Some(500));
}

#[tokio::test]
async fn waiting_past_the_deadline_times_out() {
    let server = MockServer::start().await;

    // Never finishes, so only the deadline can end the loop.
    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("processing")))
        .mount(&server)
        .await;

    let error = client(&server)
        .prerecorded()
        .job(Uuid::parse_str(JOB_ID).unwrap())
        .wait_with(Duration::from_millis(1), Some(Duration::from_millis(20)))
        .await
        .unwrap_err();

    let Error::Timeout { timeout } = &error else {
        panic!("expected a timeout, got {error:?}");
    };
    assert_eq!(*timeout, Duration::from_millis(20));
}

#[tokio::test]
async fn an_already_finished_job_returns_without_waiting() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("done")))
        .expect(1)
        .mount(&server)
        .await;

    // A long interval would stall the test if `wait_with` slept before its first poll.
    let job = client(&server)
        .prerecorded()
        .job(Uuid::parse_str(JOB_ID).unwrap())
        .wait_with(Duration::from_secs(300), None)
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Done);
}

#[tokio::test]
async fn transcribe_uploads_submits_and_waits() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/upload"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "audio_url": "https://api.gladia.io/file/abc",
            "audio_metadata": {
                "id": "7a1b0c2d-0000-4000-8000-0000000000ff",
                "filename": "meeting.wav",
                "extension": "wav",
                "size": 3,
                "audio_duration": 1.5,
                "number_of_channels": 1
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    // The init body must carry the uploaded URL and the configured features, with the
    // diarization flag set from the config rather than by the caller.
    Mock::given(method("POST"))
        .and(path("/v2/pre-recorded"))
        .and(body_json(serde_json::json!({
            "audio_url": "https://api.gladia.io/file/abc",
            "diarization": true,
            "diarization_config": { "max_speakers": 4 },
            "sentences": true
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": JOB_ID,
            "result_url": format!("https://api.gladia.io/v2/pre-recorded/{JOB_ID}")
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("done")))
        .expect(1)
        .mount(&server)
        .await;

    let job = client(&server)
        .prerecorded()
        .transcribe_file("meeting.wav", &b"abc"[..], |request| {
            request
                .with_diarization(
                    DiarizationConfigDto::builder()
                        .max_speakers(Some(4u64))
                        .try_into()
                        .unwrap(),
                )
                .with_sentences()
        })
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Done);
}

#[tokio::test]
async fn transcribe_url_skips_the_upload() {
    let server = MockServer::start().await;

    // No upload mock is mounted: reaching `/v2/upload` would fail the test.
    Mock::given(method("POST"))
        .and(path("/v2/pre-recorded"))
        .and(body_json(serde_json::json!({
            "audio_url": "https://example.com/meeting.wav"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": JOB_ID,
            "result_url": format!("https://api.gladia.io/v2/pre-recorded/{JOB_ID}")
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("done")))
        .mount(&server)
        .await;

    let job = client(&server)
        .prerecorded()
        .transcribe_url("https://example.com/meeting.wav", |request| request)
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Done);
}

#[tokio::test]
async fn a_failed_upload_is_not_retried() {
    let server = MockServer::start().await;

    // A 503 is transient, so the default retry strategy would replay it. Gladia's
    // POST endpoints create things and take no idempotency key, so a replay that
    // lands after the first attempt was processed leaves a duplicate.
    Mock::given(method("POST"))
        .and(path("/v2/upload"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder()
        .with_api_key("test-key")
        .with_base_url(server.uri())
        .with_max_retries(3u32)
        .build()
        .unwrap();

    let error = client
        .prerecorded()
        .upload_file("meeting.wav", &b"abc"[..])
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(503));
}

#[tokio::test]
async fn a_failed_get_is_retried() {
    let server = MockServer::start().await;

    // GET is idempotent, so a transient failure is worth replaying.
    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(2)
        .expect(2)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(job_body("done")))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder()
        .with_api_key("test-key")
        .with_base_url(server.uri())
        .with_max_retries(3u32)
        .build()
        .unwrap();

    let job = client
        .prerecorded()
        .get(Uuid::parse_str(JOB_ID).unwrap())
        .await
        .unwrap();

    assert_eq!(job.status, PreRecordedResponseStatus::Done);
}

#[tokio::test]
async fn a_slow_poll_cannot_outlast_the_deadline() {
    let server = MockServer::start().await;

    // The response takes far longer than the deadline allows, so the deadline has to
    // bound the in-flight request rather than only the gap between polls.
    Mock::given(method("GET"))
        .and(path(format!("/v2/pre-recorded/{JOB_ID}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(job_body("processing"))
                .set_delay(Duration::from_secs(30)),
        )
        .mount(&server)
        .await;

    let started = std::time::Instant::now();
    let error = client(&server)
        .prerecorded()
        .job(Uuid::parse_str(JOB_ID).unwrap())
        .wait_with(Duration::from_millis(1), Some(Duration::from_millis(50)))
        .await
        .unwrap_err();

    assert!(matches!(error, Error::Timeout { .. }), "{error:?}");
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "the deadline did not bound the request: {:?}",
        started.elapsed()
    );
}
