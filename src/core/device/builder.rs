
use ash::vk;
use ash::version::{ InstanceV1_0, DeviceV1_0 };
use ash::vk::uint32_t;

use core::instance::HaInstance;
use core::physical::HaPhysicalDevice;
use core::device::HaLogicalDevice;
use core::device::DeviceQueueIdentifier;
use core::device::queue::QueueUsage;
use core::device::queue::{ HaQueue, QueueInfoTmp };
use core::error::LogicalDeviceError;

use utility::cast;
use constant::VERBOSE;

use std::ptr;
use std::os::raw::c_char;

// TODO: The generation step hasn't been well test.

// FIXME: Remove #[allow(dead_code)] after being able to configure priority.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrefabQueuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest
}

impl PrefabQueuePriority {

    fn value(&self) -> f32 {
        match *self {
            | PrefabQueuePriority::Highest => 1.0,
            | PrefabQueuePriority::High    => 0.8,
            | PrefabQueuePriority::Medium  => 0.6,
            | PrefabQueuePriority::Low     => 0.4,
            | PrefabQueuePriority::Lowest  => 0.2,
        }
    }
}

pub struct LogicalDeviceBuilder<'a, 'b> {

    /// the queue family index of each queue.
    family_indices      : Vec<uint32_t>,
    /// the priority value of each queue.
    priorities          : Vec<f32>,
    /// the usage of each queue.
    usages              : Vec<QueueUsage>,
    /// the inner queue index in queue family of each queue.
    queue_indices       : Vec<uint32_t>,
    /// the count of queues in each queue family.
    family_queue_counts : Vec<uint32_t>,
    /// current total queue count
    total_queue_count   : usize,

    instance            : &'a HaInstance,
    physical_device     : &'b HaPhysicalDevice,
}

impl<'a, 'b> LogicalDeviceBuilder<'a, 'b> {

    pub fn init(instance: &'a HaInstance, physical: &'b HaPhysicalDevice) -> LogicalDeviceBuilder<'a, 'b> {

        let queue_family_count = physical.families.queue_families_count();

        LogicalDeviceBuilder {
            family_indices      : vec![],
            priorities          : vec![],
            usages              : vec![],
            queue_indices       : vec![],
            family_queue_counts : vec![0; queue_family_count],
            total_queue_count   : 0,

            instance,
            physical_device     : physical,
        }
    }

    #[allow(dead_code)]
    pub fn add_queue(&mut self, usage: QueueUsage, priority: PrefabQueuePriority) -> DeviceQueueIdentifier {
        let queue_index = self.setup_queue(usage, priority);
        DeviceQueueIdentifier::Custom(queue_index)
    }

    fn setup_queue(&mut self, usage: QueueUsage, priority: PrefabQueuePriority) -> usize {

        // TODO: Add more usage configuration
        let family_index = match usage {
            | QueueUsage::Graphics => self.physical_device.families.family_indices.graphics_index,
            | QueueUsage::Present  => self.physical_device.families.family_indices.present_index,
            | QueueUsage::Transfer => self.physical_device.families.family_indices.transfer_index,
        };

        self.family_indices.push(family_index);
        self.priorities.push(priority.value());
        self.usages.push(usage);
        self.queue_indices.push(self.family_queue_counts[family_index as usize]);
        self.family_queue_counts[family_index as usize] += 1;
        self.total_queue_count += 1;

        self.queue_indices.len() - 1
    }

    fn generate_queue_create_info(&self) -> (Vec<vk::DeviceQueueCreateInfo>, Vec<QueueInfoTmp>) {

        let mut queue_create_infos = vec![];

        for (family_index, &queue_count) in self.family_queue_counts.iter().enumerate() {
            if queue_count == 0 { continue }

            let mut queue_priorities = vec![];

            for (index, &priority) in self.priorities.iter().enumerate() {
                if self.family_indices[index] == family_index as uint32_t {
                    queue_priorities.push(priority);
                }
            }

            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type             : vk::StructureType::DeviceQueueCreateInfo,
                p_next             : ptr::null(),
                // flags is reserved for future use in API version 1.0.82
                flags              : vk::DeviceQueueCreateFlags::empty(),
                queue_family_index : family_index as uint32_t,
                queue_count        : queue_priorities.len() as uint32_t,
                p_queue_priorities : queue_priorities.as_ptr(),
            };

            queue_create_infos.push(queue_create_info);
        }


        let mut queue_info_tmps = vec![];

        for index in 0..self.total_queue_count {

            let tmp_queue = QueueInfoTmp {
                usage        : self.usages[index],
                priority     : self.priorities[index],
                family_index : self.family_indices[index],
                queue_index  : self.queue_indices[index],
            };
            queue_info_tmps.push(tmp_queue);
        }

        if VERBOSE {
            self.print_queue_infos(&queue_create_infos, &queue_info_tmps);
        }

        (queue_create_infos, queue_info_tmps)
    }

    pub fn build(&mut self) -> Result<HaLogicalDevice, LogicalDeviceError> {

        // TODO: Add configuration for priorities.
        let _ = self.setup_queue(QueueUsage::Graphics, PrefabQueuePriority::Highest);
        let _ = self.setup_queue(QueueUsage::Present,  PrefabQueuePriority::Highest);
        let _ = self.setup_queue(QueueUsage::Transfer, PrefabQueuePriority::Highest);


        let (queue_create_infos, queue_info_tmps) = self.generate_queue_create_info();

        let enable_features = self.physical_device.features.get_enable_features();
        let enable_layer_names = cast::to_array_ptr(&self.instance.enable_layer_names);
        let enable_extension_names: Vec<*const c_char> = cast::to_array_ptr(&self.physical_device.extensions.enables);

        let device_create_info = vk::DeviceCreateInfo {
            s_type                     : vk::StructureType::DeviceCreateInfo,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
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
            self.instance.handle.create_device(self.physical_device.handle, &device_create_info, None)
                .or(Err(LogicalDeviceError::DeviceCreationError))?
        };

        // Custom queues
        let mut all_queues: Vec<HaQueue> = vec![];
        for queue_info_tmp in queue_info_tmps.iter() {
            let queue_handle = unsafe {
                handle.get_device_queue(queue_info_tmp.family_index, queue_info_tmp.queue_index)
            };
            let queue = HaQueue::new(queue_handle, &queue_info_tmp);
            all_queues.push(queue);
        }

        let transfer_queue = all_queues.pop().unwrap();
        let present_queue  = all_queues.pop().unwrap();
        let graphics_queue = all_queues.pop().unwrap();

        // Default queues

        let device = HaLogicalDevice {
            handle,
            queues: all_queues,

            graphics_queue,
            present_queue,
            transfer_queue,
        };
        Ok(device)
    }

    fn print_queue_infos(&self, family_infos: &Vec<vk::DeviceQueueCreateInfo>, queue_infos: &Vec<QueueInfoTmp>) {

        println!("[Info] Generate Queue Family: {}", family_infos.len());
        println!("\tfamily index | queue count | priorities");
        for family_info in family_infos.iter() {
            println!("\t{:12} | {:11} | {:?}", family_info.queue_family_index, family_info.queue_count, family_info.p_queue_priorities);
        }

        println!("[Info] Generate Queue: {}", queue_infos.len());
        println!("\tpriority | family index | queue index | usage");
        for queue_info in queue_infos.iter() {
            println!("\t{:8} | {:12} | {:11} | {:?}", queue_info.priority, queue_info.family_index, queue_info.queue_index, queue_info.usage);
        }
    }
}
