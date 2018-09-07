
#[macro_use]
extern crate ash;
extern crate winit;
extern crate num;
extern crate image;

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
mod utility;

pub mod pipeline;
pub mod resources;
pub mod prelude;
pub mod sync;
