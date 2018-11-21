
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::physical::config::PhysicalInspectProperty;
use core::error::PhysicalDeviceError;
use core::surface::HaSurface;

use types::vkuint;

#[derive(Debug, Clone)]
pub struct QueueFamilyIndices {

    pub graphics_index: vkuint,
    pub present_index : vkuint,
    pub transfer_index: vkuint,
    pub is_share_same_family: bool,
}

struct QueueCapabilityIndices {

    graphics       : Option<usize>,
    compute        : Option<usize>,
    transfer       : Option<usize>,
    sparse_binding : Option<usize>,
    protected      : Option<usize>,
}

pub(crate) struct PhysicalQueueFamilies {

    pub families: Vec<vk::QueueFamilyProperties>,
    pub family_indices: QueueFamilyIndices,

    capability_indices: QueueCapabilityIndices,
}

#[derive(Debug, Clone)]
pub struct PhysicalQueueFamilyConfig {

    require_capabilities: Vec<vk::QueueFlags>,
}

impl PhysicalQueueFamilies {

    pub fn query(instance: &HaInstance, physical_device: vk::PhysicalDevice, surface: &HaSurface)
        -> Result<PhysicalQueueFamilies, PhysicalDeviceError> {
        let families = unsafe {
            instance.handle.get_physical_device_queue_family_properties(physical_device)
        };

        let mut candidate_graphics_index = None;
        let mut candidate_present_index = None;
        let mut candidate_transfer_index = None;

        let mut family_index: vkuint = 0; // queue family index
        for queue_family in families.iter() {
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS & vk::QueueFlags::TRANSFER) {
                candidate_graphics_index = Some(family_index);
                candidate_transfer_index = Some(family_index);
            }

            if queue_family.queue_count > 0 && surface.query_is_family_presentable(physical_device, family_index) {
                candidate_present_index = Some(family_index);
            }

            if candidate_graphics_index.is_some() && candidate_present_index.is_some() && candidate_transfer_index.is_some() {
                break
            }

            family_index += 1;
        }

        let graphics_index = candidate_graphics_index
            .ok_or(PhysicalDeviceError::GraphicsQueueNotSupportError)?;
        let present_index = candidate_present_index
            .ok_or(PhysicalDeviceError::PresentQueueNotSupportError)?;
        let transfer_index = candidate_transfer_index
            .ok_or(PhysicalDeviceError::TransferQueueNotSupportError)?;
        let is_share_same_family = graphics_index == present_index && graphics_index == transfer_index;

        let family_indices = QueueFamilyIndices {
            graphics_index,
            present_index,
            transfer_index,
            is_share_same_family,
        };

        let capability_indices = generate_operation_indices(&families);

        let queue_families = PhysicalQueueFamilies {
            families,
            family_indices,
            capability_indices,
        };

        Ok(queue_families)
    }

    pub fn is_queue_support_capability(&self, check_family_index: vkuint, capability: vk::QueueFlags) -> bool {

        let family = &self.families[check_family_index as usize];

        family.queue_flags.contains(capability)
    }

    pub fn is_queue_count_enough(&self, check_family_index: vkuint, request_queue_count: usize) -> bool {

        let family = &self.families[check_family_index as usize];
        family.queue_count as usize >= request_queue_count
    }
}

impl PhysicalInspectProperty for PhysicalQueueFamilies {
    type ConfigType = PhysicalQueueFamilyConfig;

    fn inspect(&self, config: &Self::ConfigType) -> bool {

        config.require_capabilities.iter()
            .all(|&requirement| {
                match requirement {
                    | vk::QueueFlags::GRAPHICS => self.capability_indices.graphics.is_some(),
                    | vk::QueueFlags::COMPUTE  => self.capability_indices.compute.is_some(),
                    | vk::QueueFlags::TRANSFER => self.capability_indices.transfer.is_some(),
                    | vk::QueueFlags::SPARSE_BINDING => self.capability_indices.sparse_binding.is_some(),
                    | vk::QueueFlags::PROTECTED => self.capability_indices.protected.is_some(),
                    | _ => false,
                }
            })
    }

    fn set(&mut self, _config: &Self::ConfigType) {
        // nothing to set, leave it empty...
    }
}



fn generate_operation_indices(families: &Vec<vk::QueueFamilyProperties>) -> QueueCapabilityIndices {

    let mut result = QueueCapabilityIndices {
        graphics: None,
        compute: None,
        transfer: None,
        sparse_binding: None,
        protected: None,
    };

    for (index, family) in families.iter().enumerate() {

        let test_flags = family.queue_flags;

        if result.graphics.is_none() && test_flags.contains(vk::QueueFlags::GRAPHICS) {
            result.graphics = Some(index);
        }
        if result.compute.is_none() && test_flags.contains(vk::QueueFlags::COMPUTE) {
            result.compute = Some(index);
        }
        if result.transfer.is_none() && test_flags.contains(vk::QueueFlags::TRANSFER) {
            result.transfer = Some(index);
        }
        if result.sparse_binding.is_none() && test_flags.contains(vk::QueueFlags::SPARSE_BINDING) {
            result.sparse_binding = Some(index);
        }
        if result.protected.is_none() && test_flags.contains(vk::QueueFlags::PROTECTED) {
            result.protected = Some(index);
        }

        if result.graphics.is_some() && result.compute.is_some() && result.transfer.is_some() && result.sparse_binding.is_some() && result.protected.is_some() {
            break
        }
    }

    result
}
