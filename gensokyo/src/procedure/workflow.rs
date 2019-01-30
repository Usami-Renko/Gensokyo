
use gsvk::core::GsDevice;
use gsvk::sync::{ GsFence, GsSemaphore };

use crate::procedure::context::ProgramContext;
use crate::procedure::chain::ChainResource;
use crate::procedure::loops::RoutineFlow;
use crate::initialize::initializer::AssetInitializer;

use crate::input::{ ActionNerve, SceneAction };
use crate::error::GsResult;


pub trait GraphicsRoutine {

    // lifetime
    #[allow(unused_variables)]
    fn ready(&mut self, device: &GsDevice) -> GsResult<()> {
        Ok(())
    }

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, delta_time: f32) -> GsResult<&GsSemaphore>;

    #[allow(unused_variables)]
    fn closure(&mut self, device: &GsDevice) -> GsResult<()> {
        // Do nothing...
        Ok(())
    }

    fn clean_resources(&mut self, _device: &GsDevice) -> GsResult<()> {
        // Do nothing.
        Ok(())
    }

    fn reload_res(&mut self, initializer: AssetInitializer) -> GsResult<()>;

    fn clean_routine(&mut self, _device: &GsDevice) {
        // Empty...
    }

    // input
    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction;
}

pub struct RoutineBuilder<'env> {

    context: &'env ProgramContext,
    chain: ChainResource,
}

impl<'env> RoutineBuilder<'env> {

    pub(super) fn new(context: &'env mut ProgramContext) -> GsResult<RoutineBuilder<'env>> {

        let window = context.window()?;
        let chain = ChainResource::new(context, window)?;

        let builder = RoutineBuilder { context, chain };
        Ok(builder)
    }

    pub fn assets_loader(&self) -> AssetInitializer {

        self.chain.assets_loader(&self.context.vulkan_context, &self.context.config.resources)
    }

    pub fn build<Routine>(self, routine: Routine) -> RoutineFlow<Routine>
        where
            Routine: GraphicsRoutine {

        RoutineFlow::new(routine, self.chain)
    }
}
