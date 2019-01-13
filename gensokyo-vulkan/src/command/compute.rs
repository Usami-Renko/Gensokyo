
use crate::command::record::{ GsCmdRecorder, GsVkCommandType };
use crate::utils::phantom::Compute;

impl GsVkCommandType for Compute {
    // Empty...
}

pub trait GsCmdComputeApi {
    // Yet Empty...
}

impl GsCmdComputeApi for GsCmdRecorder<Compute> {
    // Yet Empty...
}
