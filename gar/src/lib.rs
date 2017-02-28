#![warn(missing_docs)]
#![cfg_attr(feature = "clippy", warn(missing_docs_in_private_items))]
#![warn(unused_extern_crates)]

extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;
extern crate shader_version;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod context;
pub mod options;
pub mod draw;
pub mod backend;
mod render;

pub use context::Context;
