//! Pre-recorded transcription: upload a file, submit a job, read the result.
//!
//! Reached through [`Client::prerecorded`]. The endpoints map one-to-one onto the
//! API, and each returns a generated type from [`crate::model`]:
//!
//! | Method | Endpoint |
//! | --- | --- |
//! | [`upload`] | `POST /v2/upload` |
//! | [`upload_url`] | `POST /v2/upload` |
//! | [`init`] / [`submit`] | `POST /v2/pre-recorded` |
//! | [`get`] | `GET /v2/pre-recorded/{id}` |
//! | [`list`] | `GET /v2/pre-recorded` |
//! | [`delete`] | `DELETE /v2/pre-recorded/{id}` |
//! | [`file`] | `GET /v2/pre-recorded/{id}/file` |
//!
//! A transcription is asynchronous: [`init`] returns as soon as the job is accepted,
//! and the result has to be fetched with [`get`] once the job reaches a terminal
//! status. [`submit`] wraps that up: it returns a [`JobHandle`] whose [`wait`] polls
//! until the job finishes.
//!
//! [`wait`]: JobHandle::wait
//!
//! [`Client::prerecorded`]: crate::Client::prerecorded
//! [`upload`]: PreRecorded::upload
//! [`upload_url`]: PreRecorded::upload_url
//! [`init`]: PreRecorded::init
//! [`submit`]: PreRecorded::submit
//! [`get`]: PreRecorded::get
//! [`list`]: PreRecorded::list
//! [`delete`]: PreRecorded::delete
//! [`file`]: PreRecorded::file

mod handle;
mod list;
mod request;

use bytes::Bytes;
use uuid::Uuid;

pub use self::handle::JobHandle;
pub use self::list::ListQuery;
pub use self::request::TranscriptionRequest;
use crate::client::Client;
use crate::error::Result;
use crate::model::{
    AudioUploadResponse, InitPreRecordedTranscriptionResponse, InitTranscriptionRequest,
    ListPreRecordedResponse, PreRecordedResponse,
};

/// The pre-recorded transcription endpoints.
///
/// Obtained from [`Client::prerecorded`]. Borrows the client, so it is free to create
/// and is normally used inline rather than stored.
///
/// [`Client::prerecorded`]: crate::Client::prerecorded
#[derive(Debug, Clone, Copy)]
pub struct PreRecorded<'a> {
    client: &'a Client,
}

/// The route prefix shared by every pre-recorded endpoint.
const ROUTE: &str = "v2/pre-recorded";

