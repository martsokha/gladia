//! An ergonomic builder over the generated [`InitTranscriptionRequest`].
//!
//! The wire format pairs each optional feature with a separate boolean:
//! `diarization: true` alongside `diarization_config: {...}`. Nine features work this
//! way, so a faithful builder would let a caller set a config while leaving its flag
//! false, in which case the API silently ignores the config.
//!
//! [`TranscriptionRequest`] removes that possibility: one method sets both, so the
//! flag and its config cannot disagree. The generated type stays reachable through
//! [`into_inner`] and [`From`] for anything this does not cover.
//!
//! [`into_inner`]: TranscriptionRequest::into_inner

use crate::model::{
    AudioToLlmListConfigDto, CallbackConfigDto, CustomSpellingConfigDto, CustomVocabularyConfigDto,
    DiarizationConfigDto, InitTranscriptionRequest, LanguageConfig, PiiRedactionConfigDto,
    SubtitlesConfigDto, SummarizationConfigDto, TranscriptionSupportedModels, TranslationConfigDto,
};

/// A transcription request.
///
/// Built from the audio to transcribe, then narrowed with the features wanted:
///
/// ```
/// use gladia::model::DiarizationConfigDto;
/// use gladia::prerecorded::TranscriptionRequest;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let request = TranscriptionRequest::new("https://example.com/meeting.wav")
///     .with_diarization(
///         DiarizationConfigDto::builder()
///             .max_speakers(Some(4u64))
///             .try_into()?,
///     )
///     .with_sentences()
///     .build();
/// # let _ = request;
/// # Ok(())
/// # }
/// ```
///
/// Each `with_*` that takes a config sets the matching boolean too, so the two cannot
/// contradict each other. Features whose config is entirely optional also have a
/// `with_*_default` that enables them without one.
#[derive(Debug, Clone, PartialEq)]
pub struct TranscriptionRequest {
    inner: InitTranscriptionRequest,
}

impl TranscriptionRequest {
    /// Starts a request for the audio at `audio_url`.
    ///
    /// The URL is either one returned by [`PreRecorded::upload`], or any URL Gladia can
    /// fetch (an S3 link, a public file). Every other field is left unset, so the API's
    /// own defaults apply.
    ///
    /// [`PreRecorded::upload`]: super::PreRecorded::upload
    pub fn new(audio_url: impl Into<String>) -> Self {
        Self {
            inner: InitTranscriptionRequest {
                audio_url: audio_url.into(),
                audio_to_llm: None,
                audio_to_llm_config: None,
                callback: None,
                callback_config: None,
                callback_url: None,
                custom_metadata: serde_json::Map::new(),
                custom_spelling: None,
                custom_spelling_config: None,
                custom_vocabulary: None,
                custom_vocabulary_config: None,
                diarization: None,
                diarization_config: None,
                language_config: None,
                model: None,
                named_entity_recognition: None,
                pii_redaction: None,
                pii_redaction_config: None,
                punctuation_enhanced: None,
                sentences: None,
                sentiment_analysis: None,
                subtitles: None,
                subtitles_config: None,
                summarization: None,
                summarization_config: None,
                translation: None,
                translation_config: None,
            },
        }
    }

    /// Selects the transcription model. Defaults to `solaria-1`.
    pub fn with_model(mut self, model: TranscriptionSupportedModels) -> Self {
        self.inner.model = Some(model);
        self
    }

    /// Sets the language configuration, including code-switching.
    ///
    /// Left unset, the language is detected automatically.
    pub fn with_language(mut self, language: LanguageConfig) -> Self {
        self.inner.language_config = Some(language);
        self
    }

    /// Attaches arbitrary JSON to the job, returned on the response and usable as a
    /// filter in [`ListQuery::with_custom_metadata`].
    ///
    /// [`ListQuery::with_custom_metadata`]: super::ListQuery::with_custom_metadata
    pub fn with_custom_metadata(
        mut self,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        self.inner.custom_metadata = metadata;
        self
    }

