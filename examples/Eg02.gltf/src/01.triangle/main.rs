
pub mod input_desc;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/gensokyo.toml";
const VERTEX_SHADER_SOURCE_PATH  : &str = "src/01.triangle/triangle.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/01.triangle/triangle.frag";
const MODEL_GLTF_PATH: &str = "src/01.triangle/triangle.gltf";

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
