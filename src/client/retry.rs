//! Scoping retries to the requests that can be replayed safely.

use reqwest::{Method, Request};
use reqwest_middleware::{Middleware, Next, Result};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use crate::client::ReqwestResponse;

/// Applies the wrapped retry middleware only to requests that can be replayed safely.
///
/// `reqwest-retry` classifies purely on the response status: a `POST` that timed out
/// or returned a 5xx is retried like any other request. Its [`RetryableStrategy`] sees
/// only the response, which does not carry the method, and it offers no per-request
/// opt-out, so the scoping has to happen a layer up.
///
/// It matters because Gladia's `POST` endpoints create things and accept no
/// idempotency key. A retry landing after the server already processed the first
/// attempt leaves behind a duplicate upload, or a second billed transcription job.
///
/// [`RetryableStrategy`]: reqwest_retry::RetryableStrategy
pub(crate) struct IdempotentOnly(RetryTransientMiddleware<ExponentialBackoff>);

impl IdempotentOnly {
    /// Wraps a retry middleware so it applies only to idempotent methods.
    pub(crate) fn new(inner: RetryTransientMiddleware<ExponentialBackoff>) -> Self {
        Self(inner)
    }
}

#[async_trait::async_trait]
impl Middleware for IdempotentOnly {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<ReqwestResponse> {
        if is_idempotent(req.method()) {
            self.0.handle(req, extensions, next).await
        } else {
            next.run(req, extensions).await
        }
    }
}

/// Whether a method can be repeated without creating something a second time.
fn is_idempotent(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::OPTIONS | Method::TRACE | Method::PUT | Method::DELETE
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_endpoints_that_create_things_are_not_replayable() {
        // `upload` and `init` are POSTs, and a replayed one bills twice.
        assert!(!is_idempotent(&Method::POST));
        assert!(!is_idempotent(&Method::PATCH));

        assert!(is_idempotent(&Method::GET));
        assert!(is_idempotent(&Method::DELETE));
    }
}
