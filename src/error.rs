//! Error and result types for the crate.

/// A convenient alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A boxed source error, used to preserve the cause chain.
type Source = Box<dyn std::error::Error + Send + Sync>;

/// The error type returned by client operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The request could not be constructed: a missing API key, an unparseable base
    /// URL or route, an invalid header, or a malformed multipart body.
    #[error("invalid request: {message}")]
    InvalidRequest {
        /// What was invalid.
        message: String,
        /// The underlying cause, if any.
        #[source]
        source: Option<Source>,
    },

    /// A response could not be decoded (e.g. malformed JSON, or an unrecognized
    /// message on the live transcription socket).
    #[error("failed to decode response: {message}")]
    Decode {
        /// What failed to decode.
        message: String,
        /// The underlying cause, if any.
        #[source]
        source: Option<Source>,
    },

    /// The HTTP request failed: connection, timeout, retries exhausted, or the client
    /// could not be built.
    #[error("http transport error: {0}")]
    Transport(#[from] reqwest_middleware::Error),

    /// The API responded with a non-success status code.
    ///
    /// `message` is the `message` field of Gladia's JSON error envelope when present,
    /// otherwise the raw response body. `validation_errors` carries the field-level
    /// detail Gladia returns on a `422`.
    #[error("api returned status {status}: {message}")]
    Api {
        /// The HTTP status code returned by the API.
        status: u16,
        /// The error message from the envelope, or the raw body.
        message: String,
        /// Gladia's short error label (e.g. `bad_request`), when present.
        kind: Option<String>,
        /// Field-level validation detail, when the API provides it, as raw JSON.
        validation_errors: Option<serde_json::Value>,
    },
}

/// Gladia's JSON error envelope:
/// `{"statusCode": 422, "message": "...", "error": "...", "validation_errors": ...}`.
#[derive(serde::Deserialize)]
struct ErrorEnvelope {
    message: String,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    validation_errors: Option<serde_json::Value>,
}

impl Error {
    /// The HTTP status code, if this error carries one (an [`Api`] response, or a
    /// transport error generated from a response).
    ///
    /// [`Api`]: Error::Api
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::Transport(e) => e.status().map(|s| s.as_u16()),
            _ => None,
        }
    }

    /// An `InvalidRequest` error with a message and a source cause.
    pub(crate) fn invalid_request(message: impl Into<String>, source: impl Into<Source>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
            source: Some(source.into()),
        }
    }

    /// An `InvalidRequest` error with just a message.
    pub(crate) fn invalid_message(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
            source: None,
        }
    }

    /// A `Decode` error with a message and a source cause.
    #[expect(dead_code, reason = "used by the endpoint modules")]
    pub(crate) fn decode(message: impl Into<String>, source: impl Into<Source>) -> Self {
        Self::Decode {
            message: message.into(),
            source: Some(source.into()),
        }
    }

    /// A `Decode` error with just a message.
    #[expect(dead_code, reason = "used by the endpoint modules")]
    pub(crate) fn decode_message(message: impl Into<String>) -> Self {
        Self::Decode {
            message: message.into(),
            source: None,
        }
    }

    /// Builds an `Api` error from a status code and raw response body, parsing
    /// Gladia's error envelope when the body is one.
    #[allow(dead_code, reason = "used by the endpoint modules")]
    pub(crate) fn api(status: u16, body: &str) -> Self {
        match serde_json::from_str::<ErrorEnvelope>(body) {
            Ok(env) => Self::Api {
                status,
                message: env.message,
                kind: env.error,
                validation_errors: env.validation_errors,
            },
            // Not an envelope (HTML, empty, plain text): keep the raw body.
            Err(_) => Self::Api {
                status,
                message: body.to_owned(),
                kind: None,
                validation_errors: None,
            },
        }
    }
}

/// Converts a URL parse error into an [`Error::InvalidRequest`].
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::invalid_request("invalid url", e)
    }
}

/// Converts a client-build error into an [`Error::Transport`].
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Transport(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_parses_gladias_error_envelope() {
        let body = r#"{"statusCode":422,"message":"bad audio_url","error":"unprocessable_entity","validation_errors":["audio_url must be a URL"]}"#;
        let Error::Api {
            status,
            message,
            kind,
            validation_errors,
        } = Error::api(422, body)
        else {
            panic!("expected an api error");
        };
        assert_eq!(status, 422);
        assert_eq!(message, "bad audio_url");
        assert_eq!(kind.as_deref(), Some("unprocessable_entity"));
        assert!(validation_errors.is_some());
    }

    #[test]
    fn api_falls_back_to_the_raw_body() {
        let Error::Api { message, kind, .. } = Error::api(502, "<html>bad gateway</html>") else {
            panic!("expected an api error");
        };
        assert_eq!(message, "<html>bad gateway</html>");
        assert_eq!(kind, None);
    }
}
