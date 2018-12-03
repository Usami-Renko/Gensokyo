
#![recursion_limit = "128"]

extern crate winit;
#[macro_use]
extern crate ash;
extern crate num;
extern crate image;
extern crate smallvec;
extern crate cgmath;
extern crate tobj;
extern crate gltf;
#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate gensokyo_vulkan as gsvk;
#[macro_use]
extern crate gensokyo_macros;

mod config;
mod input;
mod assets;
mod procedure;
mod toolkit;
mod utils;

pub mod prelude;
