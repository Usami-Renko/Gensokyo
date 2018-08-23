
pub mod builder;

// TODO: Remove this module in the future.
pub mod tmp;

use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::layout::HaPipelineLayout;

use constant::VERBOSE;

pub struct GraphicsPipeline {

    handle: vk::Pipeline,
    layout: HaPipelineLayout,
    render_pass: vk::RenderPass,
}

impl GraphicsPipeline {

    pub fn clean(&self, device: &HaLogicalDevice) {
        unsafe { device.handle.destroy_pipeline(self.handle, None); }
        self.layout.cleanup(device);
        unsafe { device.handle.destroy_render_pass(self.render_pass, None); }

        if VERBOSE {
            println!("[Info] Graphics Pipeline has been destory.");
        }
    }
}
