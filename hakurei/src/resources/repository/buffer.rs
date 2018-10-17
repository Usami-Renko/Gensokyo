
use ash::vk;

use core::device::{ HaDevice, HaLogicalDevice };
use core::physical::HaPhyDevice;

use resources::allocator::BufferAllocateInfos;
use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::command::CommandBufferUsageFlag;
use resources::command::{ CmdVertexBindingInfos, CmdIndexBindingInfo };
use resources::memory::{ HaMemoryAbstract, HaMemoryType };
use resources::repository::{ BufferDataUploader, BufferDataUpdater };
use resources::error::{ AllocatorError, MemoryError };

use utility::memory::spaces_to_offsets;

pub struct HaBufferRepository {

    device  : Option<HaDevice>,
    physical: Option<HaPhyDevice>,
    buffers : Vec<HaBuffer>,
    memory  : Option<Box<HaMemoryAbstract>>,

    /// The offset of each buffer in memory.
    offsets: Vec<vk::DeviceSize>,

    allocate_infos: Option<BufferAllocateInfos>,
}

impl HaBufferRepository {

    pub fn empty() -> HaBufferRepository {
        HaBufferRepository {

            device  : None,
            physical: None,
            buffers : vec![],
            memory  : None,

            offsets: vec![],
            allocate_infos: None,
        }
    }

    pub(crate) fn store(device: &HaDevice, physical: &HaPhyDevice, buffers: Vec<HaBuffer>, memory: Box<HaMemoryAbstract>, allocate_infos: BufferAllocateInfos) -> HaBufferRepository {

        let offsets = spaces_to_offsets(&allocate_infos.spaces);

        HaBufferRepository {
            device  : Some(device.clone()),
            physical: Some(physical.clone()),
            memory  : Some(memory),

            buffers, offsets,
            allocate_infos: Some(allocate_infos),
        }
    }

    pub fn data_uploader(&mut self) -> Result<BufferDataUploader, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            BufferDataUploader::new(&self.physical.as_ref().unwrap(), &self.device.as_ref().unwrap(), memory, &self.offsets, &self.allocate_infos)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn data_updater(&mut self) -> Result<BufferDataUpdater, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            let updater = match memory.memory_type() {
                | HaMemoryType::HostMemory
                | HaMemoryType::StagingMemory => {
                    BufferDataUpdater::new(&self.device.as_ref().unwrap(), memory, &self.offsets)
                },
                | HaMemoryType::CachedMemory
                | HaMemoryType::DeviceMemory => {
                    return Err(AllocatorError::Memory(MemoryError::MemoryUnableToUpdate))
                }
            };

            Ok(updater)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn vertex_binding_infos(&self, items: &[&BufferSubItem]) -> CmdVertexBindingInfos {

        let mut handles = vec![];
        let mut offsets  = vec![];
        for item in items.iter() {
            handles.push(item.handle);
            offsets.push(item.offset);
        }

        CmdVertexBindingInfos {
            handles,
            offsets,
        }
    }

    pub fn index_binding_info(&self, item: &BufferSubItem) -> CmdIndexBindingInfo {

        CmdIndexBindingInfo {
            handle: item.handle,
            offset: item.offset,
        }
    }

    pub fn cleanup(&mut self) {
        for buffer in self.buffers.iter() {
            buffer.cleanup(&self.device.as_ref().unwrap());
        }

        if let Some(ref memory) = self.memory {
            memory.cleanup(&self.device.as_ref().unwrap());
        }

        self.buffers.clear();
        self.offsets.clear();
    }
}

impl HaBufferRepository {

    pub fn copy_buffers_to_buffers(device: &HaDevice, from_items: &[BufferSubItem], to_items: &[BufferSubItem])
        -> Result<(), AllocatorError> {

        let mut transfer = HaLogicalDevice::transfer(device);
        {
            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record();
            recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            for (from, to) in from_items.iter().zip(to_items.iter()) {
                // TODO: Only support one region.
                let copy_region = [
                    vk::BufferCopy {
                        src_offset: from.offset,
                        dst_offset: to.offset,
                        size      : to.size,
                    },
                ];

                recorder.copy_buffer(from.handle, to.handle, &copy_region);
            }

            recorder.end_record()?;
        }

        transfer.excute()?;

        Ok(())
    }
}
