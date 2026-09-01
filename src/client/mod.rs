//! The async HTTP client and its builder.

mod builder;
mod headers;
mod retry;

use std::sync::Arc;
use std::time::Duration;

pub(crate) use reqwest::Response as ReqwestResponse;
use reqwest_middleware::{
    ClientBuilder as MiddlewareBuilder, ClientWithMiddleware, RequestBuilder,
};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use url::Url;

pub use self::builder::ClientBuilder;
pub(crate) use self::headers::Headers;
use self::retry::IdempotentOnly;
use crate::error::{Error, Result};
use crate::prerecorded::PreRecorded;

/// The header carrying the Gladia API key on every request.
pub(crate) const API_KEY_HEADER: &str = "x-gladia-key";

/// An async client for the Gladia API.
///
/// Construct one with [`Client::builder`], supplying the API key issued in the
/// [Gladia dashboard]. The client is cheap to clone: internally it is an [`Arc`] around
/// shared state, so clones share one connection pool. Requests pass through a
/// [`reqwest-middleware`] stack that applies a per-request timeout and retries
/// transient failures with exponential backoff.
///
/// ```no_run
/// use gladia::prelude::*;
///
/// # async fn run() -> Result<()> {
/// let client = Client::builder().with_api_key("gladia-api-key").build()?;
/// # let _ = client;
/// # Ok(())
/// # }
/// ```
///
/// [Gladia dashboard]: https://app.gladia.io
/// [`reqwest-middleware`]: https://docs.rs/reqwest-middleware
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<ClientImpl>,
}

/// The shared state behind a [`Client`], held by an [`Arc`] so clones are cheap.
#[derive(Debug)]
struct ClientImpl {
    http: ClientWithMiddleware,
    base_url: Url,
}

impl Client {
    /// Returns a new [`ClientBuilder`].
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// The base URL this client targets.
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// The underlying [`reqwest::Client`], for requests this crate doesn't model.
    ///
    /// A hidden escape hatch (`#[doc(hidden)]`, not part of the stable API): build any
    /// request against [`base_url`] and send it with the client's configured TLS,
    /// connection pool, and API key. This is the bare client, so it does *not* carry
    /// the retry middleware. Use [`as_client_with_middleware`] to keep retries.
    ///
    /// [`base_url`]: Self::base_url
    /// [`as_client_with_middleware`]: Self::as_client_with_middleware
    #[doc(hidden)]
    pub fn as_client(&self) -> &reqwest::Client {
        self.inner.http.as_ref()
    }

    /// The underlying [`reqwest_middleware::ClientWithMiddleware`], for requests this
    /// crate doesn't model.
    ///
    /// A hidden escape hatch (`#[doc(hidden)]`, not part of the stable API) like
    /// [`as_client`], but with the retry middleware applied, so custom requests get
    /// the same exponential-backoff retries as the crate's own. The API key is carried
    /// via the client's default headers, like every other request.
    ///
    /// [`as_client`]: Self::as_client
    #[doc(hidden)]
    pub fn as_client_with_middleware(&self) -> &reqwest_middleware::ClientWithMiddleware {
        &self.inner.http
    }

