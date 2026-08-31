//! Builds a client from the `GLADIA_API_KEY` environment variable.
//!
//! ```sh
//! GLADIA_API_KEY=... cargo run --example client
//! ```

use std::time::Duration;

use gladia::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("GLADIA_API_KEY").expect("GLADIA_API_KEY must be set");

    let client = Client::builder()
        .with_api_key(api_key)
        .with_timeout(Duration::from_secs(30))
        .with_max_retries(3u32)
        .with_user_agent(concat!("gladia-rs/", env!("CARGO_PKG_VERSION")))
        .build()?;

    println!("client ready, targeting {}", client.base_url());
    Ok(())
}
