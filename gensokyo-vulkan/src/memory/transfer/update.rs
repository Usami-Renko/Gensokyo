
use crate::core::device::GsDevice;

use crate::buffer::BufferInstance;
use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::traits::MemoryDataDelegate;
use crate::error::VkResult;

use crate::types::vkbytes;

pub struct GsBufferDataUpdater {

    device: GsDevice,
    agency: Box<dyn MemoryDataDelegate>,

    is_finished: bool,
}

impl GsBufferDataUpdater {

    pub(crate) fn new(device: &GsDevice, memory: &GsBufferMemory) -> VkResult<GsBufferDataUpdater> {

        let mut agency = memory.to_update_agency()?;
        agency.prepare(device)?;

        let updater = GsBufferDataUpdater {
            device: device.clone(),
            agency,
            is_finished: false,
        };
        Ok(updater)
    }

    pub fn update(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> VkResult<&mut GsBufferDataUpdater> {

        let writer = to.acquire_write_ptr(&mut self.agency)?;
        writer.write_data(data);

        Ok(self)
    }

    pub fn update_align(&mut self, to: &impl BufferInstance, data: &[impl Copy], alignment: vkbytes) -> VkResult<&mut GsBufferDataUpdater> {

        let writer = to.acquire_write_ptr(&mut self.agency)?;
        writer.write_data_with_alignment(data, alignment);

        Ok(self)
    }

    pub fn update_v2(&mut self, to: &impl GsBufferUpdatable) -> VkResult<&mut GsBufferDataUpdater> {

        let func = to.update_func();
        func(to, self)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> VkResult<()> {

        self.is_finished = true;
        self.agency.finish(&self.device)
    }
}

impl Drop for GsBufferDataUpdater {

    fn drop(&mut self) {
        debug_assert!(self.is_finished, "function GsBufferDataUpdater::finish must be call before it drops.");
    }
}

pub trait GsBufferUpdatable {

    fn update_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUpdater) -> VkResult<()>>;
}