    /// Returns a handle to the pre-recorded transcription endpoints.
    ///
    /// The handle borrows the client, so it is free to create and can be used inline:
    /// `client.prerecorded().get(id).await?`.
    pub fn prerecorded(&self) -> PreRecorded<'_> {
        PreRecorded::new(self)
    }

    /// Returns a handle to the live transcription endpoints.
    #[cfg(feature = "live")]
    #[cfg_attr(docsrs, doc(cfg(feature = "live")))]
    pub fn live(&self) -> crate::live::Live<'_> {
        crate::live::Live::new(self)
    }

    /// Joins a route onto the base URL, tolerating an optional leading slash.
    pub(crate) fn route_url(&self, route: &str) -> Result<Url> {
        let route = route.trim_start_matches('/');
        Ok(self.inner.base_url.join(route)?)
    }

    /// Begins a `GET` request to `route`.
    pub(crate) fn get(&self, route: &str) -> Result<RequestBuilder> {
        Ok(self.inner.http.get(self.route_url(route)?))
    }

    /// Begins a `POST` request to `route`.
    pub(crate) fn post(&self, route: &str) -> Result<RequestBuilder> {
        Ok(self.inner.http.post(self.route_url(route)?))
    }

    /// Begins a `DELETE` request to `route`.
    pub(crate) fn delete(&self, route: &str) -> Result<RequestBuilder> {
        Ok(self.inner.http.delete(self.route_url(route)?))
    }

    /// Sends a request, mapping any non-success status to [`Error::Api`].
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, err))]
    pub(crate) async fn send(&self, req: RequestBuilder) -> Result<ReqwestResponse> {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::api(status.as_u16(), &body));
        }
        Ok(resp)
    }

    /// Sends a request and deserializes a JSON response body.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, err))]
    pub(crate) async fn send_json<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let resp = self.send(req).await?;
        // Read the body before deserializing so a malformed payload can be reported
        // with the text that failed, rather than just a position.
        let body = resp
            .bytes()
            .await
            .map_err(reqwest_middleware::Error::from)?;
        serde_json::from_slice(&body).map_err(|e| {
            Error::decode(
                format!(
                    "unexpected response body: {}",
                    String::from_utf8_lossy(&body)
                ),
                e,
            )
        })
    }

    /// Assembles a [`Client`] from resolved configuration, used by [`ClientBuilder`].
    ///
    /// Builds the HTTP middleware stack: a per-request timeout enforced by reqwest,
    /// with retries layered on top via reqwest-middleware so each attempt gets its
    /// own timeout. Redirects are refused, and the retry layer is installed only when
    /// retries are enabled.
    pub(crate) fn assemble(
        base_url: String,
        api_key: Option<String>,
        timeout: Option<Duration>,
        max_retries: u32,
        mut headers: reqwest::header::HeaderMap,
    ) -> Result<Self> {
        // The API key is a constant credential for the life of the client, so it lives
        // in the default headers rather than being applied per request. This also
        // authenticates requests made through the `as_client*` escape hatches.
        let api_key = api_key.ok_or_else(|| Error::invalid_message("an api key is required"))?;
        let mut value: reqwest::header::HeaderValue = api_key
            .parse()
            .map_err(|e| Error::invalid_request("invalid api key", e))?;
        value.set_sensitive(true);
        headers.insert(API_KEY_HEADER, value);

        // The API key travels in a custom header, and reqwest only strips a fixed set
        // of credential headers (`Authorization`, `Cookie`, `Proxy-Authorization`,
        // `WWW-Authenticate`) when a redirect crosses origins, so a custom one would
        // be forwarded. Refusing to follow redirects keeps the key from ever reaching
        // a host other than the configured one.
        let mut http = reqwest::Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none());
        if let Some(timeout) = timeout {
            http = http.timeout(timeout);
        }
        let inner = http.build()?;

        let mut http = MiddlewareBuilder::new(inner);
        // `RetryTransientMiddleware` clones the request up front and rejects streaming
        // bodies, so it is installed only when it can actually retry.
        if max_retries > 0 {
            let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
            http = http.with(IdempotentOnly::new(
                RetryTransientMiddleware::new_with_policy(retry_policy),
            ));
        }
        let http = http.build();

        let mut base_url = Url::parse(&base_url)?;

        // A base URL must end in `/` for [`Url::join`] to treat it as a directory
        // rather than replacing the final path segment.
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }

        Ok(Self {
            inner: Arc::new(ClientImpl { http, base_url }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client(base_url: &str) -> Client {
        Client::builder()
            .with_api_key("test-key")
            .with_base_url(base_url)
            .build()
            .unwrap()
    }

    #[test]
    fn retries_can_be_disabled() {
        // With `max_retries` at 0 the retry middleware is omitted entirely, so it
        // never clones the request and streaming bodies stay usable.
        let client = Client::builder()
            .with_api_key("test-key")
            .with_max_retries(0u32)
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn an_api_key_is_required() {
        assert!(matches!(
            Client::builder().build(),
            Err(Error::InvalidRequest { .. })
        ));
    }

    #[test]
    fn base_url_gets_a_trailing_slash() {
        // Without normalization, `Url::join` would drop the final path segment.
        assert_eq!(
            client("https://api.gladia.io").base_url().as_str(),
            "https://api.gladia.io/"
        );
        assert_eq!(
            client("https://api.gladia.io/").base_url().as_str(),
            "https://api.gladia.io/"
        );
    }

    #[test]
    fn route_joins_tolerating_a_leading_slash() {
        let c = client("https://api.gladia.io");
        assert_eq!(
            c.route_url("v2/pre-recorded").unwrap().as_str(),
            "https://api.gladia.io/v2/pre-recorded"
        );
        assert_eq!(
            c.route_url("/v2/pre-recorded").unwrap().as_str(),
            "https://api.gladia.io/v2/pre-recorded"
        );
    }

    #[test]
    fn route_preserves_a_path_prefix() {
        let c = client("https://gateway.example.com/gladia");
        assert_eq!(
            c.route_url("v2/pre-recorded").unwrap().as_str(),
            "https://gateway.example.com/gladia/v2/pre-recorded"
        );
    }
}
