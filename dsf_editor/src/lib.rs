#![forbid(unsafe_code)]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations
)]

//! This crate houses all code for the level editor.

#[macro_use]
extern crate log;

pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
