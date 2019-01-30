
use gsvk::sync::GsSemaphore;
use gsvk::sync::GsFence;

use crate::initialize::initializer::AssetInitializer;
use crate::initialize::traits::{ TryFromInitializer, TryFromInitializerP1 };
use crate::error::GsResult;

impl TryFromInitializerP1<bool> for GsFence {

    fn new(initializer: &AssetInitializer, is_sign: bool) -> GsResult<GsFence> {
        Ok(GsFence::create(&initializer.device, is_sign)?)
    }
}

impl TryFromInitializer for GsSemaphore {

    fn new(initializer: &AssetInitializer) -> GsResult<GsSemaphore> {
        Ok(GsSemaphore::create(&initializer.device)?)
    }
}

