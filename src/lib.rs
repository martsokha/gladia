#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

// A TLS backend is required for HTTPS; without one, requests to `https://api.gladia.io`
// fail at runtime. Force the choice at compile time.
#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!(
    "a TLS backend is required: enable either the `rustls-tls` (default) or \
     `native-tls` feature"
);

mod client;
mod error;

#[doc(hidden)]
pub mod prelude;

pub use crate::client::{Client, ClientBuilder};
pub use crate::error::{Error, Result};
