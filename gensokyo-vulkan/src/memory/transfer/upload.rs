
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::BufferInstance;
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::traits::MemoryDataDelegate;
use crate::memory::error::AllocatorError;


pub struct GsBufferDataUploader {

    device: GsDevice,
    agency: Box<dyn MemoryDataDelegate>,

    is_finished: bool,
}

impl GsBufferDataUploader {

    pub(crate) fn new(physical: &GsPhyDevice, device: &GsDevice, memory: &GsBufferMemory, allocate_infos: &BufferAllocateInfos) -> Result<GsBufferDataUploader, AllocatorError> {

        let mut agency = memory.to_upload_agency(device, physical, allocate_infos)?;
        agency.prepare(device)?;

        let uploader = GsBufferDataUploader {
            device: device.clone(),
            agency,
            is_finished: false,
        };
        Ok(uploader)
    }

    pub fn upload(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> Result<&mut GsBufferDataUploader, AllocatorError> {

        let writer = self.agency.acquire_write_ptr(to.as_block_ref(), to.repository_index())?;
        writer.write_data(data);

        Ok(self)
    }

    pub fn upload_v2<D>(&mut self, to: &impl BufferUploadDst<D>, data: &D) -> Result<&mut GsBufferDataUploader, AllocatorError> {

        let func = to.upload_func();
        func(to, self, data)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.is_finished = true;
        self.agency.finish(&self.device)
    }
}

impl Drop for GsBufferDataUploader {

    fn drop(&mut self) {
        debug_assert!(self.is_finished, "function GsBufferDataUploader::finish must be call before it drops.");
    }
}

pub trait BufferUploadDst<D> {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &D) -> Result<(), AllocatorError>>;
}
