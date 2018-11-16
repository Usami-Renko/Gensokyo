
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::command::buffer::{ HaCommandBuffer, CommandBufferUsage };
use resources::command::infos::CmdBufferBindingInfo;
use resources::command::infos::{ CmdViewportInfo, CmdScissorInfo, CmdDepthBiasInfo, CmdDepthBoundInfo };
use resources::command::traits::ToDescriptorSetItem;
use resources::command::traits::IntoVKBarrier;
use resources::image::HaImageBarrier;
use resources::error::CommandError;

use pipeline::graphics::HaGraphicsPipeline;
use pipeline::stages::PipelineStageFlag;
use pipeline::state::depth_stencil::StencilFaceFlag;
use pipeline::pass::DependencyFlag;

use utils::types::{ vkint, vksint, vkfloat };
use utils::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub struct HaCommandRecorder {

    buffer: Option<HaCommandBuffer>,
    handle: vk::CommandBuffer,
    usage : CommandBufferUsage,
    device: HaDevice,
}

impl HaCommandRecorder {

    pub fn new(device: &HaDevice, command: HaCommandBuffer) -> HaCommandRecorder {

        let handle = command.handle;
        let usage = command.usage;

        HaCommandRecorder {
            buffer: Some(command),
            device: device.clone(),
            handle, usage,
        }
    }

    pub fn begin_record(&self, flags: &[CommandBufferUsageFlag]) -> Result<&HaCommandRecorder, CommandError> {

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: ptr::null(),
            flags : flags.flags(),
            // TODO: Add configuration for secondary command buffer
            // Inheritance_info is used if commandBuffer is a secondary command buffer.
            // If this is a primary command buffer, then this value is ignored.
            p_inheritance_info: ptr::null(),
        };

        unsafe {
            self.device.handle.begin_command_buffer(self.handle, &begin_info)
                .or(Err(CommandError::RecordBeginError))?
        };

