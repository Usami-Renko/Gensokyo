
#[macro_use]
extern crate ash;
extern crate winit;
extern crate num;
extern crate shaderc;
#[macro_use]
extern crate hakurei_macros;

#[cfg(target_os = "macos")]
extern crate metal;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "windows")]
extern crate winapi;

pub mod core;
pub mod pipeline;

pub mod memory;
pub mod buffer;
pub mod image;
pub mod descriptor;
pub mod command;
pub mod sync;

pub mod utils;
pub mod types;

// TODO: Remove this const variable.
const VERBOSE: bool = false;