impl<'a> PreRecorded<'a> {
    /// Wraps a client. Called by [`Client::prerecorded`].
    ///
    /// [`Client::prerecorded`]: crate::Client::prerecorded
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Uploads audio, returning the URL to pass to [`init`].
    ///
    /// `filename` is sent as the multipart filename; the API uses its extension to
    /// identify the container, so it should carry one (`meeting.wav`, not `meeting`).
    ///
    /// For audio already reachable by URL, skip the upload and pass that URL to
    /// [`init`] directly, or use [`upload_url`] to register it first.
    ///
    /// `POST /v2/upload`
    ///
    /// [`init`]: Self::init
    /// [`upload_url`]: Self::upload_url
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, audio), err))]
    pub async fn upload(
        &self,
        filename: impl Into<String> + std::fmt::Debug,
        audio: impl Into<Bytes>,
    ) -> Result<AudioUploadResponse> {
        let part = reqwest::multipart::Part::bytes(audio.into().to_vec())
            .file_name(filename.into())
            // The API dispatches on the extension in the filename, but a part with no
            // content type is rejected outright, so send the generic binary type.
            .mime_str("application/octet-stream")
            .map_err(|e| crate::Error::invalid_request("invalid multipart part", e))?;

        let form = reqwest::multipart::Form::new().part("audio", part);
        let req = self.client.post("v2/upload")?.multipart(form);

        self.client.send_json(req).await
    }

    /// Registers audio that is already reachable by URL, returning Gladia's own URL
    /// for it along with the extracted metadata.
    ///
    /// This is only needed for the metadata, or to have Gladia fetch the file once and
    /// reuse it: [`init`] accepts an external URL directly.
    ///
    /// `POST /v2/upload`
    ///
    /// [`init`]: Self::init
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn upload_url(
        &self,
        audio_url: impl Into<String> + std::fmt::Debug,
    ) -> Result<AudioUploadResponse> {
        let body = serde_json::json!({ "audio_url": audio_url.into() });
        let req = self.client.post("v2/upload")?.json(&body);

        self.client.send_json(req).await
    }

    /// Submits a transcription job, returning its id and result URL.
    ///
    /// The job is queued, not finished: fetch the result with [`get`] once it reaches
    /// a terminal status.
    ///
    /// `POST /v2/pre-recorded`
    ///
    /// [`get`]: Self::get
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, request), err))]
    pub async fn init(
        &self,
        request: &InitTranscriptionRequest,
    ) -> Result<InitPreRecordedTranscriptionResponse> {
        let req = self.client.post(ROUTE)?.json(request);

        self.client.send_json(req).await
    }

    /// Submits a transcription job and returns a handle for awaiting it.
    ///
    /// The same request as [`init`], but the response is wrapped in a [`JobHandle`],
    /// so the result is fetched with [`wait`] rather than by polling [`get`] by hand:
    ///
    /// ```no_run
    /// # use gladia::model::InitTranscriptionRequest;
    /// # use gladia::prelude::*;
    /// # async fn run(client: Client, request: InitTranscriptionRequest) -> Result<()> {
    /// let job = client.prerecorded().submit(&request).await?.wait().await?;
    /// # let _ = job;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// `POST /v2/pre-recorded`
    ///
    /// [`init`]: Self::init
    /// [`get`]: Self::get
    /// [`wait`]: JobHandle::wait
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, request), err))]
    pub async fn submit(&self, request: &InitTranscriptionRequest) -> Result<JobHandle> {
        let submitted = self.init(request).await?;

        Ok(JobHandle::new(self.client.clone(), submitted.id))
    }

    /// Uploads audio, transcribes it, and waits for the result.
    ///
    /// The whole flow in one call: [`upload`], then [`submit`], then [`wait`]. For
    /// audio already reachable by URL, use [`transcribe_url`] and skip the upload.
    ///
    /// ```no_run
    /// # use gladia::prelude::*;
    /// # async fn run(client: Client) -> Result<()> {
    /// let audio = std::fs::read("meeting.wav").expect("readable audio file");
    /// let job = client
    ///     .prerecorded()
    ///     .transcribe("meeting.wav", audio, |request| request.with_sentences())
    ///     .await?;
    /// # let _ = job;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// `configure` receives a [`TranscriptionRequest`] already pointed at the uploaded
    /// audio, so it only adds the features wanted. Pass
    /// <code>|request| request</code> for a plain transcription.
    ///
    /// A job that finishes in the `error` status is returned as `Ok`, like [`wait`].
    /// Use [`submit`] and [`JobHandle::into_result`] to treat that as an error instead.
    /// For control over polling, use [`submit`] and [`JobHandle::wait_with`].
    ///
    /// [`upload`]: Self::upload
    /// [`submit`]: Self::submit
    /// [`wait`]: JobHandle::wait
    /// [`transcribe_url`]: Self::transcribe_url
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, audio, configure), err)
    )]
    pub async fn transcribe(
        &self,
        filename: impl Into<String> + std::fmt::Debug,
        audio: impl Into<Bytes>,
        configure: impl FnOnce(TranscriptionRequest) -> TranscriptionRequest,
    ) -> Result<PreRecordedResponse> {
        let upload = self.upload(filename, audio).await?;

        self.transcribe_url(upload.audio_url, configure).await
    }

    /// Transcribes audio already reachable by URL, and waits for the result.
    ///
    /// Like [`transcribe`], without the upload: `audio_url` is either one returned by
    /// [`upload`] or any URL Gladia can fetch.
    ///
    /// [`transcribe`]: Self::transcribe
    /// [`upload`]: Self::upload
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, configure), err))]
    pub async fn transcribe_url(
        &self,
        audio_url: impl Into<String> + std::fmt::Debug,
        configure: impl FnOnce(TranscriptionRequest) -> TranscriptionRequest,
    ) -> Result<PreRecordedResponse> {
        let request = configure(TranscriptionRequest::new(audio_url)).build();

        self.submit(&request).await?.wait().await
    }

    /// Returns a handle to an already-submitted job.
    ///
    /// For resuming from a stored id: jobs outlive the process that submitted them.
    /// Nothing is fetched until the handle is polled.
    pub fn job(&self, id: Uuid) -> JobHandle {
        JobHandle::new(self.client.clone(), id)
    }

    /// Fetches a transcription job by id.
    ///
    /// The `result` field is populated once `status` is
    /// [`Done`](crate::model::PreRecordedResponseStatus::Done).
    ///
    /// `GET /v2/pre-recorded/{id}`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn get(&self, id: Uuid) -> Result<PreRecordedResponse> {
        let req = self.client.get(&format!("{ROUTE}/{id}"))?;

        self.client.send_json(req).await
    }

    /// Lists transcription jobs, most recent first.
    ///
    /// Filters and pagination are set on [`ListQuery`]; pass
    /// [`ListQuery::default()`](Default::default) for the first page with the API's
    /// own defaults.
    ///
    /// `GET /v2/pre-recorded`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn list(&self, query: &ListQuery) -> Result<ListPreRecordedResponse> {
        let mut url = self.client.route_url(ROUTE)?;
        query.apply(&mut url);

        let req = self.client.as_client_with_middleware().get(url);

        self.client.send_json(req).await
    }

    /// Deletes a transcription job and its stored audio.
    ///
    /// `DELETE /v2/pre-recorded/{id}`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let req = self.client.delete(&format!("{ROUTE}/{id}"))?;
        self.client.send(req).await?;

        Ok(())
    }

    /// Downloads the audio stored for a job.
    ///
    /// `GET /v2/pre-recorded/{id}/file`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn file(&self, id: Uuid) -> Result<Bytes> {
        let req = self.client.get(&format!("{ROUTE}/{id}/file"))?;
        let resp = self.client.send(req).await?;

        resp.bytes()
            .await
            .map_err(|e| crate::Error::Transport(e.into()))
    }
}
