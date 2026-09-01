//! Query parameters for [`PreRecorded::list`].
//!
//! [`PreRecorded::list`]: super::PreRecorded::list

use chrono::{DateTime, Utc};
use url::Url;

use crate::model::PreRecordedResponseStatus;

/// Filters and pagination for [`PreRecorded::list`].
///
/// Every field is optional; an unset one is simply not sent, leaving the API's own
/// default in place. Build one by chaining setters onto the default:
///
/// ```
/// use gladia::model::PreRecordedResponseStatus;
/// use gladia::prerecorded::ListQuery;
///
/// let query = ListQuery::default()
///     .with_limit(50)
///     .with_status([PreRecordedResponseStatus::Done]);
/// ```
///
/// [`PreRecorded::list`]: super::PreRecorded::list
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListQuery {
    offset: Option<u64>,
    limit: Option<u64>,
    date: Option<DateTime<Utc>>,
    before_date: Option<DateTime<Utc>>,
    after_date: Option<DateTime<Utc>>,
    status: Vec<PreRecordedResponseStatus>,
    custom_metadata: Option<serde_json::Value>,
}

impl ListQuery {
    /// Skips this many jobs before the first returned.
    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Returns at most this many jobs.
    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Keeps only jobs created on this date.
    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    /// Keeps only jobs created before this instant.
    pub fn with_before_date(mut self, before: DateTime<Utc>) -> Self {
        self.before_date = Some(before);
        self
    }

    /// Keeps only jobs created after this instant.
    pub fn with_after_date(mut self, after: DateTime<Utc>) -> Self {
        self.after_date = Some(after);
        self
    }

    /// Keeps only jobs in one of these statuses.
    ///
    /// Replaces any previously set statuses. An empty iterator clears the filter.
    pub fn with_status(
        mut self,
        status: impl IntoIterator<Item = PreRecordedResponseStatus>,
    ) -> Self {
        self.status = status.into_iter().collect();
        self
    }

    /// Keeps only jobs whose `custom_metadata` matches this JSON object.
    ///
    /// The value is the metadata passed to
    /// [`InitTranscriptionRequest::custom_metadata`](crate::model::InitTranscriptionRequest).
    pub fn with_custom_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.custom_metadata = Some(metadata);
        self
    }

    /// Appends the set parameters to `url`'s query string.
    pub(crate) fn apply(&self, url: &mut Url) {
        let mut query = url.query_pairs_mut();

        if let Some(offset) = self.offset {
            query.append_pair("offset", &offset.to_string());
        }
        if let Some(limit) = self.limit {
            query.append_pair("limit", &limit.to_string());
        }
        for (key, value) in [
            ("date", self.date),
            ("before_date", self.before_date),
            ("after_date", self.after_date),
        ] {
            if let Some(value) = value {
                query.append_pair(key, &value.to_rfc3339());
            }
        }
        // Repeated key, which is how the API expects an array parameter.
        for status in &self.status {
            query.append_pair("status", &status.to_string());
        }
        if let Some(metadata) = &self.custom_metadata {
            query.append_pair("custom_metadata", &metadata.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query_of(query: &ListQuery) -> String {
        let mut url = Url::parse("https://api.gladia.io/v2/pre-recorded").unwrap();
        query.apply(&mut url);
        url.query().unwrap_or_default().to_owned()
    }

    #[test]
    fn an_empty_query_sends_nothing() {
        assert_eq!(query_of(&ListQuery::default()), "");
    }

    #[test]
    fn pagination_is_serialized() {
        let query = ListQuery::default().with_offset(20).with_limit(10);
        assert_eq!(query_of(&query), "offset=20&limit=10");
    }

    #[test]
    fn statuses_repeat_the_key() {
        let query = ListQuery::default().with_status([
            PreRecordedResponseStatus::Queued,
            PreRecordedResponseStatus::Processing,
        ]);
        assert_eq!(query_of(&query), "status=queued&status=processing");
    }

    #[test]
    fn dates_are_rfc3339() {
        let date = DateTime::parse_from_rfc3339("2026-09-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let query = ListQuery::default().with_after_date(date);
        assert_eq!(
            query_of(&query),
            "after_date=2026-09-01T00%3A00%3A00%2B00%3A00"
        );
    }

    #[test]
    fn custom_metadata_is_sent_as_json() {
        let query =
            ListQuery::default().with_custom_metadata(serde_json::json!({ "team": "research" }));
        assert_eq!(
            query_of(&query),
            "custom_metadata=%7B%22team%22%3A%22research%22%7D"
        );
    }
}
