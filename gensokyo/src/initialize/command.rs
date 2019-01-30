
use ash::vk;

use gsvk::core::device::DeviceQueueIdentifier;

use gsvk::command::{ GsCommandBuffer, GsCommandPool };
use gsvk::command::{ GsVkCommandType, GsCmdRecorder };
use gsvk::command::CmdPipelineAbs;

use gsvk::pipeline::target::GsVkPipelineType;
use gsvk::utils::phantom::Transfer;

use crate::initialize::initializer::AssetInitializer;
use crate::initialize::traits::{ TryFromInitializerP1, FromInitializerP1, FromInitializerP2 };
use crate::error::GsResult;

impl TryFromInitializerP1<DeviceQueueIdentifier> for GsCommandPool {

    fn new(initializer: &AssetInitializer, queue: DeviceQueueIdentifier) -> GsResult<GsCommandPool> {
        Ok(GsCommandPool::create(&initializer.device, queue, vk::CommandPoolCreateFlags::empty())?)
    }
}

impl FromInitializerP1<GsCommandBuffer> for GsCmdRecorder<Transfer> {

    fn new(initializer: &AssetInitializer, command: GsCommandBuffer) -> GsCmdRecorder<Transfer> {
        GsCmdRecorder::create_copy(&initializer.device, command)
    }
}

impl<'a, T, P> FromInitializerP2<&P, GsCommandBuffer> for GsCmdRecorder<T>
    where
        P: CmdPipelineAbs,
        T: GsVkPipelineType + GsVkCommandType {

    fn new(initializer: &AssetInitializer, pipeline: &P, command: GsCommandBuffer) -> GsCmdRecorder<T> {

        GsCmdRecorder::create(&initializer.device, command, pipeline)
    }
}
