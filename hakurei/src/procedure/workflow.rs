
use gsvk::core::device::HaDevice;
use gsvk::sync::{ HaFence, HaSemaphore };
use gsvk::types::vkuint;

use procedure::env::ProgramEnv;
use procedure::chain::ChainResource;
use procedure::loader::AssetsLoader;
use procedure::loops::RoutineFlow;
use procedure::error::{ RuntimeError, ProcedureError };

use input::{ ActionNerve, SceneAction };


pub trait GraphicsRoutine {

    // lifetime
    #[allow(unused_variables)]
    fn ready(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {
        Ok(())
    }

    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, delta_time: f32) -> Result<&HaSemaphore, ProcedureError>;

    #[allow(unused_variables)]
    fn closure(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {
        Ok(())
    }

    fn clean_resources(&mut self, device: &HaDevice) -> Result<(), ProcedureError>;

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError>;

    fn clean_routine(&mut self, device: &HaDevice);

    // input
    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction;
}

pub struct RoutineBuilder<'env> {

    env: &'env ProgramEnv,
    chain: ChainResource,
}

impl<'env, 'a> RoutineBuilder<'env> where 'env: 'a {

    pub(super) fn new(env: &'env mut ProgramEnv) -> Result<RoutineBuilder<'env>, RuntimeError> {

        let window = env.window()?;
        let chain = ChainResource::new(env, window)?;

        let builder = RoutineBuilder {
            env, chain,
        };

        Ok(builder)
    }

    pub fn assets_loader(&'env self) -> AssetsLoader<'a> {

        self.chain.assets_loader(&self.env.vulkan_env, &self.env.config.resources)
    }

    pub fn build<Routine>(self, routine: Routine) -> RoutineFlow<Routine> where Routine: GraphicsRoutine {

        RoutineFlow::new(routine, self.chain)
    }
}
