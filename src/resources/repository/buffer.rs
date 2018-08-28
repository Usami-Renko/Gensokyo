
use ash::vk;

use core::device::HaLogicalDevice;

use resources::buffer::HaBuffer;
use resources::memory::HaDeviceMemory;
use resources::error::AllocatorError;
use resources::memory::HaMemoryAbstract;

pub struct HaBufferRepository {

    buffers: Vec<HaBuffer>,
    memory : Option<HaDeviceMemory>,

    /// The size of each buffer occupy.
    spaces : Vec<vk::DeviceSize>,
    /// The offset of each buffer in meomory.
    offsets: Vec<vk::DeviceSize>,
}

pub struct BufferBindingInfos {

    pub(crate) handles: Vec<vk::Buffer>,
    pub(crate) offsets: Vec<vk::DeviceSize>,
}


impl HaBufferRepository {

    pub fn empty() -> HaBufferRepository {
        HaBufferRepository {
            buffers: vec![],
            memory : None,

            spaces : vec![],
            offsets: vec![],
        }
    }

    pub fn store(buffers: Vec<HaBuffer>, memory: HaDeviceMemory, spaces: Vec<vk::DeviceSize>) -> HaBufferRepository {

        let mut current: vk::DeviceSize = 0;
        let mut offsets = vec![];
        for space in spaces.iter() {
            offsets.push(current);
            current += space;
        }

        HaBufferRepository {
            buffers,
            memory: Some(memory),

            spaces,
            offsets,
        }
    }

    pub fn tranfer_data<D: Copy>(&self, device: &HaLogicalDevice, data: &Vec<D>, buffer_index: usize) -> Result<(), AllocatorError> {

        let memory = self.memory.as_ref().ok_or(AllocatorError::MemoryNotYetAllocated)?;

        let data_size = self.spaces[buffer_index];
        let data_ptr = memory.map(device, self.offsets[buffer_index], data_size)
            .map_err(|e| AllocatorError::Memory(e))?;
        self.buffers[buffer_index].copy_data(data_ptr, data_size, data);
        memory.unmap(device);

        Ok(())
    }

    pub fn binding_infos(&self) -> BufferBindingInfos {

        let handles: Vec<vk::Buffer> = self.buffers.iter().map(|b| b.handle).collect();
        let offsets = self.offsets.clone();

        BufferBindingInfos {
            handles,
            offsets,
        }
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        for buffer in self.buffers.iter() {
            buffer.cleanup(device);
        }

        if let Some(ref memory) = self.memory {
            memory.cleanup(device);
        }
    }
}
