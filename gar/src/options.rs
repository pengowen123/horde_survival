//! Graphics configuration
//!
//! If the `serde` feature is used, all types in this module implement `Serialize` and
//! `Deserialize`.

use glutin;

/// Options for the graphics context
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Options {
    pub context_options: ContextOptions,
    pub renderer_options: RendererOptions,
}

/// Options for the graphics context
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContextOptions {}

/// Options for the renderer
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RendererOptions {}
