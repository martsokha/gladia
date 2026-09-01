//! Wire types for the Gladia API.
//!
//! These mirror the API's request and response bodies one-to-one and are generated
//! from `docs/openapi.json` by `cargo xtask codegen`. They are re-exported here so
//! callers import from `gladia::model` rather than depending on the generated
//! module's path, which lets the generator's layout change without breaking anyone.

mod generated;

pub use self::generated::*;
