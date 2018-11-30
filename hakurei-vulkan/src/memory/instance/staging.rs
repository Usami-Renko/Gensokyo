
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::{ HaBuffer, BufferBlock };
use buffer::allocator::BufferAllocateInfos;

use memory::target::HaMemory;
use memory::structs::{ HaMemoryType, MemoryMapStatus, MemoryRange, MemoryMapAlias };
use memory::types::Staging;
use memory::traits::{ HaMemoryAbstract, MemoryMappable };
use memory::selector::MemorySelector;
use memory::transfer::DataCopyer;
use memory::instance::HaBufferMemoryAbs;
use memory::transfer::MemoryDataDelegate;
use memory::error::{ MemoryError, AllocatorError };

use utils::memory::MemoryWritePtr;
use types::vkbytes;


pub struct HaStagingMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMappable for HaStagingMemory {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.target.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl HaMemoryAbstract for HaStagingMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::StagingMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaStagingMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;
        let map_status = MemoryMapStatus::from_unmap();

        let memory = HaStagingMemory {
            target, map_status,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMappable> {
        Some(self)
    }
}

impl HaBufferMemoryAbs for HaStagingMemory {

    fn to_agency(&self, _: &HaDevice, _: &HaPhyDevice, _: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = StagingDataAgency::new(self)?;
        Ok(Box::new(agency))
    }
}

pub struct StagingDataAgency {

    map_alias: MemoryMapAlias,
    ranges_to_flush: Vec<MemoryRange>,
}

impl StagingDataAgency {

    pub fn new(memory: &HaStagingMemory) -> Result<StagingDataAgency, MemoryError> {

        let agency = StagingDataAgency {
            map_alias: MemoryMapAlias {
                handle: memory.target.handle,
                status: memory.map_status.clone(),
                is_coherent: memory.target.is_coherent_memroy(),
            },
            ranges_to_flush: vec![],
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for StagingDataAgency {

    fn prepare(&mut self, device: &HaDevice) -> Result<(), MemoryError> {

        self.map_alias.map_range(device, None)?;

        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, _: usize) -> Result<MemoryWritePtr, MemoryError> {

        self.ranges_to_flush.push(MemoryRange { offset: block.memory_offset, size: block.size });

        let data_ptr = unsafe {
            self.map_alias.status.data_ptr(block.memory_offset)
        }.ok_or(MemoryError::MemoryPtrInvalidError)?;

        let writer = MemoryWritePtr::new(data_ptr, block.size);
        Ok(writer)
    }

    fn finish(&mut self, device: &HaDevice) -> Result<(), AllocatorError> {

        if !self.map_alias.is_coherent {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.map_alias.flush_ranges(device, &self.ranges_to_flush)?;
        }

        self.map_alias.unmap(device);

        Ok(())
    }
}


pub struct UploadStagingResource {

    buffers: Vec<HaBuffer>,
    src_memory: HaStagingMemory,

    src_blocks: Vec<BufferBlock>,
    dst_blocks: Vec<BufferBlock>,

    ranges_to_flush: Vec<MemoryRange>,
}

impl UploadStagingResource {

    pub fn new(device: &HaDevice, physical: &HaPhyDevice, allocate_infos: &BufferAllocateInfos) -> Result<UploadStagingResource, MemoryError> {

        let mut memory_selector = MemorySelector::init(physical, HaMemoryType::StagingMemory);

        // generate buffers
        let mut buffers = vec![];
        for buffer_desc in allocate_infos.infos.iter() {

            let buffer = buffer_desc.build(device, Staging, None)
                .or(Err(MemoryError::AllocateMemoryError))?;

            memory_selector.try(&buffer)?;
            buffers.push(buffer);
        }

        // allocate memory
        let mut src_memory = HaStagingMemory::allocate(
            device, allocate_infos.spaces.iter().sum(), &memory_selector
        )?;

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in buffers.iter().enumerate() {
            src_memory.bind_to_buffer(device, &buffer, offset)?;
            offset += allocate_infos.spaces[i];
        }

        src_memory.map_range(device, None)?;

        let resource = UploadStagingResource {
            buffers, src_memory,
            src_blocks: vec![],
            dst_blocks: vec![],
            ranges_to_flush: vec![],
        };

        Ok(resource)
    }

    pub fn append_dst_block(&mut self, to: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError> {

        let dst_block = to.clone();

        let src_block = BufferBlock {
            handle: self.buffers[repository_index].handle,
            size: dst_block.size,
            memory_offset: dst_block.memory_offset,
        };

        self.ranges_to_flush.push(MemoryRange { offset: dst_block.memory_offset, size: dst_block.size });

        let data_ptr = unsafe {
            self.src_memory.map_status.data_ptr(src_block.memory_offset)
                .ok_or(MemoryError::MemoryPtrInvalidError)?
        };

        let writer = MemoryWritePtr::new(data_ptr, src_block.size);

        self.src_blocks.push(src_block);
        self.dst_blocks.push(dst_block);

        Ok(writer)
    }

    pub fn finish_src_transfer(&mut self, device: &HaDevice) -> Result<(), AllocatorError> {

        if !self.src_memory.target.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.src_memory.flush_ranges(device, &self.ranges_to_flush)?;
        }

        self.src_memory.unmap(device);

        Ok(())
    }

    pub fn transfer(&self, device: &HaDevice) -> Result<(), AllocatorError> {

        let mut data_copyer = DataCopyer::new(device)?;
        for (src, dst) in self.src_blocks.iter().zip(self.dst_blocks.iter()) {
            data_copyer.copy_buffer_to_buffer(src, dst);
        }

        data_copyer.done()?;

        Ok(())
    }

    pub fn cleanup(&mut self, device: &HaDevice) {

        self.buffers.iter()
            .for_each(|buffer| buffer.cleanup(device));
        self.src_memory.cleanup(device);
    }
}
