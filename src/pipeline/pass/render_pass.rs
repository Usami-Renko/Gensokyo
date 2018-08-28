
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use std::ptr;


pub struct HaRenderPass {

    pub handle: vk::RenderPass,
    pub clear_values: Vec<vk::ClearValue>,
}

impl HaRenderPass {

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_render_pass(self.handle, None);}
    }
}



/// Create Render Pass
pub fn temp_render_pass(device: &HaLogicalDevice) -> HaRenderPass {

    let color_attchemnt = vk::AttachmentDescription {
        flags: vk::AttachmentDescriptionFlags::empty(),
        format: vk::Format::B8g8r8a8Unorm,
        samples: vk::SAMPLE_COUNT_1_BIT,
        load_op: vk::AttachmentLoadOp::Clear,
        store_op: vk::AttachmentStoreOp::Store,
        stencil_load_op: vk::AttachmentLoadOp::DontCare,
        stencil_store_op: vk::AttachmentStoreOp::DontCare,
        initial_layout: vk::ImageLayout::Undefined,
        final_layout: vk::ImageLayout::PresentSrcKhr,
    };

    let color_attachment_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::ColorAttachmentOptimal,
    };

    let subpass = vk::SubpassDescription {
        flags: vk::SubpassDescriptionFlags::empty(),
        pipeline_bind_point: vk::PipelineBindPoint::Graphics,
        input_attachment_count: 0,
        p_input_attachments: ptr::null(),
        color_attachment_count: 1,
        p_color_attachments: &color_attachment_ref,
        p_resolve_attachments: ptr::null(),
        p_depth_stencil_attachment: ptr::null(),
        preserve_attachment_count: 0,
        p_preserve_attachments: ptr::null(),
    };

    let render_pass_attachemnts = [
        color_attchemnt,
    ];

    let renderpass_create_info = vk::RenderPassCreateInfo {
        s_type: vk::StructureType::RenderPassCreateInfo,
        p_next: ptr::null(),
        flags: vk::RenderPassCreateFlags::empty(),
        attachment_count: render_pass_attachemnts.len() as u32,
        p_attachments: render_pass_attachemnts.as_ptr(),
        subpass_count: 1,
        p_subpasses: &subpass,
        dependency_count: 0,
        p_dependencies: ptr::null(),
    };

    let handle = unsafe {
        device.handle.create_render_pass(&renderpass_create_info, None)
            .unwrap()
    };

    HaRenderPass {
        handle,
        clear_values: vec![
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                }
            }
        ],
    }
}
