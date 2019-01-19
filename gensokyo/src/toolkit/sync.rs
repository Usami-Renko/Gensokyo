
use crate::error::GsResult;

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
        let fence = GsFence::setup(&self.device, is_sign)?;
        Ok(fence)
    }

    pub fn semaphore(&self) -> GsResult<GsSemaphore> {

        let semaphore = GsSemaphore::setup(&self.device)?;
        Ok(semaphore)
    }

    pub fn multi_fences(&self, is_sign: bool, count: usize) -> GsResult<Vec<GsFence>> {

        let mut fences = Vec::with_capacity(count);
        for _ in 0..count {
            let fence = GsFence::setup(&self.device, is_sign)?;
            fences.push(fence);
        }

        Ok(fences)
    }

    pub fn multi_semaphores(&self, count: usize) -> GsResult<Vec<GsSemaphore>> {

        let mut semaphores = Vec::with_capacity(count);
        for _ in 0..count {
            let semaphore = GsSemaphore::setup(&self.device)?;
            semaphores.push(semaphore);
        }

        Ok(semaphores)
    }
}
