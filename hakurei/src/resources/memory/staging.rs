
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::{ HaBuffer, BufferItem, BufferStorageType};
use vk::resources::memory::{ HaMemory, HaMemoryType, HaMemoryAbstract, MemorySelector };
use vk::resources::memory::{ MemoryMapable, MemoryMapStatus, MemoryRange };
use vk::resources::transfer::DataCopyer;
use vk::resources::error::{ AllocatorError, MemoryError };
use vk::utils::memory::MemoryWritePtr;
use vk::utils::types::vkMemorySize;

use resources::memory::traits::{ HaMemoryEntityAbs, MemoryDataUploadable };
use resources::allocator::buffer::BufferAllocateInfos;

pub struct HaStagingMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaStagingMemory {}

impl HaMemoryEntityAbs for HaStagingMemory {}

impl HaMemoryAbstract for HaStagingMemory {

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::StagingMemory
    }

    fn allocate(device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<HaStagingMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;

        let memory = HaStagingMemory {
            target,
            map_status: MemoryMapStatus::from_unmap(),
        };
        Ok(memory)
    }
}

impl HaStagingMemory {

    fn enable_map(&mut self, device: &HaDevice, is_enable: bool) -> Result<(), MemoryError> {

        if is_enable {
            if !self.map_status.is_map {
                let ptr = self.map_range(device, None)?;
                self.map_status.set_map(ptr);
            }
        } else {
            if self.map_status.is_map {
                self.unmap(device);
                self.map_status.invaild_map();
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

    fn map_memory_ptr(&mut self, _: &mut Option<UploadStagingResource>, item: &BufferItem, offset: vkMemorySize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr.offset(offset as isize)
        };

        let writer = MemoryWritePtr::new(ptr, item.size);
        let range = MemoryRange { offset, size: item.size };

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

    pub fn map_memory_ptr(staging: &mut Option<UploadStagingResource>, item: &BufferItem, _offset: vkMemorySize)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        if let Some(ref mut staging) = staging {

            let result = staging.append_dst_item(item)?;
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

    src_items: Vec<BufferItem>,
    dst_items: Vec<BufferItem>,
}

impl UploadStagingResource {

    fn new(physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>) -> Result<UploadStagingResource, MemoryError> {

        if let Some(allo_infos) = allocate_infos {

            let mut memory_selector = MemorySelector::init(physical, HaMemoryType::StagingMemory);

            // generate buffers
            let mut buffers = vec![];
            for config in allo_infos.infos.iter() {
                let staging_config = config.to_staging_info().unwrap();
                let buffer = staging_config.build(device, None, BufferStorageType::Staging)
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
                src_items: vec![],
                dst_items: vec![],
            };

            Ok(resource)
        } else {

            Err(MemoryError::AllocateInfoMissing)
        }
    }

    fn append_dst_item(&mut self, dst: &BufferItem) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let dst_item = dst.clone();

        // FIXME: Fix this handle.
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
        for (src, dst) in self.src_items.iter().zip(self.dst_items.iter()) {
            data_copyer.copy_buffer_to_buffer(src, dst);
        }

        data_copyer.done()?;

        Ok(())
    }

    pub fn cleanup(&self, device: &HaDevice) {

        self.buffers.iter()
            .for_each(|buffer| buffer.cleanup(device));
        self.src_memory.cleanup(device);
    }
}
