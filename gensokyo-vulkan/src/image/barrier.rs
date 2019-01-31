
use ash::vk;

use crate::image::target::GsImage;
use crate::image::view::ImageSubRange;
use crate::command::IntoVKBarrier;

use crate::types::vkuint;

use std::ptr;

pub struct ImageBarrierCI(vk::ImageMemoryBarrier);

impl ImageBarrierCI {

    pub fn new(image: &GsImage, subrange: ImageSubRange) -> ImageBarrierBuilder {

        let mut barrier = ImageBarrierCI::default();
        barrier.0.image = image.handle;
        barrier.0.subresource_range = subrange.0;

        ImageBarrierBuilder(barrier)
    }
}

impl Default for ImageBarrierCI {

    fn default() -> ImageBarrierCI {

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

        ImageBarrierCI(barrier)
    }
}

pub struct ImageBarrierBuilder(ImageBarrierCI);

impl ImageBarrierBuilder {

    pub fn build(self) -> ImageBarrierCI {
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

impl IntoVKBarrier for ImageBarrierCI {
    type BarrierType = vk::ImageMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {
        self.0
    }
}
