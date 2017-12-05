//! Systems and components to abstract the use of the game window

pub mod info;
pub mod event;

use glutin;

use std::sync::Arc;

pub type Window = Arc<glutin::GlWindow>;
