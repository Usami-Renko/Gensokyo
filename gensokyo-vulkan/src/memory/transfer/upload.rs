
use crate::core::GsDevice;

use crate::buffer::BufferInstance;
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::traits::MemoryDataDelegate;
use crate::error::VkResult;
use crate::types::vkbytes;

pub struct GsBufferDataUploader {

    device: GsDevice,
    agency: Box<dyn MemoryDataDelegate>,

    is_finished: bool,
}

impl GsBufferDataUploader {

    pub(crate) fn new(device: &GsDevice, memory: &GsBufferMemory, allocate_infos: &BufferAllocateInfos) -> VkResult<GsBufferDataUploader> {

        let mut agency = memory.to_upload_agency(device, allocate_infos)?;
        agency.prepare(device)?;

        let uploader = GsBufferDataUploader {
            device: device.clone(),
            agency,
            is_finished: false,
        };
        Ok(uploader)
    }

    pub fn upload(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> VkResult<&mut GsBufferDataUploader> {

        let writer = to.acquire_write_ptr(&mut self.agency)?;
        writer.write_data(data);

        Ok(self)
    }

    pub fn upload_align(&mut self, to: &impl BufferInstance, data: &[impl Copy], element_alignment: vkbytes) -> VkResult<&mut GsBufferDataUploader> {

        let writer = to.acquire_write_ptr(&mut self.agency)?;
        writer.write_data_with_alignment(data, element_alignment);

        Ok(self)
    }

    pub fn upload_v2<D>(&mut self, to: &impl GsBufferUploadable<D>, data: &D) -> VkResult<&mut GsBufferDataUploader> {

        let func = to.upload_func();
        func(to, self, data)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> VkResult<()> {

        self.is_finished = true;
        self.agency.finish(&self.device)
    }
}

impl Drop for GsBufferDataUploader {

    fn drop(&mut self) {
        debug_assert!(self.is_finished, "function GsBufferDataUploader::finish must be call before it drops.");
    }
}

pub trait GsBufferUploadable<D> {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &D) -> VkResult<()>>;
}
