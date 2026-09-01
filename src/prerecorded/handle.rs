//! A handle to a submitted transcription job, and the polling built on it.

use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::model::{PreRecordedResponse, PreRecordedResponseStatus};

/// A submitted transcription job.
///
/// Returned by [`PreRecorded::submit`], and constructible from a stored id with
/// [`JobHandle::new`]. Transcription jobs outlive the process that submitted them, so
/// a caller that persists an id can resume from it later.
///
/// The handle owns a [`Client`] clone, which is an `Arc` bump, so it is cheap to clone
/// and can be moved into a task.
///
/// ```no_run
/// # use gladia::prelude::*;
/// # use uuid::Uuid;
/// # async fn run(client: Client, id: Uuid) -> Result<()> {
/// // Resume a job submitted earlier, in another process.
/// let job = JobHandle::new(client, id).wait().await?;
/// # let _ = job;
/// # Ok(())
/// # }
/// ```
///
/// [`PreRecorded::submit`]: super::PreRecorded::submit
#[derive(Debug, Clone)]
pub struct JobHandle {
    client: Client,
    id: Uuid,
}

/// How long [`JobHandle::wait`] leaves between polls.
const DEFAULT_INTERVAL: Duration = Duration::from_secs(3);

impl JobHandle {
    /// Builds a handle for an existing job id.
    ///
    /// Nothing is fetched; the job is only contacted when [`status`], [`poll`], or
    /// [`wait`] is called.
    ///
    /// [`status`]: Self::status
    /// [`poll`]: Self::poll
    /// [`wait`]: Self::wait
    pub fn new(client: Client, id: Uuid) -> Self {
        Self { client, id }
    }

    /// The job's id.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Fetches the job's current state, whatever its status.
    ///
    /// This is a single request, and returns as-is: the job may still be queued or
    /// processing, in which case `result` is `None`. Use [`wait`] to block until it
    /// finishes.
    ///
    /// [`wait`]: Self::wait
    pub async fn poll(&self) -> Result<PreRecordedResponse> {
        self.client.prerecorded().get(self.id).await
    }

    /// Fetches just the job's status.
    ///
    /// A convenience over [`poll`] for a progress display that does not need the body.
    ///
    /// [`poll`]: Self::poll
    pub async fn status(&self) -> Result<PreRecordedResponseStatus> {
        Ok(self.poll().await?.status)
    }

    /// Polls until the job reaches a terminal status, then returns it.
    ///
    /// Polls every three seconds with no deadline. Use [`wait_with`] to choose the
    /// interval, set a timeout, or both.
    ///
    /// A job that finishes in the `error` status is returned like any other: the
    /// transcription failed, but the request did not. Inspect `status` and `error_code`
    /// to tell them apart, or use [`into_result`] to treat a failed job as an error.
    ///
    /// [`wait_with`]: Self::wait_with
    /// [`into_result`]: Self::into_result
    pub async fn wait(&self) -> Result<PreRecordedResponse> {
        self.wait_with(DEFAULT_INTERVAL, None).await
    }

    /// Polls at `interval` until the job reaches a terminal status or `timeout`
    /// elapses.
    ///
    /// `timeout` of `None` waits indefinitely. The first poll happens immediately, so
    /// an already-finished job returns without sleeping.
    ///
    /// Returns [`Error::Timeout`] if the deadline passes first. The job keeps running
    /// on Gladia's side: the handle can be waited on again, or its id stored and
    /// resumed later.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), err))]
    pub async fn wait_with(
        &self,
        interval: Duration,
        timeout: Option<Duration>,
    ) -> Result<PreRecordedResponse> {
        let deadline = timeout.map(|timeout| Instant::now() + timeout);

        loop {
            let job = self.poll().await?;
            if is_terminal(job.status) {
                return Ok(job);
            }

            // Check the deadline after polling, so a job that completes on the last
            // attempt is returned rather than reported as a timeout.
            if let Some(deadline) = deadline {
                let now = Instant::now();
                if now >= deadline {
                    return Err(Error::Timeout {
                        timeout: timeout.expect("a deadline implies a timeout"),
                    });
                }
                // Never sleep past the deadline.
                let remaining = deadline - now;
                tokio::time::sleep(interval.min(remaining)).await;
            } else {
                tokio::time::sleep(interval).await;
            }
        }
    }

    /// Polls until the job finishes, failing if it finished unsuccessfully.
    ///
    /// Like [`wait`], but a job in the `error` status becomes an [`Error::Job`] rather
    /// than an `Ok` carrying a failed job, for callers who only want the transcript
    /// and would otherwise have to check the status by hand.
    ///
    /// [`wait`]: Self::wait
    pub async fn into_result(&self) -> Result<PreRecordedResponse> {
        let job = self.wait().await?;

        if job.status == PreRecordedResponseStatus::Error {
            return Err(Error::Job {
                id: job.id,
                error_code: job.error_code,
            });
        }

        Ok(job)
    }
}

/// Whether a status means the job has stopped changing.
fn is_terminal(status: PreRecordedResponseStatus) -> bool {
    match status {
        PreRecordedResponseStatus::Done | PreRecordedResponseStatus::Error => true,
        PreRecordedResponseStatus::Queued | PreRecordedResponseStatus::Processing => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_statuses_end_the_poll_loop() {
        assert!(is_terminal(PreRecordedResponseStatus::Done));
        assert!(is_terminal(PreRecordedResponseStatus::Error));

        assert!(!is_terminal(PreRecordedResponseStatus::Queued));
        assert!(!is_terminal(PreRecordedResponseStatus::Processing));
    }
}
