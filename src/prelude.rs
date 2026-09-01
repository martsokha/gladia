//! Convenient re-exports of the most commonly used items.
//!
//! ```
//! use gladia::prelude::*;
//! ```

pub use crate::error::{Error, Result};
pub use crate::prerecorded::{JobHandle, ListQuery, PreRecorded, TranscriptionRequest};
pub use crate::{Client, ClientBuilder};
