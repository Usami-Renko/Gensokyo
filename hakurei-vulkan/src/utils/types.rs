
use ash::vk;
use utils::format::VKFormat;

#[allow(non_camel_case_types)]
pub type vkint = vk::uint32_t; // unsigned integer
#[allow(non_camel_case_types)]
pub type vklint = vk::uint64_t; // unsigned long integer
#[allow(non_camel_case_types)]
pub type vksint = vk::int32_t; // signed integer
#[allow(non_camel_case_types)]
pub type vkfloat = vk::c_float; // float
#[allow(non_camel_case_types)]
pub type vkchar = vk::c_char; // char
#[allow(non_camel_case_types)]
pub type vkMemorySize = vk::DeviceSize; // memory size
#[allow(non_camel_case_types)]
pub type vkformat = VKFormat; // format
#[allow(non_camel_case_types)]
pub type vkDimension2D = vk::Extent2D;
#[allow(non_camel_case_types)]
pub type vkDimension3D = vk::Extent3D;
pub const VK_TRUE : vkint = vk::VK_TRUE;
pub const VK_FALSE: vkint = vk::VK_FALSE;

pub type MemPtr = *mut ::ash::vk::c_void;
