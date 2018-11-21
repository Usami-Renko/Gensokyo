
use ash::vk;

use image::target::HaImage;
use command::IntoVKBarrier;

use types::vkuint;

use std::ptr;

pub struct HaImageBarrier(vk::ImageMemoryBarrier);

impl HaImageBarrier {

    pub fn new(image: &HaImage, subrange: vk::ImageSubresourceRange) -> HaImageBarrierBuilder {

        let mut barrier = HaImageBarrier::default();
        barrier.0.image = image.handle;
        barrier.0.subresource_range = subrange;

        HaImageBarrierBuilder(barrier)
    }
}

impl Default for HaImageBarrier {

    fn default() -> HaImageBarrier {

        let barrier = vk::ImageMemoryBarrier {
            s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
            p_next: ptr::null(),
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::empty(),
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            image: vk::Image::null(),
            subresource_range: Default::default(),
        };

        HaImageBarrier(barrier)
    }
}

pub struct HaImageBarrierBuilder(HaImageBarrier);

impl HaImageBarrierBuilder {

    pub fn build(self) -> HaImageBarrier {
        self.0
    }

    pub fn access_mask(mut self, from: vk::AccessFlags, to: vk::AccessFlags) -> Self {

        (self.0).0.src_access_mask = from;
        (self.0).0.dst_access_mask = to;
        self
    }

    pub fn layout(mut self, from: vk::ImageLayout, to: vk::ImageLayout) -> Self {

        (self.0).0.old_layout = from;
        (self.0).0.new_layout = to;
        self
    }

    pub fn queue_family_index(mut self, from: vkuint, to: vkuint) -> Self {

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
