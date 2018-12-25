
use crate::core::device::queue::GsQueue;

pub struct GsPresentQueue {

    queue: GsQueue,
}

impl GsPresentQueue {

    pub fn new(queue: GsQueue) -> GsPresentQueue {

        GsPresentQueue {
            queue
        }
    }

    pub fn queue(&self) -> &GsQueue {
        &self.queue
    }

    pub fn destroy(&self) {
        // nothing to clean
    }
}
