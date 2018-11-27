
use ash;
use ash::vk;
use ash::version::{ InstanceV1_0, DeviceV1_0 };

use core::instance::HaInstance;
use core::physical::HaPhysicalDevice;
use core::device::device::{ DeviceConfig, HaLogicalDevice };
use core::device::enums::{ PrefabQueuePriority, DeviceQueueIdentifier, QueueRequestStrategy };
use core::device::queue::{ HaQueue, QueueUsage, QueueContainer };
use core::error::LogicalDeviceError;

use utils::cast;
use types::vkuint;
use VERBOSE;

use std::collections::HashMap;
use std::rc::Rc;
use std::ptr;

pub struct LogicalDeviceBuilder<'a, 'b> {

    instance: &'a HaInstance,
    physical: &'b HaPhysicalDevice,

    queue_request: QueueRequestInfo,
    config: DeviceConfig,
}

impl<'a, 'b> LogicalDeviceBuilder<'a, 'b> {

    pub fn init(instance: &'a HaInstance, physical: &'b HaPhysicalDevice, config: &DeviceConfig) -> Result<LogicalDeviceBuilder<'a, 'b>, LogicalDeviceError> {

        let queue_request = QueueRequestInfo::new(physical, config.queue_request_strategy)?;

        let builder = LogicalDeviceBuilder {
            instance, physical, queue_request,
            config: config.clone(),
        };
        Ok(builder)
    }

    #[allow(dead_code)]
    pub fn add_queue(&mut self, usage: QueueUsage) -> DeviceQueueIdentifier {

        let queue_index = self.setup_queue(usage);

        match usage {
            | QueueUsage::Graphics => DeviceQueueIdentifier::Custom {
                identifier: Box::new(DeviceQueueIdentifier::Graphics),
                queue_index,
            },
            | QueueUsage::Present  => DeviceQueueIdentifier::Custom {
                identifier: Box::new(DeviceQueueIdentifier::Present),
                queue_index,
            },
            | QueueUsage::Transfer => DeviceQueueIdentifier::Custom {
                identifier: Box::new(DeviceQueueIdentifier::Transfer),
                queue_index,
            },
        }
    }

    // TODO: Currently queue Priority is not configuratable.
    fn setup_queue(&mut self, usage: QueueUsage) -> usize {

        let queue_index = match self.queue_request {
            | QueueRequestInfo::Queue { entity: _, ref mut logics } => {
                let logic_queue = LogicQueueInfo {
                    usage, entity_index: LogicQueueEntityIndex::SingleQueue,
                };

                let new_logic_queue_index = logics.len();
                logics.push(logic_queue);

                new_logic_queue_index
            },
            | QueueRequestInfo::Queues { ref entities, ref mut logics, ref usage_indices } => {
                let entity_queue_index = usage_indices.get(&usage).unwrap().clone();
                let entity_queue = &entities[entity_queue_index];
                let logic_queue = LogicQueueInfo {
                    usage, entity_index: LogicQueueEntityIndex::MultiQueues(entity_queue.queue_index),
                };

                let new_logic_queue_index = logics.len();
                logics.push(logic_queue);
                new_logic_queue_index
            },
        };

        queue_index
    }

    pub fn build(&mut self) -> Result<HaLogicalDevice, LogicalDeviceError> {

        // Configurate queue.
        let _ = self.setup_queue(QueueUsage::Graphics);
        let _ = self.setup_queue(QueueUsage::Present);
        let _ = self.setup_queue(QueueUsage::Transfer);

        let entity_queue_infos = self.queue_request.queue_infos();
        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = entity_queue_infos.into_iter()
            .map(|(family_index, priorities)| {
                vk::DeviceQueueCreateInfo {
                    s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                    p_next: ptr::null(),
                    // flags is reserved for future use in API version 1.1.82.
                    flags : vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index: family_index as vkuint,
                    queue_count       : priorities.len() as vkuint,
                    p_queue_priorities: priorities.as_ptr(),
                }
            }).collect();

        // Configurate device features, layers and extensions.
        let enable_features = self.physical.features.enable_features();
        let enable_layer_names = cast::to_array_ptr(&self.instance.enable_layer_names);
        let enable_extension_names = cast::to_array_ptr(self.physical.extensions.enable_extensions());

        // Create the logical device.
        let device_create_info = vk::DeviceCreateInfo {
            s_type                     : vk::StructureType::DEVICE_CREATE_INFO,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags                      : vk::DeviceCreateFlags::empty(),
            queue_create_info_count    : queue_create_infos.len() as vkuint,
            p_queue_create_infos       : queue_create_infos.as_ptr(),
            enabled_layer_count        : enable_layer_names.len() as vkuint,
            pp_enabled_layer_names     : enable_layer_names.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as vkuint,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
            p_enabled_features         : enable_features,
        };

        let handle = unsafe {
            self.instance.handle.create_device(self.physical.handle, &device_create_info, None)
                .or(Err(LogicalDeviceError::DeviceCreationError))?
        };

        if VERBOSE {
            print_queue_infos(&self.queue_request);
        }

        let mut queue_container = self.queue_request.collect_queues(&handle, &self.config)?;
        let graphics_queue = queue_container.take_last_graphics_queue();
        let present_queue = queue_container.take_last_present_queue();
        let transfer_queue = queue_container.take_last_transfer_queue();

        let device = HaLogicalDevice::new(handle, queue_container, graphics_queue, present_queue, transfer_queue);
        Ok(device)
    }
}


