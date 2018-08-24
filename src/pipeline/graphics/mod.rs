
pub mod builder;

use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::layout::HaPipelineLayout;
use pipeline::pass::HaRenderPass;

pub struct GraphicsPipeline {

    handle: vk::Pipeline,
    layout: HaPipelineLayout,
    pass:   HaRenderPass,
}

impl GraphicsPipeline {

    pub fn clean(&self, device: &HaLogicalDevice) {
        unsafe { device.handle.destroy_pipeline(self.handle, None); }
        self.layout.cleanup(device);
        self.pass.cleanup(device);
    }
}
