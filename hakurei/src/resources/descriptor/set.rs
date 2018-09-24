
use ash::vk;

use core::device::HaDevice;

use resources::descriptor::layout::HaDescriptorSetLayout;

pub(crate) struct HaDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    pub(crate) layout: HaDescriptorSetLayout,
}

impl HaDescriptorSet {

    pub(crate) fn cleanup(&self, device: &HaDevice) {
        self.layout.cleanup(device);
    }
}
