

use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::{ V1_0, InstanceV1_0, DeviceV1_0 };

use core::instance::Instance;
use core::physical::PhysicalDevice;
use core::error::DeviceError;

use utility::cast;
use constant::VERBOSE;

use std::collections::HashSet;
use std::ptr;
use std::os::raw::c_char;

pub struct LogicalDevice {

    handle: ash::Device<V1_0>,
//    queues: Vec<QueueInfo>,
//
//    graphics_queue_index: usize,
//    present_queue_index:  usize,
}

impl LogicalDevice {

    pub fn new(instance: &Instance, physical: &PhysicalDevice) -> Result<LogicalDevice, DeviceError> {

        let mut unique_queue_family_indices = HashSet::new();
        unique_queue_family_indices.insert(physical.families.family_indices.graphics_index);
        unique_queue_family_indices.insert(physical.families.family_indices.present_index);

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = vec![];

        for &family_index in unique_queue_family_indices.iter() {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type             : vk::StructureType::DeviceQueueCreateInfo,
                p_next             : ptr::null(),
                // flags is reserved for future use in API version 1.0.82
                flags              : vk::DeviceQueueCreateFlags::empty(),
                queue_family_index : family_index,
                queue_count        : queue_priorities.len() as uint32_t,
                p_queue_priorities : queue_priorities.as_ptr(),
            };
            queue_create_infos.push(queue_create_info);
        }

        let enable_features = physical.features.get_enable_features();
        let enable_layer_names = cast::to_array_ptr(&instance.enable_layer_names);

        let enable_extension_names: Vec<*const c_char> = cast::to_array_ptr(&physical.extensions.enables);

        let device_create_info = vk::DeviceCreateInfo {
            s_type                     : vk::StructureType::DeviceCreateInfo,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.0.82
            flags                      : vk::DeviceCreateFlags::empty(),
            queue_create_info_count    : queue_create_infos.len() as uint32_t,
            p_queue_create_infos       : queue_create_infos.as_ptr(),
            enabled_layer_count        : enable_layer_names.len() as uint32_t,
            pp_enabled_layer_names     : enable_layer_names.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as uint32_t,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
            p_enabled_features         : &enable_features,
        };

        let handle = unsafe {
            instance.handle.create_device(physical.handle, &device_create_info, None)
                .or(Err(DeviceError::DeviceCreationError))?
        };

        let device = LogicalDevice {
            handle,
        };

        Ok(device)
    }

    pub fn cleanup(&self) {

        unsafe {
            self.handle.destroy_device(None);

            if VERBOSE {
                println!("[Info] Logical Device had been destroy.");
            }
        }
    }
}
