
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::layout::GsPipelineLayout;
use crate::pipeline::pass::GsRenderPass;

use crate::utils::phantom::{ Graphics, Compute };

use std::marker::PhantomData;

pub struct GsPipeline<T: GsVkPipelineType> {

    phantom_type: PhantomData<T>,

    pub(crate) handle: vk::Pipeline,
    pub(crate) pass  : GsRenderPass,
    pub(crate) layout: GsPipelineLayout,

    device: GsDevice,

    frame_count: usize,
}

impl<T: GsVkPipelineType> GsPipeline<T> {

    pub(super) fn new(device: &GsDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: GsRenderPass) -> GsPipeline<T> {

        let frame_count = pass.frame_count();

        GsPipeline {
            phantom_type: PhantomData,
            handle,
            device: device.clone(),
            layout: GsPipelineLayout { handle: layout },
            pass,
            frame_count,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn destroy(&self) {

        unsafe {
            self.device.handle.destroy_pipeline(self.handle, None);
        }
        self.layout.destroy(&self.device);
        self.pass.destroy(&self.device);
    }
}


pub trait GsVkPipelineType {
    const BIND_POINT: ash::vk::PipelineBindPoint;
}

impl GsVkPipelineType for Graphics {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::GRAPHICS;
}

impl GsVkPipelineType for Compute {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::COMPUTE;
}
