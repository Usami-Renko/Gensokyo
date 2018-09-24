
use ash::vk;

use utility::marker::{ VulkanFlags, VulkanEnum };

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
    LazilyAllocatedBit,
    // /// ProtectedBit specifies that the memory type only allows device access to the memory, and allows protected queue operations to access the memory.
    // ///
    // /// Memory types must not have VK_MEMORY_PROPERTY_PROTECTED_BIT set and any of VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT set, or VK_MEMORY_PROPERTY_HOST_COHERENT_BIT set, or VK_MEMORY_PROPERTY_HOST_CACHED_BIT set.
     // ProtectedBit,
}

impl VulkanFlags for [MemoryPropertyFlag] {
    type FlagType = vk::MemoryPropertyFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::MemoryPropertyFlags::empty(), |acc, flag| {
            match *flag {
                | MemoryPropertyFlag::DeviceLocalBit     => acc | vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
                | MemoryPropertyFlag::HostVisibleBit     => acc | vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
                | MemoryPropertyFlag::HostCoherentBit    => acc | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
                | MemoryPropertyFlag::HostCachedBit      => acc | vk::MEMORY_PROPERTY_HOST_CACHED_BIT,
                | MemoryPropertyFlag::LazilyAllocatedBit => acc | vk::MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT,
                // | MemoryPropertyFlag::ProtectedBit       => acc | vk::MEMORY_PROPERTY_PROTECTED_BIT,
            }
        })
    }
}

impl VulkanEnum for MemoryPropertyFlag {
    type EnumType = vk::MemoryPropertyFlags;

    fn value(&self) -> Self::EnumType {
        match self {
            | MemoryPropertyFlag::DeviceLocalBit     => vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
            | MemoryPropertyFlag::HostVisibleBit     => vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
            | MemoryPropertyFlag::HostCoherentBit    => vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
            | MemoryPropertyFlag::HostCachedBit      => vk::MEMORY_PROPERTY_HOST_CACHED_BIT,
            | MemoryPropertyFlag::LazilyAllocatedBit => vk::MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT,
            // | MemoryPropertyFlag::ProtectedBit       => vk::MEMORY_PROPERTY_PROTECTED_BIT,
        }
    }

}
