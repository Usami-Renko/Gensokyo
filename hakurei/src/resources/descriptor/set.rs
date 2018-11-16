
use vk::resources::descriptor::{ HaDescriptorSetLayout, HaDescriptorSet, DescriptorSetItem };
use vk::resources::descriptor::DescriptorSetConfig;
use vk::utils::types::vkint;

pub struct DescriptorSet {

    item  : DescriptorSetItem,
    layout: HaDescriptorSetLayout,
    set_index: usize,
}

impl DescriptorSet {

    pub fn unset() -> DescriptorSet {
        DescriptorSet {
            item  : DescriptorSetItem::unset(),
            layout: HaDescriptorSetLayout::unset(),
            set_index: 0,
        }
    }

    pub fn new(from: &HaDescriptorSet, config: &DescriptorSetConfig, set_index: usize) -> DescriptorSet {

        let binding_indices = config.iter_binding()
            .map(|b| b.binding_content().binding)
            .collect();

        DescriptorSet {
            item: DescriptorSetItem::from(from, binding_indices),
            layout: from.layout(),
            set_index,
        }
    }
}
