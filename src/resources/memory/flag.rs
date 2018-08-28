
use ash::vk;

use utility::marker::VulkanFlags;

pub enum MemoryPropertyFlag {
    /// DeviceLocalBit specifies that memory allocated with this type is the most efficient for device access.
    ///
    /// This property will be set if and only if the memory type belongs to a heap with the VK_MEMORY_HEAP_DEVICE_LOCAL_BIT set.
    DeviceLocalBit,
    /// HostVisibleBit specifies that memory allocated with this type can be mapped for host access using vkMapMemory.
    HostVisibleBit,
    /// HostCoherentBit specifies that the host cache management commands vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges are not needed to flush host writes to the device or make device writes visible to the host, respectively.
    HostCoherentBit,
    /// HostCachedBit specifies that memory allocated with this type is cached on the host.
    ///
    /// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent.
    HostCachedBit,
    /// LazilyAllocatedBit specifies that the memory type only allows device access to the memory.
    ///
    /// Memory types must not have both VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT and VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT set.
    ///
    /// Additionally, the object’s backing memory may be provided by the implementation lazily as specified in Lazily Allocated Memory.
    LazilyAllocatedBit
}

impl VulkanFlags for [MemoryPropertyFlag] {
    type FlagType = vk::MemoryPropertyFlags;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::MemoryPropertyFlags::empty(), |acc, flag| {
            match *flag {
                | MemoryPropertyFlag::DeviceLocalBit     => acc | vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
                | MemoryPropertyFlag::HostVisibleBit     => acc | vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
                | MemoryPropertyFlag::HostCoherentBit    => acc | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
                | MemoryPropertyFlag::HostCachedBit      => acc | vk::MEMORY_PROPERTY_HOST_CACHED_BIT,
                | MemoryPropertyFlag::LazilyAllocatedBit => acc | vk::MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT,
            }
        })
    }
}