        Ok(self)
    }


    pub fn begin_render_pass(&self, pipeline: &HaGraphicsPipeline, framebuffer_index: usize) -> &HaCommandRecorder {

        let render_pass = pipeline.pass();

        let begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RenderPassBeginInfo,
            p_next: ptr::null(),
            render_pass: render_pass.handle,
            framebuffer: render_pass.framebuffers[framebuffer_index].handle,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: render_pass.framebuffer_extent,
            },
            clear_value_count: render_pass.clear_values.len() as vkint,
            p_clear_values   : render_pass.clear_values.as_ptr(),
        };

        unsafe {
            self.device.handle.cmd_begin_render_pass(self.handle, &begin_info, self.usage.usage());
        }
        self
    }

    /// Set the viewport dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::DynamicViewportFixedScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_viewport` is the index of the first viewport whose parameters are updated by the command.
    ///
    /// `viewports` specifies the new value to use as viewports.
    pub fn set_viewport(&self, first_viewport: vkint, viewports: &[CmdViewportInfo]) -> &HaCommandRecorder {

        let ports = viewports.iter()
            .map(|p| p.content).collect::<Vec<_>>();
        unsafe {
            self.device.handle.cmd_set_viewport(self.handle, first_viewport, &ports)
        };
        self
    }

    /// Set the scissor rectangles dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::FixedViewportDynamicScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_scissor` is the index of the first scissor whose state is updated by the command.
    ///
    /// `scissors` specifies the new value to use as scissor rectangles.
    pub fn set_scissor(&self, first_scissor: vkint, scissors: &[CmdScissorInfo]) -> &HaCommandRecorder {

        let scissors = scissors.iter()
            .map(|s| s.content).collect::<Vec<_>>();
        unsafe {
            self.device.handle.cmd_set_scissor(self.handle, first_scissor, &scissors)
        };
        self
    }

    /// Set the line width dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaRasterizerState::set_line_width()` on RasterizerState during pipeline creation.
    ///
    /// `width` specifies the new value to use as the width of rasterized line segments.
    pub fn set_line_width(&self, width: vkfloat) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_line_width(self.handle, width)
        };
        self
    }

    /// Set the depth bias dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaRasterizerState::set_depth_bias()` on RasterizerState during pipeline creation.
    ///
    /// `bias` specifies the new value to use as depth bias.
    pub fn set_depth_bias(&self, bias: CmdDepthBiasInfo) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_depth_bias(self.handle, bias.constant_factor, bias.clamp, bias.slope_factor)
        };
        self
    }

    /// Set the blend constants dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaBlendState::set_blend_constants()` on BlendState during pipeline creation.
    ///
    /// `constants` specifies the new value to use as blend constants.
    pub fn set_blend_constants(&self, constants: [vkfloat; 4]) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_blend_constants(self.handle, constants)
        };
        self
    }

    /// Set the depth bound dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `DepthTest::set_depth_bound()` on DepthStencilState during pipeline creation.
    ///
    /// `bound` specifies the new value to use as depth bound.
    pub fn set_depth_bound(&self, bound: CmdDepthBoundInfo) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_depth_bounds(self.handle, bound.min_bound, bound.max_bound)
        };
        self
    }

    /// Set the stencil compare mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_compare_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the compare mask.
    ///
    /// `mask` specifies the new value to use as the stencil compare mask.
    pub fn set_stencil_compare_mask(&self, face: StencilFaceFlag, mask: vkint) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_compare_mask(self.handle, face.value(), mask)
        };
        self
    }

    /// Set the stencil write mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_write_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the write mask.
    ///
    /// `mask` specifies the new value to use as the stencil write mask.
    pub fn set_stencil_write_mask(&self, face: StencilFaceFlag, mask: vkint) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_write_mask(self.handle, face.value(), mask)
        };
        self
    }

    /// Set the stencil reference dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_reference()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the reference value.
    ///
    /// `reference` specifies the set of stencil state for which to update the reference value.
    pub fn set_stencil_reference(&self, face: StencilFaceFlag, reference: vkint) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_reference(self.handle, face.value(), reference)
        };
        self
    }

    pub fn bind_pipeline(&self, pipeline: &HaGraphicsPipeline) -> &HaCommandRecorder {

        unsafe {
            self.device.handle.cmd_bind_pipeline(self.handle, pipeline.bind_point(), pipeline.handle)
        };

        self
    }

    pub fn bind_vertex_buffers(&self, first_binding: vkint, infos: &[CmdBufferBindingInfo]) -> &HaCommandRecorder {

        let mut handles = vec![];
        let mut offsets  = vec![];

        for info in infos.into_iter() {
            let item = info.block.item();
            handles.push(item.handle);

            let starting_offset = info.sub_block_index
                .map(|i| info.block.offset(i))
                .unwrap_or(0);
            offsets.push(starting_offset);
        }

        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.handle, first_binding, &handles, &offsets)
        };
        self
    }

    pub fn bind_index_buffer(&self, info: CmdBufferBindingInfo) -> &HaCommandRecorder {

        let item = info.block.item();
        let starting_offset = info.sub_block_index
            .map(|i| info.block.offset(i))
            .unwrap_or(0);

        unsafe {
            // TODO: Add configuration for IndexType.
            self.device.handle.cmd_bind_index_buffer(self.handle, item.handle, starting_offset, vk::IndexType::Uint32)
        };
        self
    }

    pub fn bind_descriptor_sets(&self, pipeline: &HaGraphicsPipeline, first_set: vkint, sets: &[&impl ToDescriptorSetItem]) -> &HaCommandRecorder {

        let handles = sets.iter()
            .map(|s| s.item().handle).collect::<Vec<_>>();

        unsafe {
            // TODO: Currently dynamic_offsets field is not configuration.
            self.device.handle.cmd_bind_descriptor_sets(
                self.handle, pipeline.bind_point(), pipeline.layout().handle, first_set, &handles, &[])
        };
        self
    }

    pub(crate) fn copy_buffer(&self, src_buffer_handle: vk::Buffer, dst_buffer_handle: vk::Buffer, regions: &[vk::BufferCopy]) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_buffer(
                self.handle, src_buffer_handle, dst_buffer_handle, regions)
        };
        self
    }

    pub(crate) fn copy_buffer_to_image(&self, src_handle: vk::Buffer, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::BufferImageCopy]) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_buffer_to_image(
                self.handle, src_handle, dst_handle, dst_layout, regions)
        };
        self
    }

    pub(crate) fn copy_image_to_buffer(&self, src_handle: vk::Image, src_layout: vk::ImageLayout, dst_buffer: vk::Buffer, regions: &[vk::BufferImageCopy]) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_image_to_buffer(
                self.handle, src_handle, src_layout, dst_buffer, regions)
        };

        self
    }

    pub(crate) fn copy_image(&self,src_handle: vk::Image, src_layout: vk::ImageLayout, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::ImageCopy]) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_image(
                self.handle, src_handle, src_layout, dst_handle, dst_layout, regions)
        };

        self
    }

    pub fn draw(&self, vertex_count: vkint, instance_count: vkint, first_vertex: vkint, first_instance: vkint) -> &HaCommandRecorder {
        unsafe {
            self.device.handle.cmd_draw(self.handle, vertex_count, instance_count, first_vertex, first_instance)
        };
        self
    }

    pub fn draw_indexed(&self, index_count: vkint, instance_count: vkint, first_index: vkint, vertex_offset: vksint, first_instance: vkint) -> &HaCommandRecorder {
        unsafe {
            self.device.handle
                .cmd_draw_indexed(self.handle, index_count, instance_count, first_index, vertex_offset, first_instance)
        };
        self
    }
