
mod data;
mod program;

#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;
extern crate cgmath;

use hakurei::prelude::*;

const MANIFEST_PATH: &'static str = "src/01.triangle/hakurei.toml";

use self::program::ModelProcedure;
use std::path::PathBuf;

fn main() {

    let procecure = ModelProcedure::new();

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program = ProgramEnv::new(Some(manifest), procecure).unwrap();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
