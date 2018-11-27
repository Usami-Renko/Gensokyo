
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::layout::HaPipelineLayout;
use pipeline::pass::HaRenderPass;
use pipeline::error::PipelineError;

pub struct HaGraphicsPipeline {

    pub(crate) handle: vk::Pipeline,
    pub(crate) pass  : HaRenderPass,
    pub(crate) layout: HaPipelineLayout,

    device: Option<HaDevice>,

    bind_point: vk::PipelineBindPoint,
    frame_count: usize,
}

impl HaGraphicsPipeline {

    pub(super) fn new(device: &HaDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: HaRenderPass) -> HaGraphicsPipeline {

        let frame_count = pass.frame_count();

        HaGraphicsPipeline {
            handle,
            device: Some(device.clone()),
            layout: HaPipelineLayout { handle: layout,  },
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

    pub(crate) fn new(pipelines: Vec<HaGraphicsPipeline>) -> GraphicsPipelineContainer {

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