enum QueueRequestInfo {

    Queue {
        entity: EntityQueueInfo,
        logics: Vec<LogicQueueInfo>,
    },
    Queues {
        entities: Vec<EntityQueueInfo>,
        logics: Vec<LogicQueueInfo>,
        usage_indices: HashMap<QueueUsage, usize>,
    },
}

impl QueueRequestInfo {

    fn new(physical: &HaPhysicalDevice, strategy: QueueRequestStrategy) -> Result<QueueRequestInfo, LogicalDeviceError> {

        match strategy {
            | QueueRequestStrategy::SingleFamilySingleQueue => {

                let candidate_indices = if physical.families.family_indices.is_share_same_family {

                    vec![physical.families.family_indices.graphics_index]
                } else {

                    vec![
                        physical.families.family_indices.graphics_index,
                        physical.families.family_indices.present_index,
                        physical.families.family_indices.transfer_index,
                    ]
                };

                let result = candidate_indices.iter().find(|family_index| {
                    physical.families.is_queue_support_capability(**family_index,
                        // Graphics and Transfer operations must support for single request.
                        vk::QueueFlags::GRAPHICS | vk::QueueFlags::TRANSFER
                    )
                });

                if let Some(optimal_index) = result {

                    let entity_info = EntityQueueInfo {
                        priority: PrefabQueuePriority::Highest,
                        family_index: optimal_index.clone(),
                        queue_index : 0,
                    };

                    let request_info = QueueRequestInfo::Queue { entity: entity_info, logics: vec![] };
                    Ok(request_info)

                } else {

                    Err(LogicalDeviceError::QueueOpsUnsupport)
                }
            },
            | QueueRequestStrategy::SingleFamilyMultiQueues => {

                // check if there are enough queues for each queue family. ---------------- //
                let mut family_queue_counts = HashMap::new();
                let candidate_indices = [
                    physical.families.family_indices.graphics_index,
                    physical.families.family_indices.present_index,
                    physical.families.family_indices.transfer_index,
                ];
                for family_index in candidate_indices.iter() {

                    if family_queue_counts.contains_key(family_index) {

                        let family_queue_count = family_queue_counts.get_mut(family_index).unwrap();
                        (*family_queue_count) += 1;
                    } else {
                        family_queue_counts.insert(family_index.clone(), 1);
                    }
                }

                let is_queue_count_enough = family_queue_counts.iter()
                    .all(|(&family_index, &request_queue_count)|
                        physical.families.is_queue_count_enough(family_index, request_queue_count)
                );
                // ------------------------------------------------------------------------ //

                if is_queue_count_enough {
                    let mut queue_info_mapping = HashMap::new();
                    queue_info_mapping.insert(QueueUsage::Graphics, 0);
                    queue_info_mapping.insert(QueueUsage::Present,  1);
                    queue_info_mapping.insert(QueueUsage::Transfer, 2);

                    let request_info = QueueRequestInfo::Queues {
                        entities: vec![
                            EntityQueueInfo {
                                priority: PrefabQueuePriority::Highest,
                                family_index: physical.families.family_indices.graphics_index,
                                queue_index : 0,
                            },
                            EntityQueueInfo {
                                priority: PrefabQueuePriority::Highest,
                                family_index: physical.families.family_indices.present_index,
                                queue_index : 1,
                            },
                            EntityQueueInfo {
                                priority: PrefabQueuePriority::Highest,
                                family_index: physical.families.family_indices.transfer_index,
                                queue_index : 2,
                            },
                        ],
                        logics: vec![],
                        usage_indices: queue_info_mapping,
                    };
                    Ok(request_info)
                } else {

                    Err(LogicalDeviceError::QueueCountNotEnough)
                }
            },
        }
    }

