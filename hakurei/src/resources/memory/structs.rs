
use ash::vk;

use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::buffer::{ HaBuffer, BufferSubItem, BufferGenerator };
use resources::memory::{ HaMemoryAbstract, MemoryDataUploadable, MemoryPropertyFlag, MemPtr };
use resources::memory::HaStagingMemory;
use resources::allocator::BufferAllocateInfos;
use resources::repository::HaBufferRepository;
use resources::error::{ MemoryError, AllocatorError };

use utility::memory::{ MemoryWritePtr, spaces_to_offsets };
use utility::marker::VulkanFlags;

use std::ptr;

pub struct MemoryRange {

    pub offset: vk::DeviceSize,
    pub size  : vk::DeviceSize,
}

pub enum HaMemoryType {
    HostMemory,
    CachedMemory,
    DeviceMemory,
    StagingMemory,
}

impl HaMemoryType {

    pub fn property_flags(&self) -> vk::MemoryPropertyFlags {
        match self {
            | HaMemoryType::HostMemory => {
                [
                    MemoryPropertyFlag::HostVisibleBit,
                    MemoryPropertyFlag::HostCoherentBit,
                ].flags()
            },
            | HaMemoryType::CachedMemory => {
                [
                    MemoryPropertyFlag::HostCachedBit,
                ].flags()
            },
            | HaMemoryType::DeviceMemory => {
                [
                    MemoryPropertyFlag::DeviceLocalBit,
                ].flags()
            },
            | HaMemoryType::StagingMemory => {
                [
                    MemoryPropertyFlag::HostVisibleBit,
                    MemoryPropertyFlag::HostCoherentBit,
                ].flags()
            },
        }
    }
}


pub struct MemoryMapStatus {

    pub data_ptr: MemPtr,
    pub is_map  : bool,
}

impl MemoryMapStatus {

    pub fn from_unmap() -> MemoryMapStatus {
        MemoryMapStatus {
            data_ptr: ptr::null_mut(),
            is_map  : false,
        }
    }

    pub fn set_map(&mut self, ptr: MemPtr) {
        self.is_map = true;
        self.data_ptr = ptr;
    }

    pub fn invaild_map(&mut self) {
        self.data_ptr = ptr::null_mut();
        self.is_map = false;
    }
}


pub(crate) struct UploadStagingResource {

    buffers   : Vec<HaBuffer>,
    src_memory: HaStagingMemory,

    src_items: Vec<BufferSubItem>,
    dst_items: Vec<BufferSubItem>,
    src_offsets: Vec<vk::DeviceSize>,
}

impl UploadStagingResource {

    pub fn new(physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>) -> Result<UploadStagingResource, MemoryError> {

        if let Some(infos) = allocate_infos {

            let mut memory_selector = MemorySelector::init(physical);

            // generate buffers
            let mut buffers = vec![];
            for config in infos.configs.iter() {
                let staging_config = config.to_staging_config();
                let buffer = staging_config.generate(device, None)
                    .or(Err(MemoryError::AllocateMemoryError))?;

                memory_selector.try(buffer.requirement.memory_type_bits, HaStagingMemory::default_flag())?;
                buffers.push(buffer);
            }

            // allocate memory
            let memory_index = memory_selector.optimal_memory()?;
            let mem_type = physical.memory.memory_type(memory_index);

            let mut src_memory = HaStagingMemory::allocate(
                device, infos.spaces.iter().sum(), memory_index, Some(mem_type)
            )?;

            // bind buffers to memory
            let mut offset = 0;
            for (i, buffer) in buffers.iter().enumerate() {
                src_memory.bind_to_buffer(device, &buffer, offset)?;
                offset += infos.spaces[i];
            }

            src_memory.prepare_data_transfer(physical, device, &None)?;

            let resource = UploadStagingResource {
                buffers,
                src_memory, src_items: vec![], dst_items: vec![],
                src_offsets: spaces_to_offsets(&infos.spaces),
            };

            Ok(resource)
        } else {

            Err(MemoryError::AllocateInfoMissing)
        }
    }

    pub fn append_dst_item(&mut self, dst: &BufferSubItem) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let dst_item = dst.clone();
        let src_item = BufferSubItem {
            handle: self.buffers[dst.buffer_index].handle,
            buffer_index: dst.buffer_index,
            size  : dst.size,
            offset: dst.offset,
        };

        // get memory wirte pointer of staging buffer.
        let offset = self.src_offsets[dst.buffer_index] + dst.offset;
        let (writer, range) = self.src_memory.map_memory_ptr(&mut None, &src_item, offset)?;

        self.src_items.push(src_item);
        self.dst_items.push(dst_item);

        Ok((writer, range))
    }

    pub fn finish_src_transfer(&mut self, device: &HaDevice, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        self.src_memory.terminate_transfer(device, &None, ranges_to_flush)
    }

    pub fn transfer(&self, device: &HaDevice) -> Result<(), AllocatorError> {

        HaBufferRepository::copy_buffers_to_buffers(device, &self.src_items, &self.dst_items)?;

        Ok(())
    }

    pub fn cleanup(&self, device: &HaDevice) {

        self.buffers.iter().for_each(|buffer| buffer.cleanup(device));
        self.src_memory.cleanup(device);
    }
}