
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::{ HaBuffer, BufferBlock };
use buffer::allocator::BufferAllocateInfos;
use buffer::allocator::types::Staging;

use memory::structs::{ HaMemoryType, MemoryMapStatus, MemoryRange };
use memory::target::HaMemory;
use memory::traits::{ HaMemoryAbstract, MemoryMapable };
use memory::selector::MemorySelector;
use memory::transfer::DataCopyer;
use memory::instance::{ HaBufferMemoryAbs, MemoryDataUploadable };
use memory::error::{ MemoryError, AllocatorError };

use utils::memory::MemoryWritePtr;
use types::vkbytes;


pub struct HaStagingMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaStagingMemory {

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl HaBufferMemoryAbs for HaStagingMemory {}

impl HaMemoryAbstract for HaStagingMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::StagingMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaStagingMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;
        let memory_boundary = target.size;

        let memory = HaStagingMemory {
            target,
            map_status: MemoryMapStatus::from_unmap(memory_boundary),
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMapable> {
        Some(self)
    }
}

impl HaStagingMemory {

    fn enable_map(&mut self, device: &HaDevice, is_enable: bool) -> Result<(), MemoryError> {

        // TODO: Refactor this logic.
        if is_enable {
            if self.map_status.is_range_available(None) {
                self.map_range(device, None)?;
            }
        } else {
            if self.map_status.is_range_available(None) == false {
                self.unmap(device);
            }
        }

        Ok(())
    }
}


impl MemoryDataUploadable for HaStagingMemory {

    fn prepare_data_transfer(&mut self, _: &HaPhyDevice, device: &HaDevice, _: &Option<BufferAllocateInfos>) -> Result<Option<UploadStagingResource>, MemoryError> {

        self.enable_map(device, true)?;
        Ok(None)
    }

    fn map_memory_ptr(&mut self, _: &mut Option<UploadStagingResource>, block: &BufferBlock, offset: vkbytes) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr
                .ok_or(MemoryError::MemoryPtrInvalidError)?
                .offset(offset as isize)
        };

        let writer = MemoryWritePtr::new(ptr, block.size);
        let range = MemoryRange { offset, size: block.size };

        Ok((writer, range))
    }

    fn terminate_transfer(&mut self, device: &HaDevice, _: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        if !self.target.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.flush_ranges(device, ranges_to_flush)?;
        }

        self.enable_map(device, false)?;

        Ok(())
    }
}

pub struct StagingUploader {}

impl StagingUploader {

    pub fn prepare_data_transfer(physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>)
        -> Result<Option<UploadStagingResource>, MemoryError> {

        let staging = UploadStagingResource::new(physical, device, allocate_infos)?;

        Ok(Some(staging))
    }

    pub fn map_memory_ptr(staging: &mut Option<UploadStagingResource>, block: &BufferBlock, _offset: vkbytes)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        if let Some(ref mut staging) = staging {

            let result = staging.append_dst_block(block)?;
            Ok(result)
        } else {
            Err(MemoryError::AllocateInfoMissing)
        }
    }

    pub fn terminate_transfer(device: &HaDevice, staging: &Option<UploadStagingResource>, _ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        if let Some(staging) = staging {
            staging.transfer(device)
                .or(Err(MemoryError::BufferToBufferCopyError))?
        } else {
            return Err(MemoryError::AllocateInfoMissing)
        }

        Ok(())
    }
}


pub struct UploadStagingResource {

    buffers   : Vec<HaBuffer>,
    src_memory: HaStagingMemory,

    src_blocks: Vec<BufferBlock>,
    dst_blocks: Vec<BufferBlock>,
}

impl UploadStagingResource {

    fn new(physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>) -> Result<UploadStagingResource, MemoryError> {

        if let Some(allo_infos) = allocate_infos {

            let mut memory_selector = MemorySelector::init(physical, HaMemoryType::StagingMemory);

            // generate buffers
            let mut buffers = vec![];
            for buffer_desc in allo_infos.infos.iter() {

                let buffer = buffer_desc.build(device, Staging, None)
                    .or(Err(MemoryError::AllocateMemoryError))?;

                memory_selector.try(&buffer)?;
                buffers.push(buffer);
            }

            // allocate memory
            let mut src_memory = HaStagingMemory::allocate(
                device, allo_infos.spaces.iter().sum(), &memory_selector
            )?;

            // bind buffers to memory
            let mut offset = 0;
            for (i, buffer) in buffers.iter().enumerate() {
                src_memory.bind_to_buffer(device, &buffer, offset)?;
                offset += allo_infos.spaces[i];
            }

            src_memory.prepare_data_transfer(physical, device, &None)?;

            let resource = UploadStagingResource {
                buffers,
                src_memory,
                src_blocks: vec![],
                dst_blocks: vec![],
            };

            Ok(resource)
        } else {

            Err(MemoryError::AllocateInfoMissing)
        }
    }

    fn append_dst_block(&mut self, dst: &BufferBlock) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

//        let block = dst.clone();
//
//        let src_item = BufferItem {
//            handle: self.buffers[dst.buffer_index].handle,
//            size: dst.size,
//            memory_offset: dst.memory_offset,
//        };
//
//        // get memory wirte pointer of staging buffer.
//        let (writer, range) = self.src_memory.map_memory_ptr(&mut None, &src_item, dst_item.memory_offset)?;
//
//        self.src_items.push(src_item);
//        self.dst_items.push(dst_item);
//
//        Ok((writer, range))

        unimplemented!()
    }

    pub fn finish_src_transfer(&mut self, device: &HaDevice, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        self.src_memory.terminate_transfer(device, &None, ranges_to_flush)
    }

    fn transfer(&self, device: &HaDevice) -> Result<(), AllocatorError> {

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
