
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::layout::GsPipelineLayout;
use crate::pipeline::pass::GsRenderPass;
use crate::pipeline::error::PipelineError;

pub struct GsGraphicsPipeline {

    pub(crate) handle: vk::Pipeline,
    pub(crate) pass  : GsRenderPass,
    pub(crate) layout: GsPipelineLayout,

    device: GsDevice,

    bind_point: vk::PipelineBindPoint,
    frame_count: usize,
}

impl GsGraphicsPipeline {

    pub(super) fn new(device: &GsDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: GsRenderPass) -> GsGraphicsPipeline {

        let frame_count = pass.frame_count();

        GsGraphicsPipeline {
            handle,
            device: device.clone(),
            layout: GsPipelineLayout { handle: layout },
            pass,
            bind_point: vk::PipelineBindPoint::GRAPHICS,
            frame_count,
        }
    }

    pub fn bind_point(&self) -> vk::PipelineBindPoint {
        self.bind_point
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

pub struct GraphicsPipelineContainer {

    pipelines: Vec<Option<GsGraphicsPipeline>>,
}

impl GraphicsPipelineContainer {

    pub(crate) fn new(pipelines: Vec<GsGraphicsPipeline>) -> GraphicsPipelineContainer {

        let pipelines = pipelines.into_iter()
            .map(|p| Some(p))
            .collect();

        GraphicsPipelineContainer {
            pipelines,
        }
    }

    pub fn take_at(&mut self, pipeline_index: usize) -> Result<GsGraphicsPipeline, PipelineError> {

        self.pipelines[pipeline_index].take()
            .ok_or(PipelineError::PipelineTakeError)
    }
}
