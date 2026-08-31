//! The [`ClientBuilder`].

use std::time::Duration;

use crate::client::{Client, Headers};
use crate::error::Result;

/// A builder for [`Client`].
///
/// Obtain one via [`Client::builder`]. An API key is required; every other field falls
/// back to a default, noted on the corresponding setter.
///
/// ```no_run
/// use gladia::prelude::*;
///
/// # fn run() -> Result<()> {
/// let client = Client::builder()
///     .with_api_key("gladia-api-key")
///     .with_timeout(std::time::Duration::from_secs(30))
///     .build()?;
/// # let _ = client;
/// # Ok(())
/// # }
/// ```
///
/// [`Client::builder`]: crate::Client::builder
#[derive(Debug, Clone, Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    headers: Headers,
}

impl ClientBuilder {
    /// The base URL used when none is configured.
    const DEFAULT_BASE_URL: &str = "https://api.gladia.io";
    /// The number of retries for transient failures used when none is configured.
    const DEFAULT_MAX_RETRIES: u32 = 3;

    /// Sets the API key sent on each request, as the `x-gladia-key` header.
    ///
    /// Required: [`build`] fails without one. Keys are issued in the
    /// [Gladia dashboard](https://app.gladia.io).
    ///
    /// [`build`]: Self::build
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL of the Gladia API.
    ///
    /// Defaults to `https://api.gladia.io`; override it to route through a proxy or
    /// gateway. The URL is parsed when [`build`] is called.
    ///
    /// [`build`]: Self::build
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the per-request timeout. Unset by default (no timeout), matching reqwest;
    /// suited to large uploads and long-running transcription requests.
    pub fn with_timeout(mut self, timeout: impl Into<Duration>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    /// Sets the maximum number of times a transient request failure is retried with
    /// exponential backoff. Defaults to `3`; set to `0` to disable retries.
    pub fn with_max_retries(mut self, max_retries: impl Into<u32>) -> Self {
        self.max_retries = Some(max_retries.into());
        self
    }

    /// Adds a custom header sent on every request.
    ///
    /// May be called multiple times; an invalid name or value is reported when
    /// [`build`] is called. Note that `x-gladia-key` is managed by [`with_api_key`].
    ///
    /// [`build`]: Self::build
    /// [`with_api_key`]: Self::with_api_key
    pub fn with_header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Sets the `User-Agent` header sent on every request.
    ///
    /// Convenience for [`with_header`] with the `User-Agent` name.
    ///
    /// [`with_header`]: Self::with_header
    pub fn with_user_agent(self, value: impl AsRef<str>) -> Self {
        self.with_header("user-agent", value)
    }

    /// Builds the [`Client`], consuming the builder.
    ///
    /// Returns an error if no API key was set, the configured base URL cannot be
    /// parsed, a custom header is invalid, or the HTTP client cannot be constructed.
    pub fn build(self) -> Result<Client> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| Self::DEFAULT_BASE_URL.to_owned());
        let max_retries = self.max_retries.unwrap_or(Self::DEFAULT_MAX_RETRIES);
        let headers = self.headers.into_map()?;

        Client::assemble(base_url, self.api_key, self.timeout, max_retries, headers)
    }
}