//    pub fn draw_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder {
//        unsafe {
//            self.device.handle.cmd_draw_indirect(self.buffer, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }
//    pub fn draw_indexed_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder {
//        unsafe {
//            self.device.handle.cmd_draw_indexed_indirect(self.buffer, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }


    #[inline]
    pub fn image_pipeline_barrrier(&self, src_stage: PipelineStageFlag, dst_stage: PipelineStageFlag, dependencies: &[DependencyFlag], image_barries: Vec<HaImageBarrier>) -> &HaCommandRecorder {

        let barriers: Vec<vk::ImageMemoryBarrier> = image_barries.into_iter()
            .map(|b| b.into_barrier()).collect();

        self.pipeline_barrrier(src_stage, dst_stage, dependencies, &[], &[], &barriers)
    }

    fn pipeline_barrrier(&self, src_stage: PipelineStageFlag, dst_stage: PipelineStageFlag, dependencies: &[DependencyFlag], memory_barries: &[vk::MemoryBarrier], buffer_barries: &[vk::BufferMemoryBarrier], image_barries: &[vk::ImageMemoryBarrier]) -> &HaCommandRecorder {

        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle, src_stage.value(), dst_stage.value(), dependencies.flags(), memory_barries, buffer_barries, image_barries)
        };
        self
    }

    pub fn end_render_pass(&self) -> &HaCommandRecorder {
        unsafe {
            // Ending the render pass will add an implicit barrier transitioning the frame buffer color attachment vk::IMAGE_LAYOUT_PRESENT_SRC_KHR for presenting it to the windowing system.
            self.device.handle.cmd_end_render_pass(self.handle)
        };
        self
    }

    pub fn end_record(&mut self) -> Result<HaCommandBuffer, CommandError> {

        let _ = unsafe {
            self.device.handle.end_command_buffer(self.handle)
                .or(Err(CommandError::RecordEndError))?
        };

        let buffer = self.buffer.take().unwrap();

        Ok(buffer)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandBufferUsageFlag {

    /// OneTimeSubmitBit specifies that each recording of the command buffer will only be submitted once,
    /// and the command buffer will be reset and recorded again between each submission.
    OneTimeSubmitBit,
    /// RenderPassContinueBit specifies that a secondary command buffer is considered to be entirely inside a render pass.
    ///
    /// If this is a primary command buffer, then this bit is ignored.
    RenderPassContinueBit,
    /// SimultaneousUseBit specifies that a command buffer can be resubmitted to a queue while it is in the pending state,
    /// and recorded into multiple primary command buffers.
    SimultaneousUseBit,
}

impl VulkanFlags for [CommandBufferUsageFlag] {
    type FlagType = vk::CommandBufferUsageFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::CommandBufferUsageFlags::empty(), |acc, flag| {
            match flag {
                | CommandBufferUsageFlag::OneTimeSubmitBit      => acc | vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
                | CommandBufferUsageFlag::RenderPassContinueBit => acc | vk::COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT,
                | CommandBufferUsageFlag::SimultaneousUseBit    => acc | vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
            }
        })
    }
}
