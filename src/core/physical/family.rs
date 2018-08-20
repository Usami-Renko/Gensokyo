
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::Instance;
use core::error::PhysicalDeviceError;
use utility::marker::VulkanFlags;

pub struct QueueFamilyIndices {

    pub graphics_index: usize,
    pub present_index:  usize,
    pub is_share_same_family: bool,
}

struct QueueOperationIndices {
    graphics      : Option<usize>,
    compute       : Option<usize>,
    transfer      : Option<usize>,
    sparse_inding : Option<usize>,
}

pub enum QueueOperationType {
    Graphics,
    Compute,
    Transfer,
    SparseBinding,
}

impl QueueOperationType {

    fn is_support(&self, queue_flag: vk::QueueFlags) -> bool {
        let inspect_flag = match *self {
            | QueueOperationType::Graphics      => vk::QUEUE_GRAPHICS_BIT,
            | QueueOperationType::Compute       => vk::QUEUE_COMPUTE_BIT,
            | QueueOperationType::Transfer      => vk::QUEUE_TRANSFER_BIT,
            | QueueOperationType::SparseBinding => vk::QUEUE_SPARSE_BINDING_BIT,
        };
        queue_flag.subset(inspect_flag)
    }
}

impl VulkanFlags for [QueueOperationType] {
    type FlagType = vk::QueueFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::QueueFlags::empty(), |acc, flag| {
            match *flag {
                | QueueOperationType::Graphics      => acc | vk::QUEUE_GRAPHICS_BIT,
                | QueueOperationType::Compute       => acc | vk::QUEUE_COMPUTE_BIT,
                | QueueOperationType::Transfer      => acc | vk::QUEUE_TRANSFER_BIT,
                | QueueOperationType::SparseBinding => acc | vk::QUEUE_SPARSE_BINDING_BIT,
            }
        })
    }
}

pub struct PhysicalQueueFamilies {

    families          : Vec<vk::QueueFamilyProperties>,
    family_indices    : QueueFamilyIndices,
    operation_indices : QueueOperationIndices,
}

impl PhysicalQueueFamilies {

    pub fn inspect(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<PhysicalQueueFamilies, PhysicalDeviceError> {

        let families = instance.handle.get_physical_device_queue_family_properties(physical_device);

        let mut back_graphics_index = None;
        let mut back_present_index  = None;

        let mut queue_index = 0_usize;
        for queue_family in families.iter() {
            if queue_family.queue_count > 0 && queue_family.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT) {
                back_graphics_index = Some(queue_index);
            }

            // TODO: Add check for Presentation Support.
            back_present_index = back_graphics_index;

            if back_graphics_index.is_some() && back_present_index.is_some() {
                break
            }

            queue_index += 1;
        }


        if back_present_index.is_none() || back_present_index.is_none() {

            return Err(PhysicalDeviceError::QueueRequirementNotSupportError)
        }

        let family_indices = QueueFamilyIndices {
            graphics_index: back_graphics_index.unwrap(),
            present_index:  back_present_index.unwrap(),
            is_share_same_family: back_present_index.unwrap() == back_present_index.unwrap(),
        };

        let operation_indices = generate_operation_indices(&families);

        let queue_families = PhysicalQueueFamilies {
            families,
            family_indices,
            operation_indices,
        };

        Ok(queue_families)
    }

    pub fn check_requirements(&self, require_operations: &Vec<QueueOperationType>) -> bool {

        require_operations.iter().all(|require_operation| {
            match *require_operation {
                | QueueOperationType::Graphics      => self.operation_indices.graphics.is_some(),
                | QueueOperationType::Compute       => self.operation_indices.compute.is_some(),
                | QueueOperationType::Transfer      => self.operation_indices.transfer.is_some(),
                | QueueOperationType::SparseBinding => self.operation_indices.sparse_inding.is_some(),
            }
        })
    }
}


fn generate_operation_indices(families: &Vec<vk::QueueFamilyProperties>) -> QueueOperationIndices {

    let mut result = QueueOperationIndices {
        graphics: None,
        compute:  None,
        transfer: None,
        sparse_inding: None,
    };

    for (index, family) in families.iter().enumerate() {
        let test_flags = family.queue_flags;

        if result.graphics.is_none() && QueueOperationType::Graphics.is_support(test_flags) {
            result.graphics = Some(index);
        }
        if result.compute.is_none() && QueueOperationType::Compute.is_support(test_flags) {
            result.compute = Some(index);
        }
        if result.transfer.is_none() && QueueOperationType::Transfer.is_support(test_flags) {
            result.transfer = Some(index);
        }
        if result.sparse_inding.is_none() && QueueOperationType::SparseBinding.is_support(test_flags) {
            result.sparse_inding = Some(index);
        }

        if result.graphics.is_some() && result.compute.is_some() && result.transfer.is_some() && result.sparse_inding.is_some() {
            break
        }
    }

    result
}
