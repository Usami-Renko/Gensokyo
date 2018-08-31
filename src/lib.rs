
extern crate winit;
#[macro_use]
extern crate ash;
extern crate num;

#[cfg(target_os = "macos")]
extern crate metal_rs;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "windows")]
extern crate winapi;

mod constant;
mod core;
mod procedure;
mod utility;

pub mod pipeline;
pub mod resources;
pub mod prelude;
pub mod sync;

