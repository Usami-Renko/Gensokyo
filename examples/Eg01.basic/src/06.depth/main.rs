
mod data;
mod program;

#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;
extern crate cgmath;

use hakurei::prelude::*;

const MANIFEST_PATH: &str = "src/06.depth/hakurei.toml";

use self::program::DepthProcedure;
use std::path::PathBuf;

fn main() {

    let procecure = DepthProcedure::new(Dimension2D { width: 800, height: 600 });

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program = ProgramEnv::new(Some(manifest), procecure).unwrap();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
