
mod data;
mod program;

#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;
extern crate cgmath;

use hakurei::prelude::*;
use hakurei::prelude::config::*;


const WINDOW_TITLE: &'static str = "06.Depth";

pub const WINDOW_WIDTH:  u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

use self::program::DepthProcedure;

fn main() {

    let mut config = EngineConfig::default();
    config.window.dimension = Dimension2D {
        width : WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    };
    config.window.title = String::from(WINDOW_TITLE);

    // feature FillModeNonSolid is need when render triangle in line mode.
    // config.core.device.features.push(PhysicalFeatureType::FillModeNonSolid);

    // specify the format of depth buffer in all pipeline or leave it default value.
    config.pipeline.depth_stencil.prefer_depth_stencil_formats = vec![Format::D32Sfloat];

    let procecure = DepthProcedure::new();

    let mut program = ProgramEnv::new(config, procecure);

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
