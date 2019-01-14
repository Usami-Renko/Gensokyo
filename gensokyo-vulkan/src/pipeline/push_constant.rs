
use ash::vk;

use crate::core::physical::GsPhysicalDevice;
use crate::pipeline::layout::ToPushConstant;
use crate::pipeline::target::GsPipelineStage;
use crate::pipeline::error::PipelineError;

use serde::ser::Serialize;

use std::mem;

pub struct GsPushConstant {

    data: Vec<u8>,
    range: vk::PushConstantRange,
}

impl GsPushConstant {

    pub fn new<D>(physical: &GsPhysicalDevice, stage: GsPipelineStage, data: &D) -> Result<GsPushConstant, PipelineError>
        where D: Serialize {

        let data_serialized = bincode::serialize(data)
            .map_err(|e| PipelineError::PushConstSerialize(e))?;
        let data_size = mem::size_of_val(&data_serialized) as _;

        if data_size > physical.properties.limits().max_push_constants_size {

            Err(PipelineError::PushConstReachMaxSize)
        } else {

            let result = GsPushConstant {
                range: vk::PushConstantRange {
                    stage_flags: stage.0,
                    offset: 0,
                    size: data_size,
                },
                data: data_serialized,
            };
            Ok(result)
        }
    }

    pub(crate) fn range(&self) -> vk::PushConstantRange {
        self.range.clone()
    }

    pub(crate) fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl ToPushConstant for GsPushConstant {

    fn push_constant(&self) -> &GsPushConstant {
        &self
    }
}
