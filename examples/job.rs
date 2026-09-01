//! Submits a transcription without waiting, then resumes it from the job id.
//!
//! Transcription jobs outlive the process that submitted them, so a long job can be
//! submitted in one run and collected in another.
//!
//! ```sh
//! GLADIA_API_KEY=... cargo run --example job -- submit https://example.com/audio.wav
//! GLADIA_API_KEY=... cargo run --example job -- collect <job-id>
//! ```

use std::time::Duration;

use gladia::model::PreRecordedResponseStatus;
use gladia::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("GLADIA_API_KEY").expect("GLADIA_API_KEY must be set");
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_default();
    let argument = args.next().unwrap_or_default();

    let client = Client::builder().with_api_key(api_key).build()?;
    let prerecorded = client.prerecorded();

    match command.as_str() {
        "submit" => {
            let request = TranscriptionRequest::new(argument).with_sentences().build();
            let handle = prerecorded.submit(&request).await?;

            println!("{}", handle.id());
            println!(
                "collect it with: cargo run --example job -- collect {}",
                handle.id()
            );
        }
        "collect" => {
            let id: Uuid = argument.parse().expect("a job id");

            let job = prerecorded
                .job(id)
                .wait_with(Duration::from_secs(5), Some(Duration::from_secs(600)))
                .await?;

            // `wait_with` returns a failed job like any other, so the status decides
            // whether there is a result to print. `into_result` would raise it as an
            // error instead, at the cost of the polling interval chosen here.
            if job.status == PreRecordedResponseStatus::Error {
                eprintln!("transcription failed (error_code {:?})", job.error_code);
                return Ok(());
            }

            println!("{}", serde_json::to_string_pretty(&job.result).unwrap());
        }
        _ => eprintln!("usage: job <submit <audio-url> | collect <job-id>>"),
    }

    Ok(())
}
