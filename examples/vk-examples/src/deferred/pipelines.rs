
use gsvk::prelude::command::*;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::descriptor::*;

pub struct PipelineResource {

    pub pipeline: GsPipeline<Graphics>,
    pub viewport: CmdViewportInfo,
    pub descriptor_set: DescriptorSet,
}
