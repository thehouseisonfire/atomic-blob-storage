use super::{AtomicBlobStoreError, BlockingResult, CleanupReport, Duration, Operation, Sender};

#[cfg(any(unix, windows))]
pub struct Submission {
    pub(crate) key_hash: [u8; 32],
    pub(crate) operation: Operation,
    pub(crate) completion_sender: Sender<CoordinatorEvent>,
}

#[cfg(any(unix, windows))]
pub struct QueuedOperation {
    pub(crate) operation: Operation,
    pub(crate) completion_sender: Sender<CoordinatorEvent>,
}

#[cfg(any(unix, windows))]
pub struct Completion {
    pub(crate) key_hash: [u8; 32],
    pub(crate) outcome: Option<(Operation, BlockingResult)>,
}

#[cfg(any(unix, windows))]
pub enum CoordinatorEvent {
    Submission(Submission),
    Completion(Completion),
    Maintenance(MaintenanceSubmission),
    MaintenanceCompletion(MaintenanceCompletion),
    Flush(Sender<Result<(), AtomicBlobStoreError>>),
    Close(CloseSubmission),
}

#[cfg(any(unix, windows))]
pub struct MaintenanceSubmission {
    pub(crate) minimum_age: Option<Duration>,
    pub(crate) sender: Sender<Result<CleanupReport, AtomicBlobStoreError>>,
    pub(crate) completion_sender: Sender<CoordinatorEvent>,
}

#[cfg(any(unix, windows))]
pub struct CloseSubmission {
    pub(crate) sender: Sender<Result<(), AtomicBlobStoreError>>,
}

#[cfg(any(unix, windows))]
pub struct MaintenanceCompletion {
    pub(crate) outcome: Option<MaintenanceOutcome>,
}

#[cfg(any(unix, windows))]
pub enum PendingEvent {
    Submission(Submission),
    Maintenance(MaintenanceSubmission),
    Flush(Sender<Result<(), AtomicBlobStoreError>>),
    Close(CloseSubmission),
}

#[cfg(any(unix, windows))]
pub type MaintenanceOutcome = (
    Sender<Result<CleanupReport, AtomicBlobStoreError>>,
    Result<CleanupReport, AtomicBlobStoreError>,
);
