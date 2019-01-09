
use crate::core::device::GsDevice;

use crate::buffer::BufferInstance;
use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::traits::MemoryDataDelegate;
use crate::memory::error::AllocatorError;

pub struct GsBufferDataUpdater {

    device: GsDevice,
    agency: Box<dyn MemoryDataDelegate>,

    is_finished: bool,
}

impl GsBufferDataUpdater {

    pub(crate) fn new(device: &GsDevice, memory: &GsBufferMemory) -> Result<GsBufferDataUpdater, AllocatorError> {

        let mut agency = memory.to_update_agency()?;
        agency.prepare(device)?;

        let updater = GsBufferDataUpdater {
            device: device.clone(),
            agency,
            is_finished: false,
        };
        Ok(updater)
    }

    pub fn update(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> Result<&mut GsBufferDataUpdater, AllocatorError> {

        let writer = self.agency.acquire_write_ptr(to.as_block_ref(), to.repository_index())?;
        writer.write_data(data);

        Ok(self)
    }

    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.is_finished = true;
        self.agency.finish(&self.device)
    }
}

impl Drop for GsBufferDataUpdater {

    fn drop(&mut self) {
        debug_assert!(self.is_finished, "function GsBufferDataUpdater::finish must be call before it drops.");
    }
}
