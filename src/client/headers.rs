//! A small header accumulator that keeps builder methods infallible.
//!
//! Header names and values are parsed eagerly into a [`HeaderMap`]; an invalid one
//! is stashed and surfaced when the request is built, mirroring how
//! [`reqwest::RequestBuilder::header`] defers its error to `send`. This keeps
//! `with_header` returning `Self` while storing already-typed headers (no
//! re-validation per request).
//!
//! The [`HeaderMap`] is wrapped in an [`Arc`] so cloning a [`Headers`] is a refcount
//! bump. Mutation goes through [`Arc::make_mut`]: chained `with_header` calls own a
//! unique handle and copy nothing; a clone-then-mutate path would copy once on first
//! write. This matches how the [`Client`] itself is cheap to clone.
//!
//! [`Client`]: crate::Client

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest_middleware::RequestBuilder;

use crate::error::{Error, Result};

/// Accumulated per-request headers, plus the first parse error if any.
///
/// `map` is `Arc`-wrapped so clones are cheap; mutation uses [`Arc::make_mut`].
#[derive(Debug, Clone, Default)]
pub(crate) struct Headers {
    map: Arc<HeaderMap>,
    error: Option<String>,
}

impl Headers {
    /// Parses and inserts a header, recording the first failure rather than failing.
    pub(crate) fn insert(&mut self, name: impl AsRef<str>, value: impl AsRef<str>) {
        if self.error.is_some() {
            return;
        }
        let name = match HeaderName::from_bytes(name.as_ref().as_bytes()) {
            Ok(name) => name,
            Err(e) => {
                self.error = Some(format!("{:?}: {e}", name.as_ref()));
                return;
            }
        };
        match HeaderValue::from_str(value.as_ref()) {
            Ok(value) => {
                Arc::make_mut(&mut self.map).insert(name, value);
            }
            Err(e) => self.error = Some(format!("{name}: {e}")),
        }
    }

    /// Applies the accumulated headers to `req`, or returns the recorded parse error.
    #[expect(dead_code, reason = "used by per-request header overrides")]
    pub(crate) fn apply(&self, mut req: RequestBuilder) -> Result<RequestBuilder> {
        if let Some(error) = &self.error {
            return Err(Error::invalid_message(format!("invalid header {error}")));
        }
        for (name, value) in self.map.iter() {
            req = req.header(name.clone(), value.clone());
        }
        Ok(req)
    }

    /// Consumes the accumulator into a [`HeaderMap`], or returns the recorded error.
    ///
    /// If the inner `Arc` is uniquely held this avoids a copy; otherwise the map is
    /// cloned out of the shared `Arc`.
    pub(crate) fn into_map(self) -> Result<HeaderMap> {
        match self.error {
            Some(error) => Err(Error::invalid_message(format!("invalid header {error}"))),
            None => Ok(Arc::try_unwrap(self.map).unwrap_or_else(|arc| (*arc).clone())),
        }
    }
}