    fn queue_infos(&self) -> Vec<(vkuint, Vec<f32>)> {

        match self {
            | QueueRequestInfo::Queue { entity, .. } => {
                vec![(
                    entity.family_index,
                    vec![entity.priority.value()]
                )]
            },
            | QueueRequestInfo::Queues { entities, .. } => {
                let mut family_indices: HashMap<vkuint, Vec<f32>> = HashMap::new();
                for entity_queue in entities.iter() {
                    if family_indices.contains_key(&entity_queue.family_index) {
                        let priorities = family_indices.get_mut(&entity_queue.family_index).unwrap();
                        priorities.push(entity_queue.priority.value());
                    } else {
                        family_indices.insert(entity_queue.family_index, vec![entity_queue.priority.value()]);
                    }
                }

                family_indices.into_iter()
                    .map(|(family_index, priorities)| (family_index, priorities)).collect()
            },
        }
    }

    fn collect_queues(&self, device: &ash::Device, config: &DeviceConfig) -> Result<QueueContainer, LogicalDeviceError> {

        match self {
            | QueueRequestInfo::Queue { entity, logics } => {
                let unique_queue = unsafe {
                    let handle = device.get_device_queue(entity.family_index, entity.queue_index);
                    HaQueue::new(handle, entity.priority, entity.family_index, entity.queue_index)
                };
                let unique_queue = Rc::new(unique_queue);

                let queue_container = collect_to_container(
                    device, logics, config, Some(&unique_queue), None
                )?;
                Ok(queue_container)
            },
            | QueueRequestInfo::Queues { entities, logics, .. } => {
                let multiqueues = entities.iter()
                    .map(|entity_queue_info| {
                        let handle = unsafe {
                            device.get_device_queue(entity_queue_info.family_index, entity_queue_info.queue_index)
                        };
                        let queue = HaQueue::new(handle, entity_queue_info.priority, entity_queue_info.family_index, entity_queue_info.queue_index);
                        Rc::new(queue)
                    }).collect();

                let queue_container = collect_to_container(device, logics, config, None, Some(&multiqueues))?;
                Ok(queue_container)
            },
        }
    }
}

#[derive(Debug, Clone)]
struct EntityQueueInfo {

    priority: PrefabQueuePriority,
    family_index: vkuint,
    queue_index : vkuint,
}

#[derive(Debug, Clone)]
struct LogicQueueInfo {

    usage: QueueUsage,
    entity_index: LogicQueueEntityIndex,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LogicQueueEntityIndex {
    SingleQueue,
    MultiQueues(vkuint),
}

fn collect_to_container(device: &ash::Device, logic_queues: &Vec<LogicQueueInfo>, config: &DeviceConfig, unique_queue: Option<&Rc<HaQueue>>, multi_queues: Option<&Vec<Rc<HaQueue>>>) ->  Result<QueueContainer, LogicalDeviceError> {

    let mut queue_container = QueueContainer::empty();
    for logic_queue_info in logic_queues.iter() {
        match logic_queue_info.entity_index {
            | LogicQueueEntityIndex::SingleQueue => {
                queue_container.add_queue(device, logic_queue_info.usage, unique_queue.unwrap(), config)?;
            },
            | LogicQueueEntityIndex::MultiQueues(queue_index) => {
                let queue = &multi_queues.unwrap()[queue_index as usize];
                queue_container.add_queue(device, logic_queue_info.usage, queue, config)?;
            },
        }
    }

    Ok(queue_container)
}


fn print_queue_infos(queue_request: &QueueRequestInfo) {

    match queue_request {
        | QueueRequestInfo::Queue { entity, logics } => {
            println!("[Info] Generate Queue Family: Single Queue");
            println!("\tfamily index | queue count | priorities");
            println!("\t{:12} | {:11} | {:?}", entity.family_index, logics.len(), entity.priority);

            println!("[Info] Generate Queue: {}", logics.len());
            println!("\tpriority | family index | queue index | usage");
            for logic_queue_info in logics.iter() {
                println!("\t{:8?} | {:12} | {:11} | {:?}", entity.priority, entity.family_index, 0, logic_queue_info.usage);
            }
        },
        | QueueRequestInfo::Queues { entities, logics, .. } => {

            println!("[Info] Generate Queue Family: Multi Queues");
            println!("\tfamily index | queue count | priorities");
            for entity_queue in entities.iter() {
                println!("\t{:12} | {:11} | {:?}", entity_queue.family_index, logics.len(), entity_queue.priority);
            }

            println!("[Info] Generate Queue: {}", logics.len());
            println!("\tpriority | family index | queue index | usage");
            for logic_queue_info in logics.iter() {
                match logic_queue_info.entity_index {
                    | LogicQueueEntityIndex::SingleQueue => panic!(),
                    | LogicQueueEntityIndex::MultiQueues(queue_index) => {
                        let entity_queue = &entities[queue_index as usize];
                        println!("\t{:8?} | {:12} | {:11} | {:?}", entity_queue.priority, entity_queue.family_index, queue_index, logic_queue_info.usage);
                    }
                }

            }
        }
    }
}
