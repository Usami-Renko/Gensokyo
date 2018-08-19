
extern crate winit;
#[macro_use]
extern crate ash;

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
mod structures;
mod window;
mod utility;

pub mod prelude;
