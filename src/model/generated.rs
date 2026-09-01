//! Types generated from the Gladia OpenAPI document.
//!
//! Do not edit by hand. Regenerate with `cargo xtask codegen` after updating
//! `docs/openapi.json`; CI fails if this file is out of date with the spec.

// The API's own descriptions carry `[Deprecated]`/`[Alpha]` markers and a bare URL,
// which rustdoc reads as a malformed link and a missing hyperlink respectively. They
// are Gladia's prose, reproduced verbatim, so the lints are silenced rather than the
// text rewritten.
#![allow(
    clippy::all,
    missing_docs,
    unreachable_pub,
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls
)]
#![cfg_attr(rustfmt, rustfmt::skip)]

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`AddonErrorDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "exception",
///    "message",
///    "status_code"
///  ],
///  "properties": {
///    "exception": {
///      "description": "Reason of the addon error",
///      "type": "string"
///    },
///    "message": {
///      "description": "Detailed message of the addon error",
///      "type": "string"
///    },
///    "status_code": {
///      "description": "Status code of the addon error",
///      "type": "integer",
///      "example": 500
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AddonErrorDto {
    ///Reason of the addon error
    pub exception: ::std::string::String,
    ///Detailed message of the addon error
    pub message: ::std::string::String,
    ///Status code of the addon error
    pub status_code: i64,
}
impl ::std::convert::From<&AddonErrorDto> for AddonErrorDto {
    fn from(value: &AddonErrorDto) -> Self {
        value.clone()
    }
}
impl AddonErrorDto {
    pub fn builder() -> builder::AddonErrorDto {
        Default::default()
    }
}
///`AudioChunkAckData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "byte_range",
///    "time_range"
///  ],
///  "properties": {
///    "byte_range": {
///      "description": "Range in bytes length of the audio chunk (relative to the whole session)",
///      "type": "array",
///      "items": {
///        "type": "integer"
///      },
///      "maxItems": 2,
///      "minItems": 2,
///      "example": [
///        1024,
///        2048
///      ]
///    },
///    "time_range": {
///      "description": "Range in seconds of the audio chunk (relative to the whole session)",
///      "type": "array",
///      "items": {
///        "type": "number"
///      },
///      "maxItems": 2,
///      "minItems": 2,
///      "example": [
///        0.8,
///        0.9
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioChunkAckData {
    ///Range in bytes length of the audio chunk (relative to the whole session)
    pub byte_range: [i64; 2usize],
    ///Range in seconds of the audio chunk (relative to the whole session)
    pub time_range: [f64; 2usize],
}
impl ::std::convert::From<&AudioChunkAckData> for AudioChunkAckData {
    fn from(value: &AudioChunkAckData) -> Self {
        value.clone()
    }
}
impl AudioChunkAckData {
    pub fn builder() -> builder::AudioChunkAckData {
        Default::default()
    }
}
///`AudioChunkAckMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "acknowledged",
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "acknowledged": {
///      "description": "Flag to indicate if the action was successfully acknowledged",
///      "type": "boolean",
///      "example": true
///    },
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the action was not successfully acknowledged",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioChunkAckData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the action was not successfully acknowledged",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "audio_chunk",
///      "type": "string",
///      "enum": [
///        "audio_chunk"
///      ],
///      "example": "audio_chunk"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioChunkAckMessage {
    ///Flag to indicate if the action was successfully acknowledged
    pub acknowledged: bool,
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the action was not successfully acknowledged
    pub data: AudioChunkAckData,
    ///Error message if the action was not successfully acknowledged
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: AudioChunkAckMessageType,
}
impl ::std::convert::From<&AudioChunkAckMessage> for AudioChunkAckMessage {
    fn from(value: &AudioChunkAckMessage) -> Self {
        value.clone()
    }
}
impl AudioChunkAckMessage {
    pub fn builder() -> builder::AudioChunkAckMessage {
        Default::default()
    }
}
///`AudioChunkAckMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "audio_chunk",
///  "type": "string",
///  "enum": [
///    "audio_chunk"
///  ],
///  "example": "audio_chunk"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum AudioChunkAckMessageType {
    #[serde(rename = "audio_chunk")]
    AudioChunk,
}
impl ::std::convert::From<&Self> for AudioChunkAckMessageType {
    fn from(value: &AudioChunkAckMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AudioChunkAckMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioChunk => f.write_str("audio_chunk"),
        }
    }
}
impl ::std::str::FromStr for AudioChunkAckMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio_chunk" => Ok(Self::AudioChunk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AudioChunkAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AudioChunkAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AudioChunkAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for AudioChunkAckMessageType {
    fn default() -> Self {
        AudioChunkAckMessageType::AudioChunk
    }
}
///`AudioChunkAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "data",
///    "type"
///  ],
///  "properties": {
///    "data": {
///      "description": "Payload of the audio chunk action",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioChunkActionData"
///        }
///      ]
///    },
///    "type": {
///      "default": "audio_chunk",
///      "type": "string",
///      "enum": [
///        "audio_chunk"
///      ],
///      "example": "audio_chunk"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioChunkAction {
    ///Payload of the audio chunk action
    pub data: AudioChunkActionData,
    #[serde(rename = "type")]
    pub type_: AudioChunkActionType,
}
impl ::std::convert::From<&AudioChunkAction> for AudioChunkAction {
    fn from(value: &AudioChunkAction) -> Self {
        value.clone()
    }
}
impl AudioChunkAction {
    pub fn builder() -> builder::AudioChunkAction {
        Default::default()
    }
}
///`AudioChunkActionData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "chunk"
///  ],
///  "properties": {
///    "chunk": {
///      "description": "Chunk encoded in base64. The chunk must contains complete frames",
///      "type": "string",
///      "example": "aGVsbG8="
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioChunkActionData {
    ///Chunk encoded in base64. The chunk must contains complete frames
    pub chunk: ::std::string::String,
}
impl ::std::convert::From<&AudioChunkActionData> for AudioChunkActionData {
    fn from(value: &AudioChunkActionData) -> Self {
        value.clone()
    }
}
impl AudioChunkActionData {
    pub fn builder() -> builder::AudioChunkActionData {
        Default::default()
    }
}
///`AudioChunkActionType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "audio_chunk",
///  "type": "string",
///  "enum": [
///    "audio_chunk"
///  ],
///  "example": "audio_chunk"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum AudioChunkActionType {
    #[serde(rename = "audio_chunk")]
    AudioChunk,
}
impl ::std::convert::From<&Self> for AudioChunkActionType {
    fn from(value: &AudioChunkActionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AudioChunkActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioChunk => f.write_str("audio_chunk"),
        }
    }
}
impl ::std::str::FromStr for AudioChunkActionType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio_chunk" => Ok(Self::AudioChunk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AudioChunkActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AudioChunkActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AudioChunkActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for AudioChunkActionType {
    fn default() -> Self {
        AudioChunkActionType::AudioChunk
    }
}
///`AudioToLlmDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "The result from a specific prompt",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioToLlmResultDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioToLlmDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///The result from a specific prompt
    pub results: AudioToLlmResultDto,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&AudioToLlmDto> for AudioToLlmDto {
    fn from(value: &AudioToLlmDto) -> Self {
        value.clone()
    }
}
impl AudioToLlmDto {
    pub fn builder() -> builder::AudioToLlmDto {
        Default::default()
    }
}
///`AudioToLlmListConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "prompts"
///  ],
///  "properties": {
///    "model": {
///      "description": "The model to use for the prompt execution. You can find the list of supported models [here](https://openrouter.ai/models).",
///      "type": "string"
///    },
///    "prompts": {
///      "description": "The list of prompts applied on the audio transcription",
///      "type": "array",
///      "items": {
///        "type": "array"
///      },
///      "minItems": 1,
///      "example": [
///        "Extract the key points from the transcription"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioToLlmListConfigDto {
    ///The model to use for the prompt execution. You can find the list of supported models [here](https://openrouter.ai/models).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    ///The list of prompts applied on the audio transcription
    pub prompts: ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
}
impl ::std::convert::From<&AudioToLlmListConfigDto> for AudioToLlmListConfigDto {
    fn from(value: &AudioToLlmListConfigDto) -> Self {
        value.clone()
    }
}
impl AudioToLlmListConfigDto {
    pub fn builder() -> builder::AudioToLlmListConfigDto {
        Default::default()
    }
}
///`AudioToLlmListDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `audio_to_llm` has been enabled, results of the AI custom analysis",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AudioToLlmDTO"
///      },
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioToLlmListDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `audio_to_llm` has been enabled, results of the AI custom analysis
    pub results: ::std::vec::Vec<AudioToLlmDto>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&AudioToLlmListDto> for AudioToLlmListDto {
    fn from(value: &AudioToLlmListDto) -> Self {
        value.clone()
    }
}
impl AudioToLlmListDto {
    pub fn builder() -> builder::AudioToLlmListDto {
        Default::default()
    }
}
///`AudioToLlmResultDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "prompt",
///    "response"
///  ],
///  "properties": {
///    "prompt": {
///      "description": "The prompt used",
///      "type": "string",
///      "nullable": true
///    },
///    "response": {
///      "description": "The result of the AI analysis",
///      "type": "string",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioToLlmResultDto {
    ///The prompt used
    pub prompt: ::std::string::String,
    ///The result of the AI analysis
    pub response: ::std::string::String,
}
impl ::std::convert::From<&AudioToLlmResultDto> for AudioToLlmResultDto {
    fn from(value: &AudioToLlmResultDto) -> Self {
        value.clone()
    }
}
impl AudioToLlmResultDto {
    pub fn builder() -> builder::AudioToLlmResultDto {
        Default::default()
    }
}
///`AudioUploadMetadataDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_duration",
///    "extension",
///    "filename",
///    "id",
///    "number_of_channels",
///    "size"
///  ],
///  "properties": {
///    "audio_duration": {
///      "description": "Uploaded audio duration",
///      "type": "number",
///      "example": 4.145782
///    },
///    "extension": {
///      "description": "Uploaded audio detected extension",
///      "type": "string",
///      "example": "wav"
///    },
///    "filename": {
///      "description": "Uploaded audio filename",
///      "type": "string",
///      "example": "short-audio-en-16000.wav"
///    },
///    "id": {
///      "description": "Uploaded audio file ID",
///      "type": "string",
///      "format": "uuid",
///      "example": "6c09400e-23d2-4bd2-be55-96a5ececfa3b"
///    },
///    "number_of_channels": {
///      "description": "Uploaded audio channel numbers",
///      "type": "integer",
///      "example": 1
///    },
///    "size": {
///      "description": "Uploaded audio size",
///      "type": "integer",
///      "example": 365702
///    },
///    "source": {
///      "description": "Uploaded audio source",
///      "type": "string",
///      "format": "uri",
///      "example": "https://files.gladia.io/example/audio-transcription/split_infinity.wav"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioUploadMetadataDto {
    pub audio_duration: f64,
    ///Uploaded audio detected extension
    pub extension: ::std::string::String,
    ///Uploaded audio filename
    pub filename: ::std::string::String,
    ///Uploaded audio file ID
    pub id: ::uuid::Uuid,
    ///Uploaded audio channel numbers
    pub number_of_channels: i64,
    ///Uploaded audio size
    pub size: i64,
    ///Uploaded audio source
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AudioUploadMetadataDto> for AudioUploadMetadataDto {
    fn from(value: &AudioUploadMetadataDto) -> Self {
        value.clone()
    }
}
impl AudioUploadMetadataDto {
    pub fn builder() -> builder::AudioUploadMetadataDto {
        Default::default()
    }
}
///`AudioUploadResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_metadata",
///    "audio_url"
///  ],
///  "properties": {
///    "audio_metadata": {
///      "description": "Uploaded audio file detected metadata",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioUploadMetadataDTO"
///        }
///      ]
///    },
///    "audio_url": {
///      "description": "Uploaded audio file Gladia URL",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/file/6c09400e-23d2-4bd2-be55-96a5ececfa3b"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AudioUploadResponse {
    ///Uploaded audio file detected metadata
    pub audio_metadata: AudioUploadMetadataDto,
    ///Uploaded audio file Gladia URL
    pub audio_url: ::std::string::String,
}
impl ::std::convert::From<&AudioUploadResponse> for AudioUploadResponse {
    fn from(value: &AudioUploadResponse) -> Self {
        value.clone()
    }
}
impl AudioUploadResponse {
    pub fn builder() -> builder::AudioUploadResponse {
        Default::default()
    }
}
///`BadRequestErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Error message",
///      "type": "string",
///      "example": "Content-Type is missing Multipart Boundary."
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 400
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "validation_errors": {
///      "description": "List of validation errors, if any",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "example": [
///        "Field \"language\" must be a string",
///        "Field \"min_speakers\" must be a number"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct BadRequestErrorResponse {
    ///Error message
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
    ///List of validation errors, if any
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub validation_errors: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&BadRequestErrorResponse> for BadRequestErrorResponse {
    fn from(value: &BadRequestErrorResponse) -> Self {
        value.clone()
    }
}
impl BadRequestErrorResponse {
    pub fn builder() -> builder::BadRequestErrorResponse {
        Default::default()
    }
}
///`CallbackConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "receive_acknowledgments": {
///      "description": "If true, acknowledgments will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_errors": {
///      "description": "If true, errors will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_final_transcripts": {
///      "description": "If true, final transcript will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_lifecycle_events": {
///      "description": "If true, lifecycle events will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_partial_transcripts": {
///      "description": "If true, partial transcript will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_post_processing_events": {
///      "description": "If true, post-processing events will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_pre_processing_events": {
///      "description": "If true, pre-processing events will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_realtime_processing_events": {
///      "description": "If true, realtime processing events will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "receive_speech_events": {
///      "description": "If true, begin and end speech events will be sent to the defined callback.",
///      "type": "boolean"
///    },
///    "url": {
///      "description": "URL on which we will do a `POST` request with configured messages",
///      "type": "string",
///      "format": "uri",
///      "example": "https://callback.example"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackConfig {
    ///If true, acknowledgments will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_acknowledgments: ::std::option::Option<bool>,
    ///If true, errors will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_errors: ::std::option::Option<bool>,
    ///If true, final transcript will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_final_transcripts: ::std::option::Option<bool>,
    ///If true, lifecycle events will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_lifecycle_events: ::std::option::Option<bool>,
    ///If true, partial transcript will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_partial_transcripts: ::std::option::Option<bool>,
    ///If true, post-processing events will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_post_processing_events: ::std::option::Option<bool>,
    ///If true, pre-processing events will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_pre_processing_events: ::std::option::Option<bool>,
    ///If true, realtime processing events will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_realtime_processing_events: ::std::option::Option<bool>,
    ///If true, begin and end speech events will be sent to the defined callback.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_speech_events: ::std::option::Option<bool>,
    ///URL on which we will do a `POST` request with configured messages
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CallbackConfig> for CallbackConfig {
    fn from(value: &CallbackConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CallbackConfig {
    fn default() -> Self {
        Self {
            receive_acknowledgments: Default::default(),
            receive_errors: Default::default(),
            receive_final_transcripts: Default::default(),
            receive_lifecycle_events: Default::default(),
            receive_partial_transcripts: Default::default(),
            receive_post_processing_events: Default::default(),
            receive_pre_processing_events: Default::default(),
            receive_realtime_processing_events: Default::default(),
            receive_speech_events: Default::default(),
            url: Default::default(),
        }
    }
}
impl CallbackConfig {
    pub fn builder() -> builder::CallbackConfig {
        Default::default()
    }
}
///`CallbackConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "url"
///  ],
///  "properties": {
///    "method": {
///      "description": "The HTTP method to be used. Allowed values are `POST` or `PUT` (default: `POST`)",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CallbackMethodEnum"
///        }
///      ],
///      "example": "POST"
///    },
///    "url": {
///      "description": "The URL to be called with the result of the transcription",
///      "type": "string",
///      "format": "uri",
///      "example": "https://callback.example"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackConfigDto {
    ///The HTTP method to be used. Allowed values are `POST` or `PUT` (default: `POST`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<CallbackMethodEnum>,
    ///The URL to be called with the result of the transcription
    pub url: ::std::string::String,
}
impl ::std::convert::From<&CallbackConfigDto> for CallbackConfigDto {
    fn from(value: &CallbackConfigDto) -> Self {
        value.clone()
    }
}
impl CallbackConfigDto {
    pub fn builder() -> builder::CallbackConfigDto {
        Default::default()
    }
}
///`CallbackLiveAudioChunkAckMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.audio_chunk",
///      "type": "string",
///      "enum": [
///        "live.audio_chunk"
///      ],
///      "example": "live.audio_chunk"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioChunkAckMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveAudioChunkAckMessage {
    pub event: CallbackLiveAudioChunkAckMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: AudioChunkAckMessage,
}
impl ::std::convert::From<&CallbackLiveAudioChunkAckMessage>
for CallbackLiveAudioChunkAckMessage {
    fn from(value: &CallbackLiveAudioChunkAckMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveAudioChunkAckMessage {
    pub fn builder() -> builder::CallbackLiveAudioChunkAckMessage {
        Default::default()
    }
}
///`CallbackLiveAudioChunkAckMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.audio_chunk",
///  "type": "string",
///  "enum": [
///    "live.audio_chunk"
///  ],
///  "example": "live.audio_chunk"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveAudioChunkAckMessageEvent {
    #[serde(rename = "live.audio_chunk")]
    LiveAudioChunk,
}
impl ::std::convert::From<&Self> for CallbackLiveAudioChunkAckMessageEvent {
    fn from(value: &CallbackLiveAudioChunkAckMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveAudioChunkAckMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveAudioChunk => f.write_str("live.audio_chunk"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveAudioChunkAckMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.audio_chunk" => Ok(Self::LiveAudioChunk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveAudioChunkAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveAudioChunkAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveAudioChunkAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveAudioChunkAckMessageEvent {
    fn default() -> Self {
        CallbackLiveAudioChunkAckMessageEvent::LiveAudioChunk
    }
}
///`CallbackLiveEndRecordingMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.end_recording",
///      "type": "string",
///      "enum": [
///        "live.end_recording"
///      ],
///      "example": "live.end_recording"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/EndRecordingMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveEndRecordingMessage {
    pub event: CallbackLiveEndRecordingMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: EndRecordingMessage,
}
impl ::std::convert::From<&CallbackLiveEndRecordingMessage>
for CallbackLiveEndRecordingMessage {
    fn from(value: &CallbackLiveEndRecordingMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveEndRecordingMessage {
    pub fn builder() -> builder::CallbackLiveEndRecordingMessage {
        Default::default()
    }
}
///`CallbackLiveEndRecordingMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.end_recording",
///  "type": "string",
///  "enum": [
///    "live.end_recording"
///  ],
///  "example": "live.end_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveEndRecordingMessageEvent {
    #[serde(rename = "live.end_recording")]
    LiveEndRecording,
}
impl ::std::convert::From<&Self> for CallbackLiveEndRecordingMessageEvent {
    fn from(value: &CallbackLiveEndRecordingMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveEndRecordingMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveEndRecording => f.write_str("live.end_recording"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveEndRecordingMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.end_recording" => Ok(Self::LiveEndRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveEndRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveEndRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveEndRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveEndRecordingMessageEvent {
    fn default() -> Self {
        CallbackLiveEndRecordingMessageEvent::LiveEndRecording
    }
}
///`CallbackLiveEndSessionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.end_session",
///      "type": "string",
///      "enum": [
///        "live.end_session"
///      ],
///      "example": "live.end_session"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/EndSessionMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveEndSessionMessage {
    pub event: CallbackLiveEndSessionMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: EndSessionMessage,
}
impl ::std::convert::From<&CallbackLiveEndSessionMessage>
for CallbackLiveEndSessionMessage {
    fn from(value: &CallbackLiveEndSessionMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveEndSessionMessage {
    pub fn builder() -> builder::CallbackLiveEndSessionMessage {
        Default::default()
    }
}
///`CallbackLiveEndSessionMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.end_session",
///  "type": "string",
///  "enum": [
///    "live.end_session"
///  ],
///  "example": "live.end_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveEndSessionMessageEvent {
    #[serde(rename = "live.end_session")]
    LiveEndSession,
}
impl ::std::convert::From<&Self> for CallbackLiveEndSessionMessageEvent {
    fn from(value: &CallbackLiveEndSessionMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveEndSessionMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveEndSession => f.write_str("live.end_session"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveEndSessionMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.end_session" => Ok(Self::LiveEndSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveEndSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveEndSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveEndSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveEndSessionMessageEvent {
    fn default() -> Self {
        CallbackLiveEndSessionMessageEvent::LiveEndSession
    }
}
///`CallbackLiveNamedEntityRecognitionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.named_entity_recognition",
///      "type": "string",
///      "enum": [
///        "live.named_entity_recognition"
///      ],
///      "example": "live.named_entity_recognition"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamedEntityRecognitionMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveNamedEntityRecognitionMessage {
    pub event: CallbackLiveNamedEntityRecognitionMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: NamedEntityRecognitionMessage,
}
impl ::std::convert::From<&CallbackLiveNamedEntityRecognitionMessage>
for CallbackLiveNamedEntityRecognitionMessage {
    fn from(value: &CallbackLiveNamedEntityRecognitionMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveNamedEntityRecognitionMessage {
    pub fn builder() -> builder::CallbackLiveNamedEntityRecognitionMessage {
        Default::default()
    }
}
///`CallbackLiveNamedEntityRecognitionMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.named_entity_recognition",
///  "type": "string",
///  "enum": [
///    "live.named_entity_recognition"
///  ],
///  "example": "live.named_entity_recognition"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveNamedEntityRecognitionMessageEvent {
    #[serde(rename = "live.named_entity_recognition")]
    LiveNamedEntityRecognition,
}
impl ::std::convert::From<&Self> for CallbackLiveNamedEntityRecognitionMessageEvent {
    fn from(value: &CallbackLiveNamedEntityRecognitionMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveNamedEntityRecognitionMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveNamedEntityRecognition => {
                f.write_str("live.named_entity_recognition")
            }
        }
    }
}
impl ::std::str::FromStr for CallbackLiveNamedEntityRecognitionMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.named_entity_recognition" => Ok(Self::LiveNamedEntityRecognition),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveNamedEntityRecognitionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveNamedEntityRecognitionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveNamedEntityRecognitionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveNamedEntityRecognitionMessageEvent {
    fn default() -> Self {
        CallbackLiveNamedEntityRecognitionMessageEvent::LiveNamedEntityRecognition
    }
}
///`CallbackLivePostFinalTranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.post_final_transcript",
///      "type": "string",
///      "enum": [
///        "live.post_final_transcript"
///      ],
///      "example": "live.post_final_transcript"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostFinalTranscriptMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLivePostFinalTranscriptMessage {
    pub event: CallbackLivePostFinalTranscriptMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: PostFinalTranscriptMessage,
}
impl ::std::convert::From<&CallbackLivePostFinalTranscriptMessage>
for CallbackLivePostFinalTranscriptMessage {
    fn from(value: &CallbackLivePostFinalTranscriptMessage) -> Self {
        value.clone()
    }
}
impl CallbackLivePostFinalTranscriptMessage {
    pub fn builder() -> builder::CallbackLivePostFinalTranscriptMessage {
        Default::default()
    }
}
///`CallbackLivePostFinalTranscriptMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.post_final_transcript",
///  "type": "string",
///  "enum": [
///    "live.post_final_transcript"
///  ],
///  "example": "live.post_final_transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLivePostFinalTranscriptMessageEvent {
    #[serde(rename = "live.post_final_transcript")]
    LivePostFinalTranscript,
}
impl ::std::convert::From<&Self> for CallbackLivePostFinalTranscriptMessageEvent {
    fn from(value: &CallbackLivePostFinalTranscriptMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLivePostFinalTranscriptMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LivePostFinalTranscript => f.write_str("live.post_final_transcript"),
        }
    }
}
impl ::std::str::FromStr for CallbackLivePostFinalTranscriptMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.post_final_transcript" => Ok(Self::LivePostFinalTranscript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLivePostFinalTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLivePostFinalTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLivePostFinalTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLivePostFinalTranscriptMessageEvent {
    fn default() -> Self {
        CallbackLivePostFinalTranscriptMessageEvent::LivePostFinalTranscript
    }
}
///`CallbackLivePostSummarizationMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.post_summarization",
///      "type": "string",
///      "enum": [
///        "live.post_summarization"
///      ],
///      "example": "live.post_summarization"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostSummarizationMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLivePostSummarizationMessage {
    pub event: CallbackLivePostSummarizationMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: PostSummarizationMessage,
}
impl ::std::convert::From<&CallbackLivePostSummarizationMessage>
for CallbackLivePostSummarizationMessage {
    fn from(value: &CallbackLivePostSummarizationMessage) -> Self {
        value.clone()
    }
}
impl CallbackLivePostSummarizationMessage {
    pub fn builder() -> builder::CallbackLivePostSummarizationMessage {
        Default::default()
    }
}
///`CallbackLivePostSummarizationMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.post_summarization",
///  "type": "string",
///  "enum": [
///    "live.post_summarization"
///  ],
///  "example": "live.post_summarization"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLivePostSummarizationMessageEvent {
    #[serde(rename = "live.post_summarization")]
    LivePostSummarization,
}
impl ::std::convert::From<&Self> for CallbackLivePostSummarizationMessageEvent {
    fn from(value: &CallbackLivePostSummarizationMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLivePostSummarizationMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LivePostSummarization => f.write_str("live.post_summarization"),
        }
    }
}
impl ::std::str::FromStr for CallbackLivePostSummarizationMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.post_summarization" => Ok(Self::LivePostSummarization),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLivePostSummarizationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLivePostSummarizationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLivePostSummarizationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLivePostSummarizationMessageEvent {
    fn default() -> Self {
        CallbackLivePostSummarizationMessageEvent::LivePostSummarization
    }
}
///`CallbackLivePostTranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.post_transcript",
///      "type": "string",
///      "enum": [
///        "live.post_transcript"
///      ],
///      "example": "live.post_transcript"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostTranscriptMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLivePostTranscriptMessage {
    pub event: CallbackLivePostTranscriptMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: PostTranscriptMessage,
}
impl ::std::convert::From<&CallbackLivePostTranscriptMessage>
for CallbackLivePostTranscriptMessage {
    fn from(value: &CallbackLivePostTranscriptMessage) -> Self {
        value.clone()
    }
}
impl CallbackLivePostTranscriptMessage {
    pub fn builder() -> builder::CallbackLivePostTranscriptMessage {
        Default::default()
    }
}
///`CallbackLivePostTranscriptMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.post_transcript",
///  "type": "string",
///  "enum": [
///    "live.post_transcript"
///  ],
///  "example": "live.post_transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLivePostTranscriptMessageEvent {
    #[serde(rename = "live.post_transcript")]
    LivePostTranscript,
}
impl ::std::convert::From<&Self> for CallbackLivePostTranscriptMessageEvent {
    fn from(value: &CallbackLivePostTranscriptMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLivePostTranscriptMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LivePostTranscript => f.write_str("live.post_transcript"),
        }
    }
}
impl ::std::str::FromStr for CallbackLivePostTranscriptMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.post_transcript" => Ok(Self::LivePostTranscript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLivePostTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLivePostTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLivePostTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLivePostTranscriptMessageEvent {
    fn default() -> Self {
        CallbackLivePostTranscriptMessageEvent::LivePostTranscript
    }
}
///`CallbackLiveSentimentAnalysisMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.sentiment_analysis",
///      "type": "string",
///      "enum": [
///        "live.sentiment_analysis"
///      ],
///      "example": "live.sentiment_analysis"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentimentAnalysisMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveSentimentAnalysisMessage {
    pub event: CallbackLiveSentimentAnalysisMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: SentimentAnalysisMessage,
}
impl ::std::convert::From<&CallbackLiveSentimentAnalysisMessage>
for CallbackLiveSentimentAnalysisMessage {
    fn from(value: &CallbackLiveSentimentAnalysisMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveSentimentAnalysisMessage {
    pub fn builder() -> builder::CallbackLiveSentimentAnalysisMessage {
        Default::default()
    }
}
///`CallbackLiveSentimentAnalysisMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.sentiment_analysis",
///  "type": "string",
///  "enum": [
///    "live.sentiment_analysis"
///  ],
///  "example": "live.sentiment_analysis"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveSentimentAnalysisMessageEvent {
    #[serde(rename = "live.sentiment_analysis")]
    LiveSentimentAnalysis,
}
impl ::std::convert::From<&Self> for CallbackLiveSentimentAnalysisMessageEvent {
    fn from(value: &CallbackLiveSentimentAnalysisMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveSentimentAnalysisMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveSentimentAnalysis => f.write_str("live.sentiment_analysis"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveSentimentAnalysisMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.sentiment_analysis" => Ok(Self::LiveSentimentAnalysis),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveSentimentAnalysisMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveSentimentAnalysisMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveSentimentAnalysisMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveSentimentAnalysisMessageEvent {
    fn default() -> Self {
        CallbackLiveSentimentAnalysisMessageEvent::LiveSentimentAnalysis
    }
}
///`CallbackLiveSpeechEndMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.speech_end",
///      "type": "string",
///      "enum": [
///        "live.speech_end"
///      ],
///      "example": "live.speech_end"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SpeechEndMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveSpeechEndMessage {
    pub event: CallbackLiveSpeechEndMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: SpeechEndMessage,
}
impl ::std::convert::From<&CallbackLiveSpeechEndMessage>
for CallbackLiveSpeechEndMessage {
    fn from(value: &CallbackLiveSpeechEndMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveSpeechEndMessage {
    pub fn builder() -> builder::CallbackLiveSpeechEndMessage {
        Default::default()
    }
}
///`CallbackLiveSpeechEndMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.speech_end",
///  "type": "string",
///  "enum": [
///    "live.speech_end"
///  ],
///  "example": "live.speech_end"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveSpeechEndMessageEvent {
    #[serde(rename = "live.speech_end")]
    LiveSpeechEnd,
}
impl ::std::convert::From<&Self> for CallbackLiveSpeechEndMessageEvent {
    fn from(value: &CallbackLiveSpeechEndMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveSpeechEndMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveSpeechEnd => f.write_str("live.speech_end"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveSpeechEndMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.speech_end" => Ok(Self::LiveSpeechEnd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveSpeechEndMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveSpeechEndMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveSpeechEndMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveSpeechEndMessageEvent {
    fn default() -> Self {
        CallbackLiveSpeechEndMessageEvent::LiveSpeechEnd
    }
}
///`CallbackLiveSpeechStartMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.speech_start",
///      "type": "string",
///      "enum": [
///        "live.speech_start"
///      ],
///      "example": "live.speech_start"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SpeechStartMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveSpeechStartMessage {
    pub event: CallbackLiveSpeechStartMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: SpeechStartMessage,
}
impl ::std::convert::From<&CallbackLiveSpeechStartMessage>
for CallbackLiveSpeechStartMessage {
    fn from(value: &CallbackLiveSpeechStartMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveSpeechStartMessage {
    pub fn builder() -> builder::CallbackLiveSpeechStartMessage {
        Default::default()
    }
}
///`CallbackLiveSpeechStartMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.speech_start",
///  "type": "string",
///  "enum": [
///    "live.speech_start"
///  ],
///  "example": "live.speech_start"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveSpeechStartMessageEvent {
    #[serde(rename = "live.speech_start")]
    LiveSpeechStart,
}
impl ::std::convert::From<&Self> for CallbackLiveSpeechStartMessageEvent {
    fn from(value: &CallbackLiveSpeechStartMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveSpeechStartMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveSpeechStart => f.write_str("live.speech_start"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveSpeechStartMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.speech_start" => Ok(Self::LiveSpeechStart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveSpeechStartMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveSpeechStartMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveSpeechStartMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveSpeechStartMessageEvent {
    fn default() -> Self {
        CallbackLiveSpeechStartMessageEvent::LiveSpeechStart
    }
}
///`CallbackLiveStartRecordingMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.start_recording",
///      "type": "string",
///      "enum": [
///        "live.start_recording"
///      ],
///      "example": "live.start_recording"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StartRecordingMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveStartRecordingMessage {
    pub event: CallbackLiveStartRecordingMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: StartRecordingMessage,
}
impl ::std::convert::From<&CallbackLiveStartRecordingMessage>
for CallbackLiveStartRecordingMessage {
    fn from(value: &CallbackLiveStartRecordingMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveStartRecordingMessage {
    pub fn builder() -> builder::CallbackLiveStartRecordingMessage {
        Default::default()
    }
}
///`CallbackLiveStartRecordingMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.start_recording",
///  "type": "string",
///  "enum": [
///    "live.start_recording"
///  ],
///  "example": "live.start_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveStartRecordingMessageEvent {
    #[serde(rename = "live.start_recording")]
    LiveStartRecording,
}
impl ::std::convert::From<&Self> for CallbackLiveStartRecordingMessageEvent {
    fn from(value: &CallbackLiveStartRecordingMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveStartRecordingMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveStartRecording => f.write_str("live.start_recording"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveStartRecordingMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.start_recording" => Ok(Self::LiveStartRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveStartRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveStartRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveStartRecordingMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveStartRecordingMessageEvent {
    fn default() -> Self {
        CallbackLiveStartRecordingMessageEvent::LiveStartRecording
    }
}
///`CallbackLiveStartSessionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.start_session",
///      "type": "string",
///      "enum": [
///        "live.start_session"
///      ],
///      "example": "live.start_session"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StartSessionMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveStartSessionMessage {
    pub event: CallbackLiveStartSessionMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: StartSessionMessage,
}
impl ::std::convert::From<&CallbackLiveStartSessionMessage>
for CallbackLiveStartSessionMessage {
    fn from(value: &CallbackLiveStartSessionMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveStartSessionMessage {
    pub fn builder() -> builder::CallbackLiveStartSessionMessage {
        Default::default()
    }
}
///`CallbackLiveStartSessionMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.start_session",
///  "type": "string",
///  "enum": [
///    "live.start_session"
///  ],
///  "example": "live.start_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveStartSessionMessageEvent {
    #[serde(rename = "live.start_session")]
    LiveStartSession,
}
impl ::std::convert::From<&Self> for CallbackLiveStartSessionMessageEvent {
    fn from(value: &CallbackLiveStartSessionMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveStartSessionMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveStartSession => f.write_str("live.start_session"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveStartSessionMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.start_session" => Ok(Self::LiveStartSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveStartSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveStartSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveStartSessionMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveStartSessionMessageEvent {
    fn default() -> Self {
        CallbackLiveStartSessionMessageEvent::LiveStartSession
    }
}
///`CallbackLiveStopRecordingAckMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.stop_recording",
///      "type": "string",
///      "enum": [
///        "live.stop_recording"
///      ],
///      "example": "live.stop_recording"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StopRecordingAckMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveStopRecordingAckMessage {
    pub event: CallbackLiveStopRecordingAckMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: StopRecordingAckMessage,
}
impl ::std::convert::From<&CallbackLiveStopRecordingAckMessage>
for CallbackLiveStopRecordingAckMessage {
    fn from(value: &CallbackLiveStopRecordingAckMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveStopRecordingAckMessage {
    pub fn builder() -> builder::CallbackLiveStopRecordingAckMessage {
        Default::default()
    }
}
///`CallbackLiveStopRecordingAckMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.stop_recording",
///  "type": "string",
///  "enum": [
///    "live.stop_recording"
///  ],
///  "example": "live.stop_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveStopRecordingAckMessageEvent {
    #[serde(rename = "live.stop_recording")]
    LiveStopRecording,
}
impl ::std::convert::From<&Self> for CallbackLiveStopRecordingAckMessageEvent {
    fn from(value: &CallbackLiveStopRecordingAckMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveStopRecordingAckMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveStopRecording => f.write_str("live.stop_recording"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveStopRecordingAckMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.stop_recording" => Ok(Self::LiveStopRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveStopRecordingAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveStopRecordingAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveStopRecordingAckMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveStopRecordingAckMessageEvent {
    fn default() -> Self {
        CallbackLiveStopRecordingAckMessageEvent::LiveStopRecording
    }
}
///`CallbackLiveTranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.transcript",
///      "type": "string",
///      "enum": [
///        "live.transcript"
///      ],
///      "example": "live.transcript"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveTranscriptMessage {
    pub event: CallbackLiveTranscriptMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: TranscriptMessage,
}
impl ::std::convert::From<&CallbackLiveTranscriptMessage>
for CallbackLiveTranscriptMessage {
    fn from(value: &CallbackLiveTranscriptMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveTranscriptMessage {
    pub fn builder() -> builder::CallbackLiveTranscriptMessage {
        Default::default()
    }
}
///`CallbackLiveTranscriptMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.transcript",
///  "type": "string",
///  "enum": [
///    "live.transcript"
///  ],
///  "example": "live.transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveTranscriptMessageEvent {
    #[serde(rename = "live.transcript")]
    LiveTranscript,
}
impl ::std::convert::From<&Self> for CallbackLiveTranscriptMessageEvent {
    fn from(value: &CallbackLiveTranscriptMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveTranscriptMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveTranscript => f.write_str("live.transcript"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveTranscriptMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.transcript" => Ok(Self::LiveTranscript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveTranscriptMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveTranscriptMessageEvent {
    fn default() -> Self {
        CallbackLiveTranscriptMessageEvent::LiveTranscript
    }
}
///`CallbackLiveTranslationMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.translation",
///      "type": "string",
///      "enum": [
///        "live.translation"
///      ],
///      "example": "live.translation"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "The live message payload as sent to the WebSocket",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationMessage"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackLiveTranslationMessage {
    pub event: CallbackLiveTranslationMessageEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The live message payload as sent to the WebSocket
    pub payload: TranslationMessage,
}
impl ::std::convert::From<&CallbackLiveTranslationMessage>
for CallbackLiveTranslationMessage {
    fn from(value: &CallbackLiveTranslationMessage) -> Self {
        value.clone()
    }
}
impl CallbackLiveTranslationMessage {
    pub fn builder() -> builder::CallbackLiveTranslationMessage {
        Default::default()
    }
}
///`CallbackLiveTranslationMessageEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.translation",
///  "type": "string",
///  "enum": [
///    "live.translation"
///  ],
///  "example": "live.translation"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackLiveTranslationMessageEvent {
    #[serde(rename = "live.translation")]
    LiveTranslation,
}
impl ::std::convert::From<&Self> for CallbackLiveTranslationMessageEvent {
    fn from(value: &CallbackLiveTranslationMessageEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackLiveTranslationMessageEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveTranslation => f.write_str("live.translation"),
        }
    }
}
impl ::std::str::FromStr for CallbackLiveTranslationMessageEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.translation" => Ok(Self::LiveTranslation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackLiveTranslationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackLiveTranslationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackLiveTranslationMessageEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackLiveTranslationMessageEvent {
    fn default() -> Self {
        CallbackLiveTranslationMessageEvent::LiveTranslation
    }
}
///The HTTP method to be used. Allowed values are `POST` or `PUT` (default: `POST`)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The HTTP method to be used. Allowed values are `POST` or `PUT` (default: `POST`)",
///  "type": "string",
///  "enum": [
///    "POST",
///    "PUT"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackMethodEnum {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
}
impl ::std::convert::From<&Self> for CallbackMethodEnum {
    fn from(value: &CallbackMethodEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackMethodEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Post => f.write_str("POST"),
            Self::Put => f.write_str("PUT"),
        }
    }
}
impl ::std::str::FromStr for CallbackMethodEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackMethodEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CallbackMethodEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallbackMethodEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CallbackTranscriptionErrorPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "event",
///    "id"
///  ],
///  "properties": {
///    "custom_metadata": {
///      "description": "Custom metadata given in the initial request",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      },
///      "nullable": true
///    },
///    "error": {
///      "description": "The error that occurred during the transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/ErrorDTO"
///        }
///      ]
///    },
///    "event": {
///      "description": "Type of event",
///      "default": "transcription.error",
///      "type": "string",
///      "enum": [
///        "transcription.error"
///      ],
///      "example": "transcription.error"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackTranscriptionErrorPayload {
    ///Custom metadata given in the initial request
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///The error that occurred during the transcription
    pub error: ErrorDto,
    ///Type of event
    pub event: CallbackTranscriptionErrorPayloadEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
}
impl ::std::convert::From<&CallbackTranscriptionErrorPayload>
for CallbackTranscriptionErrorPayload {
    fn from(value: &CallbackTranscriptionErrorPayload) -> Self {
        value.clone()
    }
}
impl CallbackTranscriptionErrorPayload {
    pub fn builder() -> builder::CallbackTranscriptionErrorPayload {
        Default::default()
    }
}
///Type of event
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Type of event",
///  "default": "transcription.error",
///  "type": "string",
///  "enum": [
///    "transcription.error"
///  ],
///  "example": "transcription.error"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackTranscriptionErrorPayloadEvent {
    #[serde(rename = "transcription.error")]
    TranscriptionError,
}
impl ::std::convert::From<&Self> for CallbackTranscriptionErrorPayloadEvent {
    fn from(value: &CallbackTranscriptionErrorPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackTranscriptionErrorPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptionError => f.write_str("transcription.error"),
        }
    }
}
impl ::std::str::FromStr for CallbackTranscriptionErrorPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription.error" => Ok(Self::TranscriptionError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackTranscriptionErrorPayloadEvent {
    fn default() -> Self {
        CallbackTranscriptionErrorPayloadEvent::TranscriptionError
    }
}
///`CallbackTranscriptionSuccessPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "id",
///    "payload"
///  ],
///  "properties": {
///    "custom_metadata": {
///      "description": "Custom metadata given in the initial request",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      },
///      "nullable": true
///    },
///    "event": {
///      "description": "Type of event",
///      "default": "transcription.success",
///      "type": "string",
///      "enum": [
///        "transcription.success"
///      ],
///      "example": "transcription.success"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "payload": {
///      "description": "Result of the transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionResultDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CallbackTranscriptionSuccessPayload {
    ///Custom metadata given in the initial request
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Type of event
    pub event: CallbackTranscriptionSuccessPayloadEvent,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///Result of the transcription
    pub payload: TranscriptionResultDto,
}
impl ::std::convert::From<&CallbackTranscriptionSuccessPayload>
for CallbackTranscriptionSuccessPayload {
    fn from(value: &CallbackTranscriptionSuccessPayload) -> Self {
        value.clone()
    }
}
impl CallbackTranscriptionSuccessPayload {
    pub fn builder() -> builder::CallbackTranscriptionSuccessPayload {
        Default::default()
    }
}
///Type of event
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Type of event",
///  "default": "transcription.success",
///  "type": "string",
///  "enum": [
///    "transcription.success"
///  ],
///  "example": "transcription.success"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CallbackTranscriptionSuccessPayloadEvent {
    #[serde(rename = "transcription.success")]
    TranscriptionSuccess,
}
impl ::std::convert::From<&Self> for CallbackTranscriptionSuccessPayloadEvent {
    fn from(value: &CallbackTranscriptionSuccessPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CallbackTranscriptionSuccessPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptionSuccess => f.write_str("transcription.success"),
        }
    }
}
impl ::std::str::FromStr for CallbackTranscriptionSuccessPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription.success" => Ok(Self::TranscriptionSuccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallbackTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CallbackTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CallbackTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for CallbackTranscriptionSuccessPayloadEvent {
    fn default() -> Self {
        CallbackTranscriptionSuccessPayloadEvent::TranscriptionSuccess
    }
}
///`CustomSpellingConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "spelling_dictionary"
///  ],
///  "properties": {
///    "spelling_dictionary": {
///      "description": "The list of spelling applied on the audio transcription",
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      },
///      "example": {
///        "Gettleman": [
///          "gettleman"
///        ],
///        "SQL": [
///          "Sequel"
///        ]
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CustomSpellingConfigDto {
    ///The list of spelling applied on the audio transcription
    pub spelling_dictionary: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
}
impl ::std::convert::From<&CustomSpellingConfigDto> for CustomSpellingConfigDto {
    fn from(value: &CustomSpellingConfigDto) -> Self {
        value.clone()
    }
}
impl CustomSpellingConfigDto {
    pub fn builder() -> builder::CustomSpellingConfigDto {
        Default::default()
    }
}
///`CustomVocabularyConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "vocabulary"
///  ],
///  "properties": {
///    "default_intensity": {
///      "description": "Default intensity for the custom vocabulary",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "example": 0.5
///    },
///    "vocabulary": {
///      "description": "Specific vocabulary list to feed the transcription model with. Each item can be a string or an object with the following properties: value, intensity, pronunciations, language.",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "$ref": "#/$defs/CustomVocabularyEntryDTO"
///          },
///          {
///            "type": "string"
///          }
///        ]
///      },
///      "example": [
///        "Westeros",
///        {
///          "value": "Stark"
///        },
///        {
///          "value": "Night's Watch",
///          "pronunciations": [
///            "Nightz Watch"
///          ],
///          "intensity": 0.4,
///          "language": "en"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CustomVocabularyConfigDto {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default_intensity: ::std::option::Option<f64>,
    ///Specific vocabulary list to feed the transcription model with. Each item can be a string or an object with the following properties: value, intensity, pronunciations, language.
    pub vocabulary: ::std::vec::Vec<CustomVocabularyConfigDtoVocabularyItem>,
}
impl ::std::convert::From<&CustomVocabularyConfigDto> for CustomVocabularyConfigDto {
    fn from(value: &CustomVocabularyConfigDto) -> Self {
        value.clone()
    }
}
impl CustomVocabularyConfigDto {
    pub fn builder() -> builder::CustomVocabularyConfigDto {
        Default::default()
    }
}
///`CustomVocabularyConfigDtoVocabularyItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "$ref": "#/$defs/CustomVocabularyEntryDTO"
///    },
///    {
///      "type": "string"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomVocabularyConfigDtoVocabularyItem {
    Variant0(CustomVocabularyEntryDto),
    Variant1(::std::string::String),
}
impl ::std::convert::From<&Self> for CustomVocabularyConfigDtoVocabularyItem {
    fn from(value: &CustomVocabularyConfigDtoVocabularyItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CustomVocabularyEntryDto>
for CustomVocabularyConfigDtoVocabularyItem {
    fn from(value: CustomVocabularyEntryDto) -> Self {
        Self::Variant0(value)
    }
}
///`CustomVocabularyEntryDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "value"
///  ],
///  "properties": {
///    "intensity": {
///      "description": "The global intensity of the feature.",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "example": 0.5
///    },
///    "language": {
///      "description": "Specify the language in which it will be pronounced when sound comparison occurs. Default to transcription language.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionLanguageCodeEnum"
///        }
///      ],
///      "example": "en"
///    },
///    "pronunciations": {
///      "description": "The pronunciations used in the transcription.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "value": {
///      "description": "The text used to replace in the transcription.",
///      "type": "string",
///      "example": "Gladia"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CustomVocabularyEntryDto {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intensity: ::std::option::Option<f64>,
    ///Specify the language in which it will be pronounced when sound comparison occurs. Default to transcription language.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<TranscriptionLanguageCodeEnum>,
    ///The pronunciations used in the transcription.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub pronunciations: ::std::vec::Vec<::std::string::String>,
    ///The text used to replace in the transcription.
    pub value: ::std::string::String,
}
impl ::std::convert::From<&CustomVocabularyEntryDto> for CustomVocabularyEntryDto {
    fn from(value: &CustomVocabularyEntryDto) -> Self {
        value.clone()
    }
}
impl CustomVocabularyEntryDto {
    pub fn builder() -> builder::CustomVocabularyEntryDto {
        Default::default()
    }
}
///`DiarizationConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "max_speakers": {
///      "description": "Maximum number of speakers in the audio",
///      "type": "integer",
///      "minimum": 0.0,
///      "example": 2
///    },
///    "min_speakers": {
///      "description": "Minimum number of speakers in the audio",
///      "type": "integer",
///      "minimum": 0.0,
///      "example": 1
///    },
///    "number_of_speakers": {
///      "description": "Exact number of speakers in the audio",
///      "type": "integer",
///      "minimum": 1.0,
///      "example": 3
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DiarizationConfigDto {
    ///Maximum number of speakers in the audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_speakers: ::std::option::Option<u64>,
    ///Minimum number of speakers in the audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_speakers: ::std::option::Option<u64>,
    ///Exact number of speakers in the audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number_of_speakers: ::std::option::Option<::std::num::NonZeroU64>,
}
impl ::std::convert::From<&DiarizationConfigDto> for DiarizationConfigDto {
    fn from(value: &DiarizationConfigDto) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DiarizationConfigDto {
    fn default() -> Self {
        Self {
            max_speakers: Default::default(),
            min_speakers: Default::default(),
            number_of_speakers: Default::default(),
        }
    }
}
impl DiarizationConfigDto {
    pub fn builder() -> builder::DiarizationConfigDto {
        Default::default()
    }
}
///`DiarizationDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "[Deprecated] If `diarization` has been enabled, the diarization result will appear here",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UtteranceDTO"
///      }
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DiarizationDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///[Deprecated] If `diarization` has been enabled, the diarization result will appear here
    pub results: ::std::vec::Vec<UtteranceDto>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&DiarizationDto> for DiarizationDto {
    fn from(value: &DiarizationDto) -> Self {
        value.clone()
    }
}
impl DiarizationDto {
    pub fn builder() -> builder::DiarizationDto {
        Default::default()
    }
}
///`DisplayModeDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `display_mode` has been enabled, proposes an alternative display output.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DisplayModeDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `display_mode` has been enabled, proposes an alternative display output.
    pub results: ::std::vec::Vec<::std::string::String>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&DisplayModeDto> for DisplayModeDto {
    fn from(value: &DisplayModeDto) -> Self {
        value.clone()
    }
}
impl DisplayModeDto {
    pub fn builder() -> builder::DisplayModeDto {
        Default::default()
    }
}
///`EndRecordingMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/EndRecordingMessageData"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "end_recording",
///      "type": "string",
///      "enum": [
///        "end_recording"
///      ],
///      "example": "end_recording"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct EndRecordingMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: EndRecordingMessageData,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: EndRecordingMessageType,
}
impl ::std::convert::From<&EndRecordingMessage> for EndRecordingMessage {
    fn from(value: &EndRecordingMessage) -> Self {
        value.clone()
    }
}
impl EndRecordingMessage {
    pub fn builder() -> builder::EndRecordingMessage {
        Default::default()
    }
}
///`EndRecordingMessageData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "recording_duration"
///  ],
///  "properties": {
///    "recording_duration": {
///      "description": "Total audio duration in seconds",
///      "type": "number",
///      "example": 344.45
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct EndRecordingMessageData {
    pub recording_duration: f64,
}
impl ::std::convert::From<&EndRecordingMessageData> for EndRecordingMessageData {
    fn from(value: &EndRecordingMessageData) -> Self {
        value.clone()
    }
}
impl EndRecordingMessageData {
    pub fn builder() -> builder::EndRecordingMessageData {
        Default::default()
    }
}
///`EndRecordingMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "end_recording",
///  "type": "string",
///  "enum": [
///    "end_recording"
///  ],
///  "example": "end_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum EndRecordingMessageType {
    #[serde(rename = "end_recording")]
    EndRecording,
}
impl ::std::convert::From<&Self> for EndRecordingMessageType {
    fn from(value: &EndRecordingMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EndRecordingMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EndRecording => f.write_str("end_recording"),
        }
    }
}
impl ::std::str::FromStr for EndRecordingMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "end_recording" => Ok(Self::EndRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EndRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EndRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EndRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for EndRecordingMessageType {
    fn default() -> Self {
        EndRecordingMessageType::EndRecording
    }
}
///`EndSessionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "end_session",
///      "type": "string",
///      "enum": [
///        "end_session"
///      ],
///      "example": "end_session"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct EndSessionMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: EndSessionMessageType,
}
impl ::std::convert::From<&EndSessionMessage> for EndSessionMessage {
    fn from(value: &EndSessionMessage) -> Self {
        value.clone()
    }
}
impl EndSessionMessage {
    pub fn builder() -> builder::EndSessionMessage {
        Default::default()
    }
}
///`EndSessionMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "end_session",
///  "type": "string",
///  "enum": [
///    "end_session"
///  ],
///  "example": "end_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum EndSessionMessageType {
    #[serde(rename = "end_session")]
    EndSession,
}
impl ::std::convert::From<&Self> for EndSessionMessageType {
    fn from(value: &EndSessionMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EndSessionMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EndSession => f.write_str("end_session"),
        }
    }
}
impl ::std::str::FromStr for EndSessionMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "end_session" => Ok(Self::EndSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EndSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EndSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EndSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for EndSessionMessageType {
    fn default() -> Self {
        EndSessionMessageType::EndSession
    }
}
///`Error`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message"
///  ],
///  "properties": {
///    "message": {
///      "description": "The error message",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct Error {
    ///The error message
    pub message: ::std::string::String,
}
impl ::std::convert::From<&Error> for Error {
    fn from(value: &Error) -> Self {
        value.clone()
    }
}
impl Error {
    pub fn builder() -> builder::Error {
        Default::default()
    }
}
///`ErrorDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "code",
///    "message"
///  ],
///  "properties": {
///    "code": {
///      "description": "Error code",
///      "type": "integer",
///      "example": 400
///    },
///    "message": {
///      "description": "Error message",
///      "type": "string",
///      "example": "Bad Request"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ErrorDto {
    ///Error code
    pub code: i64,
    ///Error message
    pub message: ::std::string::String,
}
impl ::std::convert::From<&ErrorDto> for ErrorDto {
    fn from(value: &ErrorDto) -> Self {
        value.clone()
    }
}
impl ErrorDto {
    pub fn builder() -> builder::ErrorDto {
        Default::default()
    }
}
///`FileResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_duration",
///    "filename",
///    "id",
///    "number_of_channels",
///    "source"
///  ],
///  "properties": {
///    "audio_duration": {
///      "description": "Duration of the audio file",
///      "type": "number",
///      "example": 3600,
///      "nullable": true
///    },
///    "filename": {
///      "description": "The name of the uploaded file",
///      "type": "string",
///      "nullable": true
///    },
///    "id": {
///      "description": "The file id",
///      "type": "string"
///    },
///    "number_of_channels": {
///      "description": "Number of channels in the audio file",
///      "type": "integer",
///      "minimum": 1.0,
///      "example": 1,
///      "nullable": true
///    },
///    "source": {
///      "description": "The link used to download the file if audio_url was used",
///      "type": "string",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct FileResponse {
    pub audio_duration: f64,
    ///The name of the uploaded file
    pub filename: ::std::string::String,
    ///The file id
    pub id: ::std::string::String,
    ///Number of channels in the audio file
    pub number_of_channels: ::std::num::NonZeroU64,
    ///The link used to download the file if audio_url was used
    pub source: ::std::string::String,
}
impl ::std::convert::From<&FileResponse> for FileResponse {
    fn from(value: &FileResponse) -> Self {
        value.clone()
    }
}
impl FileResponse {
    pub fn builder() -> builder::FileResponse {
        Default::default()
    }
}
///`ForbiddenErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Forbidden request",
///      "type": "string",
///      "example": "Invalid parameter"
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 403
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ForbiddenErrorResponse {
    ///Forbidden request
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&ForbiddenErrorResponse> for ForbiddenErrorResponse {
    fn from(value: &ForbiddenErrorResponse) -> Self {
        value.clone()
    }
}
impl ForbiddenErrorResponse {
    pub fn builder() -> builder::ForbiddenErrorResponse {
        Default::default()
    }
}
///`GladiaApi`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GladiaApi"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct GladiaApi(pub ::serde_json::Value);
impl ::std::ops::Deref for GladiaApi {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<GladiaApi> for ::serde_json::Value {
    fn from(value: GladiaApi) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GladiaApi> for GladiaApi {
    fn from(value: &GladiaApi) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for GladiaApi {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`InitPreRecordedTranscriptionResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "result_url"
///  ],
///  "properties": {
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "result_url": {
///      "description": "Prebuilt URL with your transcription `id` to fetch the result",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct InitPreRecordedTranscriptionResponse {
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///Prebuilt URL with your transcription `id` to fetch the result
    pub result_url: ::std::string::String,
}
impl ::std::convert::From<&InitPreRecordedTranscriptionResponse>
for InitPreRecordedTranscriptionResponse {
    fn from(value: &InitPreRecordedTranscriptionResponse) -> Self {
        value.clone()
    }
}
impl InitPreRecordedTranscriptionResponse {
    pub fn builder() -> builder::InitPreRecordedTranscriptionResponse {
        Default::default()
    }
}
///`InitStreamingResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "id",
///    "url"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Creation date",
///      "type": "string",
///      "format": "date-time",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "url": {
///      "description": "The websocket url to connect to for sending audio data. The url will contain the temporary token to authenticate the session.",
///      "type": "string",
///      "format": "uri",
///      "example": "wss://api.gladia.io/v2/live?token=4a39145c-2844-4557-8f34-34883f7be7d9"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct InitStreamingResponse {
    ///Creation date
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Id of the job
    pub id: ::uuid::Uuid,
    ///The websocket url to connect to for sending audio data. The url will contain the temporary token to authenticate the session.
    pub url: ::std::string::String,
}
impl ::std::convert::From<&InitStreamingResponse> for InitStreamingResponse {
    fn from(value: &InitStreamingResponse) -> Self {
        value.clone()
    }
}
impl InitStreamingResponse {
    pub fn builder() -> builder::InitStreamingResponse {
        Default::default()
    }
}
///`InitTranscriptionRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_url"
///  ],
///  "properties": {
///    "audio_to_llm": {
///      "description": "Enable audio to LLM processing for this audio",
///      "type": "boolean"
///    },
///    "audio_to_llm_config": {
///      "description": "Audio to LLM configuration, if `audio_to_llm` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioToLlmListConfigDTO"
///        }
///      ]
///    },
///    "audio_url": {
///      "description": "URL to a Gladia file or to an external audio or video file",
///      "type": "string",
///      "format": "uri",
///      "example": "https://files.gladia.io/example/audio-transcription/split_infinity.wav"
///    },
///    "callback": {
///      "description": "Enable callback for this transcription. If true, the `callback_config` property will be used to customize the callback behaviour",
///      "type": "boolean"
///    },
///    "callback_config": {
///      "description": "Customize the callback behaviour (url and http method)",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CallbackConfigDto"
///        }
///      ]
///    },
///    "callback_url": {
///      "description": "**[Deprecated]** Use `callback`/`callback_config` instead. Callback URL we will do a `POST` request to with the result of the transcription",
///      "deprecated": true,
///      "type": "string",
///      "format": "uri",
///      "example": "https://callback.example"
///    },
///    "custom_metadata": {
///      "description": "Custom metadata you can attach to this transcription",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      }
///    },
///    "custom_spelling": {
///      "description": "**[Alpha]** Enable custom spelling for this audio",
///      "type": "boolean"
///    },
///    "custom_spelling_config": {
///      "description": "**[Alpha]** Custom spelling configuration, if `custom_spelling` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomSpellingConfigDTO"
///        }
///      ]
///    },
///    "custom_vocabulary": {
///      "description": "**[Beta]** Can be either boolean to enable custom_vocabulary for this audio or an array with specific vocabulary list to feed the transcription model with",
///      "type": "boolean"
///    },
///    "custom_vocabulary_config": {
///      "description": "**[Beta]** Custom vocabulary configuration, if `custom_vocabulary` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomVocabularyConfigDTO"
///        }
///      ]
///    },
///    "diarization": {
///      "description": "Enable speaker recognition (diarization) for this audio",
///      "type": "boolean"
///    },
///    "diarization_config": {
///      "description": "Speaker recognition configuration, if `diarization` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/DiarizationConfigDTO"
///        }
///      ]
///    },
///    "language_config": {
///      "description": "Specify the language configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/LanguageConfig"
///        }
///      ]
///    },
///    "model": {
///      "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionSupportedModels"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "**[Alpha]** Enable named entity recognition for this audio",
///      "type": "boolean"
///    },
///    "pii_redaction": {
///      "description": "Enable PII redaction for this audio",
///      "type": "boolean"
///    },
///    "pii_redaction_config": {
///      "description": "PII redaction configuration, if `pii_redaction` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PiiRedactionConfigDTO"
///        }
///      ]
///    },
///    "punctuation_enhanced": {
///      "description": "**[Alpha]** Use enhanced punctuation for this audio",
///      "type": "boolean"
///    },
///    "sentences": {
///      "description": "Enable sentences for this audio",
///      "type": "boolean"
///    },
///    "sentiment_analysis": {
///      "description": "Enable sentiment analysis for this audio",
///      "type": "boolean"
///    },
///    "subtitles": {
///      "description": "Enable subtitles generation for this transcription",
///      "type": "boolean"
///    },
///    "subtitles_config": {
///      "description": "Configuration for subtitles generation if `subtitles` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SubtitlesConfigDTO"
///        }
///      ]
///    },
///    "summarization": {
///      "description": "Enable summarization for this audio",
///      "type": "boolean"
///    },
///    "summarization_config": {
///      "description": "Summarization configuration, if `summarization` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationConfigDTO"
///        }
///      ]
///    },
///    "translation": {
///      "description": "**[Beta]** Enable translation for this audio",
///      "type": "boolean"
///    },
///    "translation_config": {
///      "description": "**[Beta]** Translation configuration, if `translation` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationConfigDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct InitTranscriptionRequest {
    ///Enable audio to LLM processing for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_to_llm: ::std::option::Option<bool>,
    ///Audio to LLM configuration, if `audio_to_llm` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_to_llm_config: ::std::option::Option<AudioToLlmListConfigDto>,
    ///URL to a Gladia file or to an external audio or video file
    pub audio_url: ::std::string::String,
    ///Enable callback for this transcription. If true, the `callback_config` property will be used to customize the callback behaviour
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback: ::std::option::Option<bool>,
    ///Customize the callback behaviour (url and http method)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_config: ::std::option::Option<CallbackConfigDto>,
    ///**[Deprecated]** Use `callback`/`callback_config` instead. Callback URL we will do a `POST` request to with the result of the transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_url: ::std::option::Option<::std::string::String>,
    ///Custom metadata you can attach to this transcription
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///**[Alpha]** Enable custom spelling for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling: ::std::option::Option<bool>,
    ///**[Alpha]** Custom spelling configuration, if `custom_spelling` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling_config: ::std::option::Option<CustomSpellingConfigDto>,
    ///**[Beta]** Can be either boolean to enable custom_vocabulary for this audio or an array with specific vocabulary list to feed the transcription model with
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary: ::std::option::Option<bool>,
    ///**[Beta]** Custom vocabulary configuration, if `custom_vocabulary` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary_config: ::std::option::Option<CustomVocabularyConfigDto>,
    ///Enable speaker recognition (diarization) for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diarization: ::std::option::Option<bool>,
    ///Speaker recognition configuration, if `diarization` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diarization_config: ::std::option::Option<DiarizationConfigDto>,
    ///Specify the language configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language_config: ::std::option::Option<LanguageConfig>,
    ///The model used to process the audio. "solaria-1" is used by default.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<TranscriptionSupportedModels>,
    ///**[Alpha]** Enable named entity recognition for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<bool>,
    ///Enable PII redaction for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pii_redaction: ::std::option::Option<bool>,
    ///PII redaction configuration, if `pii_redaction` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pii_redaction_config: ::std::option::Option<PiiRedactionConfigDto>,
    ///**[Alpha]** Use enhanced punctuation for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub punctuation_enhanced: ::std::option::Option<bool>,
    ///Enable sentences for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentences: ::std::option::Option<bool>,
    ///Enable sentiment analysis for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<bool>,
    ///Enable subtitles generation for this transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtitles: ::std::option::Option<bool>,
    ///Configuration for subtitles generation if `subtitles` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtitles_config: ::std::option::Option<SubtitlesConfigDto>,
    ///Enable summarization for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<bool>,
    ///Summarization configuration, if `summarization` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization_config: ::std::option::Option<SummarizationConfigDto>,
    ///**[Beta]** Enable translation for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<bool>,
    ///**[Beta]** Translation configuration, if `translation` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation_config: ::std::option::Option<TranslationConfigDto>,
}
impl ::std::convert::From<&InitTranscriptionRequest> for InitTranscriptionRequest {
    fn from(value: &InitTranscriptionRequest) -> Self {
        value.clone()
    }
}
impl InitTranscriptionRequest {
    pub fn builder() -> builder::InitTranscriptionRequest {
        Default::default()
    }
}
///`LanguageConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "code_switching": {
///      "description": "If true, language will be auto-detected on each utterance. Otherwise, language will be auto-detected on first utterance and then used for the rest of the transcription. If one language is set, this option will be ignored.",
///      "type": "boolean"
///    },
///    "languages": {
///      "description": "If one language is set, it will be used for the transcription. Otherwise, language will be auto-detected by the model.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TranscriptionLanguageCodeEnum"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LanguageConfig {
    ///If true, language will be auto-detected on each utterance. Otherwise, language will be auto-detected on first utterance and then used for the rest of the transcription. If one language is set, this option will be ignored.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code_switching: ::std::option::Option<bool>,
    ///If one language is set, it will be used for the transcription. Otherwise, language will be auto-detected by the model.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<TranscriptionLanguageCodeEnum>,
}
impl ::std::convert::From<&LanguageConfig> for LanguageConfig {
    fn from(value: &LanguageConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LanguageConfig {
    fn default() -> Self {
        Self {
            code_switching: Default::default(),
            languages: Default::default(),
        }
    }
}
impl LanguageConfig {
    pub fn builder() -> builder::LanguageConfig {
        Default::default()
    }
}
///`ListHistoryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "current",
///    "first",
///    "items",
///    "next"
///  ],
///  "properties": {
///    "current": {
///      "description": "URL to fetch the current page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "first": {
///      "description": "URL to fetch the first page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "items": {
///      "description": "List of jobs",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "$ref": "#/$defs/PreRecordedResponse"
///          },
///          {
///            "$ref": "#/$defs/StreamingResponse"
///          }
///        ]
///      },
///      "discriminator": {
///        "propertyName": "kind",
///        "mapping": {
///          "pre-recorded": "#/$defs/PreRecordedResponse",
///          "live": "#/$defs/StreamingResponse"
///        }
///      }
///    },
///    "next": {
///      "description": "URL to fetch the next page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=20&limit=20",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ListHistoryResponse {
    ///URL to fetch the current page
    pub current: ::std::string::String,
    ///URL to fetch the first page
    pub first: ::std::string::String,
    ///List of jobs
    pub items: ::std::vec::Vec<ListHistoryResponseItemsItem>,
    ///URL to fetch the next page
    pub next: ::std::string::String,
}
impl ::std::convert::From<&ListHistoryResponse> for ListHistoryResponse {
    fn from(value: &ListHistoryResponse) -> Self {
        value.clone()
    }
}
impl ListHistoryResponse {
    pub fn builder() -> builder::ListHistoryResponse {
        Default::default()
    }
}
///`ListHistoryResponseItemsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "$ref": "#/$defs/PreRecordedResponse"
///    },
///    {
///      "$ref": "#/$defs/StreamingResponse"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ListHistoryResponseItemsItem {
    PreRecordedResponse(PreRecordedResponse),
    StreamingResponse(StreamingResponse),
}
impl ::std::convert::From<&Self> for ListHistoryResponseItemsItem {
    fn from(value: &ListHistoryResponseItemsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PreRecordedResponse> for ListHistoryResponseItemsItem {
    fn from(value: PreRecordedResponse) -> Self {
        Self::PreRecordedResponse(value)
    }
}
impl ::std::convert::From<StreamingResponse> for ListHistoryResponseItemsItem {
    fn from(value: StreamingResponse) -> Self {
        Self::StreamingResponse(value)
    }
}
///`ListPreRecordedResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "current",
///    "first",
///    "items",
///    "next"
///  ],
///  "properties": {
///    "current": {
///      "description": "URL to fetch the current page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "first": {
///      "description": "URL to fetch the first page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "items": {
///      "description": "List of pre-recorded transcriptions",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PreRecordedResponse"
///      }
///    },
///    "next": {
///      "description": "URL to fetch the next page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=20&limit=20",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ListPreRecordedResponse {
    ///URL to fetch the current page
    pub current: ::std::string::String,
    ///URL to fetch the first page
    pub first: ::std::string::String,
    ///List of pre-recorded transcriptions
    pub items: ::std::vec::Vec<PreRecordedResponse>,
    ///URL to fetch the next page
    pub next: ::std::string::String,
}
impl ::std::convert::From<&ListPreRecordedResponse> for ListPreRecordedResponse {
    fn from(value: &ListPreRecordedResponse) -> Self {
        value.clone()
    }
}
impl ListPreRecordedResponse {
    pub fn builder() -> builder::ListPreRecordedResponse {
        Default::default()
    }
}
///`ListStreamingResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "current",
///    "first",
///    "items",
///    "next"
///  ],
///  "properties": {
///    "current": {
///      "description": "URL to fetch the current page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "first": {
///      "description": "URL to fetch the first page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "items": {
///      "description": "List of live transcriptions",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/StreamingResponse"
///      }
///    },
///    "next": {
///      "description": "URL to fetch the next page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=20&limit=20",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ListStreamingResponse {
    ///URL to fetch the current page
    pub current: ::std::string::String,
    ///URL to fetch the first page
    pub first: ::std::string::String,
    ///List of live transcriptions
    pub items: ::std::vec::Vec<StreamingResponse>,
    ///URL to fetch the next page
    pub next: ::std::string::String,
}
impl ::std::convert::From<&ListStreamingResponse> for ListStreamingResponse {
    fn from(value: &ListStreamingResponse) -> Self {
        value.clone()
    }
}
impl ListStreamingResponse {
    pub fn builder() -> builder::ListStreamingResponse {
        Default::default()
    }
}
///`ListTranscriptionResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "current",
///    "first",
///    "items",
///    "next"
///  ],
///  "properties": {
///    "current": {
///      "description": "URL to fetch the current page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "first": {
///      "description": "URL to fetch the first page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=0&limit=20"
///    },
///    "items": {
///      "description": "List of transcriptions",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "$ref": "#/$defs/PreRecordedResponse"
///          },
///          {
///            "$ref": "#/$defs/StreamingResponse"
///          }
///        ]
///      },
///      "discriminator": {
///        "propertyName": "kind",
///        "mapping": {
///          "pre-recorded": "#/$defs/PreRecordedResponse",
///          "live": "#/$defs/StreamingResponse"
///        }
///      }
///    },
///    "next": {
///      "description": "URL to fetch the next page",
///      "type": "string",
///      "format": "uri",
///      "example": "https://api.gladia.io/v2/transcription?status=done&offset=20&limit=20",
///      "nullable": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ListTranscriptionResponse {
    ///URL to fetch the current page
    pub current: ::std::string::String,
    ///URL to fetch the first page
    pub first: ::std::string::String,
    ///List of transcriptions
    pub items: ::std::vec::Vec<ListTranscriptionResponseItemsItem>,
    ///URL to fetch the next page
    pub next: ::std::string::String,
}
impl ::std::convert::From<&ListTranscriptionResponse> for ListTranscriptionResponse {
    fn from(value: &ListTranscriptionResponse) -> Self {
        value.clone()
    }
}
impl ListTranscriptionResponse {
    pub fn builder() -> builder::ListTranscriptionResponse {
        Default::default()
    }
}
///`ListTranscriptionResponseItemsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "$ref": "#/$defs/PreRecordedResponse"
///    },
///    {
///      "$ref": "#/$defs/StreamingResponse"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ListTranscriptionResponseItemsItem {
    PreRecordedResponse(PreRecordedResponse),
    StreamingResponse(StreamingResponse),
}
impl ::std::convert::From<&Self> for ListTranscriptionResponseItemsItem {
    fn from(value: &ListTranscriptionResponseItemsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PreRecordedResponse> for ListTranscriptionResponseItemsItem {
    fn from(value: PreRecordedResponse) -> Self {
        Self::PreRecordedResponse(value)
    }
}
impl ::std::convert::From<StreamingResponse> for ListTranscriptionResponseItemsItem {
    fn from(value: StreamingResponse) -> Self {
        Self::StreamingResponse(value)
    }
}
///`LiveEventPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LiveEventPayload {
    ///Id of the job
    pub id: ::uuid::Uuid,
}
impl ::std::convert::From<&LiveEventPayload> for LiveEventPayload {
    fn from(value: &LiveEventPayload) -> Self {
        value.clone()
    }
}
impl LiveEventPayload {
    pub fn builder() -> builder::LiveEventPayload {
        Default::default()
    }
}
///`MessagesConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "receive_acknowledgments": {
///      "description": "If true, acknowledgments will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_errors": {
///      "description": "If true, errors will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_final_transcripts": {
///      "description": "If true, final transcript will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_lifecycle_events": {
///      "description": "If true, lifecycle events will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_partial_transcripts": {
///      "description": "If true, partial transcript will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_post_processing_events": {
///      "description": "If true, post-processing events will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_pre_processing_events": {
///      "description": "If true, pre-processing events will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_realtime_processing_events": {
///      "description": "If true, realtime processing events will be sent to websocket.",
///      "type": "boolean"
///    },
///    "receive_speech_events": {
///      "description": "If true, begin and end speech events will be sent to websocket.",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MessagesConfig {
    ///If true, acknowledgments will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_acknowledgments: ::std::option::Option<bool>,
    ///If true, errors will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_errors: ::std::option::Option<bool>,
    ///If true, final transcript will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_final_transcripts: ::std::option::Option<bool>,
    ///If true, lifecycle events will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_lifecycle_events: ::std::option::Option<bool>,
    ///If true, partial transcript will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_partial_transcripts: ::std::option::Option<bool>,
    ///If true, post-processing events will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_post_processing_events: ::std::option::Option<bool>,
    ///If true, pre-processing events will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_pre_processing_events: ::std::option::Option<bool>,
    ///If true, realtime processing events will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_realtime_processing_events: ::std::option::Option<bool>,
    ///If true, begin and end speech events will be sent to websocket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receive_speech_events: ::std::option::Option<bool>,
}
impl ::std::convert::From<&MessagesConfig> for MessagesConfig {
    fn from(value: &MessagesConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MessagesConfig {
    fn default() -> Self {
        Self {
            receive_acknowledgments: Default::default(),
            receive_errors: Default::default(),
            receive_final_transcripts: Default::default(),
            receive_lifecycle_events: Default::default(),
            receive_partial_transcripts: Default::default(),
            receive_post_processing_events: Default::default(),
            receive_pre_processing_events: Default::default(),
            receive_realtime_processing_events: Default::default(),
            receive_speech_events: Default::default(),
        }
    }
}
impl MessagesConfig {
    pub fn builder() -> builder::MessagesConfig {
        Default::default()
    }
}
///`ModerationDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `moderation` has been enabled, moderated transcription",
///      "type": "string",
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ModerationDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `moderation` has been enabled, moderated transcription
    pub results: ::std::string::String,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&ModerationDto> for ModerationDto {
    fn from(value: &ModerationDto) -> Self {
        value.clone()
    }
}
impl ModerationDto {
    pub fn builder() -> builder::ModerationDto {
        Default::default()
    }
}
///`NamedEntityRecognitionData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "results",
///    "utterance",
///    "utterance_id"
///  ],
///  "properties": {
///    "results": {
///      "description": "The NER results",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/NamedEntityRecognitionResult"
///      }
///    },
///    "utterance": {
///      "description": "The transcribed utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/UtteranceDTO"
///        }
///      ]
///    },
///    "utterance_id": {
///      "description": "Id of the utterance used for this result",
///      "type": "string",
///      "example": "00-00000011"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NamedEntityRecognitionData {
    ///The NER results
    pub results: ::std::vec::Vec<NamedEntityRecognitionResult>,
    ///The transcribed utterance
    pub utterance: UtteranceDto,
    ///Id of the utterance used for this result
    pub utterance_id: ::std::string::String,
}
impl ::std::convert::From<&NamedEntityRecognitionData> for NamedEntityRecognitionData {
    fn from(value: &NamedEntityRecognitionData) -> Self {
        value.clone()
    }
}
impl NamedEntityRecognitionData {
    pub fn builder() -> builder::NamedEntityRecognitionData {
        Default::default()
    }
}
///`NamedEntityRecognitionDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `named_entity_recognition` has been enabled, the detected entities.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/NamedEntityRecognitionResult"
///      },
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NamedEntityRecognitionDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `named_entity_recognition` has been enabled, the detected entities.
    pub results: ::std::vec::Vec<NamedEntityRecognitionResult>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&NamedEntityRecognitionDto> for NamedEntityRecognitionDto {
    fn from(value: &NamedEntityRecognitionDto) -> Self {
        value.clone()
    }
}
impl NamedEntityRecognitionDto {
    pub fn builder() -> builder::NamedEntityRecognitionDto {
        Default::default()
    }
}
///`NamedEntityRecognitionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamedEntityRecognitionData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "named_entity_recognition",
///      "type": "string",
///      "enum": [
///        "named_entity_recognition"
///      ],
///      "example": "named_entity_recognition"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NamedEntityRecognitionMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the addon failed
    pub data: NamedEntityRecognitionData,
    ///Error message if the addon failed
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: NamedEntityRecognitionMessageType,
}
impl ::std::convert::From<&NamedEntityRecognitionMessage>
for NamedEntityRecognitionMessage {
    fn from(value: &NamedEntityRecognitionMessage) -> Self {
        value.clone()
    }
}
impl NamedEntityRecognitionMessage {
    pub fn builder() -> builder::NamedEntityRecognitionMessage {
        Default::default()
    }
}
///`NamedEntityRecognitionMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "named_entity_recognition",
///  "type": "string",
///  "enum": [
///    "named_entity_recognition"
///  ],
///  "example": "named_entity_recognition"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum NamedEntityRecognitionMessageType {
    #[serde(rename = "named_entity_recognition")]
    NamedEntityRecognition,
}
impl ::std::convert::From<&Self> for NamedEntityRecognitionMessageType {
    fn from(value: &NamedEntityRecognitionMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NamedEntityRecognitionMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NamedEntityRecognition => f.write_str("named_entity_recognition"),
        }
    }
}
impl ::std::str::FromStr for NamedEntityRecognitionMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "named_entity_recognition" => Ok(Self::NamedEntityRecognition),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NamedEntityRecognitionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for NamedEntityRecognitionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for NamedEntityRecognitionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for NamedEntityRecognitionMessageType {
    fn default() -> Self {
        NamedEntityRecognitionMessageType::NamedEntityRecognition
    }
}
///`NamedEntityRecognitionResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "entity_type",
///    "start",
///    "text"
///  ],
///  "properties": {
///    "end": {
///      "type": "number"
///    },
///    "entity_type": {
///      "type": "string"
///    },
///    "start": {
///      "type": "number"
///    },
///    "text": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NamedEntityRecognitionResult {
    pub end: f64,
    pub entity_type: ::std::string::String,
    pub start: f64,
    pub text: ::std::string::String,
}
impl ::std::convert::From<&NamedEntityRecognitionResult>
for NamedEntityRecognitionResult {
    fn from(value: &NamedEntityRecognitionResult) -> Self {
        value.clone()
    }
}
impl NamedEntityRecognitionResult {
    pub fn builder() -> builder::NamedEntityRecognitionResult {
        Default::default()
    }
}
///`NamesConsistencyDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "Deprecated, If `name_consistency` has been enabled, Gladia will improve the consistency of the names across the transcription",
///      "type": "string",
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NamesConsistencyDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///Deprecated, If `name_consistency` has been enabled, Gladia will improve the consistency of the names across the transcription
    pub results: ::std::string::String,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&NamesConsistencyDto> for NamesConsistencyDto {
    fn from(value: &NamesConsistencyDto) -> Self {
        value.clone()
    }
}
impl NamesConsistencyDto {
    pub fn builder() -> builder::NamesConsistencyDto {
        Default::default()
    }
}
///`NotFoundErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Error message",
///      "type": "string",
///      "example": "Not found"
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 404
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NotFoundErrorResponse {
    ///Error message
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&NotFoundErrorResponse> for NotFoundErrorResponse {
    fn from(value: &NotFoundErrorResponse) -> Self {
        value.clone()
    }
}
impl NotFoundErrorResponse {
    pub fn builder() -> builder::NotFoundErrorResponse {
        Default::default()
    }
}
///`PatchRequestParamsDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PatchRequestParamsDto(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for PatchRequestParamsDto {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<PatchRequestParamsDto>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: PatchRequestParamsDto) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PatchRequestParamsDto> for PatchRequestParamsDto {
    fn from(value: &PatchRequestParamsDto) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for PatchRequestParamsDto {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`PayloadTooLargeErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Payload too large",
///      "type": "string",
///      "example": "payload too large"
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 413
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PayloadTooLargeErrorResponse {
    ///Payload too large
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&PayloadTooLargeErrorResponse>
for PayloadTooLargeErrorResponse {
    fn from(value: &PayloadTooLargeErrorResponse) -> Self {
        value.clone()
    }
}
impl PayloadTooLargeErrorResponse {
    pub fn builder() -> builder::PayloadTooLargeErrorResponse {
        Default::default()
    }
}
///`PiiRedactionConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "entity_types": {
///      "description": "The entity types to redact",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PiiRedactionEntityTypeEnum"
///        }
///      ],
///      "example": [
///        "GDPR",
///        "HEALTH_INFORMATION",
///        "HIPAA_SAFE_HARBOR",
///        "QUEBEC_PRIVACY_ACT",
///        "EMAIL_ADDRESS",
///        "NAME",
///        "PHONE_NUMBER"
///      ]
///    },
///    "processed_text_type": {
///      "description": "The type of processed text to return (marker or mask)",
///      "type": "string",
///      "enum": [
///        "MARKER",
///        "MASK"
///      ],
///      "example": "MARKER"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PiiRedactionConfigDto {
    ///The entity types to redact
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entity_types: ::std::option::Option<PiiRedactionEntityTypeEnum>,
    ///The type of processed text to return (marker or mask)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub processed_text_type: ::std::option::Option<
        PiiRedactionConfigDtoProcessedTextType,
    >,
}
impl ::std::convert::From<&PiiRedactionConfigDto> for PiiRedactionConfigDto {
    fn from(value: &PiiRedactionConfigDto) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PiiRedactionConfigDto {
    fn default() -> Self {
        Self {
            entity_types: Default::default(),
            processed_text_type: Default::default(),
        }
    }
}
impl PiiRedactionConfigDto {
    pub fn builder() -> builder::PiiRedactionConfigDto {
        Default::default()
    }
}
///The type of processed text to return (marker or mask)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of processed text to return (marker or mask)",
///  "type": "string",
///  "enum": [
///    "MARKER",
///    "MASK"
///  ],
///  "example": "MARKER"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PiiRedactionConfigDtoProcessedTextType {
    #[serde(rename = "MARKER")]
    Marker,
    #[serde(rename = "MASK")]
    Mask,
}
impl ::std::convert::From<&Self> for PiiRedactionConfigDtoProcessedTextType {
    fn from(value: &PiiRedactionConfigDtoProcessedTextType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PiiRedactionConfigDtoProcessedTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Marker => f.write_str("MARKER"),
            Self::Mask => f.write_str("MASK"),
        }
    }
}
impl ::std::str::FromStr for PiiRedactionConfigDtoProcessedTextType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MARKER" => Ok(Self::Marker),
            "MASK" => Ok(Self::Mask),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PiiRedactionConfigDtoProcessedTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PiiRedactionConfigDtoProcessedTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PiiRedactionConfigDtoProcessedTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The entity types to redact
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The entity types to redact",
///  "type": "string",
///  "enum": [
///    "APPI",
///    "APPI_SENSITIVE",
///    "CCI",
///    "CORE_ENTITIES",
///    "CPRA",
///    "GDPR",
///    "GDPR_SENSITIVE",
///    "HEALTH_INFORMATION",
///    "HIPAA_SAFE_HARBOR",
///    "LIDI",
///    "NUMERICAL_EXCL_PCI",
///    "PCI",
///    "QUEBEC_PRIVACY_ACT",
///    "ACCOUNT_NUMBER",
///    "AGE",
///    "DATE",
///    "DATE_INTERVAL",
///    "DOB",
///    "DRIVER_LICENSE",
///    "DURATION",
///    "EMAIL_ADDRESS",
///    "EVENT",
///    "FILENAME",
///    "GENDER",
///    "HEALTHCARE_NUMBER",
///    "IP_ADDRESS",
///    "LANGUAGE",
///    "LOCATION",
///    "LOCATION_ADDRESS",
///    "LOCATION_ADDRESS_STREET",
///    "LOCATION_CITY",
///    "LOCATION_COORDINATE",
///    "LOCATION_COUNTRY",
///    "LOCATION_STATE",
///    "LOCATION_ZIP",
///    "MARITAL_STATUS",
///    "MONEY",
///    "NAME",
///    "NAME_FAMILY",
///    "NAME_GIVEN",
///    "NAME_MEDICAL_PROFESSIONAL",
///    "NUMERICAL_PII",
///    "OCCUPATION",
///    "ORGANIZATION",
///    "ORGANIZATION_MEDICAL_FACILITY",
///    "ORIGIN",
///    "PASSPORT_NUMBER",
///    "PASSWORD",
///    "PHONE_NUMBER",
///    "PHYSICAL_ATTRIBUTE",
///    "POLITICAL_AFFILIATION",
///    "RELIGION",
///    "SEXUALITY",
///    "SSN",
///    "TIME",
///    "URL",
///    "USERNAME",
///    "VEHICLE_ID",
///    "ZODIAC_SIGN",
///    "BLOOD_TYPE",
///    "CONDITION",
///    "DOSE",
///    "DRUG",
///    "INJURY",
///    "MEDICAL_PROCESS",
///    "STATISTICS",
///    "BANK_ACCOUNT",
///    "CREDIT_CARD",
///    "CREDIT_CARD_EXPIRATION",
///    "CVV",
///    "ROUTING_NUMBER",
///    "CORPORATE_ACTION",
///    "DAY",
///    "EFFECT",
///    "FINANCIAL_METRIC",
///    "MEDICAL_CODE",
///    "MONTH",
///    "ORGANIZATION_ID",
///    "PRODUCT",
///    "PROJECT",
///    "TREND",
///    "YEAR"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PiiRedactionEntityTypeEnum {
    #[serde(rename = "APPI")]
    Appi,
    #[serde(rename = "APPI_SENSITIVE")]
    AppiSensitive,
    #[serde(rename = "CCI")]
    Cci,
    #[serde(rename = "CORE_ENTITIES")]
    CoreEntities,
    #[serde(rename = "CPRA")]
    Cpra,
    #[serde(rename = "GDPR")]
    Gdpr,
    #[serde(rename = "GDPR_SENSITIVE")]
    GdprSensitive,
    #[serde(rename = "HEALTH_INFORMATION")]
    HealthInformation,
    #[serde(rename = "HIPAA_SAFE_HARBOR")]
    HipaaSafeHarbor,
    #[serde(rename = "LIDI")]
    Lidi,
    #[serde(rename = "NUMERICAL_EXCL_PCI")]
    NumericalExclPci,
    #[serde(rename = "PCI")]
    Pci,
    #[serde(rename = "QUEBEC_PRIVACY_ACT")]
    QuebecPrivacyAct,
    #[serde(rename = "ACCOUNT_NUMBER")]
    AccountNumber,
    #[serde(rename = "AGE")]
    Age,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "DATE_INTERVAL")]
    DateInterval,
    #[serde(rename = "DOB")]
    Dob,
    #[serde(rename = "DRIVER_LICENSE")]
    DriverLicense,
    #[serde(rename = "DURATION")]
    Duration,
    #[serde(rename = "EMAIL_ADDRESS")]
    EmailAddress,
    #[serde(rename = "EVENT")]
    Event,
    #[serde(rename = "FILENAME")]
    Filename,
    #[serde(rename = "GENDER")]
    Gender,
    #[serde(rename = "HEALTHCARE_NUMBER")]
    HealthcareNumber,
    #[serde(rename = "IP_ADDRESS")]
    IpAddress,
    #[serde(rename = "LANGUAGE")]
    Language,
    #[serde(rename = "LOCATION")]
    Location,
    #[serde(rename = "LOCATION_ADDRESS")]
    LocationAddress,
    #[serde(rename = "LOCATION_ADDRESS_STREET")]
    LocationAddressStreet,
    #[serde(rename = "LOCATION_CITY")]
    LocationCity,
    #[serde(rename = "LOCATION_COORDINATE")]
    LocationCoordinate,
    #[serde(rename = "LOCATION_COUNTRY")]
    LocationCountry,
    #[serde(rename = "LOCATION_STATE")]
    LocationState,
    #[serde(rename = "LOCATION_ZIP")]
    LocationZip,
    #[serde(rename = "MARITAL_STATUS")]
    MaritalStatus,
    #[serde(rename = "MONEY")]
    Money,
    #[serde(rename = "NAME")]
    Name,
    #[serde(rename = "NAME_FAMILY")]
    NameFamily,
    #[serde(rename = "NAME_GIVEN")]
    NameGiven,
    #[serde(rename = "NAME_MEDICAL_PROFESSIONAL")]
    NameMedicalProfessional,
    #[serde(rename = "NUMERICAL_PII")]
    NumericalPii,
    #[serde(rename = "OCCUPATION")]
    Occupation,
    #[serde(rename = "ORGANIZATION")]
    Organization,
    #[serde(rename = "ORGANIZATION_MEDICAL_FACILITY")]
    OrganizationMedicalFacility,
    #[serde(rename = "ORIGIN")]
    Origin,
    #[serde(rename = "PASSPORT_NUMBER")]
    PassportNumber,
    #[serde(rename = "PASSWORD")]
    Password,
    #[serde(rename = "PHONE_NUMBER")]
    PhoneNumber,
    #[serde(rename = "PHYSICAL_ATTRIBUTE")]
    PhysicalAttribute,
    #[serde(rename = "POLITICAL_AFFILIATION")]
    PoliticalAffiliation,
    #[serde(rename = "RELIGION")]
    Religion,
    #[serde(rename = "SEXUALITY")]
    Sexuality,
    #[serde(rename = "SSN")]
    Ssn,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "USERNAME")]
    Username,
    #[serde(rename = "VEHICLE_ID")]
    VehicleId,
    #[serde(rename = "ZODIAC_SIGN")]
    ZodiacSign,
    #[serde(rename = "BLOOD_TYPE")]
    BloodType,
    #[serde(rename = "CONDITION")]
    Condition,
    #[serde(rename = "DOSE")]
    Dose,
    #[serde(rename = "DRUG")]
    Drug,
    #[serde(rename = "INJURY")]
    Injury,
    #[serde(rename = "MEDICAL_PROCESS")]
    MedicalProcess,
    #[serde(rename = "STATISTICS")]
    Statistics,
    #[serde(rename = "BANK_ACCOUNT")]
    BankAccount,
    #[serde(rename = "CREDIT_CARD")]
    CreditCard,
    #[serde(rename = "CREDIT_CARD_EXPIRATION")]
    CreditCardExpiration,
    #[serde(rename = "CVV")]
    Cvv,
    #[serde(rename = "ROUTING_NUMBER")]
    RoutingNumber,
    #[serde(rename = "CORPORATE_ACTION")]
    CorporateAction,
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "EFFECT")]
    Effect,
    #[serde(rename = "FINANCIAL_METRIC")]
    FinancialMetric,
    #[serde(rename = "MEDICAL_CODE")]
    MedicalCode,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "ORGANIZATION_ID")]
    OrganizationId,
    #[serde(rename = "PRODUCT")]
    Product,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "TREND")]
    Trend,
    #[serde(rename = "YEAR")]
    Year,
}
impl ::std::convert::From<&Self> for PiiRedactionEntityTypeEnum {
    fn from(value: &PiiRedactionEntityTypeEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PiiRedactionEntityTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Appi => f.write_str("APPI"),
            Self::AppiSensitive => f.write_str("APPI_SENSITIVE"),
            Self::Cci => f.write_str("CCI"),
            Self::CoreEntities => f.write_str("CORE_ENTITIES"),
            Self::Cpra => f.write_str("CPRA"),
            Self::Gdpr => f.write_str("GDPR"),
            Self::GdprSensitive => f.write_str("GDPR_SENSITIVE"),
            Self::HealthInformation => f.write_str("HEALTH_INFORMATION"),
            Self::HipaaSafeHarbor => f.write_str("HIPAA_SAFE_HARBOR"),
            Self::Lidi => f.write_str("LIDI"),
            Self::NumericalExclPci => f.write_str("NUMERICAL_EXCL_PCI"),
            Self::Pci => f.write_str("PCI"),
            Self::QuebecPrivacyAct => f.write_str("QUEBEC_PRIVACY_ACT"),
            Self::AccountNumber => f.write_str("ACCOUNT_NUMBER"),
            Self::Age => f.write_str("AGE"),
            Self::Date => f.write_str("DATE"),
            Self::DateInterval => f.write_str("DATE_INTERVAL"),
            Self::Dob => f.write_str("DOB"),
            Self::DriverLicense => f.write_str("DRIVER_LICENSE"),
            Self::Duration => f.write_str("DURATION"),
            Self::EmailAddress => f.write_str("EMAIL_ADDRESS"),
            Self::Event => f.write_str("EVENT"),
            Self::Filename => f.write_str("FILENAME"),
            Self::Gender => f.write_str("GENDER"),
            Self::HealthcareNumber => f.write_str("HEALTHCARE_NUMBER"),
            Self::IpAddress => f.write_str("IP_ADDRESS"),
            Self::Language => f.write_str("LANGUAGE"),
            Self::Location => f.write_str("LOCATION"),
            Self::LocationAddress => f.write_str("LOCATION_ADDRESS"),
            Self::LocationAddressStreet => f.write_str("LOCATION_ADDRESS_STREET"),
            Self::LocationCity => f.write_str("LOCATION_CITY"),
            Self::LocationCoordinate => f.write_str("LOCATION_COORDINATE"),
            Self::LocationCountry => f.write_str("LOCATION_COUNTRY"),
            Self::LocationState => f.write_str("LOCATION_STATE"),
            Self::LocationZip => f.write_str("LOCATION_ZIP"),
            Self::MaritalStatus => f.write_str("MARITAL_STATUS"),
            Self::Money => f.write_str("MONEY"),
            Self::Name => f.write_str("NAME"),
            Self::NameFamily => f.write_str("NAME_FAMILY"),
            Self::NameGiven => f.write_str("NAME_GIVEN"),
            Self::NameMedicalProfessional => f.write_str("NAME_MEDICAL_PROFESSIONAL"),
            Self::NumericalPii => f.write_str("NUMERICAL_PII"),
            Self::Occupation => f.write_str("OCCUPATION"),
            Self::Organization => f.write_str("ORGANIZATION"),
            Self::OrganizationMedicalFacility => {
                f.write_str("ORGANIZATION_MEDICAL_FACILITY")
            }
            Self::Origin => f.write_str("ORIGIN"),
            Self::PassportNumber => f.write_str("PASSPORT_NUMBER"),
            Self::Password => f.write_str("PASSWORD"),
            Self::PhoneNumber => f.write_str("PHONE_NUMBER"),
            Self::PhysicalAttribute => f.write_str("PHYSICAL_ATTRIBUTE"),
            Self::PoliticalAffiliation => f.write_str("POLITICAL_AFFILIATION"),
            Self::Religion => f.write_str("RELIGION"),
            Self::Sexuality => f.write_str("SEXUALITY"),
            Self::Ssn => f.write_str("SSN"),
            Self::Time => f.write_str("TIME"),
            Self::Url => f.write_str("URL"),
            Self::Username => f.write_str("USERNAME"),
            Self::VehicleId => f.write_str("VEHICLE_ID"),
            Self::ZodiacSign => f.write_str("ZODIAC_SIGN"),
            Self::BloodType => f.write_str("BLOOD_TYPE"),
            Self::Condition => f.write_str("CONDITION"),
            Self::Dose => f.write_str("DOSE"),
            Self::Drug => f.write_str("DRUG"),
            Self::Injury => f.write_str("INJURY"),
            Self::MedicalProcess => f.write_str("MEDICAL_PROCESS"),
            Self::Statistics => f.write_str("STATISTICS"),
            Self::BankAccount => f.write_str("BANK_ACCOUNT"),
            Self::CreditCard => f.write_str("CREDIT_CARD"),
            Self::CreditCardExpiration => f.write_str("CREDIT_CARD_EXPIRATION"),
            Self::Cvv => f.write_str("CVV"),
            Self::RoutingNumber => f.write_str("ROUTING_NUMBER"),
            Self::CorporateAction => f.write_str("CORPORATE_ACTION"),
            Self::Day => f.write_str("DAY"),
            Self::Effect => f.write_str("EFFECT"),
            Self::FinancialMetric => f.write_str("FINANCIAL_METRIC"),
            Self::MedicalCode => f.write_str("MEDICAL_CODE"),
            Self::Month => f.write_str("MONTH"),
            Self::OrganizationId => f.write_str("ORGANIZATION_ID"),
            Self::Product => f.write_str("PRODUCT"),
            Self::Project => f.write_str("PROJECT"),
            Self::Trend => f.write_str("TREND"),
            Self::Year => f.write_str("YEAR"),
        }
    }
}
impl ::std::str::FromStr for PiiRedactionEntityTypeEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "APPI" => Ok(Self::Appi),
            "APPI_SENSITIVE" => Ok(Self::AppiSensitive),
            "CCI" => Ok(Self::Cci),
            "CORE_ENTITIES" => Ok(Self::CoreEntities),
            "CPRA" => Ok(Self::Cpra),
            "GDPR" => Ok(Self::Gdpr),
            "GDPR_SENSITIVE" => Ok(Self::GdprSensitive),
            "HEALTH_INFORMATION" => Ok(Self::HealthInformation),
            "HIPAA_SAFE_HARBOR" => Ok(Self::HipaaSafeHarbor),
            "LIDI" => Ok(Self::Lidi),
            "NUMERICAL_EXCL_PCI" => Ok(Self::NumericalExclPci),
            "PCI" => Ok(Self::Pci),
            "QUEBEC_PRIVACY_ACT" => Ok(Self::QuebecPrivacyAct),
            "ACCOUNT_NUMBER" => Ok(Self::AccountNumber),
            "AGE" => Ok(Self::Age),
            "DATE" => Ok(Self::Date),
            "DATE_INTERVAL" => Ok(Self::DateInterval),
            "DOB" => Ok(Self::Dob),
            "DRIVER_LICENSE" => Ok(Self::DriverLicense),
            "DURATION" => Ok(Self::Duration),
            "EMAIL_ADDRESS" => Ok(Self::EmailAddress),
            "EVENT" => Ok(Self::Event),
            "FILENAME" => Ok(Self::Filename),
            "GENDER" => Ok(Self::Gender),
            "HEALTHCARE_NUMBER" => Ok(Self::HealthcareNumber),
            "IP_ADDRESS" => Ok(Self::IpAddress),
            "LANGUAGE" => Ok(Self::Language),
            "LOCATION" => Ok(Self::Location),
            "LOCATION_ADDRESS" => Ok(Self::LocationAddress),
            "LOCATION_ADDRESS_STREET" => Ok(Self::LocationAddressStreet),
            "LOCATION_CITY" => Ok(Self::LocationCity),
            "LOCATION_COORDINATE" => Ok(Self::LocationCoordinate),
            "LOCATION_COUNTRY" => Ok(Self::LocationCountry),
            "LOCATION_STATE" => Ok(Self::LocationState),
            "LOCATION_ZIP" => Ok(Self::LocationZip),
            "MARITAL_STATUS" => Ok(Self::MaritalStatus),
            "MONEY" => Ok(Self::Money),
            "NAME" => Ok(Self::Name),
            "NAME_FAMILY" => Ok(Self::NameFamily),
            "NAME_GIVEN" => Ok(Self::NameGiven),
            "NAME_MEDICAL_PROFESSIONAL" => Ok(Self::NameMedicalProfessional),
            "NUMERICAL_PII" => Ok(Self::NumericalPii),
            "OCCUPATION" => Ok(Self::Occupation),
            "ORGANIZATION" => Ok(Self::Organization),
            "ORGANIZATION_MEDICAL_FACILITY" => Ok(Self::OrganizationMedicalFacility),
            "ORIGIN" => Ok(Self::Origin),
            "PASSPORT_NUMBER" => Ok(Self::PassportNumber),
            "PASSWORD" => Ok(Self::Password),
            "PHONE_NUMBER" => Ok(Self::PhoneNumber),
            "PHYSICAL_ATTRIBUTE" => Ok(Self::PhysicalAttribute),
            "POLITICAL_AFFILIATION" => Ok(Self::PoliticalAffiliation),
            "RELIGION" => Ok(Self::Religion),
            "SEXUALITY" => Ok(Self::Sexuality),
            "SSN" => Ok(Self::Ssn),
            "TIME" => Ok(Self::Time),
            "URL" => Ok(Self::Url),
            "USERNAME" => Ok(Self::Username),
            "VEHICLE_ID" => Ok(Self::VehicleId),
            "ZODIAC_SIGN" => Ok(Self::ZodiacSign),
            "BLOOD_TYPE" => Ok(Self::BloodType),
            "CONDITION" => Ok(Self::Condition),
            "DOSE" => Ok(Self::Dose),
            "DRUG" => Ok(Self::Drug),
            "INJURY" => Ok(Self::Injury),
            "MEDICAL_PROCESS" => Ok(Self::MedicalProcess),
            "STATISTICS" => Ok(Self::Statistics),
            "BANK_ACCOUNT" => Ok(Self::BankAccount),
            "CREDIT_CARD" => Ok(Self::CreditCard),
            "CREDIT_CARD_EXPIRATION" => Ok(Self::CreditCardExpiration),
            "CVV" => Ok(Self::Cvv),
            "ROUTING_NUMBER" => Ok(Self::RoutingNumber),
            "CORPORATE_ACTION" => Ok(Self::CorporateAction),
            "DAY" => Ok(Self::Day),
            "EFFECT" => Ok(Self::Effect),
            "FINANCIAL_METRIC" => Ok(Self::FinancialMetric),
            "MEDICAL_CODE" => Ok(Self::MedicalCode),
            "MONTH" => Ok(Self::Month),
            "ORGANIZATION_ID" => Ok(Self::OrganizationId),
            "PRODUCT" => Ok(Self::Product),
            "PROJECT" => Ok(Self::Project),
            "TREND" => Ok(Self::Trend),
            "YEAR" => Ok(Self::Year),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PiiRedactionEntityTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PiiRedactionEntityTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PiiRedactionEntityTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PostFinalTranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingTranscriptionResultDTO"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "post_final_transcript",
///      "type": "string",
///      "enum": [
///        "post_final_transcript"
///      ],
///      "example": "post_final_transcript"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PostFinalTranscriptMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: StreamingTranscriptionResultDto,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PostFinalTranscriptMessageType,
}
impl ::std::convert::From<&PostFinalTranscriptMessage> for PostFinalTranscriptMessage {
    fn from(value: &PostFinalTranscriptMessage) -> Self {
        value.clone()
    }
}
impl PostFinalTranscriptMessage {
    pub fn builder() -> builder::PostFinalTranscriptMessage {
        Default::default()
    }
}
///`PostFinalTranscriptMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "post_final_transcript",
///  "type": "string",
///  "enum": [
///    "post_final_transcript"
///  ],
///  "example": "post_final_transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PostFinalTranscriptMessageType {
    #[serde(rename = "post_final_transcript")]
    PostFinalTranscript,
}
impl ::std::convert::From<&Self> for PostFinalTranscriptMessageType {
    fn from(value: &PostFinalTranscriptMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PostFinalTranscriptMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PostFinalTranscript => f.write_str("post_final_transcript"),
        }
    }
}
impl ::std::str::FromStr for PostFinalTranscriptMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "post_final_transcript" => Ok(Self::PostFinalTranscript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PostFinalTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PostFinalTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PostFinalTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PostFinalTranscriptMessageType {
    fn default() -> Self {
        PostFinalTranscriptMessageType::PostFinalTranscript
    }
}
///`PostProcessingConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "chapterization": {
///      "description": "Deprecated: this parameter is ignored.",
///      "type": "boolean"
///    },
///    "summarization": {
///      "description": "If true, generates summarization for the whole transcription.",
///      "type": "boolean"
///    },
///    "summarization_config": {
///      "description": "Summarization configuration, if `summarization` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationConfigDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PostProcessingConfig {
    ///Deprecated: this parameter is ignored.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chapterization: ::std::option::Option<bool>,
    ///If true, generates summarization for the whole transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<bool>,
    ///Summarization configuration, if `summarization` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization_config: ::std::option::Option<SummarizationConfigDto>,
}
impl ::std::convert::From<&PostProcessingConfig> for PostProcessingConfig {
    fn from(value: &PostProcessingConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PostProcessingConfig {
    fn default() -> Self {
        Self {
            chapterization: Default::default(),
            summarization: Default::default(),
            summarization_config: Default::default(),
        }
    }
}
impl PostProcessingConfig {
    pub fn builder() -> builder::PostProcessingConfig {
        Default::default()
    }
}
///`PostSummarizationMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostSummarizationMessageData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "post_summarization",
///      "type": "string",
///      "enum": [
///        "post_summarization"
///      ],
///      "example": "post_summarization"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PostSummarizationMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the addon failed
    pub data: PostSummarizationMessageData,
    ///Error message if the addon failed
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PostSummarizationMessageType,
}
impl ::std::convert::From<&PostSummarizationMessage> for PostSummarizationMessage {
    fn from(value: &PostSummarizationMessage) -> Self {
        value.clone()
    }
}
impl PostSummarizationMessage {
    pub fn builder() -> builder::PostSummarizationMessage {
        Default::default()
    }
}
///`PostSummarizationMessageData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "results"
///  ],
///  "properties": {
///    "results": {
///      "description": "The summarization",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PostSummarizationMessageData {
    ///The summarization
    pub results: ::std::string::String,
}
impl ::std::convert::From<&PostSummarizationMessageData>
for PostSummarizationMessageData {
    fn from(value: &PostSummarizationMessageData) -> Self {
        value.clone()
    }
}
impl PostSummarizationMessageData {
    pub fn builder() -> builder::PostSummarizationMessageData {
        Default::default()
    }
}
///`PostSummarizationMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "post_summarization",
///  "type": "string",
///  "enum": [
///    "post_summarization"
///  ],
///  "example": "post_summarization"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PostSummarizationMessageType {
    #[serde(rename = "post_summarization")]
    PostSummarization,
}
impl ::std::convert::From<&Self> for PostSummarizationMessageType {
    fn from(value: &PostSummarizationMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PostSummarizationMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PostSummarization => f.write_str("post_summarization"),
        }
    }
}
impl ::std::str::FromStr for PostSummarizationMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "post_summarization" => Ok(Self::PostSummarization),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PostSummarizationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PostSummarizationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PostSummarizationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PostSummarizationMessageType {
    fn default() -> Self {
        PostSummarizationMessageType::PostSummarization
    }
}
///`PostTranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionDTO"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "post_transcript",
///      "type": "string",
///      "enum": [
///        "post_transcript"
///      ],
///      "example": "post_transcript"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PostTranscriptMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: TranscriptionDto,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PostTranscriptMessageType,
}
impl ::std::convert::From<&PostTranscriptMessage> for PostTranscriptMessage {
    fn from(value: &PostTranscriptMessage) -> Self {
        value.clone()
    }
}
impl PostTranscriptMessage {
    pub fn builder() -> builder::PostTranscriptMessage {
        Default::default()
    }
}
///`PostTranscriptMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "post_transcript",
///  "type": "string",
///  "enum": [
///    "post_transcript"
///  ],
///  "example": "post_transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PostTranscriptMessageType {
    #[serde(rename = "post_transcript")]
    PostTranscript,
}
impl ::std::convert::From<&Self> for PostTranscriptMessageType {
    fn from(value: &PostTranscriptMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PostTranscriptMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PostTranscript => f.write_str("post_transcript"),
        }
    }
}
impl ::std::str::FromStr for PostTranscriptMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "post_transcript" => Ok(Self::PostTranscript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PostTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PostTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PostTranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PostTranscriptMessageType {
    fn default() -> Self {
        PostTranscriptMessageType::PostTranscript
    }
}
///`PreProcessingConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "audio_enhancer": {
///      "description": "If true, apply pre-processing to the audio stream to enhance the quality.",
///      "type": "boolean"
///    },
///    "speech_threshold": {
///      "description": "Sensitivity configuration for Speech Threshold. A value close to 1 will apply stricter thresholds, making it less likely to detect background sounds as speech.",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PreProcessingConfig {
    ///If true, apply pre-processing to the audio stream to enhance the quality.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_enhancer: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub speech_threshold: ::std::option::Option<f64>,
}
impl ::std::convert::From<&PreProcessingConfig> for PreProcessingConfig {
    fn from(value: &PreProcessingConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PreProcessingConfig {
    fn default() -> Self {
        Self {
            audio_enhancer: Default::default(),
            speech_threshold: Default::default(),
        }
    }
}
impl PreProcessingConfig {
    pub fn builder() -> builder::PreProcessingConfig {
        Default::default()
    }
}
///`PreRecordedEventPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PreRecordedEventPayload {
    ///Id of the job
    pub id: ::uuid::Uuid,
}
impl ::std::convert::From<&PreRecordedEventPayload> for PreRecordedEventPayload {
    fn from(value: &PreRecordedEventPayload) -> Self {
        value.clone()
    }
}
impl PreRecordedEventPayload {
    pub fn builder() -> builder::PreRecordedEventPayload {
        Default::default()
    }
}
///`PreRecordedRequestParamsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_url"
///  ],
///  "properties": {
///    "audio_to_llm": {
///      "description": "Enable audio to LLM processing for this audio",
///      "type": "boolean"
///    },
///    "audio_to_llm_config": {
///      "description": "Audio to LLM configuration, if `audio_to_llm` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioToLlmListConfigDTO"
///        }
///      ]
///    },
///    "audio_url": {
///      "type": "string",
///      "format": "uri",
///      "nullable": true
///    },
///    "callback": {
///      "description": "Enable callback for this transcription. If true, the `callback_config` property will be used to customize the callback behaviour",
///      "type": "boolean"
///    },
///    "callback_config": {
///      "description": "Customize the callback behaviour (url and http method)",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CallbackConfigDto"
///        }
///      ]
///    },
///    "callback_url": {
///      "description": "**[Deprecated]** Use `callback`/`callback_config` instead. Callback URL we will do a `POST` request to with the result of the transcription",
///      "deprecated": true,
///      "type": "string",
///      "format": "uri",
///      "example": "https://callback.example"
///    },
///    "custom_spelling": {
///      "description": "**[Alpha]** Enable custom spelling for this audio",
///      "type": "boolean"
///    },
///    "custom_spelling_config": {
///      "description": "**[Alpha]** Custom spelling configuration, if `custom_spelling` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomSpellingConfigDTO"
///        }
///      ]
///    },
///    "custom_vocabulary": {
///      "description": "**[Beta]** Can be either boolean to enable custom_vocabulary for this audio or an array with specific vocabulary list to feed the transcription model with",
///      "type": "boolean"
///    },
///    "custom_vocabulary_config": {
///      "description": "**[Beta]** Custom vocabulary configuration, if `custom_vocabulary` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomVocabularyConfigDTO"
///        }
///      ]
///    },
///    "diarization": {
///      "description": "Enable speaker recognition (diarization) for this audio",
///      "type": "boolean"
///    },
///    "diarization_config": {
///      "description": "Speaker recognition configuration, if `diarization` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/DiarizationConfigDTO"
///        }
///      ]
///    },
///    "language_config": {
///      "description": "Specify the language configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/LanguageConfig"
///        }
///      ]
///    },
///    "model": {
///      "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionSupportedModels"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "**[Alpha]** Enable named entity recognition for this audio",
///      "type": "boolean"
///    },
///    "pii_redaction": {
///      "description": "Enable PII redaction for this audio",
///      "type": "boolean"
///    },
///    "pii_redaction_config": {
///      "description": "PII redaction configuration, if `pii_redaction` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PiiRedactionConfigDTO"
///        }
///      ]
///    },
///    "punctuation_enhanced": {
///      "description": "**[Alpha]** Use enhanced punctuation for this audio",
///      "type": "boolean"
///    },
///    "sentences": {
///      "description": "Enable sentences for this audio",
///      "type": "boolean"
///    },
///    "sentiment_analysis": {
///      "description": "Enable sentiment analysis for this audio",
///      "type": "boolean"
///    },
///    "subtitles": {
///      "description": "Enable subtitles generation for this transcription",
///      "type": "boolean"
///    },
///    "subtitles_config": {
///      "description": "Configuration for subtitles generation if `subtitles` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SubtitlesConfigDTO"
///        }
///      ]
///    },
///    "summarization": {
///      "description": "Enable summarization for this audio",
///      "type": "boolean"
///    },
///    "summarization_config": {
///      "description": "Summarization configuration, if `summarization` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationConfigDTO"
///        }
///      ]
///    },
///    "translation": {
///      "description": "**[Beta]** Enable translation for this audio",
///      "type": "boolean"
///    },
///    "translation_config": {
///      "description": "**[Beta]** Translation configuration, if `translation` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationConfigDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PreRecordedRequestParamsResponse {
    ///Enable audio to LLM processing for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_to_llm: ::std::option::Option<bool>,
    ///Audio to LLM configuration, if `audio_to_llm` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_to_llm_config: ::std::option::Option<AudioToLlmListConfigDto>,
    pub audio_url: ::std::string::String,
    ///Enable callback for this transcription. If true, the `callback_config` property will be used to customize the callback behaviour
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback: ::std::option::Option<bool>,
    ///Customize the callback behaviour (url and http method)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_config: ::std::option::Option<CallbackConfigDto>,
    ///**[Deprecated]** Use `callback`/`callback_config` instead. Callback URL we will do a `POST` request to with the result of the transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_url: ::std::option::Option<::std::string::String>,
    ///**[Alpha]** Enable custom spelling for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling: ::std::option::Option<bool>,
    ///**[Alpha]** Custom spelling configuration, if `custom_spelling` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling_config: ::std::option::Option<CustomSpellingConfigDto>,
    ///**[Beta]** Can be either boolean to enable custom_vocabulary for this audio or an array with specific vocabulary list to feed the transcription model with
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary: ::std::option::Option<bool>,
    ///**[Beta]** Custom vocabulary configuration, if `custom_vocabulary` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary_config: ::std::option::Option<CustomVocabularyConfigDto>,
    ///Enable speaker recognition (diarization) for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diarization: ::std::option::Option<bool>,
    ///Speaker recognition configuration, if `diarization` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diarization_config: ::std::option::Option<DiarizationConfigDto>,
    ///Specify the language configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language_config: ::std::option::Option<LanguageConfig>,
    ///The model used to process the audio. "solaria-1" is used by default.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<TranscriptionSupportedModels>,
    ///**[Alpha]** Enable named entity recognition for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<bool>,
    ///Enable PII redaction for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pii_redaction: ::std::option::Option<bool>,
    ///PII redaction configuration, if `pii_redaction` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pii_redaction_config: ::std::option::Option<PiiRedactionConfigDto>,
    ///**[Alpha]** Use enhanced punctuation for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub punctuation_enhanced: ::std::option::Option<bool>,
    ///Enable sentences for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentences: ::std::option::Option<bool>,
    ///Enable sentiment analysis for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<bool>,
    ///Enable subtitles generation for this transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtitles: ::std::option::Option<bool>,
    ///Configuration for subtitles generation if `subtitles` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtitles_config: ::std::option::Option<SubtitlesConfigDto>,
    ///Enable summarization for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<bool>,
    ///Summarization configuration, if `summarization` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization_config: ::std::option::Option<SummarizationConfigDto>,
    ///**[Beta]** Enable translation for this audio
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<bool>,
    ///**[Beta]** Translation configuration, if `translation` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation_config: ::std::option::Option<TranslationConfigDto>,
}
impl ::std::convert::From<&PreRecordedRequestParamsResponse>
for PreRecordedRequestParamsResponse {
    fn from(value: &PreRecordedRequestParamsResponse) -> Self {
        value.clone()
    }
}
impl PreRecordedRequestParamsResponse {
    pub fn builder() -> builder::PreRecordedRequestParamsResponse {
        Default::default()
    }
}
///`PreRecordedResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "id",
///    "kind",
///    "post_session_metadata",
///    "request_id",
///    "status",
///    "version"
///  ],
///  "properties": {
///    "completed_at": {
///      "description": "Completion date when status is \"done\" or \"error\"",
///      "type": "string",
///      "format": "date-time",
///      "example": "2026-01-01T00:00:00.000Z",
///      "nullable": true
///    },
///    "created_at": {
///      "description": "Creation date",
///      "type": "string",
///      "format": "date-time",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "custom_metadata": {
///      "description": "Custom metadata given in the initial request",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      }
///    },
///    "error_code": {
///      "description": "HTTP status code of the error if status is \"error\"",
///      "type": "integer",
///      "maximum": 599.0,
///      "minimum": 400.0,
///      "example": 500,
///      "nullable": true
///    },
///    "file": {
///      "description": "The file data you uploaded. Can be null if status is \"error\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/FileResponse"
///        }
///      ],
///      "nullable": true
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "kind": {
///      "default": "pre-recorded",
///      "type": "string",
///      "enum": [
///        "pre-recorded"
///      ],
///      "example": "pre-recorded"
///    },
///    "post_session_metadata": {
///      "description": "For debugging purposes, send data that could help to identify issues",
///      "type": "object"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-45463597"
///    },
///    "request_params": {
///      "description": "Parameters used for this pre-recorded transcription. Can be null if status is \"error\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PreRecordedRequestParamsResponse"
///        }
///      ],
///      "nullable": true
///    },
///    "result": {
///      "description": "Pre-recorded transcription's result when status is \"done\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionResultDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "status": {
///      "description": "\"queued\": the job has been queued. \"processing\": the job is being processed. \"done\": the job has been processed and the result is available. \"error\": an error occurred during the job's processing.",
///      "type": "string",
///      "enum": [
///        "queued",
///        "processing",
///        "done",
///        "error"
///      ]
///    },
///    "version": {
///      "description": "API version",
///      "type": "integer",
///      "example": 2
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PreRecordedResponse {
    ///Completion date when status is "done" or "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub completed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///Creation date
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Custom metadata given in the initial request
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///HTTP status code of the error if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<i64>,
    ///The file data you uploaded. Can be null if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<FileResponse>,
    ///Id of the job
    pub id: ::uuid::Uuid,
    pub kind: PreRecordedResponseKind,
    ///For debugging purposes, send data that could help to identify issues
    pub post_session_metadata: ::serde_json::Map<
        ::std::string::String,
        ::serde_json::Value,
    >,
    ///Debug id
    pub request_id: ::std::string::String,
    ///Parameters used for this pre-recorded transcription. Can be null if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub request_params: ::std::option::Option<PreRecordedRequestParamsResponse>,
    ///Pre-recorded transcription's result when status is "done"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<TranscriptionResultDto>,
    ///"queued": the job has been queued. "processing": the job is being processed. "done": the job has been processed and the result is available. "error": an error occurred during the job's processing.
    pub status: PreRecordedResponseStatus,
    ///API version
    pub version: i64,
}
impl ::std::convert::From<&PreRecordedResponse> for PreRecordedResponse {
    fn from(value: &PreRecordedResponse) -> Self {
        value.clone()
    }
}
impl PreRecordedResponse {
    pub fn builder() -> builder::PreRecordedResponse {
        Default::default()
    }
}
///`PreRecordedResponseKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "pre-recorded",
///  "type": "string",
///  "enum": [
///    "pre-recorded"
///  ],
///  "example": "pre-recorded"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PreRecordedResponseKind {
    #[serde(rename = "pre-recorded")]
    PreRecorded,
}
impl ::std::convert::From<&Self> for PreRecordedResponseKind {
    fn from(value: &PreRecordedResponseKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PreRecordedResponseKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PreRecorded => f.write_str("pre-recorded"),
        }
    }
}
impl ::std::str::FromStr for PreRecordedResponseKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pre-recorded" => Ok(Self::PreRecorded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PreRecordedResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PreRecordedResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PreRecordedResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PreRecordedResponseKind {
    fn default() -> Self {
        PreRecordedResponseKind::PreRecorded
    }
}
///"queued": the job has been queued. "processing": the job is being processed. "done": the job has been processed and the result is available. "error": an error occurred during the job's processing.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\"queued\": the job has been queued. \"processing\": the job is being processed. \"done\": the job has been processed and the result is available. \"error\": an error occurred during the job's processing.",
///  "type": "string",
///  "enum": [
///    "queued",
///    "processing",
///    "done",
///    "error"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PreRecordedResponseStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error,
}
impl ::std::convert::From<&Self> for PreRecordedResponseStatus {
    fn from(value: &PreRecordedResponseStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PreRecordedResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::Processing => f.write_str("processing"),
            Self::Done => f.write_str("done"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for PreRecordedResponseStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "processing" => Ok(Self::Processing),
            "done" => Ok(Self::Done),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PreRecordedResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PreRecordedResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PreRecordedResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RealtimeProcessingConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "custom_spelling": {
///      "description": "If true, enable custom spelling for the transcription.",
///      "type": "boolean"
///    },
///    "custom_spelling_config": {
///      "description": "Custom spelling configuration, if `custom_spelling` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomSpellingConfigDTO"
///        }
///      ]
///    },
///    "custom_vocabulary": {
///      "description": "If true, enable custom vocabulary for the transcription.",
///      "type": "boolean"
///    },
///    "custom_vocabulary_config": {
///      "description": "Custom vocabulary configuration, if `custom_vocabulary` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CustomVocabularyConfigDTO"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "If true, enable named entity recognition for the transcription.",
///      "type": "boolean"
///    },
///    "sentiment_analysis": {
///      "description": "If true, enable sentiment analysis for the transcription.",
///      "type": "boolean"
///    },
///    "translation": {
///      "description": "If true, enable translation for the transcription",
///      "type": "boolean"
///    },
///    "translation_config": {
///      "description": "Translation configuration, if `translation` is enabled",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationConfigDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct RealtimeProcessingConfig {
    ///If true, enable custom spelling for the transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling: ::std::option::Option<bool>,
    ///Custom spelling configuration, if `custom_spelling` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_spelling_config: ::std::option::Option<CustomSpellingConfigDto>,
    ///If true, enable custom vocabulary for the transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary: ::std::option::Option<bool>,
    ///Custom vocabulary configuration, if `custom_vocabulary` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_vocabulary_config: ::std::option::Option<CustomVocabularyConfigDto>,
    ///If true, enable named entity recognition for the transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<bool>,
    ///If true, enable sentiment analysis for the transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<bool>,
    ///If true, enable translation for the transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<bool>,
    ///Translation configuration, if `translation` is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation_config: ::std::option::Option<TranslationConfigDto>,
}
impl ::std::convert::From<&RealtimeProcessingConfig> for RealtimeProcessingConfig {
    fn from(value: &RealtimeProcessingConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RealtimeProcessingConfig {
    fn default() -> Self {
        Self {
            custom_spelling: Default::default(),
            custom_spelling_config: Default::default(),
            custom_vocabulary: Default::default(),
            custom_vocabulary_config: Default::default(),
            named_entity_recognition: Default::default(),
            sentiment_analysis: Default::default(),
            translation: Default::default(),
            translation_config: Default::default(),
        }
    }
}
impl RealtimeProcessingConfig {
    pub fn builder() -> builder::RealtimeProcessingConfig {
        Default::default()
    }
}
///`SentencesDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `sentences` has been enabled, transcription as sentences.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SentencesDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `sentences` has been enabled, transcription as sentences.
    pub results: ::std::vec::Vec<::std::string::String>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&SentencesDto> for SentencesDto {
    fn from(value: &SentencesDto) -> Self {
        value.clone()
    }
}
impl SentencesDto {
    pub fn builder() -> builder::SentencesDto {
        Default::default()
    }
}
///`SentimentAnalysisData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "results",
///    "utterance",
///    "utterance_id"
///  ],
///  "properties": {
///    "results": {
///      "description": "The sentiment analysis results",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SentimentAnalysisResult"
///      }
///    },
///    "utterance": {
///      "description": "The transcribed utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/UtteranceDTO"
///        }
///      ]
///    },
///    "utterance_id": {
///      "description": "Id of the utterance used for this result",
///      "type": "string",
///      "example": "00-00000011"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SentimentAnalysisData {
    ///The sentiment analysis results
    pub results: ::std::vec::Vec<SentimentAnalysisResult>,
    ///The transcribed utterance
    pub utterance: UtteranceDto,
    ///Id of the utterance used for this result
    pub utterance_id: ::std::string::String,
}
impl ::std::convert::From<&SentimentAnalysisData> for SentimentAnalysisData {
    fn from(value: &SentimentAnalysisData) -> Self {
        value.clone()
    }
}
impl SentimentAnalysisData {
    pub fn builder() -> builder::SentimentAnalysisData {
        Default::default()
    }
}
///`SentimentAnalysisDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `sentiment_analysis` has been enabled, Gladia will analyze the sentiments and emotions of the audio",
///      "type": "string"
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SentimentAnalysisDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `sentiment_analysis` has been enabled, Gladia will analyze the sentiments and emotions of the audio
    pub results: ::std::string::String,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&SentimentAnalysisDto> for SentimentAnalysisDto {
    fn from(value: &SentimentAnalysisDto) -> Self {
        value.clone()
    }
}
impl SentimentAnalysisDto {
    pub fn builder() -> builder::SentimentAnalysisDto {
        Default::default()
    }
}
///`SentimentAnalysisMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentimentAnalysisData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "sentiment_analysis",
///      "type": "string",
///      "enum": [
///        "sentiment_analysis"
///      ],
///      "example": "sentiment_analysis"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SentimentAnalysisMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the addon failed
    pub data: SentimentAnalysisData,
    ///Error message if the addon failed
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: SentimentAnalysisMessageType,
}
impl ::std::convert::From<&SentimentAnalysisMessage> for SentimentAnalysisMessage {
    fn from(value: &SentimentAnalysisMessage) -> Self {
        value.clone()
    }
}
impl SentimentAnalysisMessage {
    pub fn builder() -> builder::SentimentAnalysisMessage {
        Default::default()
    }
}
///`SentimentAnalysisMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "sentiment_analysis",
///  "type": "string",
///  "enum": [
///    "sentiment_analysis"
///  ],
///  "example": "sentiment_analysis"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SentimentAnalysisMessageType {
    #[serde(rename = "sentiment_analysis")]
    SentimentAnalysis,
}
impl ::std::convert::From<&Self> for SentimentAnalysisMessageType {
    fn from(value: &SentimentAnalysisMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SentimentAnalysisMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SentimentAnalysis => f.write_str("sentiment_analysis"),
        }
    }
}
impl ::std::str::FromStr for SentimentAnalysisMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sentiment_analysis" => Ok(Self::SentimentAnalysis),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SentimentAnalysisMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SentimentAnalysisMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SentimentAnalysisMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SentimentAnalysisMessageType {
    fn default() -> Self {
        SentimentAnalysisMessageType::SentimentAnalysis
    }
}
///`SentimentAnalysisResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "channel",
///    "emotion",
///    "end",
///    "sentiment",
///    "start",
///    "text"
///  ],
///  "properties": {
///    "channel": {
///      "type": "number"
///    },
///    "emotion": {
///      "type": "string"
///    },
///    "end": {
///      "type": "number"
///    },
///    "sentiment": {
///      "type": "string"
///    },
///    "start": {
///      "type": "number"
///    },
///    "text": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SentimentAnalysisResult {
    pub channel: f64,
    pub emotion: ::std::string::String,
    pub end: f64,
    pub sentiment: ::std::string::String,
    pub start: f64,
    pub text: ::std::string::String,
}
impl ::std::convert::From<&SentimentAnalysisResult> for SentimentAnalysisResult {
    fn from(value: &SentimentAnalysisResult) -> Self {
        value.clone()
    }
}
impl SentimentAnalysisResult {
    pub fn builder() -> builder::SentimentAnalysisResult {
        Default::default()
    }
}
///`SpeechEndMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SpeechMessageData"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "speech_end",
///      "type": "string",
///      "enum": [
///        "speech_end"
///      ],
///      "example": "speech_end"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SpeechEndMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: SpeechMessageData,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: SpeechEndMessageType,
}
impl ::std::convert::From<&SpeechEndMessage> for SpeechEndMessage {
    fn from(value: &SpeechEndMessage) -> Self {
        value.clone()
    }
}
impl SpeechEndMessage {
    pub fn builder() -> builder::SpeechEndMessage {
        Default::default()
    }
}
///`SpeechEndMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "speech_end",
///  "type": "string",
///  "enum": [
///    "speech_end"
///  ],
///  "example": "speech_end"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SpeechEndMessageType {
    #[serde(rename = "speech_end")]
    SpeechEnd,
}
impl ::std::convert::From<&Self> for SpeechEndMessageType {
    fn from(value: &SpeechEndMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SpeechEndMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SpeechEnd => f.write_str("speech_end"),
        }
    }
}
impl ::std::str::FromStr for SpeechEndMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speech_end" => Ok(Self::SpeechEnd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpeechEndMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SpeechEndMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpeechEndMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SpeechEndMessageType {
    fn default() -> Self {
        SpeechEndMessageType::SpeechEnd
    }
}
///`SpeechMessageData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "channel",
///    "time"
///  ],
///  "properties": {
///    "channel": {
///      "description": "Channel of the speech event",
///      "type": "number",
///      "example": 1
///    },
///    "time": {
///      "description": "Timestamp in seconds of the speech event",
///      "type": "number",
///      "example": 12.56
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SpeechMessageData {
    pub channel: f64,
    pub time: f64,
}
impl ::std::convert::From<&SpeechMessageData> for SpeechMessageData {
    fn from(value: &SpeechMessageData) -> Self {
        value.clone()
    }
}
impl SpeechMessageData {
    pub fn builder() -> builder::SpeechMessageData {
        Default::default()
    }
}
///`SpeechStartMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SpeechMessageData"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "speech_start",
///      "type": "string",
///      "enum": [
///        "speech_start"
///      ],
///      "example": "speech_start"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SpeechStartMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: SpeechMessageData,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: SpeechStartMessageType,
}
impl ::std::convert::From<&SpeechStartMessage> for SpeechStartMessage {
    fn from(value: &SpeechStartMessage) -> Self {
        value.clone()
    }
}
impl SpeechStartMessage {
    pub fn builder() -> builder::SpeechStartMessage {
        Default::default()
    }
}
///`SpeechStartMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "speech_start",
///  "type": "string",
///  "enum": [
///    "speech_start"
///  ],
///  "example": "speech_start"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SpeechStartMessageType {
    #[serde(rename = "speech_start")]
    SpeechStart,
}
impl ::std::convert::From<&Self> for SpeechStartMessageType {
    fn from(value: &SpeechStartMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SpeechStartMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SpeechStart => f.write_str("speech_start"),
        }
    }
}
impl ::std::str::FromStr for SpeechStartMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speech_start" => Ok(Self::SpeechStart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpeechStartMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SpeechStartMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpeechStartMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SpeechStartMessageType {
    fn default() -> Self {
        SpeechStartMessageType::SpeechStart
    }
}
///`StartRecordingMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "start_recording",
///      "type": "string",
///      "enum": [
///        "start_recording"
///      ],
///      "example": "start_recording"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StartRecordingMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: StartRecordingMessageType,
}
impl ::std::convert::From<&StartRecordingMessage> for StartRecordingMessage {
    fn from(value: &StartRecordingMessage) -> Self {
        value.clone()
    }
}
impl StartRecordingMessage {
    pub fn builder() -> builder::StartRecordingMessage {
        Default::default()
    }
}
///`StartRecordingMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "start_recording",
///  "type": "string",
///  "enum": [
///    "start_recording"
///  ],
///  "example": "start_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StartRecordingMessageType {
    #[serde(rename = "start_recording")]
    StartRecording,
}
impl ::std::convert::From<&Self> for StartRecordingMessageType {
    fn from(value: &StartRecordingMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StartRecordingMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StartRecording => f.write_str("start_recording"),
        }
    }
}
impl ::std::str::FromStr for StartRecordingMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "start_recording" => Ok(Self::StartRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StartRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StartRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StartRecordingMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StartRecordingMessageType {
    fn default() -> Self {
        StartRecordingMessageType::StartRecording
    }
}
///`StartSessionMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "start_session",
///      "type": "string",
///      "enum": [
///        "start_session"
///      ],
///      "example": "start_session"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StartSessionMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: StartSessionMessageType,
}
impl ::std::convert::From<&StartSessionMessage> for StartSessionMessage {
    fn from(value: &StartSessionMessage) -> Self {
        value.clone()
    }
}
impl StartSessionMessage {
    pub fn builder() -> builder::StartSessionMessage {
        Default::default()
    }
}
///`StartSessionMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "start_session",
///  "type": "string",
///  "enum": [
///    "start_session"
///  ],
///  "example": "start_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StartSessionMessageType {
    #[serde(rename = "start_session")]
    StartSession,
}
impl ::std::convert::From<&Self> for StartSessionMessageType {
    fn from(value: &StartSessionMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StartSessionMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StartSession => f.write_str("start_session"),
        }
    }
}
impl ::std::str::FromStr for StartSessionMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "start_session" => Ok(Self::StartSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StartSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StartSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StartSessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StartSessionMessageType {
    fn default() -> Self {
        StartSessionMessageType::StartSession
    }
}
///`StopRecordingAckData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "recording_duration",
///    "recording_left_to_process"
///  ],
///  "properties": {
///    "recording_duration": {
///      "description": "Total audio duration in seconds",
///      "type": "number",
///      "example": 344.45
///    },
///    "recording_left_to_process": {
///      "description": "Audio duration left to process in seconds",
///      "type": "number",
///      "example": 11.23
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StopRecordingAckData {
    pub recording_duration: f64,
    pub recording_left_to_process: f64,
}
impl ::std::convert::From<&StopRecordingAckData> for StopRecordingAckData {
    fn from(value: &StopRecordingAckData) -> Self {
        value.clone()
    }
}
impl StopRecordingAckData {
    pub fn builder() -> builder::StopRecordingAckData {
        Default::default()
    }
}
///`StopRecordingAckMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "acknowledged",
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "acknowledged": {
///      "description": "Flag to indicate if the action was successfully acknowledged",
///      "type": "boolean",
///      "example": true
///    },
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the action was not successfully acknowledged",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StopRecordingAckData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the action was not successfully acknowledged",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "stop_recording",
///      "type": "string",
///      "enum": [
///        "stop_recording"
///      ],
///      "example": "stop_recording"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StopRecordingAckMessage {
    ///Flag to indicate if the action was successfully acknowledged
    pub acknowledged: bool,
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the action was not successfully acknowledged
    pub data: StopRecordingAckData,
    ///Error message if the action was not successfully acknowledged
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: StopRecordingAckMessageType,
}
impl ::std::convert::From<&StopRecordingAckMessage> for StopRecordingAckMessage {
    fn from(value: &StopRecordingAckMessage) -> Self {
        value.clone()
    }
}
impl StopRecordingAckMessage {
    pub fn builder() -> builder::StopRecordingAckMessage {
        Default::default()
    }
}
///`StopRecordingAckMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "stop_recording",
///  "type": "string",
///  "enum": [
///    "stop_recording"
///  ],
///  "example": "stop_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StopRecordingAckMessageType {
    #[serde(rename = "stop_recording")]
    StopRecording,
}
impl ::std::convert::From<&Self> for StopRecordingAckMessageType {
    fn from(value: &StopRecordingAckMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StopRecordingAckMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StopRecording => f.write_str("stop_recording"),
        }
    }
}
impl ::std::str::FromStr for StopRecordingAckMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop_recording" => Ok(Self::StopRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StopRecordingAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StopRecordingAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StopRecordingAckMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StopRecordingAckMessageType {
    fn default() -> Self {
        StopRecordingAckMessageType::StopRecording
    }
}
///`StopRecordingAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "type": {
///      "default": "stop_recording",
///      "type": "string",
///      "enum": [
///        "stop_recording"
///      ],
///      "example": "stop_recording"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StopRecordingAction {
    #[serde(rename = "type")]
    pub type_: StopRecordingActionType,
}
impl ::std::convert::From<&StopRecordingAction> for StopRecordingAction {
    fn from(value: &StopRecordingAction) -> Self {
        value.clone()
    }
}
impl StopRecordingAction {
    pub fn builder() -> builder::StopRecordingAction {
        Default::default()
    }
}
///`StopRecordingActionType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "stop_recording",
///  "type": "string",
///  "enum": [
///    "stop_recording"
///  ],
///  "example": "stop_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StopRecordingActionType {
    #[serde(rename = "stop_recording")]
    StopRecording,
}
impl ::std::convert::From<&Self> for StopRecordingActionType {
    fn from(value: &StopRecordingActionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StopRecordingActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StopRecording => f.write_str("stop_recording"),
        }
    }
}
impl ::std::str::FromStr for StopRecordingActionType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop_recording" => Ok(Self::StopRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StopRecordingActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StopRecordingActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StopRecordingActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StopRecordingActionType {
    fn default() -> Self {
        StopRecordingActionType::StopRecording
    }
}
///`StreamingRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "bit_depth": {
///      "description": "The bit depth of the audio stream",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedBitDepthEnum"
///        }
///      ]
///    },
///    "callback": {
///      "description": "If true, messages will be sent to configured url.",
///      "type": "boolean"
///    },
///    "callback_config": {
///      "description": "Specify the callback configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CallbackConfig"
///        }
///      ]
///    },
///    "channels": {
///      "description": "The number of channels of the audio stream",
///      "type": "integer",
///      "maximum": 8.0,
///      "minimum": 1.0
///    },
///    "custom_metadata": {
///      "description": "Custom metadata you can attach to this live transcription",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      }
///    },
///    "encoding": {
///      "description": "The encoding format of the audio stream. Supported formats: \n- PCM: 8, 16, 24, and 32 bits \n- A-law: 8 bits \n- μ-law: 8 bits \n\nNote: No need to add WAV headers to raw audio as the API supports both formats.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedEncodingEnum"
///        }
///      ]
///    },
///    "endpointing": {
///      "description": "The endpointing duration in seconds. Endpointing is the duration of silence which will cause an utterance to be considered as finished",
///      "type": "number",
///      "maximum": 10.0,
///      "minimum": 0.01
///    },
///    "language_config": {
///      "description": "Specify the language configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/LanguageConfig"
///        }
///      ]
///    },
///    "maximum_duration_without_endpointing": {
///      "description": "The maximum duration in seconds without endpointing. If endpointing is not detected after this duration, current utterance will be considered as finished",
///      "type": "number",
///      "maximum": 60.0,
///      "minimum": 5.0
///    },
///    "messages_config": {
///      "description": "Specify the websocket messages configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/MessagesConfig"
///        }
///      ]
///    },
///    "model": {
///      "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedModels"
///        }
///      ]
///    },
///    "post_processing": {
///      "description": "Specify the post-processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostProcessingConfig"
///        }
///      ]
///    },
///    "pre_processing": {
///      "description": "Specify the pre-processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PreProcessingConfig"
///        }
///      ]
///    },
///    "realtime_processing": {
///      "description": "Specify the realtime processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/RealtimeProcessingConfig"
///        }
///      ]
///    },
///    "sample_rate": {
///      "description": "The sample rate of the audio stream",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedSampleRateEnum"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StreamingRequest {
    ///The bit depth of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bit_depth: ::std::option::Option<StreamingSupportedBitDepthEnum>,
    ///If true, messages will be sent to configured url.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback: ::std::option::Option<bool>,
    ///Specify the callback configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_config: ::std::option::Option<CallbackConfig>,
    ///The number of channels of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub channels: ::std::option::Option<::std::num::NonZeroU64>,
    ///Custom metadata you can attach to this live transcription
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    /**The encoding format of the audio stream. Supported formats:
- PCM: 8, 16, 24, and 32 bits
- A-law: 8 bits
- μ-law: 8 bits

Note: No need to add WAV headers to raw audio as the API supports both formats.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<StreamingSupportedEncodingEnum>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub endpointing: ::std::option::Option<f64>,
    ///Specify the language configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language_config: ::std::option::Option<LanguageConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum_duration_without_endpointing: ::std::option::Option<f64>,
    ///Specify the websocket messages configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub messages_config: ::std::option::Option<MessagesConfig>,
    ///The model used to process the audio. "solaria-1" is used by default.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<StreamingSupportedModels>,
    ///Specify the post-processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_processing: ::std::option::Option<PostProcessingConfig>,
    ///Specify the pre-processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pre_processing: ::std::option::Option<PreProcessingConfig>,
    ///Specify the realtime processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realtime_processing: ::std::option::Option<RealtimeProcessingConfig>,
    ///The sample rate of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sample_rate: ::std::option::Option<StreamingSupportedSampleRateEnum>,
}
impl ::std::convert::From<&StreamingRequest> for StreamingRequest {
    fn from(value: &StreamingRequest) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for StreamingRequest {
    fn default() -> Self {
        Self {
            bit_depth: Default::default(),
            callback: Default::default(),
            callback_config: Default::default(),
            channels: Default::default(),
            custom_metadata: Default::default(),
            encoding: Default::default(),
            endpointing: Default::default(),
            language_config: Default::default(),
            maximum_duration_without_endpointing: Default::default(),
            messages_config: Default::default(),
            model: Default::default(),
            post_processing: Default::default(),
            pre_processing: Default::default(),
            realtime_processing: Default::default(),
            sample_rate: Default::default(),
        }
    }
}
impl StreamingRequest {
    pub fn builder() -> builder::StreamingRequest {
        Default::default()
    }
}
///`StreamingRequestParamsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "bit_depth": {
///      "description": "The bit depth of the audio stream",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedBitDepthEnum"
///        }
///      ]
///    },
///    "callback": {
///      "description": "If true, messages will be sent to configured url.",
///      "type": "boolean"
///    },
///    "callback_config": {
///      "description": "Specify the callback configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/CallbackConfig"
///        }
///      ]
///    },
///    "channels": {
///      "description": "The number of channels of the audio stream",
///      "type": "integer",
///      "maximum": 8.0,
///      "minimum": 1.0
///    },
///    "encoding": {
///      "description": "The encoding format of the audio stream. Supported formats: \n- PCM: 8, 16, 24, and 32 bits \n- A-law: 8 bits \n- μ-law: 8 bits \n\nNote: No need to add WAV headers to raw audio as the API supports both formats.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedEncodingEnum"
///        }
///      ]
///    },
///    "endpointing": {
///      "description": "The endpointing duration in seconds. Endpointing is the duration of silence which will cause an utterance to be considered as finished",
///      "type": "number",
///      "maximum": 10.0,
///      "minimum": 0.01
///    },
///    "language_config": {
///      "description": "Specify the language configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/LanguageConfig"
///        }
///      ]
///    },
///    "maximum_duration_without_endpointing": {
///      "description": "The maximum duration in seconds without endpointing. If endpointing is not detected after this duration, current utterance will be considered as finished",
///      "type": "number",
///      "maximum": 60.0,
///      "minimum": 5.0
///    },
///    "messages_config": {
///      "description": "Specify the websocket messages configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/MessagesConfig"
///        }
///      ]
///    },
///    "model": {
///      "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedModels"
///        }
///      ]
///    },
///    "post_processing": {
///      "description": "Specify the post-processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PostProcessingConfig"
///        }
///      ]
///    },
///    "pre_processing": {
///      "description": "Specify the pre-processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/PreProcessingConfig"
///        }
///      ]
///    },
///    "realtime_processing": {
///      "description": "Specify the realtime processing configuration",
///      "allOf": [
///        {
///          "$ref": "#/$defs/RealtimeProcessingConfig"
///        }
///      ]
///    },
///    "sample_rate": {
///      "description": "The sample rate of the audio stream",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingSupportedSampleRateEnum"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StreamingRequestParamsResponse {
    ///The bit depth of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bit_depth: ::std::option::Option<StreamingSupportedBitDepthEnum>,
    ///If true, messages will be sent to configured url.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback: ::std::option::Option<bool>,
    ///Specify the callback configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_config: ::std::option::Option<CallbackConfig>,
    ///The number of channels of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub channels: ::std::option::Option<::std::num::NonZeroU64>,
    /**The encoding format of the audio stream. Supported formats:
- PCM: 8, 16, 24, and 32 bits
- A-law: 8 bits
- μ-law: 8 bits

Note: No need to add WAV headers to raw audio as the API supports both formats.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<StreamingSupportedEncodingEnum>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub endpointing: ::std::option::Option<f64>,
    ///Specify the language configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language_config: ::std::option::Option<LanguageConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum_duration_without_endpointing: ::std::option::Option<f64>,
    ///Specify the websocket messages configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub messages_config: ::std::option::Option<MessagesConfig>,
    ///The model used to process the audio. "solaria-1" is used by default.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<StreamingSupportedModels>,
    ///Specify the post-processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_processing: ::std::option::Option<PostProcessingConfig>,
    ///Specify the pre-processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pre_processing: ::std::option::Option<PreProcessingConfig>,
    ///Specify the realtime processing configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realtime_processing: ::std::option::Option<RealtimeProcessingConfig>,
    ///The sample rate of the audio stream
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sample_rate: ::std::option::Option<StreamingSupportedSampleRateEnum>,
}
impl ::std::convert::From<&StreamingRequestParamsResponse>
for StreamingRequestParamsResponse {
    fn from(value: &StreamingRequestParamsResponse) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for StreamingRequestParamsResponse {
    fn default() -> Self {
        Self {
            bit_depth: Default::default(),
            callback: Default::default(),
            callback_config: Default::default(),
            channels: Default::default(),
            encoding: Default::default(),
            endpointing: Default::default(),
            language_config: Default::default(),
            maximum_duration_without_endpointing: Default::default(),
            messages_config: Default::default(),
            model: Default::default(),
            post_processing: Default::default(),
            pre_processing: Default::default(),
            realtime_processing: Default::default(),
            sample_rate: Default::default(),
        }
    }
}
impl StreamingRequestParamsResponse {
    pub fn builder() -> builder::StreamingRequestParamsResponse {
        Default::default()
    }
}
///`StreamingResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "id",
///    "kind",
///    "post_session_metadata",
///    "request_id",
///    "status",
///    "version"
///  ],
///  "properties": {
///    "completed_at": {
///      "description": "Completion date when status is \"done\" or \"error\"",
///      "type": "string",
///      "format": "date-time",
///      "example": "2026-01-01T00:00:00.000Z",
///      "nullable": true
///    },
///    "created_at": {
///      "description": "Creation date",
///      "type": "string",
///      "format": "date-time",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "custom_metadata": {
///      "description": "Custom metadata given in the initial request",
///      "type": "object",
///      "additionalProperties": true,
///      "example": {
///        "user": "John Doe"
///      }
///    },
///    "error_code": {
///      "description": "HTTP status code of the error if status is \"error\"",
///      "type": "integer",
///      "maximum": 599.0,
///      "minimum": 400.0,
///      "example": 500,
///      "nullable": true
///    },
///    "file": {
///      "description": "The file data you uploaded. Can be null if status is \"error\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/FileResponse"
///        }
///      ],
///      "nullable": true
///    },
///    "id": {
///      "description": "Id of the job",
///      "type": "string",
///      "format": "uuid",
///      "example": "45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "kind": {
///      "default": "live",
///      "type": "string",
///      "enum": [
///        "live"
///      ],
///      "example": "live"
///    },
///    "post_session_metadata": {
///      "description": "For debugging purposes, send data that could help to identify issues",
///      "type": "object"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-45463597"
///    },
///    "request_params": {
///      "description": "Parameters used for this live transcription. Can be null if status is \"error\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingRequestParamsResponse"
///        }
///      ],
///      "nullable": true
///    },
///    "result": {
///      "description": "Live transcription's result when status is \"done\"",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StreamingTranscriptionResultWithMessagesDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "status": {
///      "description": "\"queued\": the job has been queued. \"processing\": the job is being processed. \"done\": the job has been processed and the result is available. \"error\": an error occurred during the job's processing.",
///      "type": "string",
///      "enum": [
///        "queued",
///        "processing",
///        "done",
///        "error"
///      ]
///    },
///    "version": {
///      "description": "API version",
///      "type": "integer",
///      "example": 2
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StreamingResponse {
    ///Completion date when status is "done" or "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub completed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///Creation date
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Custom metadata given in the initial request
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub custom_metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///HTTP status code of the error if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<i64>,
    ///The file data you uploaded. Can be null if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<FileResponse>,
    ///Id of the job
    pub id: ::uuid::Uuid,
    pub kind: StreamingResponseKind,
    ///For debugging purposes, send data that could help to identify issues
    pub post_session_metadata: ::serde_json::Map<
        ::std::string::String,
        ::serde_json::Value,
    >,
    ///Debug id
    pub request_id: ::std::string::String,
    ///Parameters used for this live transcription. Can be null if status is "error"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub request_params: ::std::option::Option<StreamingRequestParamsResponse>,
    ///Live transcription's result when status is "done"
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<StreamingTranscriptionResultWithMessagesDto>,
    ///"queued": the job has been queued. "processing": the job is being processed. "done": the job has been processed and the result is available. "error": an error occurred during the job's processing.
    pub status: StreamingResponseStatus,
    ///API version
    pub version: i64,
}
impl ::std::convert::From<&StreamingResponse> for StreamingResponse {
    fn from(value: &StreamingResponse) -> Self {
        value.clone()
    }
}
impl StreamingResponse {
    pub fn builder() -> builder::StreamingResponse {
        Default::default()
    }
}
///`StreamingResponseKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live",
///  "type": "string",
///  "enum": [
///    "live"
///  ],
///  "example": "live"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StreamingResponseKind {
    #[serde(rename = "live")]
    Live,
}
impl ::std::convert::From<&Self> for StreamingResponseKind {
    fn from(value: &StreamingResponseKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamingResponseKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Live => f.write_str("live"),
        }
    }
}
impl ::std::str::FromStr for StreamingResponseKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live" => Ok(Self::Live),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamingResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamingResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamingResponseKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StreamingResponseKind {
    fn default() -> Self {
        StreamingResponseKind::Live
    }
}
///"queued": the job has been queued. "processing": the job is being processed. "done": the job has been processed and the result is available. "error": an error occurred during the job's processing.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\"queued\": the job has been queued. \"processing\": the job is being processed. \"done\": the job has been processed and the result is available. \"error\": an error occurred during the job's processing.",
///  "type": "string",
///  "enum": [
///    "queued",
///    "processing",
///    "done",
///    "error"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StreamingResponseStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error,
}
impl ::std::convert::From<&Self> for StreamingResponseStatus {
    fn from(value: &StreamingResponseStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamingResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::Processing => f.write_str("processing"),
            Self::Done => f.write_str("done"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for StreamingResponseStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "processing" => Ok(Self::Processing),
            "done" => Ok(Self::Done),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamingResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamingResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamingResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`StreamingSupportedBitDepthEnum`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The bit depth of the audio stream",
///  "type": "number",
///  "enum": [
///    8,
///    16,
///    24,
///    32
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct StreamingSupportedBitDepthEnum(f64);
impl ::std::ops::Deref for StreamingSupportedBitDepthEnum {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<StreamingSupportedBitDepthEnum> for f64 {
    fn from(value: StreamingSupportedBitDepthEnum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StreamingSupportedBitDepthEnum>
for StreamingSupportedBitDepthEnum {
    fn from(value: &StreamingSupportedBitDepthEnum) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<f64> for StreamingSupportedBitDepthEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: f64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![8_f64, 16_f64, 24_f64, 32_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for StreamingSupportedBitDepthEnum {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
/**The encoding format of the audio stream. Supported formats:
- PCM: 8, 16, 24, and 32 bits
- A-law: 8 bits
- μ-law: 8 bits

Note: No need to add WAV headers to raw audio as the API supports both formats.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The encoding format of the audio stream. Supported formats: \n- PCM: 8, 16, 24, and 32 bits \n- A-law: 8 bits \n- μ-law: 8 bits \n\nNote: No need to add WAV headers to raw audio as the API supports both formats.",
///  "type": "string",
///  "enum": [
///    "wav/pcm",
///    "wav/alaw",
///    "wav/ulaw"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StreamingSupportedEncodingEnum {
    #[serde(rename = "wav/pcm")]
    WavPcm,
    #[serde(rename = "wav/alaw")]
    WavAlaw,
    #[serde(rename = "wav/ulaw")]
    WavUlaw,
}
impl ::std::convert::From<&Self> for StreamingSupportedEncodingEnum {
    fn from(value: &StreamingSupportedEncodingEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamingSupportedEncodingEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WavPcm => f.write_str("wav/pcm"),
            Self::WavAlaw => f.write_str("wav/alaw"),
            Self::WavUlaw => f.write_str("wav/ulaw"),
        }
    }
}
impl ::std::str::FromStr for StreamingSupportedEncodingEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wav/pcm" => Ok(Self::WavPcm),
            "wav/alaw" => Ok(Self::WavAlaw),
            "wav/ulaw" => Ok(Self::WavUlaw),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamingSupportedEncodingEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamingSupportedEncodingEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamingSupportedEncodingEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The model used to process the audio. "solaria-1" is used by default.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///  "type": "string",
///  "enum": [
///    "solaria-1"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StreamingSupportedModels {
    #[serde(rename = "solaria-1")]
    Solaria1,
}
impl ::std::convert::From<&Self> for StreamingSupportedModels {
    fn from(value: &StreamingSupportedModels) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamingSupportedModels {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Solaria1 => f.write_str("solaria-1"),
        }
    }
}
impl ::std::str::FromStr for StreamingSupportedModels {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "solaria-1" => Ok(Self::Solaria1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamingSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamingSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamingSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`StreamingSupportedRegions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "us-west",
///    "eu-west"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StreamingSupportedRegions {
    #[serde(rename = "us-west")]
    UsWest,
    #[serde(rename = "eu-west")]
    EuWest,
}
impl ::std::convert::From<&Self> for StreamingSupportedRegions {
    fn from(value: &StreamingSupportedRegions) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StreamingSupportedRegions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UsWest => f.write_str("us-west"),
            Self::EuWest => f.write_str("eu-west"),
        }
    }
}
impl ::std::str::FromStr for StreamingSupportedRegions {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "us-west" => Ok(Self::UsWest),
            "eu-west" => Ok(Self::EuWest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StreamingSupportedRegions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StreamingSupportedRegions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StreamingSupportedRegions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`StreamingSupportedSampleRateEnum`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The sample rate of the audio stream",
///  "type": "number",
///  "enum": [
///    8000,
///    16000,
///    32000,
///    44100,
///    48000
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct StreamingSupportedSampleRateEnum(f64);
impl ::std::ops::Deref for StreamingSupportedSampleRateEnum {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<StreamingSupportedSampleRateEnum> for f64 {
    fn from(value: StreamingSupportedSampleRateEnum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StreamingSupportedSampleRateEnum>
for StreamingSupportedSampleRateEnum {
    fn from(value: &StreamingSupportedSampleRateEnum) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<f64> for StreamingSupportedSampleRateEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: f64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![8000_f64, 16000_f64, 32000_f64, 44100_f64, 48000_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for StreamingSupportedSampleRateEnum {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`StreamingTranscriptionResultDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "metadata"
///  ],
///  "properties": {
///    "metadata": {
///      "description": "Metadata for the given transcription & audio file",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionMetadataDTO"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "If `named_entity_recognition` has been enabled, the detected entities",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamedEntityRecognitionDTO"
///        }
///      ]
///    },
///    "sentiment_analysis": {
///      "description": "If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentimentAnalysisDTO"
///        }
///      ]
///    },
///    "summarization": {
///      "description": "If `summarization` has been enabled, summarization of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationDTO"
///        }
///      ]
///    },
///    "transcription": {
///      "description": "Transcription of the audio speech",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionDTO"
///        }
///      ]
///    },
///    "translation": {
///      "description": "If `translation` has been enabled, translation of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StreamingTranscriptionResultDto {
    ///Metadata for the given transcription & audio file
    pub metadata: TranscriptionMetadataDto,
    ///If `named_entity_recognition` has been enabled, the detected entities
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<NamedEntityRecognitionDto>,
    ///If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<SentimentAnalysisDto>,
    ///If `summarization` has been enabled, summarization of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<SummarizationDto>,
    ///Transcription of the audio speech
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<TranscriptionDto>,
    ///If `translation` has been enabled, translation of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<TranslationDto>,
}
impl ::std::convert::From<&StreamingTranscriptionResultDto>
for StreamingTranscriptionResultDto {
    fn from(value: &StreamingTranscriptionResultDto) -> Self {
        value.clone()
    }
}
impl StreamingTranscriptionResultDto {
    pub fn builder() -> builder::StreamingTranscriptionResultDto {
        Default::default()
    }
}
///`StreamingTranscriptionResultWithMessagesDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "metadata"
///  ],
///  "properties": {
///    "messages": {
///      "description": "Real-Time messages sent by the server during the live transcription",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "metadata": {
///      "description": "Metadata for the given transcription & audio file",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionMetadataDTO"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "If `named_entity_recognition` has been enabled, the detected entities",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamedEntityRecognitionDTO"
///        }
///      ]
///    },
///    "sentiment_analysis": {
///      "description": "If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentimentAnalysisDTO"
///        }
///      ]
///    },
///    "summarization": {
///      "description": "If `summarization` has been enabled, summarization of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationDTO"
///        }
///      ]
///    },
///    "transcription": {
///      "description": "Transcription of the audio speech",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionDTO"
///        }
///      ]
///    },
///    "translation": {
///      "description": "If `translation` has been enabled, translation of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StreamingTranscriptionResultWithMessagesDto {
    ///Real-Time messages sent by the server during the live transcription
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub messages: ::std::vec::Vec<::std::string::String>,
    ///Metadata for the given transcription & audio file
    pub metadata: TranscriptionMetadataDto,
    ///If `named_entity_recognition` has been enabled, the detected entities
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<NamedEntityRecognitionDto>,
    ///If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<SentimentAnalysisDto>,
    ///If `summarization` has been enabled, summarization of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<SummarizationDto>,
    ///Transcription of the audio speech
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<TranscriptionDto>,
    ///If `translation` has been enabled, translation of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<TranslationDto>,
}
impl ::std::convert::From<&StreamingTranscriptionResultWithMessagesDto>
for StreamingTranscriptionResultWithMessagesDto {
    fn from(value: &StreamingTranscriptionResultWithMessagesDto) -> Self {
        value.clone()
    }
}
impl StreamingTranscriptionResultWithMessagesDto {
    pub fn builder() -> builder::StreamingTranscriptionResultWithMessagesDto {
        Default::default()
    }
}
///`StructuredDataExtractionDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `structured_data_extraction` has been enabled, results of the AI structured data extraction for the defined classes.",
///      "type": "string",
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StructuredDataExtractionDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `structured_data_extraction` has been enabled, results of the AI structured data extraction for the defined classes.
    pub results: ::std::string::String,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&StructuredDataExtractionDto> for StructuredDataExtractionDto {
    fn from(value: &StructuredDataExtractionDto) -> Self {
        value.clone()
    }
}
impl StructuredDataExtractionDto {
    pub fn builder() -> builder::StructuredDataExtractionDto {
        Default::default()
    }
}
///`SubtitleDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "format",
///    "subtitles"
///  ],
///  "properties": {
///    "format": {
///      "description": "Format of the current subtitle",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SubtitlesFormatEnum"
///        }
///      ],
///      "example": "srt"
///    },
///    "subtitles": {
///      "description": "Transcription on the asked subtitle format",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SubtitleDto {
    ///Format of the current subtitle
    pub format: SubtitlesFormatEnum,
    ///Transcription on the asked subtitle format
    pub subtitles: ::std::string::String,
}
impl ::std::convert::From<&SubtitleDto> for SubtitleDto {
    fn from(value: &SubtitleDto) -> Self {
        value.clone()
    }
}
impl SubtitleDto {
    pub fn builder() -> builder::SubtitleDto {
        Default::default()
    }
}
///`SubtitlesConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "formats": {
///      "description": "Subtitles formats you want your transcription to be formatted to",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SubtitlesFormatEnum"
///      },
///      "minItems": 1,
///      "example": [
///        "srt"
///      ]
///    },
///    "maximum_characters_per_row": {
///      "description": "Maximum number of characters per row in a subtitle",
///      "type": "integer",
///      "minimum": 1.0
///    },
///    "maximum_duration": {
///      "description": "Maximum duration of a subtitle in seconds",
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    },
///    "maximum_rows_per_caption": {
///      "description": "Maximum number of rows per caption",
///      "type": "integer",
///      "maximum": 5.0,
///      "minimum": 1.0
///    },
///    "minimum_duration": {
///      "description": "Minimum duration of a subtitle in seconds",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "style": {
///      "description": "Style of the subtitles. Compliance mode refers to : https://loc.gov/preservation/digital/formats//fdd/fdd000569.shtml#:~:text=SRT%20files%20are%20basic%20text,alongside%2C%20example%3A%20%22MyVideo123",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SubtitlesStyleEnum"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SubtitlesConfigDto {
    ///Subtitles formats you want your transcription to be formatted to
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub formats: ::std::vec::Vec<SubtitlesFormatEnum>,
    ///Maximum number of characters per row in a subtitle
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum_characters_per_row: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum_duration: ::std::option::Option<f64>,
    ///Maximum number of rows per caption
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum_rows_per_caption: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum_duration: ::std::option::Option<f64>,
    ///Style of the subtitles. Compliance mode refers to : https://loc.gov/preservation/digital/formats//fdd/fdd000569.shtml#:~:text=SRT%20files%20are%20basic%20text,alongside%2C%20example%3A%20%22MyVideo123
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style: ::std::option::Option<SubtitlesStyleEnum>,
}
impl ::std::convert::From<&SubtitlesConfigDto> for SubtitlesConfigDto {
    fn from(value: &SubtitlesConfigDto) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SubtitlesConfigDto {
    fn default() -> Self {
        Self {
            formats: Default::default(),
            maximum_characters_per_row: Default::default(),
            maximum_duration: Default::default(),
            maximum_rows_per_caption: Default::default(),
            minimum_duration: Default::default(),
            style: Default::default(),
        }
    }
}
impl SubtitlesConfigDto {
    pub fn builder() -> builder::SubtitlesConfigDto {
        Default::default()
    }
}
///Subtitles formats you want your transcription to be formatted to
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Subtitles formats you want your transcription to be formatted to",
///  "type": "string",
///  "enum": [
///    "srt",
///    "vtt"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SubtitlesFormatEnum {
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "vtt")]
    Vtt,
}
impl ::std::convert::From<&Self> for SubtitlesFormatEnum {
    fn from(value: &SubtitlesFormatEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubtitlesFormatEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Srt => f.write_str("srt"),
            Self::Vtt => f.write_str("vtt"),
        }
    }
}
impl ::std::str::FromStr for SubtitlesFormatEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "srt" => Ok(Self::Srt),
            "vtt" => Ok(Self::Vtt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubtitlesFormatEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SubtitlesFormatEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubtitlesFormatEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Style of the subtitles. Compliance mode refers to : https://loc.gov/preservation/digital/formats//fdd/fdd000569.shtml#:~:text=SRT%20files%20are%20basic%20text,alongside%2C%20example%3A%20%22MyVideo123
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Style of the subtitles. Compliance mode refers to : https://loc.gov/preservation/digital/formats//fdd/fdd000569.shtml#:~:text=SRT%20files%20are%20basic%20text,alongside%2C%20example%3A%20%22MyVideo123",
///  "type": "string",
///  "enum": [
///    "default",
///    "compliance"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SubtitlesStyleEnum {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "compliance")]
    Compliance,
}
impl ::std::convert::From<&Self> for SubtitlesStyleEnum {
    fn from(value: &SubtitlesStyleEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubtitlesStyleEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Compliance => f.write_str("compliance"),
        }
    }
}
impl ::std::str::FromStr for SubtitlesStyleEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "compliance" => Ok(Self::Compliance),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubtitlesStyleEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SubtitlesStyleEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubtitlesStyleEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SummarizationConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "type": {
///      "description": "The type of summarization to apply",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummaryTypesEnum"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SummarizationConfigDto {
    ///The type of summarization to apply
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<SummaryTypesEnum>,
}
impl ::std::convert::From<&SummarizationConfigDto> for SummarizationConfigDto {
    fn from(value: &SummarizationConfigDto) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SummarizationConfigDto {
    fn default() -> Self {
        Self { type_: Default::default() }
    }
}
impl SummarizationConfigDto {
    pub fn builder() -> builder::SummarizationConfigDto {
        Default::default()
    }
}
///`SummarizationDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "If `summarization` has been enabled, summary of the transcription",
///      "type": "string",
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SummarizationDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///If `summarization` has been enabled, summary of the transcription
    pub results: ::std::string::String,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&SummarizationDto> for SummarizationDto {
    fn from(value: &SummarizationDto) -> Self {
        value.clone()
    }
}
impl SummarizationDto {
    pub fn builder() -> builder::SummarizationDto {
        Default::default()
    }
}
///The type of summarization to apply
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of summarization to apply",
///  "type": "string",
///  "enum": [
///    "general",
///    "bullet_points",
///    "concise"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SummaryTypesEnum {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "bullet_points")]
    BulletPoints,
    #[serde(rename = "concise")]
    Concise,
}
impl ::std::convert::From<&Self> for SummaryTypesEnum {
    fn from(value: &SummaryTypesEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SummaryTypesEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::General => f.write_str("general"),
            Self::BulletPoints => f.write_str("bullet_points"),
            Self::Concise => f.write_str("concise"),
        }
    }
}
impl ::std::str::FromStr for SummaryTypesEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "general" => Ok(Self::General),
            "bullet_points" => Ok(Self::BulletPoints),
            "concise" => Ok(Self::Concise),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SummaryTypesEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SummaryTypesEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SummaryTypesEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranscriptMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptMessageData"
///        }
///      ]
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "transcript",
///      "type": "string",
///      "enum": [
///        "transcript"
///      ],
///      "example": "transcript"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data
    pub data: TranscriptMessageData,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: TranscriptMessageType,
}
impl ::std::convert::From<&TranscriptMessage> for TranscriptMessage {
    fn from(value: &TranscriptMessage) -> Self {
        value.clone()
    }
}
impl TranscriptMessage {
    pub fn builder() -> builder::TranscriptMessage {
        Default::default()
    }
}
///`TranscriptMessageData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "is_final",
///    "utterance"
///  ],
///  "properties": {
///    "id": {
///      "description": "Id of the utterance",
///      "type": "string",
///      "example": "00-00000011"
///    },
///    "is_final": {
///      "description": "Flag to indicate if the transcript is final or not",
///      "type": "boolean",
///      "example": true
///    },
///    "utterance": {
///      "description": "The transcribed utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/UtteranceDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptMessageData {
    ///Id of the utterance
    pub id: ::std::string::String,
    ///Flag to indicate if the transcript is final or not
    pub is_final: bool,
    ///The transcribed utterance
    pub utterance: UtteranceDto,
}
impl ::std::convert::From<&TranscriptMessageData> for TranscriptMessageData {
    fn from(value: &TranscriptMessageData) -> Self {
        value.clone()
    }
}
impl TranscriptMessageData {
    pub fn builder() -> builder::TranscriptMessageData {
        Default::default()
    }
}
///`TranscriptMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "transcript",
///  "type": "string",
///  "enum": [
///    "transcript"
///  ],
///  "example": "transcript"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranscriptMessageType {
    #[serde(rename = "transcript")]
    Transcript,
}
impl ::std::convert::From<&Self> for TranscriptMessageType {
    fn from(value: &TranscriptMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranscriptMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transcript => f.write_str("transcript"),
        }
    }
}
impl ::std::str::FromStr for TranscriptMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcript" => Ok(Self::Transcript),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for TranscriptMessageType {
    fn default() -> Self {
        TranscriptMessageType::Transcript
    }
}
///`TranscriptionDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "full_transcript",
///    "languages",
///    "utterances"
///  ],
///  "properties": {
///    "full_transcript": {
///      "description": "All transcription on text format without any other information",
///      "type": "string"
///    },
///    "languages": {
///      "description": "All the detected languages in the audio sorted from the most detected to the less detected",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TranscriptionLanguageCodeEnum"
///      },
///      "example": [
///        "en"
///      ]
///    },
///    "sentences": {
///      "description": "If `sentences` has been enabled, sentences results",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SentencesDTO"
///      }
///    },
///    "subtitles": {
///      "description": "If `subtitles` has been enabled, subtitles results",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SubtitleDTO"
///      }
///    },
///    "utterances": {
///      "description": "Transcribed speech utterances present in the audio",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UtteranceDTO"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionDto {
    ///All transcription on text format without any other information
    pub full_transcript: ::std::string::String,
    ///All the detected languages in the audio sorted from the most detected to the less detected
    pub languages: ::std::vec::Vec<TranscriptionLanguageCodeEnum>,
    ///If `sentences` has been enabled, sentences results
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sentences: ::std::vec::Vec<SentencesDto>,
    ///If `subtitles` has been enabled, subtitles results
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub subtitles: ::std::vec::Vec<SubtitleDto>,
    ///Transcribed speech utterances present in the audio
    pub utterances: ::std::vec::Vec<UtteranceDto>,
}
impl ::std::convert::From<&TranscriptionDto> for TranscriptionDto {
    fn from(value: &TranscriptionDto) -> Self {
        value.clone()
    }
}
impl TranscriptionDto {
    pub fn builder() -> builder::TranscriptionDto {
        Default::default()
    }
}
///Specify the language in which it will be pronounced when sound comparison occurs. Default to transcription language.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specify the language in which it will be pronounced when sound comparison occurs. Default to transcription language.",
///  "type": "string",
///  "enum": [
///    "af",
///    "am",
///    "ar",
///    "as",
///    "az",
///    "ba",
///    "be",
///    "bg",
///    "bn",
///    "bo",
///    "br",
///    "bs",
///    "ca",
///    "cs",
///    "cy",
///    "da",
///    "de",
///    "el",
///    "en",
///    "es",
///    "et",
///    "eu",
///    "fa",
///    "fi",
///    "fo",
///    "fr",
///    "gl",
///    "gu",
///    "ha",
///    "haw",
///    "he",
///    "hi",
///    "hr",
///    "ht",
///    "hu",
///    "hy",
///    "id",
///    "is",
///    "it",
///    "ja",
///    "jw",
///    "ka",
///    "kk",
///    "km",
///    "kn",
///    "ko",
///    "la",
///    "lb",
///    "ln",
///    "lo",
///    "lt",
///    "lv",
///    "mg",
///    "mi",
///    "mk",
///    "ml",
///    "mn",
///    "mr",
///    "ms",
///    "mt",
///    "my",
///    "ne",
///    "nl",
///    "nn",
///    "no",
///    "oc",
///    "pa",
///    "pl",
///    "ps",
///    "pt",
///    "ro",
///    "ru",
///    "sa",
///    "sd",
///    "si",
///    "sk",
///    "sl",
///    "sn",
///    "so",
///    "sq",
///    "sr",
///    "su",
///    "sv",
///    "sw",
///    "ta",
///    "te",
///    "tg",
///    "th",
///    "tk",
///    "tl",
///    "tr",
///    "tt",
///    "uk",
///    "ur",
///    "uz",
///    "vi",
///    "yi",
///    "yo",
///    "zh"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranscriptionLanguageCodeEnum {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bo")]
    Bo,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "ha")]
    Ha,
    #[serde(rename = "haw")]
    Haw,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "jw")]
    Jw,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "lo")]
    Lo,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "si")]
    Si,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "su")]
    Su,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tl")]
    Tl,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "yo")]
    Yo,
    #[serde(rename = "zh")]
    Zh,
}
impl ::std::convert::From<&Self> for TranscriptionLanguageCodeEnum {
    fn from(value: &TranscriptionLanguageCodeEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranscriptionLanguageCodeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Af => f.write_str("af"),
            Self::Am => f.write_str("am"),
            Self::Ar => f.write_str("ar"),
            Self::As => f.write_str("as"),
            Self::Az => f.write_str("az"),
            Self::Ba => f.write_str("ba"),
            Self::Be => f.write_str("be"),
            Self::Bg => f.write_str("bg"),
            Self::Bn => f.write_str("bn"),
            Self::Bo => f.write_str("bo"),
            Self::Br => f.write_str("br"),
            Self::Bs => f.write_str("bs"),
            Self::Ca => f.write_str("ca"),
            Self::Cs => f.write_str("cs"),
            Self::Cy => f.write_str("cy"),
            Self::Da => f.write_str("da"),
            Self::De => f.write_str("de"),
            Self::El => f.write_str("el"),
            Self::En => f.write_str("en"),
            Self::Es => f.write_str("es"),
            Self::Et => f.write_str("et"),
            Self::Eu => f.write_str("eu"),
            Self::Fa => f.write_str("fa"),
            Self::Fi => f.write_str("fi"),
            Self::Fo => f.write_str("fo"),
            Self::Fr => f.write_str("fr"),
            Self::Gl => f.write_str("gl"),
            Self::Gu => f.write_str("gu"),
            Self::Ha => f.write_str("ha"),
            Self::Haw => f.write_str("haw"),
            Self::He => f.write_str("he"),
            Self::Hi => f.write_str("hi"),
            Self::Hr => f.write_str("hr"),
            Self::Ht => f.write_str("ht"),
            Self::Hu => f.write_str("hu"),
            Self::Hy => f.write_str("hy"),
            Self::Id => f.write_str("id"),
            Self::Is => f.write_str("is"),
            Self::It => f.write_str("it"),
            Self::Ja => f.write_str("ja"),
            Self::Jw => f.write_str("jw"),
            Self::Ka => f.write_str("ka"),
            Self::Kk => f.write_str("kk"),
            Self::Km => f.write_str("km"),
            Self::Kn => f.write_str("kn"),
            Self::Ko => f.write_str("ko"),
            Self::La => f.write_str("la"),
            Self::Lb => f.write_str("lb"),
            Self::Ln => f.write_str("ln"),
            Self::Lo => f.write_str("lo"),
            Self::Lt => f.write_str("lt"),
            Self::Lv => f.write_str("lv"),
            Self::Mg => f.write_str("mg"),
            Self::Mi => f.write_str("mi"),
            Self::Mk => f.write_str("mk"),
            Self::Ml => f.write_str("ml"),
            Self::Mn => f.write_str("mn"),
            Self::Mr => f.write_str("mr"),
            Self::Ms => f.write_str("ms"),
            Self::Mt => f.write_str("mt"),
            Self::My => f.write_str("my"),
            Self::Ne => f.write_str("ne"),
            Self::Nl => f.write_str("nl"),
            Self::Nn => f.write_str("nn"),
            Self::No => f.write_str("no"),
            Self::Oc => f.write_str("oc"),
            Self::Pa => f.write_str("pa"),
            Self::Pl => f.write_str("pl"),
            Self::Ps => f.write_str("ps"),
            Self::Pt => f.write_str("pt"),
            Self::Ro => f.write_str("ro"),
            Self::Ru => f.write_str("ru"),
            Self::Sa => f.write_str("sa"),
            Self::Sd => f.write_str("sd"),
            Self::Si => f.write_str("si"),
            Self::Sk => f.write_str("sk"),
            Self::Sl => f.write_str("sl"),
            Self::Sn => f.write_str("sn"),
            Self::So => f.write_str("so"),
            Self::Sq => f.write_str("sq"),
            Self::Sr => f.write_str("sr"),
            Self::Su => f.write_str("su"),
            Self::Sv => f.write_str("sv"),
            Self::Sw => f.write_str("sw"),
            Self::Ta => f.write_str("ta"),
            Self::Te => f.write_str("te"),
            Self::Tg => f.write_str("tg"),
            Self::Th => f.write_str("th"),
            Self::Tk => f.write_str("tk"),
            Self::Tl => f.write_str("tl"),
            Self::Tr => f.write_str("tr"),
            Self::Tt => f.write_str("tt"),
            Self::Uk => f.write_str("uk"),
            Self::Ur => f.write_str("ur"),
            Self::Uz => f.write_str("uz"),
            Self::Vi => f.write_str("vi"),
            Self::Yi => f.write_str("yi"),
            Self::Yo => f.write_str("yo"),
            Self::Zh => f.write_str("zh"),
        }
    }
}
impl ::std::str::FromStr for TranscriptionLanguageCodeEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "af" => Ok(Self::Af),
            "am" => Ok(Self::Am),
            "ar" => Ok(Self::Ar),
            "as" => Ok(Self::As),
            "az" => Ok(Self::Az),
            "ba" => Ok(Self::Ba),
            "be" => Ok(Self::Be),
            "bg" => Ok(Self::Bg),
            "bn" => Ok(Self::Bn),
            "bo" => Ok(Self::Bo),
            "br" => Ok(Self::Br),
            "bs" => Ok(Self::Bs),
            "ca" => Ok(Self::Ca),
            "cs" => Ok(Self::Cs),
            "cy" => Ok(Self::Cy),
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "el" => Ok(Self::El),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "et" => Ok(Self::Et),
            "eu" => Ok(Self::Eu),
            "fa" => Ok(Self::Fa),
            "fi" => Ok(Self::Fi),
            "fo" => Ok(Self::Fo),
            "fr" => Ok(Self::Fr),
            "gl" => Ok(Self::Gl),
            "gu" => Ok(Self::Gu),
            "ha" => Ok(Self::Ha),
            "haw" => Ok(Self::Haw),
            "he" => Ok(Self::He),
            "hi" => Ok(Self::Hi),
            "hr" => Ok(Self::Hr),
            "ht" => Ok(Self::Ht),
            "hu" => Ok(Self::Hu),
            "hy" => Ok(Self::Hy),
            "id" => Ok(Self::Id),
            "is" => Ok(Self::Is),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "jw" => Ok(Self::Jw),
            "ka" => Ok(Self::Ka),
            "kk" => Ok(Self::Kk),
            "km" => Ok(Self::Km),
            "kn" => Ok(Self::Kn),
            "ko" => Ok(Self::Ko),
            "la" => Ok(Self::La),
            "lb" => Ok(Self::Lb),
            "ln" => Ok(Self::Ln),
            "lo" => Ok(Self::Lo),
            "lt" => Ok(Self::Lt),
            "lv" => Ok(Self::Lv),
            "mg" => Ok(Self::Mg),
            "mi" => Ok(Self::Mi),
            "mk" => Ok(Self::Mk),
            "ml" => Ok(Self::Ml),
            "mn" => Ok(Self::Mn),
            "mr" => Ok(Self::Mr),
            "ms" => Ok(Self::Ms),
            "mt" => Ok(Self::Mt),
            "my" => Ok(Self::My),
            "ne" => Ok(Self::Ne),
            "nl" => Ok(Self::Nl),
            "nn" => Ok(Self::Nn),
            "no" => Ok(Self::No),
            "oc" => Ok(Self::Oc),
            "pa" => Ok(Self::Pa),
            "pl" => Ok(Self::Pl),
            "ps" => Ok(Self::Ps),
            "pt" => Ok(Self::Pt),
            "ro" => Ok(Self::Ro),
            "ru" => Ok(Self::Ru),
            "sa" => Ok(Self::Sa),
            "sd" => Ok(Self::Sd),
            "si" => Ok(Self::Si),
            "sk" => Ok(Self::Sk),
            "sl" => Ok(Self::Sl),
            "sn" => Ok(Self::Sn),
            "so" => Ok(Self::So),
            "sq" => Ok(Self::Sq),
            "sr" => Ok(Self::Sr),
            "su" => Ok(Self::Su),
            "sv" => Ok(Self::Sv),
            "sw" => Ok(Self::Sw),
            "ta" => Ok(Self::Ta),
            "te" => Ok(Self::Te),
            "tg" => Ok(Self::Tg),
            "th" => Ok(Self::Th),
            "tk" => Ok(Self::Tk),
            "tl" => Ok(Self::Tl),
            "tr" => Ok(Self::Tr),
            "tt" => Ok(Self::Tt),
            "uk" => Ok(Self::Uk),
            "ur" => Ok(Self::Ur),
            "uz" => Ok(Self::Uz),
            "vi" => Ok(Self::Vi),
            "yi" => Ok(Self::Yi),
            "yo" => Ok(Self::Yo),
            "zh" => Ok(Self::Zh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptionLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranscriptionLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptionLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranscriptionMetadataDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "audio_duration",
///    "billing_time",
///    "number_of_distinct_channels",
///    "transcription_time"
///  ],
///  "properties": {
///    "audio_duration": {
///      "description": "Duration of the transcribed audio file",
///      "type": "number",
///      "example": 3600
///    },
///    "billing_time": {
///      "description": "Billed duration in seconds (audio_duration * number_of_distinct_channels)",
///      "type": "number",
///      "example": 3600
///    },
///    "number_of_distinct_channels": {
///      "description": "Number of distinct channels in the transcribed audio file",
///      "type": "integer",
///      "minimum": 1.0,
///      "example": 1
///    },
///    "transcription_time": {
///      "description": "Duration of the transcription in seconds",
///      "type": "number",
///      "example": 20
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionMetadataDto {
    pub audio_duration: f64,
    pub billing_time: f64,
    ///Number of distinct channels in the transcribed audio file
    pub number_of_distinct_channels: ::std::num::NonZeroU64,
    pub transcription_time: f64,
}
impl ::std::convert::From<&TranscriptionMetadataDto> for TranscriptionMetadataDto {
    fn from(value: &TranscriptionMetadataDto) -> Self {
        value.clone()
    }
}
impl TranscriptionMetadataDto {
    pub fn builder() -> builder::TranscriptionMetadataDto {
        Default::default()
    }
}
///`TranscriptionResultDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "metadata"
///  ],
///  "properties": {
///    "audio_to_llm": {
///      "description": "If `audio_to_llm` has been enabled, audio to llm results of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AudioToLlmListDTO"
///        }
///      ]
///    },
///    "diarization": {
///      "description": "If `diarization` has been requested and an error has occurred, the result will appear here",
///      "allOf": [
///        {
///          "$ref": "#/$defs/DiarizationDTO"
///        }
///      ]
///    },
///    "display_mode": {
///      "description": "If `display_mode` has been enabled, the output will be reordered, creating new utterances when speakers overlapped",
///      "allOf": [
///        {
///          "$ref": "#/$defs/DisplayModeDTO"
///        }
///      ]
///    },
///    "metadata": {
///      "description": "Metadata for the given transcription & audio file",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionMetadataDTO"
///        }
///      ]
///    },
///    "moderation": {
///      "description": "If `moderation` has been enabled, moderation of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/ModerationDTO"
///        }
///      ]
///    },
///    "name_consistency": {
///      "description": "If `name_consistency` has been enabled, Gladia will improve consistency of the names accross the transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamesConsistencyDTO"
///        }
///      ]
///    },
///    "named_entity_recognition": {
///      "description": "If `named_entity_recognition` has been enabled, the detected entities",
///      "allOf": [
///        {
///          "$ref": "#/$defs/NamedEntityRecognitionDTO"
///        }
///      ]
///    },
///    "sentences": {
///      "description": "If `sentences` has been enabled, sentences of the audio speech transcription. Deprecated: content will move to the `transcription` object.",
///      "deprecated": true,
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentencesDTO"
///        }
///      ]
///    },
///    "sentiment_analysis": {
///      "description": "If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SentimentAnalysisDTO"
///        }
///      ]
///    },
///    "structured_data_extraction": {
///      "description": "If `structured_data_extraction` has been enabled, structured data extraction results",
///      "allOf": [
///        {
///          "$ref": "#/$defs/StructuredDataExtractionDTO"
///        }
///      ]
///    },
///    "summarization": {
///      "description": "If `summarization` has been enabled, summarization of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/SummarizationDTO"
///        }
///      ]
///    },
///    "transcription": {
///      "description": "Transcription of the audio speech",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionDTO"
///        }
///      ]
///    },
///    "translation": {
///      "description": "If `translation` has been enabled, translation of the audio speech transcription",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationDTO"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranscriptionResultDto {
    ///If `audio_to_llm` has been enabled, audio to llm results of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_to_llm: ::std::option::Option<AudioToLlmListDto>,
    ///If `diarization` has been requested and an error has occurred, the result will appear here
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diarization: ::std::option::Option<DiarizationDto>,
    ///If `display_mode` has been enabled, the output will be reordered, creating new utterances when speakers overlapped
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display_mode: ::std::option::Option<DisplayModeDto>,
    ///Metadata for the given transcription & audio file
    pub metadata: TranscriptionMetadataDto,
    ///If `moderation` has been enabled, moderation of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ModerationDto>,
    ///If `name_consistency` has been enabled, Gladia will improve consistency of the names accross the transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name_consistency: ::std::option::Option<NamesConsistencyDto>,
    ///If `named_entity_recognition` has been enabled, the detected entities
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub named_entity_recognition: ::std::option::Option<NamedEntityRecognitionDto>,
    ///If `sentences` has been enabled, sentences of the audio speech transcription. Deprecated: content will move to the `transcription` object.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentences: ::std::option::Option<SentencesDto>,
    ///If `sentiment_analysis` has been enabled, sentiment analysis of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sentiment_analysis: ::std::option::Option<SentimentAnalysisDto>,
    ///If `structured_data_extraction` has been enabled, structured data extraction results
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub structured_data_extraction: ::std::option::Option<StructuredDataExtractionDto>,
    ///If `summarization` has been enabled, summarization of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summarization: ::std::option::Option<SummarizationDto>,
    ///Transcription of the audio speech
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transcription: ::std::option::Option<TranscriptionDto>,
    ///If `translation` has been enabled, translation of the audio speech transcription
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub translation: ::std::option::Option<TranslationDto>,
}
impl ::std::convert::From<&TranscriptionResultDto> for TranscriptionResultDto {
    fn from(value: &TranscriptionResultDto) -> Self {
        value.clone()
    }
}
impl TranscriptionResultDto {
    pub fn builder() -> builder::TranscriptionResultDto {
        Default::default()
    }
}
///The model used to process the audio. "solaria-1" is used by default.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The model used to process the audio. \"solaria-1\" is used by default.",
///  "type": "string",
///  "enum": [
///    "solaria-1",
///    "solaria-3",
///    "solaria-fusion"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranscriptionSupportedModels {
    #[serde(rename = "solaria-1")]
    Solaria1,
    #[serde(rename = "solaria-3")]
    Solaria3,
    #[serde(rename = "solaria-fusion")]
    SolariaFusion,
}
impl ::std::convert::From<&Self> for TranscriptionSupportedModels {
    fn from(value: &TranscriptionSupportedModels) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranscriptionSupportedModels {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Solaria1 => f.write_str("solaria-1"),
            Self::Solaria3 => f.write_str("solaria-3"),
            Self::SolariaFusion => f.write_str("solaria-fusion"),
        }
    }
}
impl ::std::str::FromStr for TranscriptionSupportedModels {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "solaria-1" => Ok(Self::Solaria1),
            "solaria-3" => Ok(Self::Solaria3),
            "solaria-fusion" => Ok(Self::SolariaFusion),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptionSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranscriptionSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptionSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranslationConfigDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "target_languages"
///  ],
///  "properties": {
///    "context": {
///      "description": "Context information to improve translation accuracy",
///      "type": "string"
///    },
///    "context_adaptation": {
///      "description": "Enables or disables context-aware translation features that allow the model to adapt translations based on provided context.",
///      "type": "boolean"
///    },
///    "informal": {
///      "description": "Forces the translation to use informal language forms when available in the target language.",
///      "type": "boolean"
///    },
///    "lipsync": {
///      "description": "Whether to apply lipsync to the translated transcription. ",
///      "type": "boolean"
///    },
///    "match_original_utterances": {
///      "description": "Align translated utterances with the original ones",
///      "type": "boolean"
///    },
///    "model": {
///      "description": "Model you want the translation model to use to translate",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationModelEnum"
///        }
///      ]
///    },
///    "target_languages": {
///      "description": "Target language in `iso639-1` format you want the transcription translated to",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TranslationLanguageCodeEnum"
///      },
///      "minItems": 1,
///      "example": [
///        "en"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranslationConfigDto {
    ///Context information to improve translation accuracy
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<::std::string::String>,
    ///Enables or disables context-aware translation features that allow the model to adapt translations based on provided context.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub context_adaptation: ::std::option::Option<bool>,
    ///Forces the translation to use informal language forms when available in the target language.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub informal: ::std::option::Option<bool>,
    ///Whether to apply lipsync to the translated transcription.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lipsync: ::std::option::Option<bool>,
    ///Align translated utterances with the original ones
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub match_original_utterances: ::std::option::Option<bool>,
    ///Model you want the translation model to use to translate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<TranslationModelEnum>,
    ///Target language in `iso639-1` format you want the transcription translated to
    pub target_languages: ::std::vec::Vec<TranslationLanguageCodeEnum>,
}
impl ::std::convert::From<&TranslationConfigDto> for TranslationConfigDto {
    fn from(value: &TranslationConfigDto) -> Self {
        value.clone()
    }
}
impl TranslationConfigDto {
    pub fn builder() -> builder::TranslationConfigDto {
        Default::default()
    }
}
///`TranslationData`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "original_language",
///    "target_language",
///    "translated_utterance",
///    "utterance",
///    "utterance_id"
///  ],
///  "properties": {
///    "original_language": {
///      "description": "The original language in `iso639-1` or `iso639-2` format depending on the language",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionLanguageCodeEnum"
///        }
///      ]
///    },
///    "target_language": {
///      "description": "The target language in `iso639-1` or `iso639-2` format depending on the language",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationLanguageCodeEnum"
///        }
///      ]
///    },
///    "translated_utterance": {
///      "description": "The translated utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/UtteranceDTO"
///        }
///      ]
///    },
///    "utterance": {
///      "description": "The transcribed utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/UtteranceDTO"
///        }
///      ]
///    },
///    "utterance_id": {
///      "description": "Id of the utterance used for this result",
///      "type": "string",
///      "example": "00-00000011"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranslationData {
    ///The original language in `iso639-1` or `iso639-2` format depending on the language
    pub original_language: TranscriptionLanguageCodeEnum,
    ///The target language in `iso639-1` or `iso639-2` format depending on the language
    pub target_language: TranslationLanguageCodeEnum,
    ///The translated utterance
    pub translated_utterance: UtteranceDto,
    ///The transcribed utterance
    pub utterance: UtteranceDto,
    ///Id of the utterance used for this result
    pub utterance_id: ::std::string::String,
}
impl ::std::convert::From<&TranslationData> for TranslationData {
    fn from(value: &TranslationData) -> Self {
        value.clone()
    }
}
impl TranslationData {
    pub fn builder() -> builder::TranslationData {
        Default::default()
    }
}
///`TranslationDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "exec_time",
///    "is_empty",
///    "results",
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "`null` if `success` is `true`. Contains the error details of the failed model",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "exec_time": {
///      "description": "Time audio intelligence model took to complete the task",
///      "type": "number"
///    },
///    "is_empty": {
///      "description": "The audio intelligence model returned an empty value",
///      "type": "boolean"
///    },
///    "results": {
///      "description": "List of translated transcriptions, one for each `target_languages`",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TranslationResultDTO"
///      },
///      "nullable": true
///    },
///    "success": {
///      "description": "The audio intelligence model succeeded to get a valid output",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranslationDto {
    ///`null` if `success` is `true`. Contains the error details of the failed model
    pub error: AddonErrorDto,
    pub exec_time: f64,
    ///The audio intelligence model returned an empty value
    pub is_empty: bool,
    ///List of translated transcriptions, one for each `target_languages`
    pub results: ::std::vec::Vec<TranslationResultDto>,
    ///The audio intelligence model succeeded to get a valid output
    pub success: bool,
}
impl ::std::convert::From<&TranslationDto> for TranslationDto {
    fn from(value: &TranslationDto) -> Self {
        value.clone()
    }
}
impl TranslationDto {
    pub fn builder() -> builder::TranslationDto {
        Default::default()
    }
}
///Target language in `iso639-1` format you want the transcription translated to
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Target language in `iso639-1` format you want the transcription translated to",
///  "type": "string",
///  "enum": [
///    "af",
///    "am",
///    "ar",
///    "as",
///    "az",
///    "ba",
///    "be",
///    "bg",
///    "bn",
///    "bo",
///    "br",
///    "bs",
///    "ca",
///    "cs",
///    "cy",
///    "da",
///    "de",
///    "el",
///    "en",
///    "es",
///    "et",
///    "eu",
///    "fa",
///    "fi",
///    "fo",
///    "fr",
///    "gl",
///    "gu",
///    "ha",
///    "haw",
///    "he",
///    "hi",
///    "hr",
///    "ht",
///    "hu",
///    "hy",
///    "id",
///    "is",
///    "it",
///    "ja",
///    "jw",
///    "ka",
///    "kk",
///    "km",
///    "kn",
///    "ko",
///    "la",
///    "lb",
///    "ln",
///    "lo",
///    "lt",
///    "lv",
///    "mg",
///    "mi",
///    "mk",
///    "ml",
///    "mn",
///    "mr",
///    "ms",
///    "mt",
///    "my",
///    "ne",
///    "nl",
///    "nn",
///    "no",
///    "oc",
///    "pa",
///    "pl",
///    "ps",
///    "pt",
///    "ro",
///    "ru",
///    "sa",
///    "sd",
///    "si",
///    "sk",
///    "sl",
///    "sn",
///    "so",
///    "sq",
///    "sr",
///    "su",
///    "sv",
///    "sw",
///    "ta",
///    "te",
///    "tg",
///    "th",
///    "tk",
///    "tl",
///    "tr",
///    "tt",
///    "uk",
///    "ur",
///    "uz",
///    "vi",
///    "wo",
///    "yi",
///    "yo",
///    "zh"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranslationLanguageCodeEnum {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bo")]
    Bo,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "ha")]
    Ha,
    #[serde(rename = "haw")]
    Haw,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "jw")]
    Jw,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "lo")]
    Lo,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "si")]
    Si,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "su")]
    Su,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tl")]
    Tl,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "wo")]
    Wo,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "yo")]
    Yo,
    #[serde(rename = "zh")]
    Zh,
}
impl ::std::convert::From<&Self> for TranslationLanguageCodeEnum {
    fn from(value: &TranslationLanguageCodeEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranslationLanguageCodeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Af => f.write_str("af"),
            Self::Am => f.write_str("am"),
            Self::Ar => f.write_str("ar"),
            Self::As => f.write_str("as"),
            Self::Az => f.write_str("az"),
            Self::Ba => f.write_str("ba"),
            Self::Be => f.write_str("be"),
            Self::Bg => f.write_str("bg"),
            Self::Bn => f.write_str("bn"),
            Self::Bo => f.write_str("bo"),
            Self::Br => f.write_str("br"),
            Self::Bs => f.write_str("bs"),
            Self::Ca => f.write_str("ca"),
            Self::Cs => f.write_str("cs"),
            Self::Cy => f.write_str("cy"),
            Self::Da => f.write_str("da"),
            Self::De => f.write_str("de"),
            Self::El => f.write_str("el"),
            Self::En => f.write_str("en"),
            Self::Es => f.write_str("es"),
            Self::Et => f.write_str("et"),
            Self::Eu => f.write_str("eu"),
            Self::Fa => f.write_str("fa"),
            Self::Fi => f.write_str("fi"),
            Self::Fo => f.write_str("fo"),
            Self::Fr => f.write_str("fr"),
            Self::Gl => f.write_str("gl"),
            Self::Gu => f.write_str("gu"),
            Self::Ha => f.write_str("ha"),
            Self::Haw => f.write_str("haw"),
            Self::He => f.write_str("he"),
            Self::Hi => f.write_str("hi"),
            Self::Hr => f.write_str("hr"),
            Self::Ht => f.write_str("ht"),
            Self::Hu => f.write_str("hu"),
            Self::Hy => f.write_str("hy"),
            Self::Id => f.write_str("id"),
            Self::Is => f.write_str("is"),
            Self::It => f.write_str("it"),
            Self::Ja => f.write_str("ja"),
            Self::Jw => f.write_str("jw"),
            Self::Ka => f.write_str("ka"),
            Self::Kk => f.write_str("kk"),
            Self::Km => f.write_str("km"),
            Self::Kn => f.write_str("kn"),
            Self::Ko => f.write_str("ko"),
            Self::La => f.write_str("la"),
            Self::Lb => f.write_str("lb"),
            Self::Ln => f.write_str("ln"),
            Self::Lo => f.write_str("lo"),
            Self::Lt => f.write_str("lt"),
            Self::Lv => f.write_str("lv"),
            Self::Mg => f.write_str("mg"),
            Self::Mi => f.write_str("mi"),
            Self::Mk => f.write_str("mk"),
            Self::Ml => f.write_str("ml"),
            Self::Mn => f.write_str("mn"),
            Self::Mr => f.write_str("mr"),
            Self::Ms => f.write_str("ms"),
            Self::Mt => f.write_str("mt"),
            Self::My => f.write_str("my"),
            Self::Ne => f.write_str("ne"),
            Self::Nl => f.write_str("nl"),
            Self::Nn => f.write_str("nn"),
            Self::No => f.write_str("no"),
            Self::Oc => f.write_str("oc"),
            Self::Pa => f.write_str("pa"),
            Self::Pl => f.write_str("pl"),
            Self::Ps => f.write_str("ps"),
            Self::Pt => f.write_str("pt"),
            Self::Ro => f.write_str("ro"),
            Self::Ru => f.write_str("ru"),
            Self::Sa => f.write_str("sa"),
            Self::Sd => f.write_str("sd"),
            Self::Si => f.write_str("si"),
            Self::Sk => f.write_str("sk"),
            Self::Sl => f.write_str("sl"),
            Self::Sn => f.write_str("sn"),
            Self::So => f.write_str("so"),
            Self::Sq => f.write_str("sq"),
            Self::Sr => f.write_str("sr"),
            Self::Su => f.write_str("su"),
            Self::Sv => f.write_str("sv"),
            Self::Sw => f.write_str("sw"),
            Self::Ta => f.write_str("ta"),
            Self::Te => f.write_str("te"),
            Self::Tg => f.write_str("tg"),
            Self::Th => f.write_str("th"),
            Self::Tk => f.write_str("tk"),
            Self::Tl => f.write_str("tl"),
            Self::Tr => f.write_str("tr"),
            Self::Tt => f.write_str("tt"),
            Self::Uk => f.write_str("uk"),
            Self::Ur => f.write_str("ur"),
            Self::Uz => f.write_str("uz"),
            Self::Vi => f.write_str("vi"),
            Self::Wo => f.write_str("wo"),
            Self::Yi => f.write_str("yi"),
            Self::Yo => f.write_str("yo"),
            Self::Zh => f.write_str("zh"),
        }
    }
}
impl ::std::str::FromStr for TranslationLanguageCodeEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "af" => Ok(Self::Af),
            "am" => Ok(Self::Am),
            "ar" => Ok(Self::Ar),
            "as" => Ok(Self::As),
            "az" => Ok(Self::Az),
            "ba" => Ok(Self::Ba),
            "be" => Ok(Self::Be),
            "bg" => Ok(Self::Bg),
            "bn" => Ok(Self::Bn),
            "bo" => Ok(Self::Bo),
            "br" => Ok(Self::Br),
            "bs" => Ok(Self::Bs),
            "ca" => Ok(Self::Ca),
            "cs" => Ok(Self::Cs),
            "cy" => Ok(Self::Cy),
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "el" => Ok(Self::El),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "et" => Ok(Self::Et),
            "eu" => Ok(Self::Eu),
            "fa" => Ok(Self::Fa),
            "fi" => Ok(Self::Fi),
            "fo" => Ok(Self::Fo),
            "fr" => Ok(Self::Fr),
            "gl" => Ok(Self::Gl),
            "gu" => Ok(Self::Gu),
            "ha" => Ok(Self::Ha),
            "haw" => Ok(Self::Haw),
            "he" => Ok(Self::He),
            "hi" => Ok(Self::Hi),
            "hr" => Ok(Self::Hr),
            "ht" => Ok(Self::Ht),
            "hu" => Ok(Self::Hu),
            "hy" => Ok(Self::Hy),
            "id" => Ok(Self::Id),
            "is" => Ok(Self::Is),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "jw" => Ok(Self::Jw),
            "ka" => Ok(Self::Ka),
            "kk" => Ok(Self::Kk),
            "km" => Ok(Self::Km),
            "kn" => Ok(Self::Kn),
            "ko" => Ok(Self::Ko),
            "la" => Ok(Self::La),
            "lb" => Ok(Self::Lb),
            "ln" => Ok(Self::Ln),
            "lo" => Ok(Self::Lo),
            "lt" => Ok(Self::Lt),
            "lv" => Ok(Self::Lv),
            "mg" => Ok(Self::Mg),
            "mi" => Ok(Self::Mi),
            "mk" => Ok(Self::Mk),
            "ml" => Ok(Self::Ml),
            "mn" => Ok(Self::Mn),
            "mr" => Ok(Self::Mr),
            "ms" => Ok(Self::Ms),
            "mt" => Ok(Self::Mt),
            "my" => Ok(Self::My),
            "ne" => Ok(Self::Ne),
            "nl" => Ok(Self::Nl),
            "nn" => Ok(Self::Nn),
            "no" => Ok(Self::No),
            "oc" => Ok(Self::Oc),
            "pa" => Ok(Self::Pa),
            "pl" => Ok(Self::Pl),
            "ps" => Ok(Self::Ps),
            "pt" => Ok(Self::Pt),
            "ro" => Ok(Self::Ro),
            "ru" => Ok(Self::Ru),
            "sa" => Ok(Self::Sa),
            "sd" => Ok(Self::Sd),
            "si" => Ok(Self::Si),
            "sk" => Ok(Self::Sk),
            "sl" => Ok(Self::Sl),
            "sn" => Ok(Self::Sn),
            "so" => Ok(Self::So),
            "sq" => Ok(Self::Sq),
            "sr" => Ok(Self::Sr),
            "su" => Ok(Self::Su),
            "sv" => Ok(Self::Sv),
            "sw" => Ok(Self::Sw),
            "ta" => Ok(Self::Ta),
            "te" => Ok(Self::Te),
            "tg" => Ok(Self::Tg),
            "th" => Ok(Self::Th),
            "tk" => Ok(Self::Tk),
            "tl" => Ok(Self::Tl),
            "tr" => Ok(Self::Tr),
            "tt" => Ok(Self::Tt),
            "uk" => Ok(Self::Uk),
            "ur" => Ok(Self::Ur),
            "uz" => Ok(Self::Uz),
            "vi" => Ok(Self::Vi),
            "wo" => Ok(Self::Wo),
            "yi" => Ok(Self::Yi),
            "yo" => Ok(Self::Yo),
            "zh" => Ok(Self::Zh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranslationLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranslationLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranslationLanguageCodeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranslationMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "created_at",
///    "data",
///    "error",
///    "session_id",
///    "type"
///  ],
///  "properties": {
///    "created_at": {
///      "description": "Date of creation of the message. The date is formatted as an ISO 8601 string",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    },
///    "data": {
///      "description": "The message data. \"null\" if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranslationData"
///        }
///      ],
///      "nullable": true
///    },
///    "error": {
///      "description": "Error message if the addon failed",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/Error"
///        }
///      ],
///      "example": null,
///      "nullable": true
///    },
///    "session_id": {
///      "description": "Id of the live session",
///      "type": "string",
///      "example": "4a39145c-2844-4557-8f34-34883f7be7d9"
///    },
///    "type": {
///      "default": "translation",
///      "type": "string",
///      "enum": [
///        "translation"
///      ],
///      "example": "translation"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranslationMessage {
    ///Date of creation of the message. The date is formatted as an ISO 8601 string
    pub created_at: ::std::string::String,
    ///The message data. "null" if the addon failed
    pub data: TranslationData,
    ///Error message if the addon failed
    pub error: Error,
    ///Id of the live session
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: TranslationMessageType,
}
impl ::std::convert::From<&TranslationMessage> for TranslationMessage {
    fn from(value: &TranslationMessage) -> Self {
        value.clone()
    }
}
impl TranslationMessage {
    pub fn builder() -> builder::TranslationMessage {
        Default::default()
    }
}
///`TranslationMessageType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "translation",
///  "type": "string",
///  "enum": [
///    "translation"
///  ],
///  "example": "translation"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranslationMessageType {
    #[serde(rename = "translation")]
    Translation,
}
impl ::std::convert::From<&Self> for TranslationMessageType {
    fn from(value: &TranslationMessageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranslationMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Translation => f.write_str("translation"),
        }
    }
}
impl ::std::str::FromStr for TranslationMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "translation" => Ok(Self::Translation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranslationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranslationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranslationMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for TranslationMessageType {
    fn default() -> Self {
        TranslationMessageType::Translation
    }
}
///Model you want the translation model to use to translate
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Model you want the translation model to use to translate",
///  "type": "string",
///  "enum": [
///    "base",
///    "batch",
///    "enhanced"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranslationModelEnum {
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "enhanced")]
    Enhanced,
}
impl ::std::convert::From<&Self> for TranslationModelEnum {
    fn from(value: &TranslationModelEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranslationModelEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Base => f.write_str("base"),
            Self::Batch => f.write_str("batch"),
            Self::Enhanced => f.write_str("enhanced"),
        }
    }
}
impl ::std::str::FromStr for TranslationModelEnum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "base" => Ok(Self::Base),
            "batch" => Ok(Self::Batch),
            "enhanced" => Ok(Self::Enhanced),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranslationModelEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranslationModelEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranslationModelEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranslationResultDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "error",
///    "full_transcript",
///    "languages",
///    "utterances"
///  ],
///  "properties": {
///    "error": {
///      "description": "Contains the error details of the failed addon",
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/AddonErrorDTO"
///        }
///      ],
///      "nullable": true
///    },
///    "full_transcript": {
///      "description": "All transcription on text format without any other information",
///      "type": "string"
///    },
///    "languages": {
///      "description": "All the detected languages in the audio sorted from the most detected to the less detected",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/TranslationLanguageCodeEnum"
///      },
///      "example": [
///        "en"
///      ]
///    },
///    "sentences": {
///      "description": "If `sentences` has been enabled, sentences results for this translation",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SentencesDTO"
///      }
///    },
///    "subtitles": {
///      "description": "If `subtitles` has been enabled, subtitles results for this translation",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SubtitleDTO"
///      }
///    },
///    "utterances": {
///      "description": "Transcribed speech utterances present in the audio",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UtteranceDTO"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TranslationResultDto {
    ///Contains the error details of the failed addon
    pub error: AddonErrorDto,
    ///All transcription on text format without any other information
    pub full_transcript: ::std::string::String,
    ///All the detected languages in the audio sorted from the most detected to the less detected
    pub languages: ::std::vec::Vec<TranslationLanguageCodeEnum>,
    ///If `sentences` has been enabled, sentences results for this translation
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sentences: ::std::vec::Vec<SentencesDto>,
    ///If `subtitles` has been enabled, subtitles results for this translation
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub subtitles: ::std::vec::Vec<SubtitleDto>,
    ///Transcribed speech utterances present in the audio
    pub utterances: ::std::vec::Vec<UtteranceDto>,
}
impl ::std::convert::From<&TranslationResultDto> for TranslationResultDto {
    fn from(value: &TranslationResultDto) -> Self {
        value.clone()
    }
}
impl TranslationResultDto {
    pub fn builder() -> builder::TranslationResultDto {
        Default::default()
    }
}
///`UnauthorizedErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Error message",
///      "type": "string",
///      "example": "gladia key not found"
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 401
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UnauthorizedErrorResponse {
    ///Error message
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&UnauthorizedErrorResponse> for UnauthorizedErrorResponse {
    fn from(value: &UnauthorizedErrorResponse) -> Self {
        value.clone()
    }
}
impl UnauthorizedErrorResponse {
    pub fn builder() -> builder::UnauthorizedErrorResponse {
        Default::default()
    }
}
///`UnprocessableEntityErrorResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "message",
///    "path",
///    "request_id",
///    "statusCode",
///    "timestamp"
///  ],
///  "properties": {
///    "message": {
///      "description": "Error message",
///      "type": "string",
///      "example": "Invalid parameter"
///    },
///    "path": {
///      "description": "Path to the API endpoint",
///      "type": "string",
///      "example": "/v2/transcription/45463597-20b7-4af7-b3b3-f5fb778203ab"
///    },
///    "request_id": {
///      "description": "Debug id",
///      "type": "string",
///      "example": "G-821fe9df"
///    },
///    "statusCode": {
///      "description": "HTTP status code of the error",
///      "type": "number",
///      "example": 422
///    },
///    "timestamp": {
///      "description": "Date of when the error occurred",
///      "type": "string",
///      "example": "2026-01-01T00:00:00.000Z"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UnprocessableEntityErrorResponse {
    ///Error message
    pub message: ::std::string::String,
    ///Path to the API endpoint
    pub path: ::std::string::String,
    ///Debug id
    pub request_id: ::std::string::String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    ///Date of when the error occurred
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&UnprocessableEntityErrorResponse>
for UnprocessableEntityErrorResponse {
    fn from(value: &UnprocessableEntityErrorResponse) -> Self {
        value.clone()
    }
}
impl UnprocessableEntityErrorResponse {
    pub fn builder() -> builder::UnprocessableEntityErrorResponse {
        Default::default()
    }
}
///`UploadBody`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct UploadBody(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for UploadBody {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<UploadBody>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: UploadBody) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UploadBody> for UploadBody {
    fn from(value: &UploadBody) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for UploadBody {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`UtteranceDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "channel",
///    "confidence",
///    "end",
///    "language",
///    "start",
///    "text",
///    "words"
///  ],
///  "properties": {
///    "channel": {
///      "description": "Audio channel of where this utterance has been transcribed from",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "confidence": {
///      "description": "Confidence on the transcribed utterance (1 = 100% confident)",
///      "type": "number"
///    },
///    "end": {
///      "description": "End timestamp in seconds of this utterance",
///      "type": "number"
///    },
///    "language": {
///      "description": "Spoken language in this utterance",
///      "allOf": [
///        {
///          "$ref": "#/$defs/TranscriptionLanguageCodeEnum"
///        }
///      ],
///      "example": "en"
///    },
///    "speaker": {
///      "description": "If `diarization` enabled, speaker identification number",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "start": {
///      "description": "Start timestamp in seconds of this utterance",
///      "type": "number"
///    },
///    "text": {
///      "description": "Transcription for this utterance",
///      "type": "string"
///    },
///    "words": {
///      "description": "List of words of the utterance, split by timestamp",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/WordDTO"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UtteranceDto {
    ///Audio channel of where this utterance has been transcribed from
    pub channel: u64,
    pub confidence: f64,
    pub end: f64,
    ///Spoken language in this utterance
    pub language: TranscriptionLanguageCodeEnum,
    ///If `diarization` enabled, speaker identification number
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub speaker: ::std::option::Option<u64>,
    pub start: f64,
    ///Transcription for this utterance
    pub text: ::std::string::String,
    ///List of words of the utterance, split by timestamp
    pub words: ::std::vec::Vec<WordDto>,
}
impl ::std::convert::From<&UtteranceDto> for UtteranceDto {
    fn from(value: &UtteranceDto) -> Self {
        value.clone()
    }
}
impl UtteranceDto {
    pub fn builder() -> builder::UtteranceDto {
        Default::default()
    }
}
///`WebhookLiveEndRecordingPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.end_recording",
///      "type": "string",
///      "enum": [
///        "live.end_recording"
///      ],
///      "example": "live.end_recording"
///    },
///    "payload": {
///      "$ref": "#/$defs/LiveEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookLiveEndRecordingPayload {
    pub event: WebhookLiveEndRecordingPayloadEvent,
    pub payload: LiveEventPayload,
}
impl ::std::convert::From<&WebhookLiveEndRecordingPayload>
for WebhookLiveEndRecordingPayload {
    fn from(value: &WebhookLiveEndRecordingPayload) -> Self {
        value.clone()
    }
}
impl WebhookLiveEndRecordingPayload {
    pub fn builder() -> builder::WebhookLiveEndRecordingPayload {
        Default::default()
    }
}
///`WebhookLiveEndRecordingPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.end_recording",
///  "type": "string",
///  "enum": [
///    "live.end_recording"
///  ],
///  "example": "live.end_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookLiveEndRecordingPayloadEvent {
    #[serde(rename = "live.end_recording")]
    LiveEndRecording,
}
impl ::std::convert::From<&Self> for WebhookLiveEndRecordingPayloadEvent {
    fn from(value: &WebhookLiveEndRecordingPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookLiveEndRecordingPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveEndRecording => f.write_str("live.end_recording"),
        }
    }
}
impl ::std::str::FromStr for WebhookLiveEndRecordingPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.end_recording" => Ok(Self::LiveEndRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookLiveEndRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookLiveEndRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookLiveEndRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookLiveEndRecordingPayloadEvent {
    fn default() -> Self {
        WebhookLiveEndRecordingPayloadEvent::LiveEndRecording
    }
}
///`WebhookLiveEndSessionPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.end_session",
///      "type": "string",
///      "enum": [
///        "live.end_session"
///      ],
///      "example": "live.end_session"
///    },
///    "payload": {
///      "$ref": "#/$defs/LiveEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookLiveEndSessionPayload {
    pub event: WebhookLiveEndSessionPayloadEvent,
    pub payload: LiveEventPayload,
}
impl ::std::convert::From<&WebhookLiveEndSessionPayload>
for WebhookLiveEndSessionPayload {
    fn from(value: &WebhookLiveEndSessionPayload) -> Self {
        value.clone()
    }
}
impl WebhookLiveEndSessionPayload {
    pub fn builder() -> builder::WebhookLiveEndSessionPayload {
        Default::default()
    }
}
///`WebhookLiveEndSessionPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.end_session",
///  "type": "string",
///  "enum": [
///    "live.end_session"
///  ],
///  "example": "live.end_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookLiveEndSessionPayloadEvent {
    #[serde(rename = "live.end_session")]
    LiveEndSession,
}
impl ::std::convert::From<&Self> for WebhookLiveEndSessionPayloadEvent {
    fn from(value: &WebhookLiveEndSessionPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookLiveEndSessionPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveEndSession => f.write_str("live.end_session"),
        }
    }
}
impl ::std::str::FromStr for WebhookLiveEndSessionPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.end_session" => Ok(Self::LiveEndSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookLiveEndSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookLiveEndSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookLiveEndSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookLiveEndSessionPayloadEvent {
    fn default() -> Self {
        WebhookLiveEndSessionPayloadEvent::LiveEndSession
    }
}
///`WebhookLiveStartRecordingPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.start_recording",
///      "type": "string",
///      "enum": [
///        "live.start_recording"
///      ],
///      "example": "live.start_recording"
///    },
///    "payload": {
///      "$ref": "#/$defs/LiveEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookLiveStartRecordingPayload {
    pub event: WebhookLiveStartRecordingPayloadEvent,
    pub payload: LiveEventPayload,
}
impl ::std::convert::From<&WebhookLiveStartRecordingPayload>
for WebhookLiveStartRecordingPayload {
    fn from(value: &WebhookLiveStartRecordingPayload) -> Self {
        value.clone()
    }
}
impl WebhookLiveStartRecordingPayload {
    pub fn builder() -> builder::WebhookLiveStartRecordingPayload {
        Default::default()
    }
}
///`WebhookLiveStartRecordingPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.start_recording",
///  "type": "string",
///  "enum": [
///    "live.start_recording"
///  ],
///  "example": "live.start_recording"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookLiveStartRecordingPayloadEvent {
    #[serde(rename = "live.start_recording")]
    LiveStartRecording,
}
impl ::std::convert::From<&Self> for WebhookLiveStartRecordingPayloadEvent {
    fn from(value: &WebhookLiveStartRecordingPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookLiveStartRecordingPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveStartRecording => f.write_str("live.start_recording"),
        }
    }
}
impl ::std::str::FromStr for WebhookLiveStartRecordingPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.start_recording" => Ok(Self::LiveStartRecording),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookLiveStartRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookLiveStartRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookLiveStartRecordingPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookLiveStartRecordingPayloadEvent {
    fn default() -> Self {
        WebhookLiveStartRecordingPayloadEvent::LiveStartRecording
    }
}
///`WebhookLiveStartSessionPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "live.start_session",
///      "type": "string",
///      "enum": [
///        "live.start_session"
///      ],
///      "example": "live.start_session"
///    },
///    "payload": {
///      "$ref": "#/$defs/LiveEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookLiveStartSessionPayload {
    pub event: WebhookLiveStartSessionPayloadEvent,
    pub payload: LiveEventPayload,
}
impl ::std::convert::From<&WebhookLiveStartSessionPayload>
for WebhookLiveStartSessionPayload {
    fn from(value: &WebhookLiveStartSessionPayload) -> Self {
        value.clone()
    }
}
impl WebhookLiveStartSessionPayload {
    pub fn builder() -> builder::WebhookLiveStartSessionPayload {
        Default::default()
    }
}
///`WebhookLiveStartSessionPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "live.start_session",
///  "type": "string",
///  "enum": [
///    "live.start_session"
///  ],
///  "example": "live.start_session"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookLiveStartSessionPayloadEvent {
    #[serde(rename = "live.start_session")]
    LiveStartSession,
}
impl ::std::convert::From<&Self> for WebhookLiveStartSessionPayloadEvent {
    fn from(value: &WebhookLiveStartSessionPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookLiveStartSessionPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LiveStartSession => f.write_str("live.start_session"),
        }
    }
}
impl ::std::str::FromStr for WebhookLiveStartSessionPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live.start_session" => Ok(Self::LiveStartSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookLiveStartSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookLiveStartSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookLiveStartSessionPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookLiveStartSessionPayloadEvent {
    fn default() -> Self {
        WebhookLiveStartSessionPayloadEvent::LiveStartSession
    }
}
///`WebhookTranscriptionCreatedPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "transcription.created",
///      "type": "string",
///      "enum": [
///        "transcription.created"
///      ],
///      "example": "transcription.created"
///    },
///    "payload": {
///      "$ref": "#/$defs/PreRecordedEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookTranscriptionCreatedPayload {
    pub event: WebhookTranscriptionCreatedPayloadEvent,
    pub payload: PreRecordedEventPayload,
}
impl ::std::convert::From<&WebhookTranscriptionCreatedPayload>
for WebhookTranscriptionCreatedPayload {
    fn from(value: &WebhookTranscriptionCreatedPayload) -> Self {
        value.clone()
    }
}
impl WebhookTranscriptionCreatedPayload {
    pub fn builder() -> builder::WebhookTranscriptionCreatedPayload {
        Default::default()
    }
}
///`WebhookTranscriptionCreatedPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "transcription.created",
///  "type": "string",
///  "enum": [
///    "transcription.created"
///  ],
///  "example": "transcription.created"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookTranscriptionCreatedPayloadEvent {
    #[serde(rename = "transcription.created")]
    TranscriptionCreated,
}
impl ::std::convert::From<&Self> for WebhookTranscriptionCreatedPayloadEvent {
    fn from(value: &WebhookTranscriptionCreatedPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookTranscriptionCreatedPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptionCreated => f.write_str("transcription.created"),
        }
    }
}
impl ::std::str::FromStr for WebhookTranscriptionCreatedPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription.created" => Ok(Self::TranscriptionCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookTranscriptionCreatedPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookTranscriptionCreatedPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookTranscriptionCreatedPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookTranscriptionCreatedPayloadEvent {
    fn default() -> Self {
        WebhookTranscriptionCreatedPayloadEvent::TranscriptionCreated
    }
}
///`WebhookTranscriptionErrorPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "transcription.error",
///      "type": "string",
///      "enum": [
///        "transcription.error"
///      ],
///      "example": "transcription.error"
///    },
///    "payload": {
///      "$ref": "#/$defs/PreRecordedEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookTranscriptionErrorPayload {
    pub event: WebhookTranscriptionErrorPayloadEvent,
    pub payload: PreRecordedEventPayload,
}
impl ::std::convert::From<&WebhookTranscriptionErrorPayload>
for WebhookTranscriptionErrorPayload {
    fn from(value: &WebhookTranscriptionErrorPayload) -> Self {
        value.clone()
    }
}
impl WebhookTranscriptionErrorPayload {
    pub fn builder() -> builder::WebhookTranscriptionErrorPayload {
        Default::default()
    }
}
///`WebhookTranscriptionErrorPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "transcription.error",
///  "type": "string",
///  "enum": [
///    "transcription.error"
///  ],
///  "example": "transcription.error"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookTranscriptionErrorPayloadEvent {
    #[serde(rename = "transcription.error")]
    TranscriptionError,
}
impl ::std::convert::From<&Self> for WebhookTranscriptionErrorPayloadEvent {
    fn from(value: &WebhookTranscriptionErrorPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookTranscriptionErrorPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptionError => f.write_str("transcription.error"),
        }
    }
}
impl ::std::str::FromStr for WebhookTranscriptionErrorPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription.error" => Ok(Self::TranscriptionError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookTranscriptionErrorPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookTranscriptionErrorPayloadEvent {
    fn default() -> Self {
        WebhookTranscriptionErrorPayloadEvent::TranscriptionError
    }
}
///`WebhookTranscriptionSuccessPayload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "event",
///    "payload"
///  ],
///  "properties": {
///    "event": {
///      "default": "transcription.success",
///      "type": "string",
///      "enum": [
///        "transcription.success"
///      ],
///      "example": "transcription.success"
///    },
///    "payload": {
///      "$ref": "#/$defs/PreRecordedEventPayload"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WebhookTranscriptionSuccessPayload {
    pub event: WebhookTranscriptionSuccessPayloadEvent,
    pub payload: PreRecordedEventPayload,
}
impl ::std::convert::From<&WebhookTranscriptionSuccessPayload>
for WebhookTranscriptionSuccessPayload {
    fn from(value: &WebhookTranscriptionSuccessPayload) -> Self {
        value.clone()
    }
}
impl WebhookTranscriptionSuccessPayload {
    pub fn builder() -> builder::WebhookTranscriptionSuccessPayload {
        Default::default()
    }
}
///`WebhookTranscriptionSuccessPayloadEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "transcription.success",
///  "type": "string",
///  "enum": [
///    "transcription.success"
///  ],
///  "example": "transcription.success"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WebhookTranscriptionSuccessPayloadEvent {
    #[serde(rename = "transcription.success")]
    TranscriptionSuccess,
}
impl ::std::convert::From<&Self> for WebhookTranscriptionSuccessPayloadEvent {
    fn from(value: &WebhookTranscriptionSuccessPayloadEvent) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WebhookTranscriptionSuccessPayloadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TranscriptionSuccess => f.write_str("transcription.success"),
        }
    }
}
impl ::std::str::FromStr for WebhookTranscriptionSuccessPayloadEvent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transcription.success" => Ok(Self::TranscriptionSuccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WebhookTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WebhookTranscriptionSuccessPayloadEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WebhookTranscriptionSuccessPayloadEvent {
    fn default() -> Self {
        WebhookTranscriptionSuccessPayloadEvent::TranscriptionSuccess
    }
}
///`WordDto`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "end",
///    "start",
///    "word"
///  ],
///  "properties": {
///    "confidence": {
///      "description": "Confidence on the transcribed word (1 = 100% confident)",
///      "type": "number"
///    },
///    "end": {
///      "description": "End timestamps in seconds of the spoken word",
///      "type": "number"
///    },
///    "start": {
///      "description": "Start timestamps in seconds of the spoken word",
///      "type": "number"
///    },
///    "word": {
///      "description": "Spoken word",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WordDto {
    pub confidence: f64,
    pub end: f64,
    pub start: f64,
    ///Spoken word
    pub word: ::std::string::String,
}
impl ::std::convert::From<&WordDto> for WordDto {
    fn from(value: &WordDto) -> Self {
        value.clone()
    }
}
impl WordDto {
    pub fn builder() -> builder::WordDto {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AddonErrorDto {
        exception: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for AddonErrorDto {
        fn default() -> Self {
            Self {
                exception: Err("no value supplied for exception".to_string()),
                message: Err("no value supplied for message".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
            }
        }
    }
    impl AddonErrorDto {
        pub fn exception<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.exception = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exception: {}", e)
                });
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AddonErrorDto> for super::AddonErrorDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddonErrorDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exception: value.exception?,
                message: value.message?,
                status_code: value.status_code?,
            })
        }
    }
    impl ::std::convert::From<super::AddonErrorDto> for AddonErrorDto {
        fn from(value: super::AddonErrorDto) -> Self {
            Self {
                exception: Ok(value.exception),
                message: Ok(value.message),
                status_code: Ok(value.status_code),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioChunkAckData {
        byte_range: ::std::result::Result<[i64; 2usize], ::std::string::String>,
        time_range: ::std::result::Result<[f64; 2usize], ::std::string::String>,
    }
    impl ::std::default::Default for AudioChunkAckData {
        fn default() -> Self {
            Self {
                byte_range: Err("no value supplied for byte_range".to_string()),
                time_range: Err("no value supplied for time_range".to_string()),
            }
        }
    }
    impl AudioChunkAckData {
        pub fn byte_range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[i64; 2usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.byte_range = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for byte_range: {}", e)
                });
            self
        }
        pub fn time_range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[f64; 2usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.time_range = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for time_range: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioChunkAckData> for super::AudioChunkAckData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioChunkAckData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                byte_range: value.byte_range?,
                time_range: value.time_range?,
            })
        }
    }
    impl ::std::convert::From<super::AudioChunkAckData> for AudioChunkAckData {
        fn from(value: super::AudioChunkAckData) -> Self {
            Self {
                byte_range: Ok(value.byte_range),
                time_range: Ok(value.time_range),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioChunkAckMessage {
        acknowledged: ::std::result::Result<bool, ::std::string::String>,
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::AudioChunkAckData, ::std::string::String>,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::AudioChunkAckMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AudioChunkAckMessage {
        fn default() -> Self {
            Self {
                acknowledged: Err("no value supplied for acknowledged".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AudioChunkAckMessage {
        pub fn acknowledged<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledged = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for acknowledged: {}", e)
                });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioChunkAckData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioChunkAckMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioChunkAckMessage> for super::AudioChunkAckMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioChunkAckMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledged: value.acknowledged?,
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AudioChunkAckMessage> for AudioChunkAckMessage {
        fn from(value: super::AudioChunkAckMessage) -> Self {
            Self {
                acknowledged: Ok(value.acknowledged),
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioChunkAction {
        data: ::std::result::Result<super::AudioChunkActionData, ::std::string::String>,
        type_: ::std::result::Result<super::AudioChunkActionType, ::std::string::String>,
    }
    impl ::std::default::Default for AudioChunkAction {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AudioChunkAction {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioChunkActionData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioChunkActionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioChunkAction> for super::AudioChunkAction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioChunkAction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AudioChunkAction> for AudioChunkAction {
        fn from(value: super::AudioChunkAction) -> Self {
            Self {
                data: Ok(value.data),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioChunkActionData {
        chunk: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AudioChunkActionData {
        fn default() -> Self {
            Self {
                chunk: Err("no value supplied for chunk".to_string()),
            }
        }
    }
    impl AudioChunkActionData {
        pub fn chunk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for chunk: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioChunkActionData> for super::AudioChunkActionData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioChunkActionData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { chunk: value.chunk? })
        }
    }
    impl ::std::convert::From<super::AudioChunkActionData> for AudioChunkActionData {
        fn from(value: super::AudioChunkActionData) -> Self {
            Self { chunk: Ok(value.chunk) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioToLlmDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            super::AudioToLlmResultDto,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for AudioToLlmDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl AudioToLlmDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioToLlmResultDto>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioToLlmDto> for super::AudioToLlmDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioToLlmDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::AudioToLlmDto> for AudioToLlmDto {
        fn from(value: super::AudioToLlmDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioToLlmListConfigDto {
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompts: ::std::result::Result<
            ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AudioToLlmListConfigDto {
        fn default() -> Self {
            Self {
                model: Ok(Default::default()),
                prompts: Err("no value supplied for prompts".to_string()),
            }
        }
    }
    impl AudioToLlmListConfigDto {
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn prompts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.prompts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prompts: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioToLlmListConfigDto>
    for super::AudioToLlmListConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioToLlmListConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
                prompts: value.prompts?,
            })
        }
    }
    impl ::std::convert::From<super::AudioToLlmListConfigDto>
    for AudioToLlmListConfigDto {
        fn from(value: super::AudioToLlmListConfigDto) -> Self {
            Self {
                model: Ok(value.model),
                prompts: Ok(value.prompts),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioToLlmListDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::AudioToLlmDto>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for AudioToLlmListDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl AudioToLlmListDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AudioToLlmDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioToLlmListDto> for super::AudioToLlmListDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioToLlmListDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::AudioToLlmListDto> for AudioToLlmListDto {
        fn from(value: super::AudioToLlmListDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioToLlmResultDto {
        prompt: ::std::result::Result<::std::string::String, ::std::string::String>,
        response: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AudioToLlmResultDto {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
                response: Err("no value supplied for response".to_string()),
            }
        }
    }
    impl AudioToLlmResultDto {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prompt: {}", e)
                });
            self
        }
        pub fn response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.response = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for response: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioToLlmResultDto> for super::AudioToLlmResultDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioToLlmResultDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
                response: value.response?,
            })
        }
    }
    impl ::std::convert::From<super::AudioToLlmResultDto> for AudioToLlmResultDto {
        fn from(value: super::AudioToLlmResultDto) -> Self {
            Self {
                prompt: Ok(value.prompt),
                response: Ok(value.response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioUploadMetadataDto {
        audio_duration: ::std::result::Result<f64, ::std::string::String>,
        extension: ::std::result::Result<::std::string::String, ::std::string::String>,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        number_of_channels: ::std::result::Result<i64, ::std::string::String>,
        size: ::std::result::Result<i64, ::std::string::String>,
        source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AudioUploadMetadataDto {
        fn default() -> Self {
            Self {
                audio_duration: Err("no value supplied for audio_duration".to_string()),
                extension: Err("no value supplied for extension".to_string()),
                filename: Err("no value supplied for filename".to_string()),
                id: Err("no value supplied for id".to_string()),
                number_of_channels: Err(
                    "no value supplied for number_of_channels".to_string(),
                ),
                size: Err("no value supplied for size".to_string()),
                source: Ok(Default::default()),
            }
        }
    }
    impl AudioUploadMetadataDto {
        pub fn audio_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_duration: {}", e)
                });
            self
        }
        pub fn extension<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.extension = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for extension: {}", e)
                });
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for filename: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn number_of_channels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_channels = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for number_of_channels: {}", e
                    )
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioUploadMetadataDto>
    for super::AudioUploadMetadataDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioUploadMetadataDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_duration: value.audio_duration?,
                extension: value.extension?,
                filename: value.filename?,
                id: value.id?,
                number_of_channels: value.number_of_channels?,
                size: value.size?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::AudioUploadMetadataDto> for AudioUploadMetadataDto {
        fn from(value: super::AudioUploadMetadataDto) -> Self {
            Self {
                audio_duration: Ok(value.audio_duration),
                extension: Ok(value.extension),
                filename: Ok(value.filename),
                id: Ok(value.id),
                number_of_channels: Ok(value.number_of_channels),
                size: Ok(value.size),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioUploadResponse {
        audio_metadata: ::std::result::Result<
            super::AudioUploadMetadataDto,
            ::std::string::String,
        >,
        audio_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AudioUploadResponse {
        fn default() -> Self {
            Self {
                audio_metadata: Err("no value supplied for audio_metadata".to_string()),
                audio_url: Err("no value supplied for audio_url".to_string()),
            }
        }
    }
    impl AudioUploadResponse {
        pub fn audio_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioUploadMetadataDto>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_metadata: {}", e)
                });
            self
        }
        pub fn audio_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_url: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioUploadResponse> for super::AudioUploadResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioUploadResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_metadata: value.audio_metadata?,
                audio_url: value.audio_url?,
            })
        }
    }
    impl ::std::convert::From<super::AudioUploadResponse> for AudioUploadResponse {
        fn from(value: super::AudioUploadResponse) -> Self {
            Self {
                audio_metadata: Ok(value.audio_metadata),
                audio_url: Ok(value.audio_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BadRequestErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
        validation_errors: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BadRequestErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                validation_errors: Ok(Default::default()),
            }
        }
    }
    impl BadRequestErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
        pub fn validation_errors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.validation_errors = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for validation_errors: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<BadRequestErrorResponse>
    for super::BadRequestErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BadRequestErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
                validation_errors: value.validation_errors?,
            })
        }
    }
    impl ::std::convert::From<super::BadRequestErrorResponse>
    for BadRequestErrorResponse {
        fn from(value: super::BadRequestErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
                validation_errors: Ok(value.validation_errors),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackConfig {
        receive_acknowledgments: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_errors: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_final_transcripts: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_lifecycle_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_partial_transcripts: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_post_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_pre_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_realtime_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_speech_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackConfig {
        fn default() -> Self {
            Self {
                receive_acknowledgments: Ok(Default::default()),
                receive_errors: Ok(Default::default()),
                receive_final_transcripts: Ok(Default::default()),
                receive_lifecycle_events: Ok(Default::default()),
                receive_partial_transcripts: Ok(Default::default()),
                receive_post_processing_events: Ok(Default::default()),
                receive_pre_processing_events: Ok(Default::default()),
                receive_realtime_processing_events: Ok(Default::default()),
                receive_speech_events: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl CallbackConfig {
        pub fn receive_acknowledgments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_acknowledgments = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_acknowledgments: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_errors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_errors = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for receive_errors: {}", e)
                });
            self
        }
        pub fn receive_final_transcripts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_final_transcripts = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_final_transcripts: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_lifecycle_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_lifecycle_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_lifecycle_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_partial_transcripts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_partial_transcripts = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_partial_transcripts: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_post_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_post_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_post_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_pre_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_pre_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_pre_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_realtime_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_realtime_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_realtime_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_speech_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_speech_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_speech_events: {}",
                        e
                    )
                });
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackConfig> for super::CallbackConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                receive_acknowledgments: value.receive_acknowledgments?,
                receive_errors: value.receive_errors?,
                receive_final_transcripts: value.receive_final_transcripts?,
                receive_lifecycle_events: value.receive_lifecycle_events?,
                receive_partial_transcripts: value.receive_partial_transcripts?,
                receive_post_processing_events: value.receive_post_processing_events?,
                receive_pre_processing_events: value.receive_pre_processing_events?,
                receive_realtime_processing_events: value
                    .receive_realtime_processing_events?,
                receive_speech_events: value.receive_speech_events?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackConfig> for CallbackConfig {
        fn from(value: super::CallbackConfig) -> Self {
            Self {
                receive_acknowledgments: Ok(value.receive_acknowledgments),
                receive_errors: Ok(value.receive_errors),
                receive_final_transcripts: Ok(value.receive_final_transcripts),
                receive_lifecycle_events: Ok(value.receive_lifecycle_events),
                receive_partial_transcripts: Ok(value.receive_partial_transcripts),
                receive_post_processing_events: Ok(value.receive_post_processing_events),
                receive_pre_processing_events: Ok(value.receive_pre_processing_events),
                receive_realtime_processing_events: Ok(
                    value.receive_realtime_processing_events,
                ),
                receive_speech_events: Ok(value.receive_speech_events),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackConfigDto {
        method: ::std::result::Result<
            ::std::option::Option<super::CallbackMethodEnum>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackConfigDto {
        fn default() -> Self {
            Self {
                method: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl CallbackConfigDto {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallbackMethodEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for method: {}", e)
                });
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackConfigDto> for super::CallbackConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackConfigDto> for CallbackConfigDto {
        fn from(value: super::CallbackConfigDto) -> Self {
            Self {
                method: Ok(value.method),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveAudioChunkAckMessage {
        event: ::std::result::Result<
            super::CallbackLiveAudioChunkAckMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::AudioChunkAckMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveAudioChunkAckMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveAudioChunkAckMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveAudioChunkAckMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AudioChunkAckMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveAudioChunkAckMessage>
    for super::CallbackLiveAudioChunkAckMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveAudioChunkAckMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveAudioChunkAckMessage>
    for CallbackLiveAudioChunkAckMessage {
        fn from(value: super::CallbackLiveAudioChunkAckMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveEndRecordingMessage {
        event: ::std::result::Result<
            super::CallbackLiveEndRecordingMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::EndRecordingMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveEndRecordingMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveEndRecordingMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveEndRecordingMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndRecordingMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveEndRecordingMessage>
    for super::CallbackLiveEndRecordingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveEndRecordingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveEndRecordingMessage>
    for CallbackLiveEndRecordingMessage {
        fn from(value: super::CallbackLiveEndRecordingMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveEndSessionMessage {
        event: ::std::result::Result<
            super::CallbackLiveEndSessionMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<super::EndSessionMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackLiveEndSessionMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveEndSessionMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveEndSessionMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndSessionMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveEndSessionMessage>
    for super::CallbackLiveEndSessionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveEndSessionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveEndSessionMessage>
    for CallbackLiveEndSessionMessage {
        fn from(value: super::CallbackLiveEndSessionMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveNamedEntityRecognitionMessage {
        event: ::std::result::Result<
            super::CallbackLiveNamedEntityRecognitionMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::NamedEntityRecognitionMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveNamedEntityRecognitionMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveNamedEntityRecognitionMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CallbackLiveNamedEntityRecognitionMessageEvent,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NamedEntityRecognitionMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveNamedEntityRecognitionMessage>
    for super::CallbackLiveNamedEntityRecognitionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveNamedEntityRecognitionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveNamedEntityRecognitionMessage>
    for CallbackLiveNamedEntityRecognitionMessage {
        fn from(value: super::CallbackLiveNamedEntityRecognitionMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLivePostFinalTranscriptMessage {
        event: ::std::result::Result<
            super::CallbackLivePostFinalTranscriptMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::PostFinalTranscriptMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLivePostFinalTranscriptMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLivePostFinalTranscriptMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CallbackLivePostFinalTranscriptMessageEvent,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostFinalTranscriptMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLivePostFinalTranscriptMessage>
    for super::CallbackLivePostFinalTranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLivePostFinalTranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLivePostFinalTranscriptMessage>
    for CallbackLivePostFinalTranscriptMessage {
        fn from(value: super::CallbackLivePostFinalTranscriptMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLivePostSummarizationMessage {
        event: ::std::result::Result<
            super::CallbackLivePostSummarizationMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::PostSummarizationMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLivePostSummarizationMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLivePostSummarizationMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLivePostSummarizationMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostSummarizationMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLivePostSummarizationMessage>
    for super::CallbackLivePostSummarizationMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLivePostSummarizationMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLivePostSummarizationMessage>
    for CallbackLivePostSummarizationMessage {
        fn from(value: super::CallbackLivePostSummarizationMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLivePostTranscriptMessage {
        event: ::std::result::Result<
            super::CallbackLivePostTranscriptMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::PostTranscriptMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLivePostTranscriptMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLivePostTranscriptMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLivePostTranscriptMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostTranscriptMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLivePostTranscriptMessage>
    for super::CallbackLivePostTranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLivePostTranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLivePostTranscriptMessage>
    for CallbackLivePostTranscriptMessage {
        fn from(value: super::CallbackLivePostTranscriptMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveSentimentAnalysisMessage {
        event: ::std::result::Result<
            super::CallbackLiveSentimentAnalysisMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::SentimentAnalysisMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveSentimentAnalysisMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveSentimentAnalysisMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveSentimentAnalysisMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SentimentAnalysisMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveSentimentAnalysisMessage>
    for super::CallbackLiveSentimentAnalysisMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveSentimentAnalysisMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveSentimentAnalysisMessage>
    for CallbackLiveSentimentAnalysisMessage {
        fn from(value: super::CallbackLiveSentimentAnalysisMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveSpeechEndMessage {
        event: ::std::result::Result<
            super::CallbackLiveSpeechEndMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<super::SpeechEndMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackLiveSpeechEndMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveSpeechEndMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveSpeechEndMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechEndMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveSpeechEndMessage>
    for super::CallbackLiveSpeechEndMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveSpeechEndMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveSpeechEndMessage>
    for CallbackLiveSpeechEndMessage {
        fn from(value: super::CallbackLiveSpeechEndMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveSpeechStartMessage {
        event: ::std::result::Result<
            super::CallbackLiveSpeechStartMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<super::SpeechStartMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackLiveSpeechStartMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveSpeechStartMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveSpeechStartMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechStartMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveSpeechStartMessage>
    for super::CallbackLiveSpeechStartMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveSpeechStartMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveSpeechStartMessage>
    for CallbackLiveSpeechStartMessage {
        fn from(value: super::CallbackLiveSpeechStartMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveStartRecordingMessage {
        event: ::std::result::Result<
            super::CallbackLiveStartRecordingMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::StartRecordingMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveStartRecordingMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveStartRecordingMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveStartRecordingMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StartRecordingMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveStartRecordingMessage>
    for super::CallbackLiveStartRecordingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveStartRecordingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveStartRecordingMessage>
    for CallbackLiveStartRecordingMessage {
        fn from(value: super::CallbackLiveStartRecordingMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveStartSessionMessage {
        event: ::std::result::Result<
            super::CallbackLiveStartSessionMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::StartSessionMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveStartSessionMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveStartSessionMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveStartSessionMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StartSessionMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveStartSessionMessage>
    for super::CallbackLiveStartSessionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveStartSessionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveStartSessionMessage>
    for CallbackLiveStartSessionMessage {
        fn from(value: super::CallbackLiveStartSessionMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveStopRecordingAckMessage {
        event: ::std::result::Result<
            super::CallbackLiveStopRecordingAckMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::StopRecordingAckMessage,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackLiveStopRecordingAckMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveStopRecordingAckMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveStopRecordingAckMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StopRecordingAckMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveStopRecordingAckMessage>
    for super::CallbackLiveStopRecordingAckMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveStopRecordingAckMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveStopRecordingAckMessage>
    for CallbackLiveStopRecordingAckMessage {
        fn from(value: super::CallbackLiveStopRecordingAckMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveTranscriptMessage {
        event: ::std::result::Result<
            super::CallbackLiveTranscriptMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<super::TranscriptMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackLiveTranscriptMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveTranscriptMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveTranscriptMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveTranscriptMessage>
    for super::CallbackLiveTranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveTranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveTranscriptMessage>
    for CallbackLiveTranscriptMessage {
        fn from(value: super::CallbackLiveTranscriptMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackLiveTranslationMessage {
        event: ::std::result::Result<
            super::CallbackLiveTranslationMessageEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<super::TranslationMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackLiveTranslationMessage {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackLiveTranslationMessage {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackLiveTranslationMessageEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranslationMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackLiveTranslationMessage>
    for super::CallbackLiveTranslationMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackLiveTranslationMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackLiveTranslationMessage>
    for CallbackLiveTranslationMessage {
        fn from(value: super::CallbackLiveTranslationMessage) -> Self {
            Self {
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackTranscriptionErrorPayload {
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        error: ::std::result::Result<super::ErrorDto, ::std::string::String>,
        event: ::std::result::Result<
            super::CallbackTranscriptionErrorPayloadEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    }
    impl ::std::default::Default for CallbackTranscriptionErrorPayload {
        fn default() -> Self {
            Self {
                custom_metadata: Ok(Default::default()),
                error: Err("no value supplied for error".to_string()),
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl CallbackTranscriptionErrorPayload {
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackTranscriptionErrorPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackTranscriptionErrorPayload>
    for super::CallbackTranscriptionErrorPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackTranscriptionErrorPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom_metadata: value.custom_metadata?,
                error: value.error?,
                event: value.event?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackTranscriptionErrorPayload>
    for CallbackTranscriptionErrorPayload {
        fn from(value: super::CallbackTranscriptionErrorPayload) -> Self {
            Self {
                custom_metadata: Ok(value.custom_metadata),
                error: Ok(value.error),
                event: Ok(value.event),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallbackTranscriptionSuccessPayload {
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        event: ::std::result::Result<
            super::CallbackTranscriptionSuccessPayloadEvent,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        payload: ::std::result::Result<
            super::TranscriptionResultDto,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallbackTranscriptionSuccessPayload {
        fn default() -> Self {
            Self {
                custom_metadata: Ok(Default::default()),
                event: Err("no value supplied for event".to_string()),
                id: Err("no value supplied for id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl CallbackTranscriptionSuccessPayload {
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallbackTranscriptionSuccessPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionResultDto>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CallbackTranscriptionSuccessPayload>
    for super::CallbackTranscriptionSuccessPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallbackTranscriptionSuccessPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom_metadata: value.custom_metadata?,
                event: value.event?,
                id: value.id?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::CallbackTranscriptionSuccessPayload>
    for CallbackTranscriptionSuccessPayload {
        fn from(value: super::CallbackTranscriptionSuccessPayload) -> Self {
            Self {
                custom_metadata: Ok(value.custom_metadata),
                event: Ok(value.event),
                id: Ok(value.id),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomSpellingConfigDto {
        spelling_dictionary: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<::std::string::String>,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomSpellingConfigDto {
        fn default() -> Self {
            Self {
                spelling_dictionary: Err(
                    "no value supplied for spelling_dictionary".to_string(),
                ),
            }
        }
    }
    impl CustomSpellingConfigDto {
        pub fn spelling_dictionary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<::std::string::String>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.spelling_dictionary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for spelling_dictionary: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomSpellingConfigDto>
    for super::CustomSpellingConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomSpellingConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                spelling_dictionary: value.spelling_dictionary?,
            })
        }
    }
    impl ::std::convert::From<super::CustomSpellingConfigDto>
    for CustomSpellingConfigDto {
        fn from(value: super::CustomSpellingConfigDto) -> Self {
            Self {
                spelling_dictionary: Ok(value.spelling_dictionary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomVocabularyConfigDto {
        default_intensity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        vocabulary: ::std::result::Result<
            ::std::vec::Vec<super::CustomVocabularyConfigDtoVocabularyItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomVocabularyConfigDto {
        fn default() -> Self {
            Self {
                default_intensity: Ok(Default::default()),
                vocabulary: Err("no value supplied for vocabulary".to_string()),
            }
        }
    }
    impl CustomVocabularyConfigDto {
        pub fn default_intensity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_intensity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for default_intensity: {}", e
                    )
                });
            self
        }
        pub fn vocabulary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CustomVocabularyConfigDtoVocabularyItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.vocabulary = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for vocabulary: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomVocabularyConfigDto>
    for super::CustomVocabularyConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomVocabularyConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_intensity: value.default_intensity?,
                vocabulary: value.vocabulary?,
            })
        }
    }
    impl ::std::convert::From<super::CustomVocabularyConfigDto>
    for CustomVocabularyConfigDto {
        fn from(value: super::CustomVocabularyConfigDto) -> Self {
            Self {
                default_intensity: Ok(value.default_intensity),
                vocabulary: Ok(value.vocabulary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomVocabularyEntryDto {
        intensity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        language: ::std::result::Result<
            ::std::option::Option<super::TranscriptionLanguageCodeEnum>,
            ::std::string::String,
        >,
        pronunciations: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CustomVocabularyEntryDto {
        fn default() -> Self {
            Self {
                intensity: Ok(Default::default()),
                language: Ok(Default::default()),
                pronunciations: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl CustomVocabularyEntryDto {
        pub fn intensity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.intensity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intensity: {}", e)
                });
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranscriptionLanguageCodeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn pronunciations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pronunciations = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pronunciations: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomVocabularyEntryDto>
    for super::CustomVocabularyEntryDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomVocabularyEntryDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                intensity: value.intensity?,
                language: value.language?,
                pronunciations: value.pronunciations?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::CustomVocabularyEntryDto>
    for CustomVocabularyEntryDto {
        fn from(value: super::CustomVocabularyEntryDto) -> Self {
            Self {
                intensity: Ok(value.intensity),
                language: Ok(value.language),
                pronunciations: Ok(value.pronunciations),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiarizationConfigDto {
        max_speakers: ::std::result::Result<
            ::std::option::Option<u64>,
            ::std::string::String,
        >,
        min_speakers: ::std::result::Result<
            ::std::option::Option<u64>,
            ::std::string::String,
        >,
        number_of_speakers: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiarizationConfigDto {
        fn default() -> Self {
            Self {
                max_speakers: Ok(Default::default()),
                min_speakers: Ok(Default::default()),
                number_of_speakers: Ok(Default::default()),
            }
        }
    }
    impl DiarizationConfigDto {
        pub fn max_speakers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_speakers = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for max_speakers: {}", e)
                });
            self
        }
        pub fn min_speakers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_speakers = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for min_speakers: {}", e)
                });
            self
        }
        pub fn number_of_speakers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_speakers = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for number_of_speakers: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DiarizationConfigDto> for super::DiarizationConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiarizationConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_speakers: value.max_speakers?,
                min_speakers: value.min_speakers?,
                number_of_speakers: value.number_of_speakers?,
            })
        }
    }
    impl ::std::convert::From<super::DiarizationConfigDto> for DiarizationConfigDto {
        fn from(value: super::DiarizationConfigDto) -> Self {
            Self {
                max_speakers: Ok(value.max_speakers),
                min_speakers: Ok(value.min_speakers),
                number_of_speakers: Ok(value.number_of_speakers),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiarizationDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::UtteranceDto>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for DiarizationDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl DiarizationDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UtteranceDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DiarizationDto> for super::DiarizationDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiarizationDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::DiarizationDto> for DiarizationDto {
        fn from(value: super::DiarizationDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DisplayModeDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for DisplayModeDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl DisplayModeDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DisplayModeDto> for super::DisplayModeDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DisplayModeDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::DisplayModeDto> for DisplayModeDto {
        fn from(value: super::DisplayModeDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EndRecordingMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<
            super::EndRecordingMessageData,
            ::std::string::String,
        >,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::EndRecordingMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EndRecordingMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EndRecordingMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndRecordingMessageData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndRecordingMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<EndRecordingMessage> for super::EndRecordingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EndRecordingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EndRecordingMessage> for EndRecordingMessage {
        fn from(value: super::EndRecordingMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EndRecordingMessageData {
        recording_duration: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for EndRecordingMessageData {
        fn default() -> Self {
            Self {
                recording_duration: Err(
                    "no value supplied for recording_duration".to_string(),
                ),
            }
        }
    }
    impl EndRecordingMessageData {
        pub fn recording_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.recording_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for recording_duration: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<EndRecordingMessageData>
    for super::EndRecordingMessageData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EndRecordingMessageData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recording_duration: value.recording_duration?,
            })
        }
    }
    impl ::std::convert::From<super::EndRecordingMessageData>
    for EndRecordingMessageData {
        fn from(value: super::EndRecordingMessageData) -> Self {
            Self {
                recording_duration: Ok(value.recording_duration),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EndSessionMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::EndSessionMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EndSessionMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EndSessionMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndSessionMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<EndSessionMessage> for super::EndSessionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EndSessionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EndSessionMessage> for EndSessionMessage {
        fn from(value: super::EndSessionMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Error {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Error {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl Error {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Error> for super::Error {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Error,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { message: value.message? })
        }
    }
    impl ::std::convert::From<super::Error> for Error {
        fn from(value: super::Error) -> Self {
            Self { message: Ok(value.message) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ErrorDto {
        code: ::std::result::Result<i64, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ErrorDto {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl ErrorDto {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ErrorDto> for super::ErrorDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ErrorDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::ErrorDto> for ErrorDto {
        fn from(value: super::ErrorDto) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileResponse {
        audio_duration: ::std::result::Result<f64, ::std::string::String>,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        number_of_channels: ::std::result::Result<
            ::std::num::NonZeroU64,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FileResponse {
        fn default() -> Self {
            Self {
                audio_duration: Err("no value supplied for audio_duration".to_string()),
                filename: Err("no value supplied for filename".to_string()),
                id: Err("no value supplied for id".to_string()),
                number_of_channels: Err(
                    "no value supplied for number_of_channels".to_string(),
                ),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl FileResponse {
        pub fn audio_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_duration: {}", e)
                });
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for filename: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn number_of_channels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_channels = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for number_of_channels: {}", e
                    )
                });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FileResponse> for super::FileResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_duration: value.audio_duration?,
                filename: value.filename?,
                id: value.id?,
                number_of_channels: value.number_of_channels?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::FileResponse> for FileResponse {
        fn from(value: super::FileResponse) -> Self {
            Self {
                audio_duration: Ok(value.audio_duration),
                filename: Ok(value.filename),
                id: Ok(value.id),
                number_of_channels: Ok(value.number_of_channels),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ForbiddenErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ForbiddenErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl ForbiddenErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ForbiddenErrorResponse>
    for super::ForbiddenErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ForbiddenErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::ForbiddenErrorResponse> for ForbiddenErrorResponse {
        fn from(value: super::ForbiddenErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitPreRecordedTranscriptionResponse {
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        result_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InitPreRecordedTranscriptionResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                result_url: Err("no value supplied for result_url".to_string()),
            }
        }
    }
    impl InitPreRecordedTranscriptionResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn result_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for result_url: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<InitPreRecordedTranscriptionResponse>
    for super::InitPreRecordedTranscriptionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitPreRecordedTranscriptionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                result_url: value.result_url?,
            })
        }
    }
    impl ::std::convert::From<super::InitPreRecordedTranscriptionResponse>
    for InitPreRecordedTranscriptionResponse {
        fn from(value: super::InitPreRecordedTranscriptionResponse) -> Self {
            Self {
                id: Ok(value.id),
                result_url: Ok(value.result_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitStreamingResponse {
        created_at: ::std::result::Result<
            ::chrono::DateTime<::chrono::offset::Utc>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InitStreamingResponse {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl InitStreamingResponse {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<InitStreamingResponse>
    for super::InitStreamingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitStreamingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::InitStreamingResponse> for InitStreamingResponse {
        fn from(value: super::InitStreamingResponse) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitTranscriptionRequest {
        audio_to_llm: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        audio_to_llm_config: ::std::result::Result<
            ::std::option::Option<super::AudioToLlmListConfigDto>,
            ::std::string::String,
        >,
        audio_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        callback: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        callback_config: ::std::result::Result<
            ::std::option::Option<super::CallbackConfigDto>,
            ::std::string::String,
        >,
        callback_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        custom_spelling: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_spelling_config: ::std::result::Result<
            ::std::option::Option<super::CustomSpellingConfigDto>,
            ::std::string::String,
        >,
        custom_vocabulary: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_vocabulary_config: ::std::result::Result<
            ::std::option::Option<super::CustomVocabularyConfigDto>,
            ::std::string::String,
        >,
        diarization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        diarization_config: ::std::result::Result<
            ::std::option::Option<super::DiarizationConfigDto>,
            ::std::string::String,
        >,
        language_config: ::std::result::Result<
            ::std::option::Option<super::LanguageConfig>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::TranscriptionSupportedModels>,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        pii_redaction: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        pii_redaction_config: ::std::result::Result<
            ::std::option::Option<super::PiiRedactionConfigDto>,
            ::std::string::String,
        >,
        punctuation_enhanced: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        sentences: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        subtitles: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        subtitles_config: ::std::result::Result<
            ::std::option::Option<super::SubtitlesConfigDto>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        summarization_config: ::std::result::Result<
            ::std::option::Option<super::SummarizationConfigDto>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        translation_config: ::std::result::Result<
            ::std::option::Option<super::TranslationConfigDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InitTranscriptionRequest {
        fn default() -> Self {
            Self {
                audio_to_llm: Ok(Default::default()),
                audio_to_llm_config: Ok(Default::default()),
                audio_url: Err("no value supplied for audio_url".to_string()),
                callback: Ok(Default::default()),
                callback_config: Ok(Default::default()),
                callback_url: Ok(Default::default()),
                custom_metadata: Ok(Default::default()),
                custom_spelling: Ok(Default::default()),
                custom_spelling_config: Ok(Default::default()),
                custom_vocabulary: Ok(Default::default()),
                custom_vocabulary_config: Ok(Default::default()),
                diarization: Ok(Default::default()),
                diarization_config: Ok(Default::default()),
                language_config: Ok(Default::default()),
                model: Ok(Default::default()),
                named_entity_recognition: Ok(Default::default()),
                pii_redaction: Ok(Default::default()),
                pii_redaction_config: Ok(Default::default()),
                punctuation_enhanced: Ok(Default::default()),
                sentences: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                subtitles: Ok(Default::default()),
                subtitles_config: Ok(Default::default()),
                summarization: Ok(Default::default()),
                summarization_config: Ok(Default::default()),
                translation: Ok(Default::default()),
                translation_config: Ok(Default::default()),
            }
        }
    }
    impl InitTranscriptionRequest {
        pub fn audio_to_llm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_to_llm = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_to_llm: {}", e)
                });
            self
        }
        pub fn audio_to_llm_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AudioToLlmListConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio_to_llm_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for audio_to_llm_config: {}", e
                    )
                });
            self
        }
        pub fn audio_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_url: {}", e)
                });
            self
        }
        pub fn callback<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback: {}", e)
                });
            self
        }
        pub fn callback_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallbackConfigDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_config: {}", e)
                });
            self
        }
        pub fn callback_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_url: {}", e)
                });
            self
        }
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn custom_spelling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_spelling: {}", e)
                });
            self
        }
        pub fn custom_spelling_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomSpellingConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_spelling_config: {}",
                        e
                    )
                });
            self
        }
        pub fn custom_vocabulary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary: {}", e
                    )
                });
            self
        }
        pub fn custom_vocabulary_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomVocabularyConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary_config: {}",
                        e
                    )
                });
            self
        }
        pub fn diarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for diarization: {}", e)
                });
            self
        }
        pub fn diarization_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiarizationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.diarization_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for diarization_config: {}", e
                    )
                });
            self
        }
        pub fn language_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language_config: {}", e)
                });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranscriptionSupportedModels>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn pii_redaction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pii_redaction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pii_redaction: {}", e)
                });
            self
        }
        pub fn pii_redaction_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PiiRedactionConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pii_redaction_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for pii_redaction_config: {}", e
                    )
                });
            self
        }
        pub fn punctuation_enhanced<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.punctuation_enhanced = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for punctuation_enhanced: {}", e
                    )
                });
            self
        }
        pub fn sentences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentences: {}", e)
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn subtitles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtitles: {}", e)
                });
            self
        }
        pub fn subtitles_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SubtitlesConfigDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for subtitles_config: {}", e
                    )
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn summarization_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SummarizationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.summarization_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for summarization_config: {}", e
                    )
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
        pub fn translation_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranslationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.translation_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for translation_config: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<InitTranscriptionRequest>
    for super::InitTranscriptionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitTranscriptionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_to_llm: value.audio_to_llm?,
                audio_to_llm_config: value.audio_to_llm_config?,
                audio_url: value.audio_url?,
                callback: value.callback?,
                callback_config: value.callback_config?,
                callback_url: value.callback_url?,
                custom_metadata: value.custom_metadata?,
                custom_spelling: value.custom_spelling?,
                custom_spelling_config: value.custom_spelling_config?,
                custom_vocabulary: value.custom_vocabulary?,
                custom_vocabulary_config: value.custom_vocabulary_config?,
                diarization: value.diarization?,
                diarization_config: value.diarization_config?,
                language_config: value.language_config?,
                model: value.model?,
                named_entity_recognition: value.named_entity_recognition?,
                pii_redaction: value.pii_redaction?,
                pii_redaction_config: value.pii_redaction_config?,
                punctuation_enhanced: value.punctuation_enhanced?,
                sentences: value.sentences?,
                sentiment_analysis: value.sentiment_analysis?,
                subtitles: value.subtitles?,
                subtitles_config: value.subtitles_config?,
                summarization: value.summarization?,
                summarization_config: value.summarization_config?,
                translation: value.translation?,
                translation_config: value.translation_config?,
            })
        }
    }
    impl ::std::convert::From<super::InitTranscriptionRequest>
    for InitTranscriptionRequest {
        fn from(value: super::InitTranscriptionRequest) -> Self {
            Self {
                audio_to_llm: Ok(value.audio_to_llm),
                audio_to_llm_config: Ok(value.audio_to_llm_config),
                audio_url: Ok(value.audio_url),
                callback: Ok(value.callback),
                callback_config: Ok(value.callback_config),
                callback_url: Ok(value.callback_url),
                custom_metadata: Ok(value.custom_metadata),
                custom_spelling: Ok(value.custom_spelling),
                custom_spelling_config: Ok(value.custom_spelling_config),
                custom_vocabulary: Ok(value.custom_vocabulary),
                custom_vocabulary_config: Ok(value.custom_vocabulary_config),
                diarization: Ok(value.diarization),
                diarization_config: Ok(value.diarization_config),
                language_config: Ok(value.language_config),
                model: Ok(value.model),
                named_entity_recognition: Ok(value.named_entity_recognition),
                pii_redaction: Ok(value.pii_redaction),
                pii_redaction_config: Ok(value.pii_redaction_config),
                punctuation_enhanced: Ok(value.punctuation_enhanced),
                sentences: Ok(value.sentences),
                sentiment_analysis: Ok(value.sentiment_analysis),
                subtitles: Ok(value.subtitles),
                subtitles_config: Ok(value.subtitles_config),
                summarization: Ok(value.summarization),
                summarization_config: Ok(value.summarization_config),
                translation: Ok(value.translation),
                translation_config: Ok(value.translation_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LanguageConfig {
        code_switching: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        languages: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionLanguageCodeEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LanguageConfig {
        fn default() -> Self {
            Self {
                code_switching: Ok(Default::default()),
                languages: Ok(Default::default()),
            }
        }
    }
    impl LanguageConfig {
        pub fn code_switching<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_switching = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for code_switching: {}", e)
                });
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TranscriptionLanguageCodeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for languages: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LanguageConfig> for super::LanguageConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LanguageConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_switching: value.code_switching?,
                languages: value.languages?,
            })
        }
    }
    impl ::std::convert::From<super::LanguageConfig> for LanguageConfig {
        fn from(value: super::LanguageConfig) -> Self {
            Self {
                code_switching: Ok(value.code_switching),
                languages: Ok(value.languages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListHistoryResponse {
        current: ::std::result::Result<::std::string::String, ::std::string::String>,
        first: ::std::result::Result<::std::string::String, ::std::string::String>,
        items: ::std::result::Result<
            ::std::vec::Vec<super::ListHistoryResponseItemsItem>,
            ::std::string::String,
        >,
        next: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListHistoryResponse {
        fn default() -> Self {
            Self {
                current: Err("no value supplied for current".to_string()),
                first: Err("no value supplied for first".to_string()),
                items: Err("no value supplied for items".to_string()),
                next: Err("no value supplied for next".to_string()),
            }
        }
    }
    impl ListHistoryResponse {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn first<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.first = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for first: {}", e)
                });
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ListHistoryResponseItemsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for items: {}", e)
                });
            self
        }
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListHistoryResponse> for super::ListHistoryResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListHistoryResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                first: value.first?,
                items: value.items?,
                next: value.next?,
            })
        }
    }
    impl ::std::convert::From<super::ListHistoryResponse> for ListHistoryResponse {
        fn from(value: super::ListHistoryResponse) -> Self {
            Self {
                current: Ok(value.current),
                first: Ok(value.first),
                items: Ok(value.items),
                next: Ok(value.next),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPreRecordedResponse {
        current: ::std::result::Result<::std::string::String, ::std::string::String>,
        first: ::std::result::Result<::std::string::String, ::std::string::String>,
        items: ::std::result::Result<
            ::std::vec::Vec<super::PreRecordedResponse>,
            ::std::string::String,
        >,
        next: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListPreRecordedResponse {
        fn default() -> Self {
            Self {
                current: Err("no value supplied for current".to_string()),
                first: Err("no value supplied for first".to_string()),
                items: Err("no value supplied for items".to_string()),
                next: Err("no value supplied for next".to_string()),
            }
        }
    }
    impl ListPreRecordedResponse {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn first<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.first = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for first: {}", e)
                });
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PreRecordedResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for items: {}", e)
                });
            self
        }
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPreRecordedResponse>
    for super::ListPreRecordedResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPreRecordedResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                first: value.first?,
                items: value.items?,
                next: value.next?,
            })
        }
    }
    impl ::std::convert::From<super::ListPreRecordedResponse>
    for ListPreRecordedResponse {
        fn from(value: super::ListPreRecordedResponse) -> Self {
            Self {
                current: Ok(value.current),
                first: Ok(value.first),
                items: Ok(value.items),
                next: Ok(value.next),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListStreamingResponse {
        current: ::std::result::Result<::std::string::String, ::std::string::String>,
        first: ::std::result::Result<::std::string::String, ::std::string::String>,
        items: ::std::result::Result<
            ::std::vec::Vec<super::StreamingResponse>,
            ::std::string::String,
        >,
        next: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListStreamingResponse {
        fn default() -> Self {
            Self {
                current: Err("no value supplied for current".to_string()),
                first: Err("no value supplied for first".to_string()),
                items: Err("no value supplied for items".to_string()),
                next: Err("no value supplied for next".to_string()),
            }
        }
    }
    impl ListStreamingResponse {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn first<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.first = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for first: {}", e)
                });
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::StreamingResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for items: {}", e)
                });
            self
        }
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListStreamingResponse>
    for super::ListStreamingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListStreamingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                first: value.first?,
                items: value.items?,
                next: value.next?,
            })
        }
    }
    impl ::std::convert::From<super::ListStreamingResponse> for ListStreamingResponse {
        fn from(value: super::ListStreamingResponse) -> Self {
            Self {
                current: Ok(value.current),
                first: Ok(value.first),
                items: Ok(value.items),
                next: Ok(value.next),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTranscriptionResponse {
        current: ::std::result::Result<::std::string::String, ::std::string::String>,
        first: ::std::result::Result<::std::string::String, ::std::string::String>,
        items: ::std::result::Result<
            ::std::vec::Vec<super::ListTranscriptionResponseItemsItem>,
            ::std::string::String,
        >,
        next: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListTranscriptionResponse {
        fn default() -> Self {
            Self {
                current: Err("no value supplied for current".to_string()),
                first: Err("no value supplied for first".to_string()),
                items: Err("no value supplied for items".to_string()),
                next: Err("no value supplied for next".to_string()),
            }
        }
    }
    impl ListTranscriptionResponse {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn first<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.first = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for first: {}", e)
                });
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ListTranscriptionResponseItemsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for items: {}", e)
                });
            self
        }
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTranscriptionResponse>
    for super::ListTranscriptionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTranscriptionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                first: value.first?,
                items: value.items?,
                next: value.next?,
            })
        }
    }
    impl ::std::convert::From<super::ListTranscriptionResponse>
    for ListTranscriptionResponse {
        fn from(value: super::ListTranscriptionResponse) -> Self {
            Self {
                current: Ok(value.current),
                first: Ok(value.first),
                items: Ok(value.items),
                next: Ok(value.next),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveEventPayload {
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    }
    impl ::std::default::Default for LiveEventPayload {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl LiveEventPayload {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveEventPayload> for super::LiveEventPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveEventPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::LiveEventPayload> for LiveEventPayload {
        fn from(value: super::LiveEventPayload) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessagesConfig {
        receive_acknowledgments: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_errors: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_final_transcripts: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_lifecycle_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_partial_transcripts: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_post_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_pre_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_realtime_processing_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        receive_speech_events: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessagesConfig {
        fn default() -> Self {
            Self {
                receive_acknowledgments: Ok(Default::default()),
                receive_errors: Ok(Default::default()),
                receive_final_transcripts: Ok(Default::default()),
                receive_lifecycle_events: Ok(Default::default()),
                receive_partial_transcripts: Ok(Default::default()),
                receive_post_processing_events: Ok(Default::default()),
                receive_pre_processing_events: Ok(Default::default()),
                receive_realtime_processing_events: Ok(Default::default()),
                receive_speech_events: Ok(Default::default()),
            }
        }
    }
    impl MessagesConfig {
        pub fn receive_acknowledgments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_acknowledgments = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_acknowledgments: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_errors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_errors = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for receive_errors: {}", e)
                });
            self
        }
        pub fn receive_final_transcripts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_final_transcripts = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_final_transcripts: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_lifecycle_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_lifecycle_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_lifecycle_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_partial_transcripts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_partial_transcripts = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_partial_transcripts: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_post_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_post_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_post_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_pre_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_pre_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_pre_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_realtime_processing_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_realtime_processing_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_realtime_processing_events: {}",
                        e
                    )
                });
            self
        }
        pub fn receive_speech_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.receive_speech_events = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for receive_speech_events: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<MessagesConfig> for super::MessagesConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessagesConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                receive_acknowledgments: value.receive_acknowledgments?,
                receive_errors: value.receive_errors?,
                receive_final_transcripts: value.receive_final_transcripts?,
                receive_lifecycle_events: value.receive_lifecycle_events?,
                receive_partial_transcripts: value.receive_partial_transcripts?,
                receive_post_processing_events: value.receive_post_processing_events?,
                receive_pre_processing_events: value.receive_pre_processing_events?,
                receive_realtime_processing_events: value
                    .receive_realtime_processing_events?,
                receive_speech_events: value.receive_speech_events?,
            })
        }
    }
    impl ::std::convert::From<super::MessagesConfig> for MessagesConfig {
        fn from(value: super::MessagesConfig) -> Self {
            Self {
                receive_acknowledgments: Ok(value.receive_acknowledgments),
                receive_errors: Ok(value.receive_errors),
                receive_final_transcripts: Ok(value.receive_final_transcripts),
                receive_lifecycle_events: Ok(value.receive_lifecycle_events),
                receive_partial_transcripts: Ok(value.receive_partial_transcripts),
                receive_post_processing_events: Ok(value.receive_post_processing_events),
                receive_pre_processing_events: Ok(value.receive_pre_processing_events),
                receive_realtime_processing_events: Ok(
                    value.receive_realtime_processing_events,
                ),
                receive_speech_events: Ok(value.receive_speech_events),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ModerationDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl ModerationDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationDto> for super::ModerationDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::ModerationDto> for ModerationDto {
        fn from(value: super::ModerationDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamedEntityRecognitionData {
        results: ::std::result::Result<
            ::std::vec::Vec<super::NamedEntityRecognitionResult>,
            ::std::string::String,
        >,
        utterance: ::std::result::Result<super::UtteranceDto, ::std::string::String>,
        utterance_id: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NamedEntityRecognitionData {
        fn default() -> Self {
            Self {
                results: Err("no value supplied for results".to_string()),
                utterance: Err("no value supplied for utterance".to_string()),
                utterance_id: Err("no value supplied for utterance_id".to_string()),
            }
        }
    }
    impl NamedEntityRecognitionData {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::NamedEntityRecognitionResult>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn utterance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UtteranceDto>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance: {}", e)
                });
            self
        }
        pub fn utterance_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NamedEntityRecognitionData>
    for super::NamedEntityRecognitionData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamedEntityRecognitionData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                results: value.results?,
                utterance: value.utterance?,
                utterance_id: value.utterance_id?,
            })
        }
    }
    impl ::std::convert::From<super::NamedEntityRecognitionData>
    for NamedEntityRecognitionData {
        fn from(value: super::NamedEntityRecognitionData) -> Self {
            Self {
                results: Ok(value.results),
                utterance: Ok(value.utterance),
                utterance_id: Ok(value.utterance_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamedEntityRecognitionDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::NamedEntityRecognitionResult>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for NamedEntityRecognitionDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl NamedEntityRecognitionDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::NamedEntityRecognitionResult>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NamedEntityRecognitionDto>
    for super::NamedEntityRecognitionDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamedEntityRecognitionDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::NamedEntityRecognitionDto>
    for NamedEntityRecognitionDto {
        fn from(value: super::NamedEntityRecognitionDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamedEntityRecognitionMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<
            super::NamedEntityRecognitionData,
            ::std::string::String,
        >,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::NamedEntityRecognitionMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NamedEntityRecognitionMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl NamedEntityRecognitionMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NamedEntityRecognitionData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NamedEntityRecognitionMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NamedEntityRecognitionMessage>
    for super::NamedEntityRecognitionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamedEntityRecognitionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::NamedEntityRecognitionMessage>
    for NamedEntityRecognitionMessage {
        fn from(value: super::NamedEntityRecognitionMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamedEntityRecognitionResult {
        end: ::std::result::Result<f64, ::std::string::String>,
        entity_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for NamedEntityRecognitionResult {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                start: Err("no value supplied for start".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl NamedEntityRecognitionResult {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entity_type: {}", e)
                });
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for start: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NamedEntityRecognitionResult>
    for super::NamedEntityRecognitionResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamedEntityRecognitionResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                entity_type: value.entity_type?,
                start: value.start?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::NamedEntityRecognitionResult>
    for NamedEntityRecognitionResult {
        fn from(value: super::NamedEntityRecognitionResult) -> Self {
            Self {
                end: Ok(value.end),
                entity_type: Ok(value.entity_type),
                start: Ok(value.start),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamesConsistencyDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for NamesConsistencyDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl NamesConsistencyDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NamesConsistencyDto> for super::NamesConsistencyDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamesConsistencyDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::NamesConsistencyDto> for NamesConsistencyDto {
        fn from(value: super::NamesConsistencyDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotFoundErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for NotFoundErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl NotFoundErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NotFoundErrorResponse>
    for super::NotFoundErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotFoundErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::NotFoundErrorResponse> for NotFoundErrorResponse {
        fn from(value: super::NotFoundErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PayloadTooLargeErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PayloadTooLargeErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl PayloadTooLargeErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PayloadTooLargeErrorResponse>
    for super::PayloadTooLargeErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PayloadTooLargeErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::PayloadTooLargeErrorResponse>
    for PayloadTooLargeErrorResponse {
        fn from(value: super::PayloadTooLargeErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PiiRedactionConfigDto {
        entity_types: ::std::result::Result<
            ::std::option::Option<super::PiiRedactionEntityTypeEnum>,
            ::std::string::String,
        >,
        processed_text_type: ::std::result::Result<
            ::std::option::Option<super::PiiRedactionConfigDtoProcessedTextType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PiiRedactionConfigDto {
        fn default() -> Self {
            Self {
                entity_types: Ok(Default::default()),
                processed_text_type: Ok(Default::default()),
            }
        }
    }
    impl PiiRedactionConfigDto {
        pub fn entity_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PiiRedactionEntityTypeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.entity_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for entity_types: {}", e)
                });
            self
        }
        pub fn processed_text_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PiiRedactionConfigDtoProcessedTextType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.processed_text_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for processed_text_type: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PiiRedactionConfigDto>
    for super::PiiRedactionConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PiiRedactionConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                entity_types: value.entity_types?,
                processed_text_type: value.processed_text_type?,
            })
        }
    }
    impl ::std::convert::From<super::PiiRedactionConfigDto> for PiiRedactionConfigDto {
        fn from(value: super::PiiRedactionConfigDto) -> Self {
            Self {
                entity_types: Ok(value.entity_types),
                processed_text_type: Ok(value.processed_text_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostFinalTranscriptMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<
            super::StreamingTranscriptionResultDto,
            ::std::string::String,
        >,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::PostFinalTranscriptMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostFinalTranscriptMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PostFinalTranscriptMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamingTranscriptionResultDto>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostFinalTranscriptMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostFinalTranscriptMessage>
    for super::PostFinalTranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostFinalTranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PostFinalTranscriptMessage>
    for PostFinalTranscriptMessage {
        fn from(value: super::PostFinalTranscriptMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostProcessingConfig {
        chapterization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        summarization_config: ::std::result::Result<
            ::std::option::Option<super::SummarizationConfigDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostProcessingConfig {
        fn default() -> Self {
            Self {
                chapterization: Ok(Default::default()),
                summarization: Ok(Default::default()),
                summarization_config: Ok(Default::default()),
            }
        }
    }
    impl PostProcessingConfig {
        pub fn chapterization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.chapterization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for chapterization: {}", e)
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn summarization_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SummarizationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.summarization_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for summarization_config: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostProcessingConfig> for super::PostProcessingConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostProcessingConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chapterization: value.chapterization?,
                summarization: value.summarization?,
                summarization_config: value.summarization_config?,
            })
        }
    }
    impl ::std::convert::From<super::PostProcessingConfig> for PostProcessingConfig {
        fn from(value: super::PostProcessingConfig) -> Self {
            Self {
                chapterization: Ok(value.chapterization),
                summarization: Ok(value.summarization),
                summarization_config: Ok(value.summarization_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostSummarizationMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<
            super::PostSummarizationMessageData,
            ::std::string::String,
        >,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::PostSummarizationMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostSummarizationMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PostSummarizationMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostSummarizationMessageData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostSummarizationMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostSummarizationMessage>
    for super::PostSummarizationMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostSummarizationMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PostSummarizationMessage>
    for PostSummarizationMessage {
        fn from(value: super::PostSummarizationMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostSummarizationMessageData {
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PostSummarizationMessageData {
        fn default() -> Self {
            Self {
                results: Err("no value supplied for results".to_string()),
            }
        }
    }
    impl PostSummarizationMessageData {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostSummarizationMessageData>
    for super::PostSummarizationMessageData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostSummarizationMessageData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { results: value.results? })
        }
    }
    impl ::std::convert::From<super::PostSummarizationMessageData>
    for PostSummarizationMessageData {
        fn from(value: super::PostSummarizationMessageData) -> Self {
            Self { results: Ok(value.results) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostTranscriptMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::TranscriptionDto, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::PostTranscriptMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostTranscriptMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PostTranscriptMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionDto>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostTranscriptMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostTranscriptMessage>
    for super::PostTranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostTranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PostTranscriptMessage> for PostTranscriptMessage {
        fn from(value: super::PostTranscriptMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PreProcessingConfig {
        audio_enhancer: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        speech_threshold: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PreProcessingConfig {
        fn default() -> Self {
            Self {
                audio_enhancer: Ok(Default::default()),
                speech_threshold: Ok(Default::default()),
            }
        }
    }
    impl PreProcessingConfig {
        pub fn audio_enhancer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_enhancer = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_enhancer: {}", e)
                });
            self
        }
        pub fn speech_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speech_threshold = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for speech_threshold: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PreProcessingConfig> for super::PreProcessingConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PreProcessingConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_enhancer: value.audio_enhancer?,
                speech_threshold: value.speech_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::PreProcessingConfig> for PreProcessingConfig {
        fn from(value: super::PreProcessingConfig) -> Self {
            Self {
                audio_enhancer: Ok(value.audio_enhancer),
                speech_threshold: Ok(value.speech_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PreRecordedEventPayload {
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    }
    impl ::std::default::Default for PreRecordedEventPayload {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl PreRecordedEventPayload {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PreRecordedEventPayload>
    for super::PreRecordedEventPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PreRecordedEventPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::PreRecordedEventPayload>
    for PreRecordedEventPayload {
        fn from(value: super::PreRecordedEventPayload) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PreRecordedRequestParamsResponse {
        audio_to_llm: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        audio_to_llm_config: ::std::result::Result<
            ::std::option::Option<super::AudioToLlmListConfigDto>,
            ::std::string::String,
        >,
        audio_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        callback: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        callback_config: ::std::result::Result<
            ::std::option::Option<super::CallbackConfigDto>,
            ::std::string::String,
        >,
        callback_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        custom_spelling: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_spelling_config: ::std::result::Result<
            ::std::option::Option<super::CustomSpellingConfigDto>,
            ::std::string::String,
        >,
        custom_vocabulary: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_vocabulary_config: ::std::result::Result<
            ::std::option::Option<super::CustomVocabularyConfigDto>,
            ::std::string::String,
        >,
        diarization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        diarization_config: ::std::result::Result<
            ::std::option::Option<super::DiarizationConfigDto>,
            ::std::string::String,
        >,
        language_config: ::std::result::Result<
            ::std::option::Option<super::LanguageConfig>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::TranscriptionSupportedModels>,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        pii_redaction: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        pii_redaction_config: ::std::result::Result<
            ::std::option::Option<super::PiiRedactionConfigDto>,
            ::std::string::String,
        >,
        punctuation_enhanced: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        sentences: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        subtitles: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        subtitles_config: ::std::result::Result<
            ::std::option::Option<super::SubtitlesConfigDto>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        summarization_config: ::std::result::Result<
            ::std::option::Option<super::SummarizationConfigDto>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        translation_config: ::std::result::Result<
            ::std::option::Option<super::TranslationConfigDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PreRecordedRequestParamsResponse {
        fn default() -> Self {
            Self {
                audio_to_llm: Ok(Default::default()),
                audio_to_llm_config: Ok(Default::default()),
                audio_url: Err("no value supplied for audio_url".to_string()),
                callback: Ok(Default::default()),
                callback_config: Ok(Default::default()),
                callback_url: Ok(Default::default()),
                custom_spelling: Ok(Default::default()),
                custom_spelling_config: Ok(Default::default()),
                custom_vocabulary: Ok(Default::default()),
                custom_vocabulary_config: Ok(Default::default()),
                diarization: Ok(Default::default()),
                diarization_config: Ok(Default::default()),
                language_config: Ok(Default::default()),
                model: Ok(Default::default()),
                named_entity_recognition: Ok(Default::default()),
                pii_redaction: Ok(Default::default()),
                pii_redaction_config: Ok(Default::default()),
                punctuation_enhanced: Ok(Default::default()),
                sentences: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                subtitles: Ok(Default::default()),
                subtitles_config: Ok(Default::default()),
                summarization: Ok(Default::default()),
                summarization_config: Ok(Default::default()),
                translation: Ok(Default::default()),
                translation_config: Ok(Default::default()),
            }
        }
    }
    impl PreRecordedRequestParamsResponse {
        pub fn audio_to_llm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_to_llm = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_to_llm: {}", e)
                });
            self
        }
        pub fn audio_to_llm_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AudioToLlmListConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio_to_llm_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for audio_to_llm_config: {}", e
                    )
                });
            self
        }
        pub fn audio_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_url: {}", e)
                });
            self
        }
        pub fn callback<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback: {}", e)
                });
            self
        }
        pub fn callback_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallbackConfigDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_config: {}", e)
                });
            self
        }
        pub fn callback_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_url = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_url: {}", e)
                });
            self
        }
        pub fn custom_spelling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_spelling: {}", e)
                });
            self
        }
        pub fn custom_spelling_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomSpellingConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_spelling_config: {}",
                        e
                    )
                });
            self
        }
        pub fn custom_vocabulary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary: {}", e
                    )
                });
            self
        }
        pub fn custom_vocabulary_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomVocabularyConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary_config: {}",
                        e
                    )
                });
            self
        }
        pub fn diarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for diarization: {}", e)
                });
            self
        }
        pub fn diarization_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiarizationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.diarization_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for diarization_config: {}", e
                    )
                });
            self
        }
        pub fn language_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language_config: {}", e)
                });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranscriptionSupportedModels>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn pii_redaction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pii_redaction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pii_redaction: {}", e)
                });
            self
        }
        pub fn pii_redaction_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PiiRedactionConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pii_redaction_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for pii_redaction_config: {}", e
                    )
                });
            self
        }
        pub fn punctuation_enhanced<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.punctuation_enhanced = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for punctuation_enhanced: {}", e
                    )
                });
            self
        }
        pub fn sentences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentences: {}", e)
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn subtitles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtitles: {}", e)
                });
            self
        }
        pub fn subtitles_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SubtitlesConfigDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for subtitles_config: {}", e
                    )
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn summarization_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SummarizationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.summarization_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for summarization_config: {}", e
                    )
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
        pub fn translation_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranslationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.translation_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for translation_config: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PreRecordedRequestParamsResponse>
    for super::PreRecordedRequestParamsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PreRecordedRequestParamsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_to_llm: value.audio_to_llm?,
                audio_to_llm_config: value.audio_to_llm_config?,
                audio_url: value.audio_url?,
                callback: value.callback?,
                callback_config: value.callback_config?,
                callback_url: value.callback_url?,
                custom_spelling: value.custom_spelling?,
                custom_spelling_config: value.custom_spelling_config?,
                custom_vocabulary: value.custom_vocabulary?,
                custom_vocabulary_config: value.custom_vocabulary_config?,
                diarization: value.diarization?,
                diarization_config: value.diarization_config?,
                language_config: value.language_config?,
                model: value.model?,
                named_entity_recognition: value.named_entity_recognition?,
                pii_redaction: value.pii_redaction?,
                pii_redaction_config: value.pii_redaction_config?,
                punctuation_enhanced: value.punctuation_enhanced?,
                sentences: value.sentences?,
                sentiment_analysis: value.sentiment_analysis?,
                subtitles: value.subtitles?,
                subtitles_config: value.subtitles_config?,
                summarization: value.summarization?,
                summarization_config: value.summarization_config?,
                translation: value.translation?,
                translation_config: value.translation_config?,
            })
        }
    }
    impl ::std::convert::From<super::PreRecordedRequestParamsResponse>
    for PreRecordedRequestParamsResponse {
        fn from(value: super::PreRecordedRequestParamsResponse) -> Self {
            Self {
                audio_to_llm: Ok(value.audio_to_llm),
                audio_to_llm_config: Ok(value.audio_to_llm_config),
                audio_url: Ok(value.audio_url),
                callback: Ok(value.callback),
                callback_config: Ok(value.callback_config),
                callback_url: Ok(value.callback_url),
                custom_spelling: Ok(value.custom_spelling),
                custom_spelling_config: Ok(value.custom_spelling_config),
                custom_vocabulary: Ok(value.custom_vocabulary),
                custom_vocabulary_config: Ok(value.custom_vocabulary_config),
                diarization: Ok(value.diarization),
                diarization_config: Ok(value.diarization_config),
                language_config: Ok(value.language_config),
                model: Ok(value.model),
                named_entity_recognition: Ok(value.named_entity_recognition),
                pii_redaction: Ok(value.pii_redaction),
                pii_redaction_config: Ok(value.pii_redaction_config),
                punctuation_enhanced: Ok(value.punctuation_enhanced),
                sentences: Ok(value.sentences),
                sentiment_analysis: Ok(value.sentiment_analysis),
                subtitles: Ok(value.subtitles),
                subtitles_config: Ok(value.subtitles_config),
                summarization: Ok(value.summarization),
                summarization_config: Ok(value.summarization_config),
                translation: Ok(value.translation),
                translation_config: Ok(value.translation_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PreRecordedResponse {
        completed_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<
            ::chrono::DateTime<::chrono::offset::Utc>,
            ::std::string::String,
        >,
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        error_code: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        file: ::std::result::Result<
            ::std::option::Option<super::FileResponse>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        kind: ::std::result::Result<
            super::PreRecordedResponseKind,
            ::std::string::String,
        >,
        post_session_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_params: ::std::result::Result<
            ::std::option::Option<super::PreRecordedRequestParamsResponse>,
            ::std::string::String,
        >,
        result: ::std::result::Result<
            ::std::option::Option<super::TranscriptionResultDto>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            super::PreRecordedResponseStatus,
            ::std::string::String,
        >,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for PreRecordedResponse {
        fn default() -> Self {
            Self {
                completed_at: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                custom_metadata: Ok(Default::default()),
                error_code: Ok(Default::default()),
                file: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                post_session_metadata: Err(
                    "no value supplied for post_session_metadata".to_string(),
                ),
                request_id: Err("no value supplied for request_id".to_string()),
                request_params: Ok(Default::default()),
                result: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl PreRecordedResponse {
        pub fn completed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.completed_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for completed_at: {}", e)
                });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_code: {}", e)
                });
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PreRecordedResponseKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn post_session_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.post_session_metadata = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for post_session_metadata: {}",
                        e
                    )
                });
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn request_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PreRecordedRequestParamsResponse>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.request_params = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_params: {}", e)
                });
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranscriptionResultDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for result: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PreRecordedResponseStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PreRecordedResponse> for super::PreRecordedResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PreRecordedResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completed_at: value.completed_at?,
                created_at: value.created_at?,
                custom_metadata: value.custom_metadata?,
                error_code: value.error_code?,
                file: value.file?,
                id: value.id?,
                kind: value.kind?,
                post_session_metadata: value.post_session_metadata?,
                request_id: value.request_id?,
                request_params: value.request_params?,
                result: value.result?,
                status: value.status?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::PreRecordedResponse> for PreRecordedResponse {
        fn from(value: super::PreRecordedResponse) -> Self {
            Self {
                completed_at: Ok(value.completed_at),
                created_at: Ok(value.created_at),
                custom_metadata: Ok(value.custom_metadata),
                error_code: Ok(value.error_code),
                file: Ok(value.file),
                id: Ok(value.id),
                kind: Ok(value.kind),
                post_session_metadata: Ok(value.post_session_metadata),
                request_id: Ok(value.request_id),
                request_params: Ok(value.request_params),
                result: Ok(value.result),
                status: Ok(value.status),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RealtimeProcessingConfig {
        custom_spelling: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_spelling_config: ::std::result::Result<
            ::std::option::Option<super::CustomSpellingConfigDto>,
            ::std::string::String,
        >,
        custom_vocabulary: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        custom_vocabulary_config: ::std::result::Result<
            ::std::option::Option<super::CustomVocabularyConfigDto>,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        translation_config: ::std::result::Result<
            ::std::option::Option<super::TranslationConfigDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RealtimeProcessingConfig {
        fn default() -> Self {
            Self {
                custom_spelling: Ok(Default::default()),
                custom_spelling_config: Ok(Default::default()),
                custom_vocabulary: Ok(Default::default()),
                custom_vocabulary_config: Ok(Default::default()),
                named_entity_recognition: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                translation: Ok(Default::default()),
                translation_config: Ok(Default::default()),
            }
        }
    }
    impl RealtimeProcessingConfig {
        pub fn custom_spelling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_spelling: {}", e)
                });
            self
        }
        pub fn custom_spelling_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomSpellingConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_spelling_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_spelling_config: {}",
                        e
                    )
                });
            self
        }
        pub fn custom_vocabulary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary: {}", e
                    )
                });
            self
        }
        pub fn custom_vocabulary_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomVocabularyConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_vocabulary_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_vocabulary_config: {}",
                        e
                    )
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
        pub fn translation_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranslationConfigDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.translation_config = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for translation_config: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RealtimeProcessingConfig>
    for super::RealtimeProcessingConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RealtimeProcessingConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom_spelling: value.custom_spelling?,
                custom_spelling_config: value.custom_spelling_config?,
                custom_vocabulary: value.custom_vocabulary?,
                custom_vocabulary_config: value.custom_vocabulary_config?,
                named_entity_recognition: value.named_entity_recognition?,
                sentiment_analysis: value.sentiment_analysis?,
                translation: value.translation?,
                translation_config: value.translation_config?,
            })
        }
    }
    impl ::std::convert::From<super::RealtimeProcessingConfig>
    for RealtimeProcessingConfig {
        fn from(value: super::RealtimeProcessingConfig) -> Self {
            Self {
                custom_spelling: Ok(value.custom_spelling),
                custom_spelling_config: Ok(value.custom_spelling_config),
                custom_vocabulary: Ok(value.custom_vocabulary),
                custom_vocabulary_config: Ok(value.custom_vocabulary_config),
                named_entity_recognition: Ok(value.named_entity_recognition),
                sentiment_analysis: Ok(value.sentiment_analysis),
                translation: Ok(value.translation),
                translation_config: Ok(value.translation_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SentencesDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for SentencesDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl SentencesDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SentencesDto> for super::SentencesDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SentencesDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::SentencesDto> for SentencesDto {
        fn from(value: super::SentencesDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SentimentAnalysisData {
        results: ::std::result::Result<
            ::std::vec::Vec<super::SentimentAnalysisResult>,
            ::std::string::String,
        >,
        utterance: ::std::result::Result<super::UtteranceDto, ::std::string::String>,
        utterance_id: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SentimentAnalysisData {
        fn default() -> Self {
            Self {
                results: Err("no value supplied for results".to_string()),
                utterance: Err("no value supplied for utterance".to_string()),
                utterance_id: Err("no value supplied for utterance_id".to_string()),
            }
        }
    }
    impl SentimentAnalysisData {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SentimentAnalysisResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn utterance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UtteranceDto>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance: {}", e)
                });
            self
        }
        pub fn utterance_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SentimentAnalysisData>
    for super::SentimentAnalysisData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SentimentAnalysisData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                results: value.results?,
                utterance: value.utterance?,
                utterance_id: value.utterance_id?,
            })
        }
    }
    impl ::std::convert::From<super::SentimentAnalysisData> for SentimentAnalysisData {
        fn from(value: super::SentimentAnalysisData) -> Self {
            Self {
                results: Ok(value.results),
                utterance: Ok(value.utterance),
                utterance_id: Ok(value.utterance_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SentimentAnalysisDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for SentimentAnalysisDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl SentimentAnalysisDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SentimentAnalysisDto> for super::SentimentAnalysisDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SentimentAnalysisDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::SentimentAnalysisDto> for SentimentAnalysisDto {
        fn from(value: super::SentimentAnalysisDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SentimentAnalysisMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::SentimentAnalysisData, ::std::string::String>,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::SentimentAnalysisMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SentimentAnalysisMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SentimentAnalysisMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SentimentAnalysisData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SentimentAnalysisMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SentimentAnalysisMessage>
    for super::SentimentAnalysisMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SentimentAnalysisMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SentimentAnalysisMessage>
    for SentimentAnalysisMessage {
        fn from(value: super::SentimentAnalysisMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SentimentAnalysisResult {
        channel: ::std::result::Result<f64, ::std::string::String>,
        emotion: ::std::result::Result<::std::string::String, ::std::string::String>,
        end: ::std::result::Result<f64, ::std::string::String>,
        sentiment: ::std::result::Result<::std::string::String, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SentimentAnalysisResult {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                emotion: Err("no value supplied for emotion".to_string()),
                end: Err("no value supplied for end".to_string()),
                sentiment: Err("no value supplied for sentiment".to_string()),
                start: Err("no value supplied for start".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl SentimentAnalysisResult {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for channel: {}", e)
                });
            self
        }
        pub fn emotion<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.emotion = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for emotion: {}", e)
                });
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn sentiment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentiment: {}", e)
                });
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for start: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SentimentAnalysisResult>
    for super::SentimentAnalysisResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SentimentAnalysisResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                emotion: value.emotion?,
                end: value.end?,
                sentiment: value.sentiment?,
                start: value.start?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::SentimentAnalysisResult>
    for SentimentAnalysisResult {
        fn from(value: super::SentimentAnalysisResult) -> Self {
            Self {
                channel: Ok(value.channel),
                emotion: Ok(value.emotion),
                end: Ok(value.end),
                sentiment: Ok(value.sentiment),
                start: Ok(value.start),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechEndMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::SpeechMessageData, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::SpeechEndMessageType, ::std::string::String>,
    }
    impl ::std::default::Default for SpeechEndMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SpeechEndMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechMessageData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechEndMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechEndMessage> for super::SpeechEndMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechEndMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechEndMessage> for SpeechEndMessage {
        fn from(value: super::SpeechEndMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechMessageData {
        channel: ::std::result::Result<f64, ::std::string::String>,
        time: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for SpeechMessageData {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                time: Err("no value supplied for time".to_string()),
            }
        }
    }
    impl SpeechMessageData {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for channel: {}", e)
                });
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechMessageData> for super::SpeechMessageData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechMessageData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                time: value.time?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechMessageData> for SpeechMessageData {
        fn from(value: super::SpeechMessageData) -> Self {
            Self {
                channel: Ok(value.channel),
                time: Ok(value.time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeechStartMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::SpeechMessageData, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::SpeechStartMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SpeechStartMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SpeechStartMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechMessageData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpeechStartMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SpeechStartMessage> for super::SpeechStartMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeechStartMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SpeechStartMessage> for SpeechStartMessage {
        fn from(value: super::SpeechStartMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StartRecordingMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::StartRecordingMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StartRecordingMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StartRecordingMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StartRecordingMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StartRecordingMessage>
    for super::StartRecordingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StartRecordingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StartRecordingMessage> for StartRecordingMessage {
        fn from(value: super::StartRecordingMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StartSessionMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::StartSessionMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StartSessionMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StartSessionMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StartSessionMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StartSessionMessage> for super::StartSessionMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StartSessionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StartSessionMessage> for StartSessionMessage {
        fn from(value: super::StartSessionMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StopRecordingAckData {
        recording_duration: ::std::result::Result<f64, ::std::string::String>,
        recording_left_to_process: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for StopRecordingAckData {
        fn default() -> Self {
            Self {
                recording_duration: Err(
                    "no value supplied for recording_duration".to_string(),
                ),
                recording_left_to_process: Err(
                    "no value supplied for recording_left_to_process".to_string(),
                ),
            }
        }
    }
    impl StopRecordingAckData {
        pub fn recording_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.recording_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for recording_duration: {}", e
                    )
                });
            self
        }
        pub fn recording_left_to_process<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.recording_left_to_process = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for recording_left_to_process: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StopRecordingAckData> for super::StopRecordingAckData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StopRecordingAckData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recording_duration: value.recording_duration?,
                recording_left_to_process: value.recording_left_to_process?,
            })
        }
    }
    impl ::std::convert::From<super::StopRecordingAckData> for StopRecordingAckData {
        fn from(value: super::StopRecordingAckData) -> Self {
            Self {
                recording_duration: Ok(value.recording_duration),
                recording_left_to_process: Ok(value.recording_left_to_process),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StopRecordingAckMessage {
        acknowledged: ::std::result::Result<bool, ::std::string::String>,
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::StopRecordingAckData, ::std::string::String>,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::StopRecordingAckMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StopRecordingAckMessage {
        fn default() -> Self {
            Self {
                acknowledged: Err("no value supplied for acknowledged".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StopRecordingAckMessage {
        pub fn acknowledged<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledged = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for acknowledged: {}", e)
                });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StopRecordingAckData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StopRecordingAckMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StopRecordingAckMessage>
    for super::StopRecordingAckMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StopRecordingAckMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledged: value.acknowledged?,
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StopRecordingAckMessage>
    for StopRecordingAckMessage {
        fn from(value: super::StopRecordingAckMessage) -> Self {
            Self {
                acknowledged: Ok(value.acknowledged),
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StopRecordingAction {
        type_: ::std::result::Result<
            super::StopRecordingActionType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StopRecordingAction {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StopRecordingAction {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StopRecordingActionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StopRecordingAction> for super::StopRecordingAction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StopRecordingAction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { type_: value.type_? })
        }
    }
    impl ::std::convert::From<super::StopRecordingAction> for StopRecordingAction {
        fn from(value: super::StopRecordingAction) -> Self {
            Self { type_: Ok(value.type_) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamingRequest {
        bit_depth: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedBitDepthEnum>,
            ::std::string::String,
        >,
        callback: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        callback_config: ::std::result::Result<
            ::std::option::Option<super::CallbackConfig>,
            ::std::string::String,
        >,
        channels: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        encoding: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedEncodingEnum>,
            ::std::string::String,
        >,
        endpointing: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        language_config: ::std::result::Result<
            ::std::option::Option<super::LanguageConfig>,
            ::std::string::String,
        >,
        maximum_duration_without_endpointing: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        messages_config: ::std::result::Result<
            ::std::option::Option<super::MessagesConfig>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedModels>,
            ::std::string::String,
        >,
        post_processing: ::std::result::Result<
            ::std::option::Option<super::PostProcessingConfig>,
            ::std::string::String,
        >,
        pre_processing: ::std::result::Result<
            ::std::option::Option<super::PreProcessingConfig>,
            ::std::string::String,
        >,
        realtime_processing: ::std::result::Result<
            ::std::option::Option<super::RealtimeProcessingConfig>,
            ::std::string::String,
        >,
        sample_rate: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedSampleRateEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StreamingRequest {
        fn default() -> Self {
            Self {
                bit_depth: Ok(Default::default()),
                callback: Ok(Default::default()),
                callback_config: Ok(Default::default()),
                channels: Ok(Default::default()),
                custom_metadata: Ok(Default::default()),
                encoding: Ok(Default::default()),
                endpointing: Ok(Default::default()),
                language_config: Ok(Default::default()),
                maximum_duration_without_endpointing: Ok(Default::default()),
                messages_config: Ok(Default::default()),
                model: Ok(Default::default()),
                post_processing: Ok(Default::default()),
                pre_processing: Ok(Default::default()),
                realtime_processing: Ok(Default::default()),
                sample_rate: Ok(Default::default()),
            }
        }
    }
    impl StreamingRequest {
        pub fn bit_depth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedBitDepthEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bit_depth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bit_depth: {}", e)
                });
            self
        }
        pub fn callback<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback: {}", e)
                });
            self
        }
        pub fn callback_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallbackConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_config: {}", e)
                });
            self
        }
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for channels: {}", e)
                });
            self
        }
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn encoding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedEncodingEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.encoding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for encoding: {}", e)
                });
            self
        }
        pub fn endpointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.endpointing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for endpointing: {}", e)
                });
            self
        }
        pub fn language_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language_config: {}", e)
                });
            self
        }
        pub fn maximum_duration_without_endpointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_duration_without_endpointing = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_duration_without_endpointing: {}",
                        e
                    )
                });
            self
        }
        pub fn messages_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MessagesConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for messages_config: {}", e)
                });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedModels>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn post_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PostProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.post_processing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_processing: {}", e)
                });
            self
        }
        pub fn pre_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PreProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pre_processing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pre_processing: {}", e)
                });
            self
        }
        pub fn realtime_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RealtimeProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.realtime_processing = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for realtime_processing: {}", e
                    )
                });
            self
        }
        pub fn sample_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedSampleRateEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sample_rate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sample_rate: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StreamingRequest> for super::StreamingRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamingRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bit_depth: value.bit_depth?,
                callback: value.callback?,
                callback_config: value.callback_config?,
                channels: value.channels?,
                custom_metadata: value.custom_metadata?,
                encoding: value.encoding?,
                endpointing: value.endpointing?,
                language_config: value.language_config?,
                maximum_duration_without_endpointing: value
                    .maximum_duration_without_endpointing?,
                messages_config: value.messages_config?,
                model: value.model?,
                post_processing: value.post_processing?,
                pre_processing: value.pre_processing?,
                realtime_processing: value.realtime_processing?,
                sample_rate: value.sample_rate?,
            })
        }
    }
    impl ::std::convert::From<super::StreamingRequest> for StreamingRequest {
        fn from(value: super::StreamingRequest) -> Self {
            Self {
                bit_depth: Ok(value.bit_depth),
                callback: Ok(value.callback),
                callback_config: Ok(value.callback_config),
                channels: Ok(value.channels),
                custom_metadata: Ok(value.custom_metadata),
                encoding: Ok(value.encoding),
                endpointing: Ok(value.endpointing),
                language_config: Ok(value.language_config),
                maximum_duration_without_endpointing: Ok(
                    value.maximum_duration_without_endpointing,
                ),
                messages_config: Ok(value.messages_config),
                model: Ok(value.model),
                post_processing: Ok(value.post_processing),
                pre_processing: Ok(value.pre_processing),
                realtime_processing: Ok(value.realtime_processing),
                sample_rate: Ok(value.sample_rate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamingRequestParamsResponse {
        bit_depth: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedBitDepthEnum>,
            ::std::string::String,
        >,
        callback: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        callback_config: ::std::result::Result<
            ::std::option::Option<super::CallbackConfig>,
            ::std::string::String,
        >,
        channels: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        encoding: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedEncodingEnum>,
            ::std::string::String,
        >,
        endpointing: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        language_config: ::std::result::Result<
            ::std::option::Option<super::LanguageConfig>,
            ::std::string::String,
        >,
        maximum_duration_without_endpointing: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        messages_config: ::std::result::Result<
            ::std::option::Option<super::MessagesConfig>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedModels>,
            ::std::string::String,
        >,
        post_processing: ::std::result::Result<
            ::std::option::Option<super::PostProcessingConfig>,
            ::std::string::String,
        >,
        pre_processing: ::std::result::Result<
            ::std::option::Option<super::PreProcessingConfig>,
            ::std::string::String,
        >,
        realtime_processing: ::std::result::Result<
            ::std::option::Option<super::RealtimeProcessingConfig>,
            ::std::string::String,
        >,
        sample_rate: ::std::result::Result<
            ::std::option::Option<super::StreamingSupportedSampleRateEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StreamingRequestParamsResponse {
        fn default() -> Self {
            Self {
                bit_depth: Ok(Default::default()),
                callback: Ok(Default::default()),
                callback_config: Ok(Default::default()),
                channels: Ok(Default::default()),
                encoding: Ok(Default::default()),
                endpointing: Ok(Default::default()),
                language_config: Ok(Default::default()),
                maximum_duration_without_endpointing: Ok(Default::default()),
                messages_config: Ok(Default::default()),
                model: Ok(Default::default()),
                post_processing: Ok(Default::default()),
                pre_processing: Ok(Default::default()),
                realtime_processing: Ok(Default::default()),
                sample_rate: Ok(Default::default()),
            }
        }
    }
    impl StreamingRequestParamsResponse {
        pub fn bit_depth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedBitDepthEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bit_depth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bit_depth: {}", e)
                });
            self
        }
        pub fn callback<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback: {}", e)
                });
            self
        }
        pub fn callback_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallbackConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.callback_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for callback_config: {}", e)
                });
            self
        }
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for channels: {}", e)
                });
            self
        }
        pub fn encoding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedEncodingEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.encoding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for encoding: {}", e)
                });
            self
        }
        pub fn endpointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.endpointing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for endpointing: {}", e)
                });
            self
        }
        pub fn language_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language_config: {}", e)
                });
            self
        }
        pub fn maximum_duration_without_endpointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_duration_without_endpointing = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_duration_without_endpointing: {}",
                        e
                    )
                });
            self
        }
        pub fn messages_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MessagesConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages_config = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for messages_config: {}", e)
                });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedModels>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn post_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PostProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.post_processing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_processing: {}", e)
                });
            self
        }
        pub fn pre_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PreProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.pre_processing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pre_processing: {}", e)
                });
            self
        }
        pub fn realtime_processing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RealtimeProcessingConfig>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.realtime_processing = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for realtime_processing: {}", e
                    )
                });
            self
        }
        pub fn sample_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingSupportedSampleRateEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sample_rate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sample_rate: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StreamingRequestParamsResponse>
    for super::StreamingRequestParamsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamingRequestParamsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bit_depth: value.bit_depth?,
                callback: value.callback?,
                callback_config: value.callback_config?,
                channels: value.channels?,
                encoding: value.encoding?,
                endpointing: value.endpointing?,
                language_config: value.language_config?,
                maximum_duration_without_endpointing: value
                    .maximum_duration_without_endpointing?,
                messages_config: value.messages_config?,
                model: value.model?,
                post_processing: value.post_processing?,
                pre_processing: value.pre_processing?,
                realtime_processing: value.realtime_processing?,
                sample_rate: value.sample_rate?,
            })
        }
    }
    impl ::std::convert::From<super::StreamingRequestParamsResponse>
    for StreamingRequestParamsResponse {
        fn from(value: super::StreamingRequestParamsResponse) -> Self {
            Self {
                bit_depth: Ok(value.bit_depth),
                callback: Ok(value.callback),
                callback_config: Ok(value.callback_config),
                channels: Ok(value.channels),
                encoding: Ok(value.encoding),
                endpointing: Ok(value.endpointing),
                language_config: Ok(value.language_config),
                maximum_duration_without_endpointing: Ok(
                    value.maximum_duration_without_endpointing,
                ),
                messages_config: Ok(value.messages_config),
                model: Ok(value.model),
                post_processing: Ok(value.post_processing),
                pre_processing: Ok(value.pre_processing),
                realtime_processing: Ok(value.realtime_processing),
                sample_rate: Ok(value.sample_rate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamingResponse {
        completed_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<
            ::chrono::DateTime<::chrono::offset::Utc>,
            ::std::string::String,
        >,
        custom_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        error_code: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        file: ::std::result::Result<
            ::std::option::Option<super::FileResponse>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        kind: ::std::result::Result<super::StreamingResponseKind, ::std::string::String>,
        post_session_metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_params: ::std::result::Result<
            ::std::option::Option<super::StreamingRequestParamsResponse>,
            ::std::string::String,
        >,
        result: ::std::result::Result<
            ::std::option::Option<super::StreamingTranscriptionResultWithMessagesDto>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            super::StreamingResponseStatus,
            ::std::string::String,
        >,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for StreamingResponse {
        fn default() -> Self {
            Self {
                completed_at: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                custom_metadata: Ok(Default::default()),
                error_code: Ok(Default::default()),
                file: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                post_session_metadata: Err(
                    "no value supplied for post_session_metadata".to_string(),
                ),
                request_id: Err("no value supplied for request_id".to_string()),
                request_params: Ok(Default::default()),
                result: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl StreamingResponse {
        pub fn completed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.completed_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for completed_at: {}", e)
                });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn custom_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_metadata: {}", e)
                });
            self
        }
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error_code: {}", e)
                });
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamingResponseKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn post_session_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.post_session_metadata = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for post_session_metadata: {}",
                        e
                    )
                });
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn request_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingRequestParamsResponse>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.request_params = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_params: {}", e)
                });
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StreamingTranscriptionResultWithMessagesDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for result: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StreamingResponseStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StreamingResponse> for super::StreamingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completed_at: value.completed_at?,
                created_at: value.created_at?,
                custom_metadata: value.custom_metadata?,
                error_code: value.error_code?,
                file: value.file?,
                id: value.id?,
                kind: value.kind?,
                post_session_metadata: value.post_session_metadata?,
                request_id: value.request_id?,
                request_params: value.request_params?,
                result: value.result?,
                status: value.status?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::StreamingResponse> for StreamingResponse {
        fn from(value: super::StreamingResponse) -> Self {
            Self {
                completed_at: Ok(value.completed_at),
                created_at: Ok(value.created_at),
                custom_metadata: Ok(value.custom_metadata),
                error_code: Ok(value.error_code),
                file: Ok(value.file),
                id: Ok(value.id),
                kind: Ok(value.kind),
                post_session_metadata: Ok(value.post_session_metadata),
                request_id: Ok(value.request_id),
                request_params: Ok(value.request_params),
                result: Ok(value.result),
                status: Ok(value.status),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamingTranscriptionResultDto {
        metadata: ::std::result::Result<
            super::TranscriptionMetadataDto,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<super::NamedEntityRecognitionDto>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<super::SentimentAnalysisDto>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<super::SummarizationDto>,
            ::std::string::String,
        >,
        transcription: ::std::result::Result<
            ::std::option::Option<super::TranscriptionDto>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<super::TranslationDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StreamingTranscriptionResultDto {
        fn default() -> Self {
            Self {
                metadata: Err("no value supplied for metadata".to_string()),
                named_entity_recognition: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                summarization: Ok(Default::default()),
                transcription: Ok(Default::default()),
                translation: Ok(Default::default()),
            }
        }
    }
    impl StreamingTranscriptionResultDto {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionMetadataDto>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for metadata: {}", e)
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NamedEntityRecognitionDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SentimentAnalysisDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummarizationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn transcription<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranscriptionDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.transcription = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for transcription: {}", e)
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranslationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StreamingTranscriptionResultDto>
    for super::StreamingTranscriptionResultDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamingTranscriptionResultDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                named_entity_recognition: value.named_entity_recognition?,
                sentiment_analysis: value.sentiment_analysis?,
                summarization: value.summarization?,
                transcription: value.transcription?,
                translation: value.translation?,
            })
        }
    }
    impl ::std::convert::From<super::StreamingTranscriptionResultDto>
    for StreamingTranscriptionResultDto {
        fn from(value: super::StreamingTranscriptionResultDto) -> Self {
            Self {
                metadata: Ok(value.metadata),
                named_entity_recognition: Ok(value.named_entity_recognition),
                sentiment_analysis: Ok(value.sentiment_analysis),
                summarization: Ok(value.summarization),
                transcription: Ok(value.transcription),
                translation: Ok(value.translation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamingTranscriptionResultWithMessagesDto {
        messages: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            super::TranscriptionMetadataDto,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<super::NamedEntityRecognitionDto>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<super::SentimentAnalysisDto>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<super::SummarizationDto>,
            ::std::string::String,
        >,
        transcription: ::std::result::Result<
            ::std::option::Option<super::TranscriptionDto>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<super::TranslationDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StreamingTranscriptionResultWithMessagesDto {
        fn default() -> Self {
            Self {
                messages: Ok(Default::default()),
                metadata: Err("no value supplied for metadata".to_string()),
                named_entity_recognition: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                summarization: Ok(Default::default()),
                transcription: Ok(Default::default()),
                translation: Ok(Default::default()),
            }
        }
    }
    impl StreamingTranscriptionResultWithMessagesDto {
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for messages: {}", e)
                });
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionMetadataDto>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for metadata: {}", e)
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NamedEntityRecognitionDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SentimentAnalysisDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummarizationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn transcription<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranscriptionDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.transcription = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for transcription: {}", e)
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranslationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StreamingTranscriptionResultWithMessagesDto>
    for super::StreamingTranscriptionResultWithMessagesDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamingTranscriptionResultWithMessagesDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                messages: value.messages?,
                metadata: value.metadata?,
                named_entity_recognition: value.named_entity_recognition?,
                sentiment_analysis: value.sentiment_analysis?,
                summarization: value.summarization?,
                transcription: value.transcription?,
                translation: value.translation?,
            })
        }
    }
    impl ::std::convert::From<super::StreamingTranscriptionResultWithMessagesDto>
    for StreamingTranscriptionResultWithMessagesDto {
        fn from(value: super::StreamingTranscriptionResultWithMessagesDto) -> Self {
            Self {
                messages: Ok(value.messages),
                metadata: Ok(value.metadata),
                named_entity_recognition: Ok(value.named_entity_recognition),
                sentiment_analysis: Ok(value.sentiment_analysis),
                summarization: Ok(value.summarization),
                transcription: Ok(value.transcription),
                translation: Ok(value.translation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StructuredDataExtractionDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for StructuredDataExtractionDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl StructuredDataExtractionDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StructuredDataExtractionDto>
    for super::StructuredDataExtractionDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StructuredDataExtractionDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::StructuredDataExtractionDto>
    for StructuredDataExtractionDto {
        fn from(value: super::StructuredDataExtractionDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubtitleDto {
        format: ::std::result::Result<super::SubtitlesFormatEnum, ::std::string::String>,
        subtitles: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SubtitleDto {
        fn default() -> Self {
            Self {
                format: Err("no value supplied for format".to_string()),
                subtitles: Err("no value supplied for subtitles".to_string()),
            }
        }
    }
    impl SubtitleDto {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubtitlesFormatEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for format: {}", e)
                });
            self
        }
        pub fn subtitles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtitles: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SubtitleDto> for super::SubtitleDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubtitleDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                subtitles: value.subtitles?,
            })
        }
    }
    impl ::std::convert::From<super::SubtitleDto> for SubtitleDto {
        fn from(value: super::SubtitleDto) -> Self {
            Self {
                format: Ok(value.format),
                subtitles: Ok(value.subtitles),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubtitlesConfigDto {
        formats: ::std::result::Result<
            ::std::vec::Vec<super::SubtitlesFormatEnum>,
            ::std::string::String,
        >,
        maximum_characters_per_row: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        maximum_duration: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        maximum_rows_per_caption: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        minimum_duration: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        style: ::std::result::Result<
            ::std::option::Option<super::SubtitlesStyleEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubtitlesConfigDto {
        fn default() -> Self {
            Self {
                formats: Ok(Default::default()),
                maximum_characters_per_row: Ok(Default::default()),
                maximum_duration: Ok(Default::default()),
                maximum_rows_per_caption: Ok(Default::default()),
                minimum_duration: Ok(Default::default()),
                style: Ok(Default::default()),
            }
        }
    }
    impl SubtitlesConfigDto {
        pub fn formats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SubtitlesFormatEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.formats = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for formats: {}", e)
                });
            self
        }
        pub fn maximum_characters_per_row<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_characters_per_row = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_characters_per_row: {}",
                        e
                    )
                });
            self
        }
        pub fn maximum_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_duration: {}", e
                    )
                });
            self
        }
        pub fn maximum_rows_per_caption<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_rows_per_caption = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for maximum_rows_per_caption: {}",
                        e
                    )
                });
            self
        }
        pub fn minimum_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum_duration = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for minimum_duration: {}", e
                    )
                });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SubtitlesStyleEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.style = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for style: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SubtitlesConfigDto> for super::SubtitlesConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubtitlesConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                formats: value.formats?,
                maximum_characters_per_row: value.maximum_characters_per_row?,
                maximum_duration: value.maximum_duration?,
                maximum_rows_per_caption: value.maximum_rows_per_caption?,
                minimum_duration: value.minimum_duration?,
                style: value.style?,
            })
        }
    }
    impl ::std::convert::From<super::SubtitlesConfigDto> for SubtitlesConfigDto {
        fn from(value: super::SubtitlesConfigDto) -> Self {
            Self {
                formats: Ok(value.formats),
                maximum_characters_per_row: Ok(value.maximum_characters_per_row),
                maximum_duration: Ok(value.maximum_duration),
                maximum_rows_per_caption: Ok(value.maximum_rows_per_caption),
                minimum_duration: Ok(value.minimum_duration),
                style: Ok(value.style),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummarizationConfigDto {
        type_: ::std::result::Result<
            ::std::option::Option<super::SummaryTypesEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SummarizationConfigDto {
        fn default() -> Self {
            Self {
                type_: Ok(Default::default()),
            }
        }
    }
    impl SummarizationConfigDto {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummaryTypesEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SummarizationConfigDto>
    for super::SummarizationConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummarizationConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { type_: value.type_? })
        }
    }
    impl ::std::convert::From<super::SummarizationConfigDto> for SummarizationConfigDto {
        fn from(value: super::SummarizationConfigDto) -> Self {
            Self { type_: Ok(value.type_) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummarizationDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<::std::string::String, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for SummarizationDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl SummarizationDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SummarizationDto> for super::SummarizationDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummarizationDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::SummarizationDto> for SummarizationDto {
        fn from(value: super::SummarizationDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::TranscriptMessageData, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::TranscriptMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TranscriptMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptMessageData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptMessage> for super::TranscriptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptMessage> for TranscriptMessage {
        fn from(value: super::TranscriptMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptMessageData {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        is_final: ::std::result::Result<bool, ::std::string::String>,
        utterance: ::std::result::Result<super::UtteranceDto, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptMessageData {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                is_final: Err("no value supplied for is_final".to_string()),
                utterance: Err("no value supplied for utterance".to_string()),
            }
        }
    }
    impl TranscriptMessageData {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn is_final<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_final = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_final: {}", e)
                });
            self
        }
        pub fn utterance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UtteranceDto>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptMessageData>
    for super::TranscriptMessageData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptMessageData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                is_final: value.is_final?,
                utterance: value.utterance?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptMessageData> for TranscriptMessageData {
        fn from(value: super::TranscriptMessageData) -> Self {
            Self {
                id: Ok(value.id),
                is_final: Ok(value.is_final),
                utterance: Ok(value.utterance),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionDto {
        full_transcript: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
        languages: ::std::result::Result<
            ::std::vec::Vec<super::TranscriptionLanguageCodeEnum>,
            ::std::string::String,
        >,
        sentences: ::std::result::Result<
            ::std::vec::Vec<super::SentencesDto>,
            ::std::string::String,
        >,
        subtitles: ::std::result::Result<
            ::std::vec::Vec<super::SubtitleDto>,
            ::std::string::String,
        >,
        utterances: ::std::result::Result<
            ::std::vec::Vec<super::UtteranceDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptionDto {
        fn default() -> Self {
            Self {
                full_transcript: Err(
                    "no value supplied for full_transcript".to_string(),
                ),
                languages: Err("no value supplied for languages".to_string()),
                sentences: Ok(Default::default()),
                subtitles: Ok(Default::default()),
                utterances: Err("no value supplied for utterances".to_string()),
            }
        }
    }
    impl TranscriptionDto {
        pub fn full_transcript<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.full_transcript = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for full_transcript: {}", e)
                });
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TranscriptionLanguageCodeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for languages: {}", e)
                });
            self
        }
        pub fn sentences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SentencesDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentences: {}", e)
                });
            self
        }
        pub fn subtitles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SubtitleDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtitles: {}", e)
                });
            self
        }
        pub fn utterances<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UtteranceDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.utterances = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterances: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionDto> for super::TranscriptionDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                full_transcript: value.full_transcript?,
                languages: value.languages?,
                sentences: value.sentences?,
                subtitles: value.subtitles?,
                utterances: value.utterances?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionDto> for TranscriptionDto {
        fn from(value: super::TranscriptionDto) -> Self {
            Self {
                full_transcript: Ok(value.full_transcript),
                languages: Ok(value.languages),
                sentences: Ok(value.sentences),
                subtitles: Ok(value.subtitles),
                utterances: Ok(value.utterances),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionMetadataDto {
        audio_duration: ::std::result::Result<f64, ::std::string::String>,
        billing_time: ::std::result::Result<f64, ::std::string::String>,
        number_of_distinct_channels: ::std::result::Result<
            ::std::num::NonZeroU64,
            ::std::string::String,
        >,
        transcription_time: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for TranscriptionMetadataDto {
        fn default() -> Self {
            Self {
                audio_duration: Err("no value supplied for audio_duration".to_string()),
                billing_time: Err("no value supplied for billing_time".to_string()),
                number_of_distinct_channels: Err(
                    "no value supplied for number_of_distinct_channels".to_string(),
                ),
                transcription_time: Err(
                    "no value supplied for transcription_time".to_string(),
                ),
            }
        }
    }
    impl TranscriptionMetadataDto {
        pub fn audio_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_duration: {}", e)
                });
            self
        }
        pub fn billing_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.billing_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for billing_time: {}", e)
                });
            self
        }
        pub fn number_of_distinct_channels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_distinct_channels = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for number_of_distinct_channels: {}",
                        e
                    )
                });
            self
        }
        pub fn transcription_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.transcription_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for transcription_time: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionMetadataDto>
    for super::TranscriptionMetadataDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionMetadataDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_duration: value.audio_duration?,
                billing_time: value.billing_time?,
                number_of_distinct_channels: value.number_of_distinct_channels?,
                transcription_time: value.transcription_time?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionMetadataDto>
    for TranscriptionMetadataDto {
        fn from(value: super::TranscriptionMetadataDto) -> Self {
            Self {
                audio_duration: Ok(value.audio_duration),
                billing_time: Ok(value.billing_time),
                number_of_distinct_channels: Ok(value.number_of_distinct_channels),
                transcription_time: Ok(value.transcription_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranscriptionResultDto {
        audio_to_llm: ::std::result::Result<
            ::std::option::Option<super::AudioToLlmListDto>,
            ::std::string::String,
        >,
        diarization: ::std::result::Result<
            ::std::option::Option<super::DiarizationDto>,
            ::std::string::String,
        >,
        display_mode: ::std::result::Result<
            ::std::option::Option<super::DisplayModeDto>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            super::TranscriptionMetadataDto,
            ::std::string::String,
        >,
        moderation: ::std::result::Result<
            ::std::option::Option<super::ModerationDto>,
            ::std::string::String,
        >,
        name_consistency: ::std::result::Result<
            ::std::option::Option<super::NamesConsistencyDto>,
            ::std::string::String,
        >,
        named_entity_recognition: ::std::result::Result<
            ::std::option::Option<super::NamedEntityRecognitionDto>,
            ::std::string::String,
        >,
        sentences: ::std::result::Result<
            ::std::option::Option<super::SentencesDto>,
            ::std::string::String,
        >,
        sentiment_analysis: ::std::result::Result<
            ::std::option::Option<super::SentimentAnalysisDto>,
            ::std::string::String,
        >,
        structured_data_extraction: ::std::result::Result<
            ::std::option::Option<super::StructuredDataExtractionDto>,
            ::std::string::String,
        >,
        summarization: ::std::result::Result<
            ::std::option::Option<super::SummarizationDto>,
            ::std::string::String,
        >,
        transcription: ::std::result::Result<
            ::std::option::Option<super::TranscriptionDto>,
            ::std::string::String,
        >,
        translation: ::std::result::Result<
            ::std::option::Option<super::TranslationDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranscriptionResultDto {
        fn default() -> Self {
            Self {
                audio_to_llm: Ok(Default::default()),
                diarization: Ok(Default::default()),
                display_mode: Ok(Default::default()),
                metadata: Err("no value supplied for metadata".to_string()),
                moderation: Ok(Default::default()),
                name_consistency: Ok(Default::default()),
                named_entity_recognition: Ok(Default::default()),
                sentences: Ok(Default::default()),
                sentiment_analysis: Ok(Default::default()),
                structured_data_extraction: Ok(Default::default()),
                summarization: Ok(Default::default()),
                transcription: Ok(Default::default()),
                translation: Ok(Default::default()),
            }
        }
    }
    impl TranscriptionResultDto {
        pub fn audio_to_llm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AudioToLlmListDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_to_llm = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for audio_to_llm: {}", e)
                });
            self
        }
        pub fn diarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiarizationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.diarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for diarization: {}", e)
                });
            self
        }
        pub fn display_mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DisplayModeDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_mode = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for display_mode: {}", e)
                });
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionMetadataDto>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for metadata: {}", e)
                });
            self
        }
        pub fn moderation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModerationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.moderation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for moderation: {}", e)
                });
            self
        }
        pub fn name_consistency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NamesConsistencyDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.name_consistency = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for name_consistency: {}", e
                    )
                });
            self
        }
        pub fn named_entity_recognition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NamedEntityRecognitionDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.named_entity_recognition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for named_entity_recognition: {}",
                        e
                    )
                });
            self
        }
        pub fn sentences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SentencesDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentences: {}", e)
                });
            self
        }
        pub fn sentiment_analysis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SentimentAnalysisDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sentiment_analysis = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for sentiment_analysis: {}", e
                    )
                });
            self
        }
        pub fn structured_data_extraction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StructuredDataExtractionDto>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.structured_data_extraction = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for structured_data_extraction: {}",
                        e
                    )
                });
            self
        }
        pub fn summarization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummarizationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.summarization = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for summarization: {}", e)
                });
            self
        }
        pub fn transcription<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranscriptionDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.transcription = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for transcription: {}", e)
                });
            self
        }
        pub fn translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranslationDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.translation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for translation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranscriptionResultDto>
    for super::TranscriptionResultDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranscriptionResultDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_to_llm: value.audio_to_llm?,
                diarization: value.diarization?,
                display_mode: value.display_mode?,
                metadata: value.metadata?,
                moderation: value.moderation?,
                name_consistency: value.name_consistency?,
                named_entity_recognition: value.named_entity_recognition?,
                sentences: value.sentences?,
                sentiment_analysis: value.sentiment_analysis?,
                structured_data_extraction: value.structured_data_extraction?,
                summarization: value.summarization?,
                transcription: value.transcription?,
                translation: value.translation?,
            })
        }
    }
    impl ::std::convert::From<super::TranscriptionResultDto> for TranscriptionResultDto {
        fn from(value: super::TranscriptionResultDto) -> Self {
            Self {
                audio_to_llm: Ok(value.audio_to_llm),
                diarization: Ok(value.diarization),
                display_mode: Ok(value.display_mode),
                metadata: Ok(value.metadata),
                moderation: Ok(value.moderation),
                name_consistency: Ok(value.name_consistency),
                named_entity_recognition: Ok(value.named_entity_recognition),
                sentences: Ok(value.sentences),
                sentiment_analysis: Ok(value.sentiment_analysis),
                structured_data_extraction: Ok(value.structured_data_extraction),
                summarization: Ok(value.summarization),
                transcription: Ok(value.transcription),
                translation: Ok(value.translation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranslationConfigDto {
        context: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        context_adaptation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        informal: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        lipsync: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        match_original_utterances: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::TranslationModelEnum>,
            ::std::string::String,
        >,
        target_languages: ::std::result::Result<
            ::std::vec::Vec<super::TranslationLanguageCodeEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranslationConfigDto {
        fn default() -> Self {
            Self {
                context: Ok(Default::default()),
                context_adaptation: Ok(Default::default()),
                informal: Ok(Default::default()),
                lipsync: Ok(Default::default()),
                match_original_utterances: Ok(Default::default()),
                model: Ok(Default::default()),
                target_languages: Err(
                    "no value supplied for target_languages".to_string(),
                ),
            }
        }
    }
    impl TranslationConfigDto {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for context: {}", e)
                });
            self
        }
        pub fn context_adaptation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.context_adaptation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for context_adaptation: {}", e
                    )
                });
            self
        }
        pub fn informal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.informal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for informal: {}", e)
                });
            self
        }
        pub fn lipsync<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lipsync = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for lipsync: {}", e)
                });
            self
        }
        pub fn match_original_utterances<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.match_original_utterances = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for match_original_utterances: {}",
                        e
                    )
                });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TranslationModelEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for model: {}", e)
                });
            self
        }
        pub fn target_languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TranslationLanguageCodeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.target_languages = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for target_languages: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranslationConfigDto> for super::TranslationConfigDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranslationConfigDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context: value.context?,
                context_adaptation: value.context_adaptation?,
                informal: value.informal?,
                lipsync: value.lipsync?,
                match_original_utterances: value.match_original_utterances?,
                model: value.model?,
                target_languages: value.target_languages?,
            })
        }
    }
    impl ::std::convert::From<super::TranslationConfigDto> for TranslationConfigDto {
        fn from(value: super::TranslationConfigDto) -> Self {
            Self {
                context: Ok(value.context),
                context_adaptation: Ok(value.context_adaptation),
                informal: Ok(value.informal),
                lipsync: Ok(value.lipsync),
                match_original_utterances: Ok(value.match_original_utterances),
                model: Ok(value.model),
                target_languages: Ok(value.target_languages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranslationData {
        original_language: ::std::result::Result<
            super::TranscriptionLanguageCodeEnum,
            ::std::string::String,
        >,
        target_language: ::std::result::Result<
            super::TranslationLanguageCodeEnum,
            ::std::string::String,
        >,
        translated_utterance: ::std::result::Result<
            super::UtteranceDto,
            ::std::string::String,
        >,
        utterance: ::std::result::Result<super::UtteranceDto, ::std::string::String>,
        utterance_id: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranslationData {
        fn default() -> Self {
            Self {
                original_language: Err(
                    "no value supplied for original_language".to_string(),
                ),
                target_language: Err(
                    "no value supplied for target_language".to_string(),
                ),
                translated_utterance: Err(
                    "no value supplied for translated_utterance".to_string(),
                ),
                utterance: Err("no value supplied for utterance".to_string()),
                utterance_id: Err("no value supplied for utterance_id".to_string()),
            }
        }
    }
    impl TranslationData {
        pub fn original_language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionLanguageCodeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.original_language = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_language: {}", e
                    )
                });
            self
        }
        pub fn target_language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranslationLanguageCodeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.target_language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for target_language: {}", e)
                });
            self
        }
        pub fn translated_utterance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UtteranceDto>,
            T::Error: ::std::fmt::Display,
        {
            self.translated_utterance = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for translated_utterance: {}", e
                    )
                });
            self
        }
        pub fn utterance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UtteranceDto>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance: {}", e)
                });
            self
        }
        pub fn utterance_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.utterance_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterance_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranslationData> for super::TranslationData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranslationData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                original_language: value.original_language?,
                target_language: value.target_language?,
                translated_utterance: value.translated_utterance?,
                utterance: value.utterance?,
                utterance_id: value.utterance_id?,
            })
        }
    }
    impl ::std::convert::From<super::TranslationData> for TranslationData {
        fn from(value: super::TranslationData) -> Self {
            Self {
                original_language: Ok(value.original_language),
                target_language: Ok(value.target_language),
                translated_utterance: Ok(value.translated_utterance),
                utterance: Ok(value.utterance),
                utterance_id: Ok(value.utterance_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranslationDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        exec_time: ::std::result::Result<f64, ::std::string::String>,
        is_empty: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::TranslationResultDto>,
            ::std::string::String,
        >,
        success: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for TranslationDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                exec_time: Err("no value supplied for exec_time".to_string()),
                is_empty: Err("no value supplied for is_empty".to_string()),
                results: Err("no value supplied for results".to_string()),
                success: Err("no value supplied for success".to_string()),
            }
        }
    }
    impl TranslationDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn exec_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exec_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exec_time: {}", e)
                });
            self
        }
        pub fn is_empty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_empty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_empty: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranslationResultDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for success: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranslationDto> for super::TranslationDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranslationDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                exec_time: value.exec_time?,
                is_empty: value.is_empty?,
                results: value.results?,
                success: value.success?,
            })
        }
    }
    impl ::std::convert::From<super::TranslationDto> for TranslationDto {
        fn from(value: super::TranslationDto) -> Self {
            Self {
                error: Ok(value.error),
                exec_time: Ok(value.exec_time),
                is_empty: Ok(value.is_empty),
                results: Ok(value.results),
                success: Ok(value.success),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranslationMessage {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        data: ::std::result::Result<super::TranslationData, ::std::string::String>,
        error: ::std::result::Result<super::Error, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::TranslationMessageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranslationMessage {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Err("no value supplied for data".to_string()),
                error: Err("no value supplied for error".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TranslationMessage {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_at: {}", e)
                });
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranslationData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for session_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranslationMessageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranslationMessage> for super::TranslationMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranslationMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                error: value.error?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TranslationMessage> for TranslationMessage {
        fn from(value: super::TranslationMessage) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                error: Ok(value.error),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranslationResultDto {
        error: ::std::result::Result<super::AddonErrorDto, ::std::string::String>,
        full_transcript: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
        languages: ::std::result::Result<
            ::std::vec::Vec<super::TranslationLanguageCodeEnum>,
            ::std::string::String,
        >,
        sentences: ::std::result::Result<
            ::std::vec::Vec<super::SentencesDto>,
            ::std::string::String,
        >,
        subtitles: ::std::result::Result<
            ::std::vec::Vec<super::SubtitleDto>,
            ::std::string::String,
        >,
        utterances: ::std::result::Result<
            ::std::vec::Vec<super::UtteranceDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranslationResultDto {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                full_transcript: Err(
                    "no value supplied for full_transcript".to_string(),
                ),
                languages: Err("no value supplied for languages".to_string()),
                sentences: Ok(Default::default()),
                subtitles: Ok(Default::default()),
                utterances: Err("no value supplied for utterances".to_string()),
            }
        }
    }
    impl TranslationResultDto {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddonErrorDto>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for error: {}", e)
                });
            self
        }
        pub fn full_transcript<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.full_transcript = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for full_transcript: {}", e)
                });
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TranslationLanguageCodeEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for languages: {}", e)
                });
            self
        }
        pub fn sentences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SentencesDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.sentences = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sentences: {}", e)
                });
            self
        }
        pub fn subtitles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SubtitleDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtitles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtitles: {}", e)
                });
            self
        }
        pub fn utterances<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UtteranceDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.utterances = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for utterances: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TranslationResultDto> for super::TranslationResultDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranslationResultDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                full_transcript: value.full_transcript?,
                languages: value.languages?,
                sentences: value.sentences?,
                subtitles: value.subtitles?,
                utterances: value.utterances?,
            })
        }
    }
    impl ::std::convert::From<super::TranslationResultDto> for TranslationResultDto {
        fn from(value: super::TranslationResultDto) -> Self {
            Self {
                error: Ok(value.error),
                full_transcript: Ok(value.full_transcript),
                languages: Ok(value.languages),
                sentences: Ok(value.sentences),
                subtitles: Ok(value.subtitles),
                utterances: Ok(value.utterances),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnauthorizedErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnauthorizedErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl UnauthorizedErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<UnauthorizedErrorResponse>
    for super::UnauthorizedErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnauthorizedErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::UnauthorizedErrorResponse>
    for UnauthorizedErrorResponse {
        fn from(value: super::UnauthorizedErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnprocessableEntityErrorResponse {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status_code: ::std::result::Result<f64, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnprocessableEntityErrorResponse {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_id: Err("no value supplied for request_id".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl UnprocessableEntityErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for request_id: {}", e)
                });
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_code: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<UnprocessableEntityErrorResponse>
    for super::UnprocessableEntityErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnprocessableEntityErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                path: value.path?,
                request_id: value.request_id?,
                status_code: value.status_code?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::UnprocessableEntityErrorResponse>
    for UnprocessableEntityErrorResponse {
        fn from(value: super::UnprocessableEntityErrorResponse) -> Self {
            Self {
                message: Ok(value.message),
                path: Ok(value.path),
                request_id: Ok(value.request_id),
                status_code: Ok(value.status_code),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UtteranceDto {
        channel: ::std::result::Result<u64, ::std::string::String>,
        confidence: ::std::result::Result<f64, ::std::string::String>,
        end: ::std::result::Result<f64, ::std::string::String>,
        language: ::std::result::Result<
            super::TranscriptionLanguageCodeEnum,
            ::std::string::String,
        >,
        speaker: ::std::result::Result<
            ::std::option::Option<u64>,
            ::std::string::String,
        >,
        start: ::std::result::Result<f64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        words: ::std::result::Result<
            ::std::vec::Vec<super::WordDto>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UtteranceDto {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                confidence: Err("no value supplied for confidence".to_string()),
                end: Err("no value supplied for end".to_string()),
                language: Err("no value supplied for language".to_string()),
                speaker: Ok(Default::default()),
                start: Err("no value supplied for start".to_string()),
                text: Err("no value supplied for text".to_string()),
                words: Err("no value supplied for words".to_string()),
            }
        }
    }
    impl UtteranceDto {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for channel: {}", e)
                });
            self
        }
        pub fn confidence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for confidence: {}", e)
                });
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranscriptionLanguageCodeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for language: {}", e)
                });
            self
        }
        pub fn speaker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speaker = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speaker: {}", e)
                });
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for start: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn words<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WordDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.words = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for words: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<UtteranceDto> for super::UtteranceDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UtteranceDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                confidence: value.confidence?,
                end: value.end?,
                language: value.language?,
                speaker: value.speaker?,
                start: value.start?,
                text: value.text?,
                words: value.words?,
            })
        }
    }
    impl ::std::convert::From<super::UtteranceDto> for UtteranceDto {
        fn from(value: super::UtteranceDto) -> Self {
            Self {
                channel: Ok(value.channel),
                confidence: Ok(value.confidence),
                end: Ok(value.end),
                language: Ok(value.language),
                speaker: Ok(value.speaker),
                start: Ok(value.start),
                text: Ok(value.text),
                words: Ok(value.words),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookLiveEndRecordingPayload {
        event: ::std::result::Result<
            super::WebhookLiveEndRecordingPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<super::LiveEventPayload, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookLiveEndRecordingPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookLiveEndRecordingPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookLiveEndRecordingPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookLiveEndRecordingPayload>
    for super::WebhookLiveEndRecordingPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookLiveEndRecordingPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookLiveEndRecordingPayload>
    for WebhookLiveEndRecordingPayload {
        fn from(value: super::WebhookLiveEndRecordingPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookLiveEndSessionPayload {
        event: ::std::result::Result<
            super::WebhookLiveEndSessionPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<super::LiveEventPayload, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookLiveEndSessionPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookLiveEndSessionPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookLiveEndSessionPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookLiveEndSessionPayload>
    for super::WebhookLiveEndSessionPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookLiveEndSessionPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookLiveEndSessionPayload>
    for WebhookLiveEndSessionPayload {
        fn from(value: super::WebhookLiveEndSessionPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookLiveStartRecordingPayload {
        event: ::std::result::Result<
            super::WebhookLiveStartRecordingPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<super::LiveEventPayload, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookLiveStartRecordingPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookLiveStartRecordingPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookLiveStartRecordingPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookLiveStartRecordingPayload>
    for super::WebhookLiveStartRecordingPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookLiveStartRecordingPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookLiveStartRecordingPayload>
    for WebhookLiveStartRecordingPayload {
        fn from(value: super::WebhookLiveStartRecordingPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookLiveStartSessionPayload {
        event: ::std::result::Result<
            super::WebhookLiveStartSessionPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<super::LiveEventPayload, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookLiveStartSessionPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookLiveStartSessionPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookLiveStartSessionPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookLiveStartSessionPayload>
    for super::WebhookLiveStartSessionPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookLiveStartSessionPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookLiveStartSessionPayload>
    for WebhookLiveStartSessionPayload {
        fn from(value: super::WebhookLiveStartSessionPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookTranscriptionCreatedPayload {
        event: ::std::result::Result<
            super::WebhookTranscriptionCreatedPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<
            super::PreRecordedEventPayload,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebhookTranscriptionCreatedPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookTranscriptionCreatedPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookTranscriptionCreatedPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PreRecordedEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookTranscriptionCreatedPayload>
    for super::WebhookTranscriptionCreatedPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookTranscriptionCreatedPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookTranscriptionCreatedPayload>
    for WebhookTranscriptionCreatedPayload {
        fn from(value: super::WebhookTranscriptionCreatedPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookTranscriptionErrorPayload {
        event: ::std::result::Result<
            super::WebhookTranscriptionErrorPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<
            super::PreRecordedEventPayload,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebhookTranscriptionErrorPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookTranscriptionErrorPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookTranscriptionErrorPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PreRecordedEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookTranscriptionErrorPayload>
    for super::WebhookTranscriptionErrorPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookTranscriptionErrorPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookTranscriptionErrorPayload>
    for WebhookTranscriptionErrorPayload {
        fn from(value: super::WebhookTranscriptionErrorPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookTranscriptionSuccessPayload {
        event: ::std::result::Result<
            super::WebhookTranscriptionSuccessPayloadEvent,
            ::std::string::String,
        >,
        payload: ::std::result::Result<
            super::PreRecordedEventPayload,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebhookTranscriptionSuccessPayload {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                payload: Err("no value supplied for payload".to_string()),
            }
        }
    }
    impl WebhookTranscriptionSuccessPayload {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookTranscriptionSuccessPayloadEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event: {}", e)
                });
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PreRecordedEventPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for payload: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookTranscriptionSuccessPayload>
    for super::WebhookTranscriptionSuccessPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookTranscriptionSuccessPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                payload: value.payload?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookTranscriptionSuccessPayload>
    for WebhookTranscriptionSuccessPayload {
        fn from(value: super::WebhookTranscriptionSuccessPayload) -> Self {
            Self {
                event: Ok(value.event),
                payload: Ok(value.payload),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WordDto {
        confidence: ::std::result::Result<f64, ::std::string::String>,
        end: ::std::result::Result<f64, ::std::string::String>,
        start: ::std::result::Result<f64, ::std::string::String>,
        word: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WordDto {
        fn default() -> Self {
            Self {
                confidence: Err("no value supplied for confidence".to_string()),
                end: Err("no value supplied for end".to_string()),
                start: Err("no value supplied for start".to_string()),
                word: Err("no value supplied for word".to_string()),
            }
        }
    }
    impl WordDto {
        pub fn confidence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for confidence: {}", e)
                });
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for start: {}", e)
                });
            self
        }
        pub fn word<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.word = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for word: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WordDto> for super::WordDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WordDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                confidence: value.confidence?,
                end: value.end?,
                start: value.start?,
                word: value.word?,
            })
        }
    }
    impl ::std::convert::From<super::WordDto> for WordDto {
        fn from(value: super::WordDto) -> Self {
            Self {
                confidence: Ok(value.confidence),
                end: Ok(value.end),
                start: Ok(value.start),
                word: Ok(value.word),
            }
        }
    }
}
