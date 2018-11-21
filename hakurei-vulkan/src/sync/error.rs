
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SyncError {

    FenceCreationError,
    SemaphoreCreationError,

    FenceTimeOutError,
    FenceResetError,
}

impl Error for SyncError {}
impl fmt::Display for SyncError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | SyncError::FenceCreationError     => "Failed to create Fence object.",
            | SyncError::SemaphoreCreationError => "Failed to create Semaphore object.",

            | SyncError::FenceTimeOutError      => "Fence timeout has expired.",
            | SyncError::FenceResetError        => "Failed to reset fence.",
        };

        write!(f, "{}", description)
    }
}
