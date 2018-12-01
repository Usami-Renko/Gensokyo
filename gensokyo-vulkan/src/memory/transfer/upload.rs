
use core::device::GsDevice;
use core::physical::GsPhyDevice;

use buffer::{ BufferBlock, BufferInstance };
use buffer::allocator::BufferAllocateInfos;
use buffer::allocator::types::BufferMemoryTypeAbs;

use memory::instance::GsBufferMemory;
use memory::utils::MemoryWritePtr;
use memory::error::{ MemoryError, AllocatorError };

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

    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.agency.finish(&self.device)
    }
}

pub trait MemoryDataDelegate {

    fn prepare(&mut self, device: &GsDevice) -> Result<(), MemoryError>;

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError>;

    fn finish(&mut self, device: &GsDevice) -> Result<(), AllocatorError>;
}
