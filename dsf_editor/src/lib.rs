#![forbid(unsafe_code)]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    clippy::all,
    clippy::doc_markdown,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

//! This crate houses all code for the level editor.

#[macro_use]
extern crate log;

pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
