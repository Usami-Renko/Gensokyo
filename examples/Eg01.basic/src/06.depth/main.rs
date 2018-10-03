
mod program;
mod data;

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
    config.core.device.features.push(PhysicalFeatureType::FillModeNonSolid);

    let procecure = DepthProcedure::new();

    let mut program = ProgramEnv::new(config, procecure);

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
