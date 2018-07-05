//! Error types for this crate

use gfx;
use glutin;
use image_utils;

use std::{fmt, io, error};

use builder::PassOutputError;

/// An error while building a `RenderGraph` or some component of it (such as when reloading shaders)
#[derive(Debug)]
pub enum BuildError<S> {
    /// Pass output access error
    PassOutput(PassOutputError),
    /// Pipeline state creation error
    PipelineState(gfx::PipelineStateError<S>),
    /// Buffer creation error
    BufferCreation(gfx::buffer::CreationError),
    /// Texture creation error
    TextureCreation(gfx::texture::CreationError),
    /// Color or depth target creation error
    TargetCreation(gfx::TargetViewError),
    /// Resource view creation error
    ResourceCreation(gfx::ResourceViewError),
    /// Texture, target, or resource view creation error
    Creation(gfx::CombinedError),
    /// Image error
    Image(image_utils::ImageError),
    /// Program linking error
    Program(gfx::shade::ProgramError),
    /// An I/O error, and the path of the file being accessed
    Io(io::Error, String),
    /// A string variant for convenience
    String(String),
    /// Custom error
    Custom(Box<(error::Error + 'static)>),
}

impl<S: fmt::Debug + fmt::Display> fmt::Display for BuildError<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::BuildError::*;
        match self {
            PassOutput(e) => writeln!(fmt, "Error accessing pass output: {}", e),
            PipelineState(e) => writeln!(fmt, "Error building pipeline state: {}", e),
            BufferCreation(e) => writeln!(fmt, "Error creating buffer: {}", e),
            TextureCreation(e) => writeln!(fmt, "Error creating texture: {}", e),
            TargetCreation(e) => writeln!(fmt, "Error creating target: {}", e),
            ResourceCreation(e) => writeln!(fmt, "Error creating resource view: {}", e),
            Creation(e) =>
                writeln!(fmt, "Error creating target, texture, or resource view: {}", e),
            Image(e) => writeln!(fmt, "Image error: {}", e),
            Program(e) => writeln!(fmt, "Program linking error: {}", e),
            Io(e, path) => writeln!(fmt, "Io error (at path `{}`): {}", path, e),
            String(e) => writeln!(fmt, "Error: {}", e),
            Custom(e) => writeln!(fmt, "Custom error: {}", e),
        }
    }
}

impl<S: fmt::Debug + fmt::Display + 'static> error::Error for BuildError<S> {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl<S> From<PassOutputError> for BuildError<S> {
    fn from(e: PassOutputError) -> Self {
        BuildError::PassOutput(e)
    }
}

impl<S> From<gfx::PipelineStateError<S>> for BuildError<S> {
    fn from(e: gfx::PipelineStateError<S>) -> Self {
        BuildError::PipelineState(e)
    }
}

impl<S> From<gfx::buffer::CreationError> for BuildError<S> {
    fn from(e: gfx::buffer::CreationError) -> Self {
        BuildError::BufferCreation(e)
    }
}

impl<S> From<gfx::texture::CreationError> for BuildError<S> {
    fn from(e: gfx::texture::CreationError) -> Self {
        BuildError::TextureCreation(e)
    }
}

impl<S> From<gfx::TargetViewError> for BuildError<S> {
    fn from(e: gfx::TargetViewError) -> Self {
        BuildError::TargetCreation(e)
    }
}

impl<S> From<gfx::ResourceViewError> for BuildError<S> {
    fn from(e: gfx::ResourceViewError) -> Self {
        BuildError::ResourceCreation(e)
    }
}

impl<S> From<gfx::CombinedError> for BuildError<S> {
    fn from(e: gfx::CombinedError) -> Self {
        BuildError::Creation(e)
    }
}

impl<S> From<image_utils::ImageError> for BuildError<S> {
    fn from(e: image_utils::ImageError) -> Self {
        BuildError::Image(e)
    }
}

impl<S> From<image_utils::TextureError> for BuildError<S> {
    fn from(e: image_utils::TextureError) -> Self {
        match e {
            image_utils::TextureError::Image(e) => BuildError::Image(e),
            image_utils::TextureError::Creation(e) => BuildError::Creation(e),
        }
    }
}

impl<S> From<gfx::shade::ProgramError> for BuildError<S> {
    fn from(e: gfx::shade::ProgramError) -> Self {
        BuildError::Program(e)
    }
}

impl<S> From<String> for BuildError<S> {
    fn from(e: String) -> Self {
        BuildError::String(e)
    }
}

/// An error while running a `RenderGraph`
#[derive(Debug)]
pub enum RunError {
    /// Buffer update error
    BufferUpdate(gfx::UpdateError<usize>),
    /// A GL context error
    ContextError(glutin::ContextError),
    /// A string variant for convenience
    String(String),
    /// A custom error
    Custom(Box<(error::Error + 'static)>),
}

impl fmt::Display for RunError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RunError::*;
        match self {
            BufferUpdate(e) => writeln!(fmt, "Error updating buffer: {}", e),
            ContextError(e) => writeln!(fmt, "Error manipulating GL context: {}", e),
            String(e) => writeln!(fmt, "Error: {}", e),
            Custom(e) => writeln!(fmt, "Custom error: {}", e)
        }
    }
}

impl error::Error for RunError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<gfx::UpdateError<usize>> for RunError {
    fn from(e: gfx::UpdateError<usize>) -> Self {
        RunError::BufferUpdate(e)
    }
}

impl From<glutin::ContextError> for RunError {
    fn from(e: glutin::ContextError) -> Self {
        RunError::ContextError(e)
    }
}

impl From<String> for RunError {
    fn from(e: String) -> Self {
        RunError::String(e)
    }
}
