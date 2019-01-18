
use ash::vk;
use ash::version::InstanceV1_0;

use crate::core::instance::GsInstance;
use crate::core::physical::GsPhysicalDevice;
use crate::core::device::device::{ DeviceConfig, GsLogicalDevice };
use crate::core::device::enums::{ PrefabQueuePriority, DeviceQueueIndex, QueueRequestStrategy };
use crate::core::device::queue::{ GsGraphicsQueue, GsPresentQueue, GsTransferQueue };
use crate::core::device::queue::{ GsQueue, QueueUsage };
use crate::core::device::queue::{ QueueRequester, SFSQ, SFMQ };
use crate::error::{ VkResult, VkError };

use crate::utils::cast;
use crate::VERBOSE;

use std::ptr;

pub struct LogicalDeviceBuilder<'a> {

    instance: &'a GsInstance,
    physical: &'a GsPhysicalDevice,

    queue_request: Box<dyn QueueRequester>,
    config: DeviceConfig,
}

impl<'a> LogicalDeviceBuilder<'a> {

    pub fn init(instance: &'a GsInstance, physical: &'a GsPhysicalDevice, config: &DeviceConfig) -> LogicalDeviceBuilder<'a> {

        let queue_request = match config.queue_request_strategy {
            | QueueRequestStrategy::SingleFamilySingleQueue => {
                Box::new(SFSQ::new(PrefabQueuePriority::Highest)) as Box<dyn QueueRequester>
            },
            | QueueRequestStrategy::SingleFamilyMultiQueues => {
                Box::new(SFMQ::new()) as Box<dyn QueueRequester>
            },
        };

        LogicalDeviceBuilder {
            instance, physical, queue_request,
            config: config.clone(),
        }
    }

    pub fn add_queue(&mut self, usage: QueueUsage, priority: Option<PrefabQueuePriority>) -> DeviceQueueIndex {

        self.queue_request.request_queue(usage, priority.unwrap_or(PrefabQueuePriority::Highest))
    }

    pub fn build(&mut self) -> VkResult<(GsLogicalDevice, Vec<GsQueue>)> {

        // Configure queue.
        let _ = self.queue_request.request_queue(QueueUsage::Graphics, PrefabQueuePriority::Highest);
        let _ = self.queue_request.request_queue(QueueUsage::Present, PrefabQueuePriority::Highest);
        let _ = self.queue_request.request_queue(QueueUsage::Transfer, PrefabQueuePriority::Highest);

        self.queue_request.inspect_queue_availability(&self.physical)?;
        let queue_infos = self.queue_request.to_queue_infos();

        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = queue_infos.iter()
            .map(|queue_info| {
                vk::DeviceQueueCreateInfo {
                    s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                    p_next: ptr::null(),
                    // flags is reserved for future use in API version 1.1.82.
                    flags : vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index: queue_info.family_index as _,
                    queue_count       : queue_info.priorities.len() as _,
                    p_queue_priorities: queue_info.priorities.as_ptr(),
                }
            }).collect();

        // Configure device features, layers and extensions.
        let enable_features = self.physical.features.enable_features();
        let enable_layer_names = cast::cstrings2ptrs(&self.instance.enable_layer_names);
        let enable_extension_names = cast::cstrings2ptrs(self.physical.extensions.borrow_enable_extensions());

        // Create the logical device.
        let device_create_info = vk::DeviceCreateInfo {
            s_type                     : vk::StructureType::DEVICE_CREATE_INFO,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags                      : vk::DeviceCreateFlags::empty(),
            queue_create_info_count    : queue_create_infos.len() as _,
            p_queue_create_infos       : queue_create_infos.as_ptr(),
            enabled_layer_count        : enable_layer_names.len() as _,
            pp_enabled_layer_names     : enable_layer_names.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as _,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
            p_enabled_features         : enable_features,
        };

        let handle = unsafe {
            self.instance.handle.create_device(self.physical.handle, &device_create_info, None)
                .or(Err(VkError::create("Logical Device")))?
        };

        if VERBOSE {
            self.queue_request.print_message();
        }

        let mut queues = self.queue_request.collect_queues(&handle);
        let transfer_queue = GsTransferQueue::new(&handle, queues.pop().unwrap(), &self.config)?;
        let present_queue = GsPresentQueue::new(queues.pop().unwrap());
        let graphics_queue = GsGraphicsQueue::new(queues.pop().unwrap());

        let device = GsLogicalDevice::new(handle, graphics_queue, present_queue, transfer_queue);
        Ok((device, queues))
    }
}
