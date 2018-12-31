
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::{ BufferBlock, BufferInstance };
use crate::buffer::allocator::BufferAllocateInfos;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;

use crate::memory::instance::GsBufferMemory;
use crate::memory::utils::MemoryWritePtr;
use crate::memory::error::{ MemoryError, AllocatorError };

use std::marker::PhantomData;


pub struct BufferDataUploader<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device: GsDevice,
    agency: Box<dyn MemoryDataDelegate>,
}

impl<M> BufferDataUploader<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn new(phantom_type: PhantomData<M>, physical: &GsPhyDevice, device: &GsDevice, memory: &GsBufferMemory, allocate_infos: &BufferAllocateInfos) -> Result<BufferDataUploader<M>, AllocatorError> {

        let mut agency = memory.to_agency(device, physical, allocate_infos)?;
        agency.prepare(device)?;

        let uploader = BufferDataUploader {
            phantom_type,
            device: device.clone(),
            agency,
        };
        Ok(uploader)
    }

    pub fn upload(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> Result<&mut BufferDataUploader<M>, AllocatorError> {

        let writer = self.agency.acquire_write_ptr(to.as_block_ref(), to.repository_index())?;
        writer.write_data(data);

        Ok(self)
    }

    // TODO: Add finish call checking to remind user remembering call this function in debug mode.
    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.agency.finish(&self.device)
    }
}

pub trait MemoryDataDelegate {

    fn prepare(&mut self, device: &GsDevice) -> Result<(), MemoryError>;

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError>;

    fn finish(&mut self, device: &GsDevice) -> Result<(), AllocatorError>;
}
