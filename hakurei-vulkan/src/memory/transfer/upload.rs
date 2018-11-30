
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::{ BufferBlock, BufferInstance };
use buffer::allocator::BufferAllocateInfos;
use buffer::allocator::types::BufferMemoryTypeAbs;

use memory::instance::HaBufferMemory;
use memory::error::{ MemoryError, AllocatorError };

use utils::memory::MemoryWritePtr;
use std::marker::PhantomData;

pub struct BufferDataUploader<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device: HaDevice,
    agency: Box<dyn MemoryDataDelegate>,
}

impl<M> BufferDataUploader<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn new(phantom_type: PhantomData<M>, physical: &HaPhyDevice, device: &HaDevice, memory: &HaBufferMemory, allocate_infos: &BufferAllocateInfos) -> Result<BufferDataUploader<M>, AllocatorError> {

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

    fn prepare(&mut self, device: &HaDevice) -> Result<(), MemoryError>;

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError>;

    fn finish(&mut self, device: &HaDevice) -> Result<(), AllocatorError>;
}
