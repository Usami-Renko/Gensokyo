
use crate::error::{ GsResult, GsError };

use gsvk::core::device::GsDevice;

pub use gsvk::sync::GsSemaphore;
pub use gsvk::sync::GsFence;


pub struct SyncKit {

    device: GsDevice,
}

impl SyncKit {

    pub(crate) fn init(device: &GsDevice) -> SyncKit {

        SyncKit {
            device : device.clone(),
        }
    }

    pub fn fence(&self, is_sign: bool) -> GsResult<GsFence> {
        GsFence::setup(&self.device, is_sign).map_err(GsError::from)
    }

    pub fn semaphore(&self) -> GsResult<GsSemaphore> {
        GsSemaphore::setup(&self.device).map_err(GsError::from)
    }
}
