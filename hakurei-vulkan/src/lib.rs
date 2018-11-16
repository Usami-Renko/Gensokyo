
#[macro_use]
extern crate ash;
extern crate winit;
extern crate num;
extern crate image;
extern crate shaderc;
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

pub mod core;
pub mod pipeline;
pub mod resources;
pub mod utils;

// TODO: Remove this const variable.
const VERBOSE: bool = false;
