
use gsvk::core::device::GsDevice;

pub use gsvk::sync::GsSemaphore;
pub use gsvk::sync::GsFence;
pub use gsvk::sync::SyncError;

pub struct SyncKit {

    device: GsDevice,
}

impl SyncKit {

    pub(crate) fn init(device: &GsDevice) -> SyncKit {

        SyncKit {
            device : device.clone(),
        }
    }

    pub fn fence(&self, is_sign: bool) -> Result<GsFence, SyncError> {
        GsFence::setup(&self.device, is_sign)
    }

    pub fn semaphore(&self) -> Result<GsSemaphore, SyncError> {
        GsSemaphore::setup(&self.device)
    }
}
