
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::layout::HaPipelineLayout;
use pipeline::pass::HaRenderPass;

pub struct HaGraphicsPipeline {

    pub(crate) device: Option<HaDevice>,
    pub(crate) handle: vk::Pipeline,
    pub pass: HaRenderPass,
    pub(crate) layout: HaPipelineLayout,

    pub(crate) bind_point: vk::PipelineBindPoint,
}

impl HaGraphicsPipeline {

    pub fn uninitialize() -> HaGraphicsPipeline {
        HaGraphicsPipeline {
            device: None,
            handle: vk::Pipeline::null(),
            pass: HaRenderPass::uninitialize(),
            layout: HaPipelineLayout::uninitialize(),

            bind_point: vk::PipelineBindPoint::Graphics,
        }
    }

    pub(super) fn new(device: &HaDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: HaRenderPass) -> HaGraphicsPipeline {
        HaGraphicsPipeline {
            device: Some(device.clone()),
            handle,
            layout: HaPipelineLayout::new(layout),
            pass,

            bind_point: vk::PipelineBindPoint::Graphics,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.pass.framebuffers.len()
    }

    pub fn cleanup(&self) {

        if let Some(ref device) = self.device {
            unsafe { device.handle.destroy_pipeline(self.handle, None); }
            self.layout.cleanup(device);
            self.pass.cleanup(device);
        }
    }
}
