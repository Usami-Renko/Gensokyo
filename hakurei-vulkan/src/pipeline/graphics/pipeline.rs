
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::layout::HaPipelineLayout;
use pipeline::pass::HaRenderPass;
use pipeline::error::PipelineError;

pub struct HaGraphicsPipeline {

    pub(crate) handle: vk::Pipeline,

    device: Option<HaDevice>,
    pass  : HaRenderPass,
    layout: HaPipelineLayout,

    bind_point: vk::PipelineBindPoint,
    frame_count: usize,
}

impl HaGraphicsPipeline {

    pub fn uninitialize() -> HaGraphicsPipeline {

        HaGraphicsPipeline {
            device: None,
            handle: vk::Pipeline::null(),
            pass: HaRenderPass::uninitialize(),
            layout: HaPipelineLayout::uninitialize(),

            bind_point: vk::PipelineBindPoint::Graphics,
            frame_count: 0,
        }
    }

    pub(super) fn new(device: &HaDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: HaRenderPass) -> HaGraphicsPipeline {

        let frame_count = pass.framebuffers.len();

        HaGraphicsPipeline {
            device: Some(device.clone()),
            handle,
            layout: HaPipelineLayout::new(layout),
            pass,

            bind_point: vk::PipelineBindPoint::Graphics,
            frame_count,
        }
    }

    pub(crate) fn pass(&self) -> &HaRenderPass {
        &self.pass
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub(crate) fn bind_point(&self) -> vk::PipelineBindPoint {
        self.bind_point
    }

    pub(crate) fn layout(&self) -> &HaPipelineLayout {
        &self.layout
    }

    pub fn cleanup(&self) {

        if let Some(ref device) = self.device {

            unsafe {
                device.handle.destroy_pipeline(self.handle, None);
            }
            self.layout.cleanup(device);
            self.pass.cleanup(device);
        }
    }
}

pub struct GraphicsPipelineContainer {

    pipelines: Vec<Option<HaGraphicsPipeline>>,
}

impl GraphicsPipelineContainer {

    pub(super) fn new(pipelines: Vec<HaGraphicsPipeline>) -> GraphicsPipelineContainer {

        let pipelines = pipelines.into_iter()
            .map(|p| Some(p))
            .collect();

        GraphicsPipelineContainer {
            pipelines,
        }
    }

    pub fn take_at(&mut self, pipeline_index: usize) -> Result<HaGraphicsPipeline, PipelineError> {

        self.pipelines[pipeline_index].take()
            .ok_or(PipelineError::PipelineTakeError)
    }
}
