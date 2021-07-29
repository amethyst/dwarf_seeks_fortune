#![forbid(unsafe_code)]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations
)]

//! This crate houses all the core game code; everything that is needed to actually play the game.

#[macro_use]
extern crate log;

pub mod components;
pub mod entities;
pub mod levels;
pub mod resources;
pub mod states;
pub mod systems;
pub mod utility;
