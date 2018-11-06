
use ash::vk;
use ash::vk::uint32_t;

#[derive(Debug, Clone)]
pub struct DescriptorSetItem {

    pub(crate) handle: vk::DescriptorSet,
    pub(crate) set_index: usize,
    pub(crate) binding_indices: Vec<uint32_t>,
}

pub struct DescriptorSet {

    pub(crate) item: DescriptorSetItem,
    pub(crate) layout: vk::DescriptorSetLayout,
}

impl DescriptorSet {

    pub fn unset() -> DescriptorSet {
        DescriptorSet {
            item: DescriptorSetItem {
                handle: vk::DescriptorSet::null(),
                set_index: 0,
                binding_indices: vec![],
            },
            layout: vk::DescriptorSetLayout::null(),
        }
    }
}

pub struct DescriptorSetIndex(pub usize);