    /// Splits the transcript into sentences.
    pub fn with_sentences(mut self) -> Self {
        self.inner.sentences = Some(true);
        self
    }

    /// Enables enhanced punctuation.
    pub fn with_punctuation_enhanced(mut self) -> Self {
        self.inner.punctuation_enhanced = Some(true);
        self
    }

    /// Enables named-entity recognition. Has no configuration.
    pub fn with_named_entity_recognition(mut self) -> Self {
        self.inner.named_entity_recognition = Some(true);
        self
    }

    /// Enables sentiment analysis. Has no configuration.
    pub fn with_sentiment_analysis(mut self) -> Self {
        self.inner.sentiment_analysis = Some(true);
        self
    }

    /// Identifies who is speaking, with the given configuration.
    ///
    /// Use [`with_diarization_default`] to let Gladia infer the speaker count.
    ///
    /// [`with_diarization_default`]: Self::with_diarization_default
    pub fn with_diarization(mut self, config: DiarizationConfigDto) -> Self {
        self.inner.diarization = Some(true);
        self.inner.diarization_config = Some(config);
        self
    }

    /// Identifies who is speaking, with Gladia's defaults.
    pub fn with_diarization_default(mut self) -> Self {
        self.inner.diarization = Some(true);
        self
    }

    /// Translates the transcript. The configuration names the target languages, so
    /// there is no default form.
    pub fn with_translation(mut self, config: TranslationConfigDto) -> Self {
        self.inner.translation = Some(true);
        self.inner.translation_config = Some(config);
        self
    }

    /// Summarizes the transcript, with the given configuration.
    ///
    /// Use [`with_summarization_default`] for Gladia's default summary type.
    ///
    /// [`with_summarization_default`]: Self::with_summarization_default
    pub fn with_summarization(mut self, config: SummarizationConfigDto) -> Self {
        self.inner.summarization = Some(true);
        self.inner.summarization_config = Some(config);
        self
    }

    /// Summarizes the transcript, with Gladia's defaults.
    pub fn with_summarization_default(mut self) -> Self {
        self.inner.summarization = Some(true);
        self
    }

    /// Generates subtitles, with the given configuration.
    ///
    /// Use [`with_subtitles_default`] for Gladia's default formats.
    ///
    /// [`with_subtitles_default`]: Self::with_subtitles_default
    pub fn with_subtitles(mut self, config: SubtitlesConfigDto) -> Self {
        self.inner.subtitles = Some(true);
        self.inner.subtitles_config = Some(config);
        self
    }

    /// Generates subtitles, with Gladia's defaults.
    pub fn with_subtitles_default(mut self) -> Self {
        self.inner.subtitles = Some(true);
        self
    }

    /// Biases recognition towards domain terms. The configuration carries the
    /// vocabulary, so there is no default form.
    pub fn with_custom_vocabulary(mut self, config: CustomVocabularyConfigDto) -> Self {
        self.inner.custom_vocabulary = Some(true);
        self.inner.custom_vocabulary_config = Some(config);
        self
    }

    /// Applies a spelling dictionary. The configuration carries the dictionary, so
    /// there is no default form.
    pub fn with_custom_spelling(mut self, config: CustomSpellingConfigDto) -> Self {
        self.inner.custom_spelling = Some(true);
        self.inner.custom_spelling_config = Some(config);
        self
    }

    /// Redacts personally identifying information, with the given configuration.
    ///
    /// Use [`with_pii_redaction_default`] for Gladia's default entity types.
    ///
    /// [`with_pii_redaction_default`]: Self::with_pii_redaction_default
    pub fn with_pii_redaction(mut self, config: PiiRedactionConfigDto) -> Self {
        self.inner.pii_redaction = Some(true);
        self.inner.pii_redaction_config = Some(config);
        self
    }

