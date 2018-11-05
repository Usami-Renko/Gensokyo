
#[macro_use]
extern crate ash;
extern crate winit;
extern crate num;
extern crate image;
extern crate smallvec;
extern crate cgmath;
extern crate shaderc;
extern crate tobj;
extern crate gltf;
#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate hakurei_macros;

#[cfg(target_os = "macos")]
extern crate metal_rs;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "windows")]
extern crate winapi;

mod config;
mod core;
mod procedure;
mod pipeline;
mod resources;
mod sync;
mod input;
mod utility;

pub mod prelude;
