
use ash::vk;
use ash::version::DeviceV1_0;

use crate::command::record::{ GsCmdRecorder, GsVkCommandType };
use crate::command::traits::IntoVKBarrier;
use crate::image::GsImageBarrier;
use crate::utils::phantom::Copy;

impl GsVkCommandType for r#Copy {
    // Empty...
}

pub trait GsCmdCopyApi {

    fn copy_buf2buf(&self, src_buffer_handle: vk::Buffer, dst_buffer_handle: vk::Buffer, regions: &[vk::BufferCopy]) -> &Self;

    fn copy_buf2img(&self, src_handle: vk::Buffer, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::BufferImageCopy]) -> &Self;

    fn copy_img2buf(&self, src_handle: vk::Image, src_layout: vk::ImageLayout, dst_buffer: vk::Buffer, regions: &[vk::BufferImageCopy]) -> &Self;

    fn copy_img2img(&self,src_handle: vk::Image, src_layout: vk::ImageLayout, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::ImageCopy]) -> &Self;

    fn image_pipeline_barrrier(&self, src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags, dependencies: vk::DependencyFlags, image_barries: Vec<GsImageBarrier>) -> &Self;
}

impl GsCmdCopyApi for GsCmdRecorder<r#Copy> {

    fn copy_buf2buf(&self, src_buffer_handle: vk::Buffer, dst_buffer_handle: vk::Buffer, regions: &[vk::BufferCopy]) -> &Self {
        unsafe {
            self.device.handle.cmd_copy_buffer(self.cmd_handle, src_buffer_handle, dst_buffer_handle, regions);
        } self
    }

    fn copy_buf2img(&self, src_handle: vk::Buffer, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::BufferImageCopy]) -> &Self {
        unsafe {
            self.device.handle.cmd_copy_buffer_to_image(self.cmd_handle, src_handle, dst_handle, dst_layout, regions);
        } self
    }

    fn copy_img2buf(&self, src_handle: vk::Image, src_layout: vk::ImageLayout, dst_buffer: vk::Buffer, regions: &[vk::BufferImageCopy]) -> &Self {
        unsafe {
            self.device.handle.cmd_copy_image_to_buffer(self.cmd_handle, src_handle, src_layout, dst_buffer, regions);
        } self
    }

    fn copy_img2img(&self,src_handle: vk::Image, src_layout: vk::ImageLayout, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::ImageCopy]) -> &Self {
        unsafe {
            self.device.handle.cmd_copy_image(self.cmd_handle, src_handle, src_layout, dst_handle, dst_layout, regions);
        } self
    }

    fn image_pipeline_barrrier(&self, src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags, dependencies: vk::DependencyFlags, image_barries: Vec<GsImageBarrier>) -> &Self {

        let barriers: Vec<vk::ImageMemoryBarrier> = image_barries.into_iter()
            .map(|b| b.into_barrier()).collect();

        unsafe {
            self.device.handle.cmd_pipeline_barrier(self.cmd_handle, src_stage, dst_stage, dependencies, &[], &[], &barriers);
        } self
    }
}
