//! A live transcription session: a [`Stream`] of messages and a handle for sending
//! audio.

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, Stream, StreamExt};
use reqwest_websocket::{Message as WsMessage, WebSocket};

use super::message::Message;
use crate::error::{Error, Result};

/// The socket `reqwest-websocket` hands back once a request has been upgraded.
type Socket = WebSocket;

/// An open live transcription session.
///
/// Yields [`Message`]s as a [`Stream`]. Audio goes the other way, through an
/// [`AudioSender`] taken with [`sender`]. Splitting the two means audio can be pumped
/// from one task while transcripts are consumed in another, which is the usual shape
/// for live transcription:
///
/// ```no_run
/// # use futures_util::StreamExt;
/// # use gladia::live::{Message, Session};
/// # use gladia::prelude::*;
/// # async fn run(mut session: Session, chunks: Vec<Vec<u8>>) -> Result<()> {
/// let mut audio = session.sender();
/// tokio::spawn(async move {
///     for chunk in chunks {
///         audio.send(chunk).await?;
///     }
///     audio.finish().await
/// });
///
/// while let Some(message) = session.next().await {
///     if let Message::Transcript(transcript) = message? {
///         println!("{:?}", transcript.data);
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
/// The stream ends when the server sends [`Message::EndSession`] or the socket closes.
///
/// [`sender`]: Session::sender
#[derive(Debug)]
pub struct Session {
    id: String,
    incoming: SplitStream<Socket>,
    outgoing: Option<SplitSink<Socket, WsMessage>>,
    finished: bool,
}

/// Sends audio into a live session.
///
/// Taken from [`Session::sender`], and independent of the session's stream, so it can
/// be moved into another task. Dropping it does not end the session; call [`finish`]
/// to tell the server no more audio is coming.
///
/// [`finish`]: AudioSender::finish
#[derive(Debug)]
pub struct AudioSender {
    outgoing: SplitSink<Socket, WsMessage>,
}

impl Session {
    /// Wraps a connected socket. Called by [`Live::start`].
    ///
    /// [`Live::start`]: super::Live::start
    pub(crate) fn new(id: String, socket: Socket) -> Self {
        let (outgoing, incoming) = socket.split();

        Self {
            id,
            incoming,
            outgoing: Some(outgoing),
            finished: false,
        }
    }

    /// The session's id, as returned when it was created.
    ///
    /// The same id identifies the session's recording through
    /// [`Live::get`](super::Live::get) once it has ended.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Takes the handle for sending audio.
    ///
    /// Available once per session: the first call returns the sender, and subsequent
    /// calls panic, since two senders on one socket would interleave frames.
    ///
    /// # Panics
    ///
    /// If the sender has already been taken.
    #[track_caller]
    pub fn sender(&mut self) -> AudioSender {
        AudioSender {
            outgoing: self
                .outgoing
                .take()
                .expect("the audio sender has already been taken from this session"),
        }
    }
}

impl AudioSender {
    /// Sends a chunk of audio.
    ///
    /// The chunk goes as a binary frame, in the encoding, sample rate, and channel
    /// count the session was configured with. Frames are sent as-is, so a caller
    /// choosing chunk sizes should follow Gladia's guidance on chunk duration.
    pub async fn send(&mut self, chunk: impl Into<bytes::Bytes>) -> Result<()> {
        self.outgoing
            .send(WsMessage::Binary(chunk.into()))
            .await
            .map_err(|e| Error::live("could not send audio", e))
    }

    /// Tells the server that recording has stopped, so it can finish processing.
    ///
    /// The session stays open: the server still sends the post-processing messages
    /// (the final transcript, any summary) before [`Message::EndSession`]. Keep reading
    /// the session's stream until it ends.
    pub async fn stop_recording(&mut self) -> Result<()> {
        let stop = serde_json::json!({ "type": "stop_recording" });

        self.outgoing
            .send(WsMessage::Text(stop.to_string()))
            .await
            .map_err(|e| Error::live("could not stop recording", e))
    }

    /// Tells the server no more audio is coming, and gives up the sender.
    ///
    /// The same request as [`stop_recording`], but it consumes the sender, so it reads
    /// as the end of the sending half at a call site that has nothing more to send.
    ///
    /// The session stays open either way. WebSocket has no half-close: a Close frame
    /// ends the connection in both directions, and a peer that receives one discards
    /// anything still in flight, so sending one here would drop the final transcript
    /// and any summary. The server closes the session itself once post-processing is
    /// done, which arrives as [`Message::EndSession`]. Keep reading the session's
    /// stream until it ends.
    ///
    /// [`stop_recording`]: Self::stop_recording
    pub async fn finish(mut self) -> Result<()> {
        self.stop_recording().await
    }
}

impl Stream for Session {
    type Item = Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Once the server has ended the session, stop polling: further frames are not
        // expected, and a `None` here is what ends a `while let` loop.
        if self.finished {
            return Poll::Ready(None);
        }

        loop {
            let frame = match self.incoming.poll_next_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Ready(Some(Ok(frame))) => frame,
                Poll::Ready(Some(Err(e))) => {
                    self.finished = true;
                    return Poll::Ready(Some(Err(Error::live("the session failed", e))));
                }
            };

            let text = match frame {
                WsMessage::Text(text) => text,
                // The server can send a JSON message as a binary frame.
                WsMessage::Binary(bytes) => match String::from_utf8(bytes.to_vec()) {
                    Ok(text) => text,
                    Err(e) => {
                        return Poll::Ready(Some(Err(Error::decode(
                            "the server sent a non-utf8 frame",
                            e,
                        ))));
                    }
                },
                WsMessage::Close { .. } => {
                    self.finished = true;
                    return Poll::Ready(None);
                }
                // Ping/Pong are handled by the transport, and carry nothing to report.
                _ => continue,
            };

            let value = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(value) => value,
                Err(e) => {
                    return Poll::Ready(Some(Err(Error::decode(
                        format!("the server sent a malformed message: {text}"),
                        e,
                    ))));
                }
            };

            return Poll::Ready(Some(match Message::from_json(value) {
                Ok(message) => {
                    if message.is_terminal() {
                        self.finished = true;
                    }
                    Ok(message)
                }
                Err(e) => Err(Error::decode("could not parse a live message", e)),
            }));
        }
    }
}
