
#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;

// TODO: Rename the crate in Cargo.toml
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo_macros as gsma;

mod config;
mod input;
mod assets;
mod procedure;
mod toolkit;
mod utils;

pub mod prelude;
