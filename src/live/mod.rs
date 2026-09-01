//! Live transcription: stream audio over a WebSocket, receive transcripts as they
//! are produced.
//!
//! Reached through [`Client::live`]. Creating a session is two steps, which [`start`]
//! does together: `POST /v2/live` returns a session id and a socket URL, and the
//! session begins when that socket is connected.
//!
//! | Method | Endpoint |
//! | --- | --- |
//! | [`start`] | `POST /v2/live`, then the returned WebSocket |
//! | [`init`] | `POST /v2/live` |
//! | [`connect`] | the WebSocket from [`init`] |
//! | [`get`] | `GET /v2/live/{id}` |
//! | [`list`] | `GET /v2/live` |
//! | [`delete`] | `DELETE /v2/live/{id}` |
//! | [`file`] | `GET /v2/live/{id}/file` |
//!
//! A [`Session`] is a [`Stream`](futures_util::Stream) of [`Message`]s, with audio
//! sent through an [`AudioSender`] taken from it. Once a session ends, its recording is
//! addressable by id like a pre-recorded job.
//!
//! [`Client::live`]: crate::Client::live
//! [`start`]: Live::start
//! [`init`]: Live::init
//! [`connect`]: Live::connect
//! [`get`]: Live::get
//! [`list`]: Live::list
//! [`delete`]: Live::delete
//! [`file`]: Live::file

mod message;
mod session;

use bytes::Bytes;
use reqwest_websocket::Upgrade as _;
use uuid::Uuid;

pub use self::message::{Ack, Envelope, Message, Payload};
pub use self::session::{AudioSender, Session};
use crate::client::Client;
use crate::error::{Error, Result};
use crate::model::{
    InitStreamingResponse, ListStreamingResponse, StreamingRequest, StreamingResponse,
};
use crate::prerecorded::ListQuery;

/// The live transcription endpoints.
///
/// Obtained from [`Client::live`]. Borrows the client, so it is free to create.
///
/// [`Client::live`]: crate::Client::live
#[derive(Debug, Clone, Copy)]
pub struct Live<'a> {
    client: &'a Client,
}

/// The route prefix shared by every live endpoint.
const ROUTE: &str = "v2/live";

impl<'a> Live<'a> {
    /// Wraps a client. Called by [`Client::live`].
    ///
    /// [`Client::live`]: crate::Client::live
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Opens a live session: creates it, then connects to its socket.
    ///
    /// ```no_run
    /// # use futures_util::StreamExt;
    /// # use gladia::live::Message;
    /// # use gladia::model::StreamingRequest;
    /// # use gladia::prelude::*;
    /// # async fn run(client: Client, request: StreamingRequest) -> Result<()> {
    /// let mut session = client.live().start(&request).await?;
    /// let mut audio = session.sender();
    ///
    /// audio.send(vec![0u8; 3200]).await?;
    /// audio.finish().await?;   // stops recording; the session stays open
    ///
    /// while let Some(message) = session.next().await {
    ///     println!("{:?}", message?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The request carries the audio format (encoding, sample rate, bit depth, and
    /// channel count), which must match the audio actually sent. Use [`init`] and
    /// [`connect`] separately to inspect the session id or defer connecting.
    ///
    /// [`init`]: Self::init
    /// [`connect`]: Self::connect
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, request), err))]
    pub async fn start(&self, request: &StreamingRequest) -> Result<Session> {
        let created = self.init(request).await?;

        self.connect(&created).await
    }

    /// Creates a live session without connecting to it.
    ///
    /// Returns the session id and the socket URL, which [`connect`] then opens. The URL
    /// embeds a single-use token, so it should be connected promptly.
    ///
    /// `POST /v2/live`
    ///
    /// [`connect`]: Self::connect
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, request), err))]
    pub async fn init(&self, request: &StreamingRequest) -> Result<InitStreamingResponse> {
        let req = self.client.post(ROUTE)?.json(request);

        self.client.send_json(req).await
    }

    /// Connects to a session created by [`init`].
    ///
    /// [`init`]: Self::init
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, created), err))]
    pub async fn connect(&self, created: &InitStreamingResponse) -> Result<Session> {
        // The socket is an upgraded request from the client's middleware stack, so the
        // handshake carries its TLS backend, proxy, default headers, and middleware.
        let socket = self
            .client
            .as_client_with_middleware()
            .get(created.url.as_str())
            .upgrade()
            .send()
            .await
            .map_err(|e| Error::live("could not open the live session", e))?
            .into_websocket()
            .await
            .map_err(|e| Error::live("could not upgrade to a websocket", e))?;

        Ok(Session::new(created.id.to_string(), socket))
    }

    /// Fetches a finished live session's recording and transcript.
    ///
    /// `GET /v2/live/{id}`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn get(&self, id: Uuid) -> Result<StreamingResponse> {
        let req = self.client.get(&format!("{ROUTE}/{id}"))?;

        self.client.send_json(req).await
    }

    /// Lists live sessions, most recent first.
    ///
    /// Takes the same [`ListQuery`] as the pre-recorded endpoint.
    ///
    /// `GET /v2/live`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn list(&self, query: &ListQuery) -> Result<ListStreamingResponse> {
        let mut url = self.client.route_url(ROUTE)?;
        query.apply(&mut url);

        let req = self.client.as_client_with_middleware().get(url);

        self.client.send_json(req).await
    }

    /// Deletes a live session and its stored audio.
    ///
    /// `DELETE /v2/live/{id}`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let req = self.client.delete(&format!("{ROUTE}/{id}"))?;
        self.client.send(req).await?;

        Ok(())
    }

    /// Downloads the audio recorded during a live session.
    ///
    /// `GET /v2/live/{id}/file`
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn file(&self, id: Uuid) -> Result<Bytes> {
        let req = self.client.get(&format!("{ROUTE}/{id}/file"))?;
        let resp = self.client.send(req).await?;

        resp.bytes().await.map_err(|e| Error::Transport(e.into()))
    }
}
