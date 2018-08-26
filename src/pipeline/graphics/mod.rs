
pub mod builder;

use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::layout::HaPipelineLayout;
use pipeline::pass::HaRenderPass;

pub struct HaGraphicsPipeline {

    pub(crate) handle: vk::Pipeline,
    layout: HaPipelineLayout,
    pub(crate) pass: HaRenderPass,

    pub(crate) bind_point: vk::PipelineBindPoint,
}

impl HaGraphicsPipeline {

    pub(super) fn new(handle: vk::Pipeline, layout: vk::PipelineLayout, pass: HaRenderPass) -> HaGraphicsPipeline {
        HaGraphicsPipeline {
            handle,
            layout: HaPipelineLayout::new(layout),
            pass,

            bind_point: vk::PipelineBindPoint::Graphics,
        }
    }

    pub(crate) fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe { device.handle.destroy_pipeline(self.handle, None); }
        self.layout.cleanup(device);
        self.pass.cleanup(device);
    }
}
