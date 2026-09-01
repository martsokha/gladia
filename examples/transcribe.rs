//! Transcribes a local audio file, printing the transcript when it finishes.
//!
//! ```sh
//! GLADIA_API_KEY=... cargo run --example transcribe -- meeting.wav
//! ```

use gladia::model::PreRecordedResponseStatus;
use gladia::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("GLADIA_API_KEY").expect("GLADIA_API_KEY must be set");
    let path = std::env::args()
        .nth(1)
        .expect("usage: transcribe <audio file>");

    let client = Client::builder().with_api_key(api_key).build()?;

    let audio = std::fs::read(&path).expect("could not read the audio file");
    let filename = std::path::Path::new(&path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("audio.wav");

    println!("transcribing {filename} ({} bytes)…", audio.len());

    // Uploads, submits, and polls every three seconds until the job finishes.
    let job = client
        .prerecorded()
        .transcribe_file(filename, audio, |request| {
            request.with_sentences().with_diarization_default()
        })
        .await?;

    if job.status == PreRecordedResponseStatus::Error {
        eprintln!("transcription failed (error_code {:?})", job.error_code);
        return Ok(());
    }

    let result = job.result.expect("a completed job carries a result");
    println!("\n{}", serde_json::to_string_pretty(&result).unwrap());

    Ok(())
}
