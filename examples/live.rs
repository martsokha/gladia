//! Streams a WAV file to Gladia as if it were live audio, printing transcripts.
//!
//! ```sh
//! GLADIA_API_KEY=... cargo run --features live --example live -- speech.wav
//! ```
//!
//! The file must be 16-bit PCM at 16 kHz, mono, matching the format the session is
//! configured for below. Audio is sent in 100 ms chunks, paced in real time, which is
//! what a microphone would produce.

use std::num::NonZeroU64;
use std::time::Duration;

use futures_util::StreamExt;
use gladia::live::Message;
use gladia::model::{
    StreamingRequest, StreamingSupportedBitDepthEnum, StreamingSupportedEncodingEnum,
    StreamingSupportedSampleRateEnum,
};
use gladia::prelude::*;

/// Bytes per 100 ms of 16-bit mono PCM at 16 kHz.
const CHUNK: usize = 16_000 * 2 / 10;
/// The WAV header this example skips; it sends raw PCM, not a container.
const WAV_HEADER: usize = 44;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("GLADIA_API_KEY").expect("GLADIA_API_KEY must be set");
    let path = std::env::args()
        .nth(1)
        .expect("usage: live <16kHz mono wav>");

    let client = Client::builder().with_api_key(api_key).build()?;

    // The numeric formats are validating newtypes: the spec types them as `number`
    // with an enum of allowed values, so they are built with `try_from`.
    let request: StreamingRequest = StreamingRequest::builder()
        .encoding(Some(StreamingSupportedEncodingEnum::WavPcm))
        .bit_depth(Some(
            StreamingSupportedBitDepthEnum::try_from(16.0).unwrap(),
        ))
        .sample_rate(Some(
            StreamingSupportedSampleRateEnum::try_from(16_000.0).unwrap(),
        ))
        .channels(NonZeroU64::new(1))
        .try_into()
        .expect("a valid streaming request");

    let mut session = client.live().start(&request).await?;
    println!("session {} open", session.id());

    // Pump audio from its own task, so transcripts are read as they arrive rather
    // than only after everything has been sent.
    let audio = std::fs::read(&path).expect("could not read the audio file");
    let mut sender = session.sender();

    tokio::spawn(async move {
        // `Bytes` slices share the buffer, so each chunk is a refcount bump rather
        // than a copy, and the task owns the audio for as long as it needs it.
        let pcm = bytes::Bytes::from(audio);
        let pcm = pcm.slice(WAV_HEADER.min(pcm.len())..);

        for chunk in pcm.chunks(CHUNK) {
            if let Err(e) = sender.send(pcm.slice_ref(chunk)).await {
                eprintln!("failed to send audio: {e}");
                return;
            }
            // Pace the send to match playback, as a microphone would.
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Stops recording; the server closes the session once post-processing is done.
        if let Err(e) = sender.finish().await {
            eprintln!("failed to stop recording: {e}");
        }
    });

    // Runs until the server closes the session.
    while let Some(message) = session.next().await {
        match message? {
            Message::Transcript(transcript) => {
                if let Some(data) = &transcript.data {
                    println!("{data}");
                }
            }
            Message::PostFinalTranscript(final_transcript) => {
                println!("\nfinal:\n{:#?}", final_transcript.data);
            }
            Message::EndSession(_) => println!("\nsession ended"),
            other => println!("  [{other:?}]"),
        }
    }

    Ok(())
}
