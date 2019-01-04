
// TODO: Rename crate in Cargo.toml.
extern crate gensokyo as gs;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo_macros as gsma;

pub mod input_desc;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/gensokyo.toml";
const VERTEX_SHADER_SOURCE_PATH  : &str = "src/02.box/box.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/02.box/box.frag";
const MODEL_GLTF_PATH: &str = "src/02.box/Box.gltf";

use example02::FilePathConstants;
use example02::program::GltfModelViewer;
use std::path::PathBuf;

fn main() {

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let paths = FilePathConstants {
        vertex_shader  : VERTEX_SHADER_SOURCE_PATH,
        framment_shader: FRAGMENT_SHADER_SOURCE_PATH,
        model_path     : MODEL_GLTF_PATH,
    };

    let builder = program_env.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = GltfModelViewer::<input_desc::Vertex>::new(asset_loader, paths).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
