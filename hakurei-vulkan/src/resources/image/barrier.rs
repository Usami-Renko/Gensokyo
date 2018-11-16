
use ash::vk;

use pipeline::pass::AccessFlag;

use resources::image::enums::ImageLayout;
use resources::image::image::HaImage;
use resources::image::ImageSubresourceRange;
use resources::command::IntoVKBarrier;

use utils::marker::{ VulkanFlags, VulkanEnum };
use utils::types::vkint;

use std::ptr;

pub struct HaImageBarrier(vk::ImageMemoryBarrier);

impl HaImageBarrier {

    pub fn new(image: &HaImage, subrange: &ImageSubresourceRange) -> HaImageBarrierBuilder {

        let barrier = vk::ImageMemoryBarrier {
            s_type: vk::StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::empty(),
            old_layout: vk::ImageLayout::Undefined,
            new_layout: vk::ImageLayout::Undefined,
            src_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            image: image.handle,
            subresource_range: subrange.gen_subrange(),
        };

        HaImageBarrierBuilder(HaImageBarrier(barrier))
    }
}

pub struct HaImageBarrierBuilder(HaImageBarrier);

impl HaImageBarrierBuilder {

    pub fn build(self) -> HaImageBarrier {
        self.0
    }

    pub fn access_mask(mut self, from: &[AccessFlag], to: &[AccessFlag]) -> Self {

        (self.0).0.src_access_mask = from.flags();
        (self.0).0.dst_access_mask = to.flags();
        self
    }

    pub fn layout(mut self, from: ImageLayout, to: ImageLayout) -> Self {

        (self.0).0.old_layout = from.value();
        (self.0).0.new_layout = to.value();
        self
    }

    pub fn queue_family_index(mut self, from: vkint, to: vkint) -> Self {

        (self.0).0.src_queue_family_index = from;
        (self.0).0.dst_queue_family_index = to;
        self
    }
}

impl IntoVKBarrier for HaImageBarrier {
    type BarrierType = vk::ImageMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {
        self.0
    }
}
