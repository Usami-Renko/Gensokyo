
use ash::vk;

use crate::core::GsDevice;

use crate::buffer::{ GsBuffer, BufferBlock, BufferCopiable };
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::target::GsMemory;
use crate::memory::types::GsMemoryType;
use crate::memory::utils::{ MemoryMapStatus, MemoryRange, MemoryMapAlias, MemoryWritePtr };
use crate::memory::traits::{ GsMemoryAbstract, MemoryMappable };
use crate::memory::filter::MemoryFilter;
use crate::memory::transfer::DataCopyer;
use crate::memory::instance::BufferMemoryAbs;
use crate::memory::transfer::MemoryDataDelegate;

use crate::error::{ VkResult, VkError };
use crate::utils::phantom::Staging;
use crate::types::vkbytes;


pub struct GsStagingMemory {

    target: GsMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMappable for GsStagingMemory {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.target.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl GsMemoryAbstract for GsStagingMemory {

    fn memory_type(&self) -> GsMemoryType {
        GsMemoryType::StagingMemory
    }

    fn target(&self) -> &GsMemory {
        &self.target
    }

    fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsStagingMemory> {

        let target = GsMemory::allocate(device, size, filter)?;
        let map_status = MemoryMapStatus::from_unmap();

        let memory = GsStagingMemory {
            target, map_status,
        };
        Ok(memory)
    }

    fn as_mut_mappable(&mut self) -> Option<&mut MemoryMappable> {
        Some(self)
    }
}

impl BufferMemoryAbs for GsStagingMemory {

    fn to_upload_agency(&self, _: &GsDevice, _: &BufferAllocateInfos) -> VkResult<Box<dyn MemoryDataDelegate>> {

        let agency = StagingDataAgency::new(self)?;
        Ok(Box::new(agency))
    }

    fn to_update_agency(&self) -> VkResult<Box<dyn MemoryDataDelegate>> {
        /// Staging memory is unable to update directly.
        unreachable!()
    }
}

pub struct StagingDataAgency {

    map_alias: MemoryMapAlias,
    ranges_to_flush: Vec<MemoryRange>,
}

impl StagingDataAgency {

    pub fn new(memory: &GsStagingMemory) -> VkResult<StagingDataAgency> {

        let agency = StagingDataAgency {
            map_alias: MemoryMapAlias {
                handle: memory.target.handle,
                status: memory.map_status.clone(),
                is_coherent: memory.target.is_coherent_memory(),
            },
            ranges_to_flush: vec![],
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for StagingDataAgency {

    fn prepare(&mut self, device: &GsDevice) -> VkResult<()> {

        self.map_alias.map_range(device, None)?;

        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, _: usize) -> VkResult<MemoryWritePtr> {

        self.ranges_to_flush.push(MemoryRange { offset: block.memory_offset, size: block.size });

        let data_ptr = unsafe {
            self.map_alias.status.data_ptr(block.memory_offset)
        }.ok_or(VkError::device("Failed to get mapped memory pointer."))?;

        let writer = MemoryWritePtr::new(data_ptr, block.size);
        Ok(writer)
    }

    fn finish(&mut self, device: &GsDevice) -> VkResult<()> {

        if !self.map_alias.is_coherent {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.map_alias.flush_ranges(device, &self.ranges_to_flush)?;
        }

        self.map_alias.unmap(device);

        Ok(())
    }
}


pub struct UploadStagingResource {

    buffers: Vec<GsBuffer>,
    src_memory: GsStagingMemory,

    src_blocks: Vec<BufferBlock>,
    dst_blocks: Vec<BufferBlock>,

    ranges_to_flush: Vec<MemoryRange>,
}

impl UploadStagingResource {

    pub fn new(device: &GsDevice, allocate_infos: &BufferAllocateInfos) -> VkResult<UploadStagingResource> {

        let mut memory_filter = MemoryFilter::new(device, GsMemoryType::StagingMemory);

        // generate buffers
        let mut buffers = vec![];
        for buffer_ci in allocate_infos.cis.iter() {

            let buffer = buffer_ci.build(device, Staging)?;
            memory_filter.filter(&buffer)?;
            buffers.push(buffer);
        }

        // allocate memory
        let mut src_memory = GsStagingMemory::allocate(
            device, allocate_infos.spaces.iter().sum(), &memory_filter
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

    pub fn append_dst_block(&mut self, to: &BufferBlock, repository_index: usize) -> VkResult<MemoryWritePtr> {

        let dst_block = to.clone();

        let src_block = BufferBlock {
            handle: self.buffers[repository_index].handle,
            size: dst_block.size,
            memory_offset: dst_block.memory_offset,
        };

        self.ranges_to_flush.push(MemoryRange { offset: dst_block.memory_offset, size: dst_block.size });

        let data_ptr = unsafe {
            self.src_memory.map_status.data_ptr(src_block.memory_offset)
                .ok_or(VkError::device("Failed to get mapped memory pointer."))?
        };

        let writer = MemoryWritePtr::new(data_ptr, src_block.size);

        self.src_blocks.push(src_block);
        self.dst_blocks.push(dst_block);

        Ok(writer)
    }

    pub fn finish_src_transfer(&mut self, device: &GsDevice) -> VkResult<()> {

        if !self.src_memory.target.is_coherent_memory() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.src_memory.flush_ranges(device, &self.ranges_to_flush)?;
        }

        self.src_memory.unmap(device);

        Ok(())
    }

    pub fn transfer(&self, device: &GsDevice) -> VkResult<()> {

        let mut data_copyer = DataCopyer::new(device)?;
        for (src, dst) in self.src_blocks.iter().zip(self.dst_blocks.iter()) {
            data_copyer.copy_buffer_to_buffer(src.full_copy(), dst.full_copy());
        }

        data_copyer.done()?;

        Ok(())
    }

    pub fn discard(&mut self, device: &GsDevice) {

        self.buffers.iter()
            .for_each(|buffer| buffer.discard(device));
        self.src_memory.discard(device);
    }
}
