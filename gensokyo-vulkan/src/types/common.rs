
use ash::vk;

use std::os::raw::{ c_float, c_char, c_void };

/// unsigned integer type commonly used in vulkan.
#[allow(non_camel_case_types)]
pub type vkuint = u32;
/// signed integer type used in vulkan.
#[allow(non_camel_case_types)]
pub type vksint = i32;
/// float type used in vulkan.
#[allow(non_camel_case_types)]
pub type vkfloat = c_float;
/// unsigned long integer type used in vulkan.
#[allow(non_camel_case_types)]
pub type vklint = u64;
/// char type used in vulkan.
#[allow(non_camel_case_types)]
pub type vkchar = c_char;
/// boolean type used in vulkan.
#[allow(non_camel_case_types)]
pub type vkbool = vk::Bool32;
/// two dimension type used in vulkan.
#[allow(non_camel_case_types)]
pub type vkDim2D = vk::Extent2D;
/// three dimension type used in vulkan.
#[allow(non_camel_case_types)]
pub type vkDim3D = vk::Extent3D;
#[allow(non_camel_case_types)]
// raw pointer type used in vulkan.
pub type vkptr = *mut c_void;
/// the number of bytes, used to measure the size of memory block(buffer, image...).
#[allow(non_camel_case_types)]
pub type vkbytes = vk::DeviceSize;

pub const VK_TRUE : vkbool = vk::TRUE;
pub const VK_FALSE: vkbool = vk::FALSE;