    /// Redacts personally identifying information, with Gladia's defaults.
    pub fn with_pii_redaction_default(mut self) -> Self {
        self.inner.pii_redaction = Some(true);
        self
    }

    /// Runs prompts against the audio. The configuration carries the prompts, so there
    /// is no default form.
    pub fn with_audio_to_llm(mut self, config: AudioToLlmListConfigDto) -> Self {
        self.inner.audio_to_llm = Some(true);
        self.inner.audio_to_llm_config = Some(config);
        self
    }

    /// Posts the result to a callback URL when the job finishes.
    ///
    /// The configuration carries the URL, so there is no default form. For the URL
    /// alone, [`with_callback_url`] is shorter.
    ///
    /// [`with_callback_url`]: Self::with_callback_url
    pub fn with_callback(mut self, config: CallbackConfigDto) -> Self {
        self.inner.callback = Some(true);
        self.inner.callback_config = Some(config);
        self
    }

    /// Posts the result to `url` when the job finishes, with the default method.
    pub fn with_callback_url(mut self, url: impl Into<String>) -> Self {
        self.inner.callback = Some(true);
        self.inner.callback_url = Some(url.into());
        self
    }

    /// Returns the wire request.
    pub fn build(self) -> InitTranscriptionRequest {
        self.inner
    }

    /// Returns the wire request, for setting a field this builder does not model.
    ///
    /// The generated type is public, so anything missing here can be set on it
    /// directly.
    pub fn into_inner(self) -> InitTranscriptionRequest {
        self.inner
    }
}

impl From<TranscriptionRequest> for InitTranscriptionRequest {
    fn from(request: TranscriptionRequest) -> Self {
        request.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json_of(request: TranscriptionRequest) -> serde_json::Value {
        serde_json::to_value(request.build()).unwrap()
    }

    #[test]
    fn only_the_audio_url_is_sent_by_default() {
        let value = json_of(TranscriptionRequest::new("https://example.com/a.wav"));

        assert_eq!(
            value,
            serde_json::json!({ "audio_url": "https://example.com/a.wav" })
        );
    }

    #[test]
    fn a_feature_config_sets_its_flag_too() {
        // The whole point of the wrapper: these two cannot be set independently.
        let config: DiarizationConfigDto = DiarizationConfigDto::builder()
            .max_speakers(Some(4u64))
            .try_into()
            .unwrap();

        let value = json_of(
            TranscriptionRequest::new("https://example.com/a.wav").with_diarization(config),
        );

        assert_eq!(value["diarization"], serde_json::json!(true));
        assert_eq!(
            value["diarization_config"]["max_speakers"],
            serde_json::json!(4)
        );
    }

    #[test]
    fn a_default_feature_sets_the_flag_without_a_config() {
        let value = json_of(
            TranscriptionRequest::new("https://example.com/a.wav").with_subtitles_default(),
        );

        assert_eq!(value["subtitles"], serde_json::json!(true));
        assert!(
            value.get("subtitles_config").is_none(),
            "an empty config should not be sent: {value}"
        );
    }

    #[test]
    fn config_free_features_are_plain_flags() {
        let value = json_of(
            TranscriptionRequest::new("https://example.com/a.wav")
                .with_sentences()
                .with_sentiment_analysis()
                .with_named_entity_recognition(),
        );

        assert_eq!(value["sentences"], serde_json::json!(true));
        assert_eq!(value["sentiment_analysis"], serde_json::json!(true));
        assert_eq!(value["named_entity_recognition"], serde_json::json!(true));
        assert_eq!(value.as_object().unwrap().len(), 4);
    }

    #[test]
    fn unset_features_stay_off_the_wire() {
        let value =
            json_of(TranscriptionRequest::new("https://example.com/a.wav").with_sentences());

        // Absent entirely rather than `"diarization": false`, so the API's default
        // applies.
        assert!(value.get("diarization").is_none(), "{value}");
        assert!(value.get("translation").is_none(), "{value}");
    }
}
