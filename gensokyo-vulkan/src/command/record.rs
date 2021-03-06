
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::command::buffer::{ GsCommandBuffer, CmdBufferUsage };
use crate::command::traits::CmdPipelineAbs;
use crate::pipeline::target::GsVkPipelineType;
use crate::error::{ VkResult, VkError };
use crate::utils::phantom::{ Graphics, Compute, Transfer };

use std::marker::PhantomData;
use std::ptr;


pub trait GsVkCommandType {
    // Empty...
}

pub struct GsCmdRecorder<T>
    where
        T: GsVkCommandType {

    phantom_type: PhantomData<T>,

    pub(super) device: GsDevice,

    pub(super) cmd_handle: vk::CommandBuffer,
    pub(super) cmd_usage: CmdBufferUsage,

    pub(super) pipeline_handle: vk::Pipeline,
    pub(super) pipeline_layout: vk::PipelineLayout,
}

impl<T> GsCmdRecorder<T>
    where
        T: GsVkCommandType + GsVkPipelineType {

    pub fn create(device: &GsDevice, command: GsCommandBuffer, pipeline: &impl CmdPipelineAbs) -> GsCmdRecorder<T> {

        GsCmdRecorder {
            phantom_type: PhantomData,
            device: device.clone(),
            cmd_handle: command.handle,
            cmd_usage : command.usage,
            pipeline_handle: pipeline.pipeline().clone(),
            pipeline_layout: pipeline.layout().clone(),
        }
    }
}

impl GsCmdRecorder<Transfer> {

    pub fn create_copy(device: &GsDevice, buffer: GsCommandBuffer) -> GsCmdRecorder<Transfer> {

        GsCmdRecorder {
            phantom_type: PhantomData,
            device: device.clone(),
            cmd_handle: buffer.handle,
            cmd_usage : buffer.usage,
            pipeline_handle: vk::Pipeline::null(),
            pipeline_layout: vk::PipelineLayout::null(),
        }
    }
}

impl<T> GsCmdRecorder<T> where T: GsVkCommandType {

    // TODO: Add configuration for vk::CommandBufferUsageFlags.
    pub fn begin_record(&self, flags: vk::CommandBufferUsageFlags) -> VkResult<&GsCmdRecorder<T>> {

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags,
            // TODO: Add configuration for secondary command buffer
            // Inheritance_info is used if commandBuffer is a secondary command buffer.
            // If this is a primary command buffer, then this value is ignored.
            p_inheritance_info: ptr::null(),
        };

        unsafe {
            self.device.logic.handle.begin_command_buffer(self.cmd_handle, &begin_info)
                .or(Err(VkError::device("Failed to begin Command Buffer recording.")))?
        };
        Ok(self)
    }

    pub fn end_record(&mut self) -> VkResult<GsCommandBuffer> {

        let _ = unsafe {
            self.device.logic.handle.end_command_buffer(self.cmd_handle)
                .or(Err(VkError::device("Failed to end Command Buffer recording.")))?
        };

        let buffer = GsCommandBuffer::new(self.cmd_handle, self.cmd_usage);
        Ok(buffer)
    }
}

impl GsCmdRecorder<Graphics> {

    pub fn switch_pipeline(&mut self, new_pipeline: &impl CmdPipelineAbs) {
        self.pipeline_handle = new_pipeline.pipeline().clone();
        self.pipeline_layout = new_pipeline.layout().clone();
    }
}

impl GsCmdRecorder<Compute> {

    pub fn switch_pipeline(&mut self, new_pipeline: &impl CmdPipelineAbs) {
        self.pipeline_handle = new_pipeline.pipeline().clone();
        self.pipeline_layout = new_pipeline.layout().clone();
    }
}
