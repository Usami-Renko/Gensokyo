
use gsvk::core::device::GsDevice;
use gsvk::sync::{ GsFence, GsSemaphore };

use crate::procedure::env::ProgramEnv;
use crate::procedure::chain::ChainResource;
use crate::procedure::loader::AssetsLoader;
use crate::procedure::loops::RoutineFlow;
use crate::procedure::error::{ RuntimeError, ProcedureError };

use crate::input::{ ActionNerve, SceneAction };


pub trait GraphicsRoutine {

    // lifetime
    #[allow(unused_variables)]
    fn ready(&mut self, device: &GsDevice) -> Result<(), ProcedureError> {
        Ok(())
    }

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, delta_time: f32) -> Result<&GsSemaphore, ProcedureError>;

    #[allow(unused_variables)]
    fn closure(&mut self, device: &GsDevice) -> Result<(), ProcedureError> {
        Ok(())
    }

    fn clean_resources(&mut self, device: &GsDevice) -> Result<(), ProcedureError>;

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError>;

    fn clean_routine(&mut self, device: &GsDevice);

    // input
    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction;
}

pub struct RoutineBuilder<'env> {

    env: &'env ProgramEnv,
    chain: ChainResource,
}

impl<'env> RoutineBuilder<'env> {

    pub(super) fn new(env: &'env mut ProgramEnv) -> Result<RoutineBuilder<'env>, RuntimeError> {

        let window = env.window()?;
        let chain = ChainResource::new(env, window)?;

        let builder = RoutineBuilder {
            env, chain,
        };

        Ok(builder)
    }

    pub fn assets_loader(&self) -> AssetsLoader {

        self.chain.assets_loader(&self.env.vulkan_env, &self.env.config.resources)
    }

    pub fn build<Routine>(self, routine: Routine) -> RoutineFlow<Routine> where Routine: GraphicsRoutine {

        RoutineFlow::new(routine, self.chain)
    }
}
