
use crate::core::device::queue::GsQueue;

pub struct GsGraphicsQueue {

    queue: GsQueue,
}

impl GsGraphicsQueue {

    pub fn new(queue: GsQueue) -> GsGraphicsQueue {

        GsGraphicsQueue {
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
