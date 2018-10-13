//! Error types for this crate

use gfx;
use glutin;
use image_utils;

use std::{error, fmt, io};

use builder::PassOutputError;
use framebuffer::FramebufferError;

/// The top-level error type for this crate
///
/// Stores the name of the pass that caused the error and can be either a `BuildError` or `RunError`
#[derive(Debug)]
pub struct Error<S> {
    pass_name: String,
    kind: ErrorKind<S>,
}

impl<S> Error<S> {
    /// Constructs a new `Error` from the name of the pass that caused it and the error itself
    pub fn new(pass_name: String, kind: ErrorKind<S>) -> Self {
        Self { pass_name, kind }
    }

    /// Returns the name of the pass that caused the error
    pub fn pass_name(&self) -> &str {
        &self.pass_name
    }

    /// Returns the kind of this error
    pub fn error_kind(&self) -> &ErrorKind<S> {
        &self.kind
    }

    /// Consumes the error, returning its error kind
    pub fn into_error_kind(self) -> ErrorKind<S> {
        self.kind
    }
}

impl<S: fmt::Debug + fmt::Display> fmt::Display for Error<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Error in pass `{}`: {}", self.pass_name, self.kind)
    }
}

impl<S: fmt::Debug + fmt::Display + 'static> error::Error for Error<S> {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// A `BuildError` or `RunError`
#[derive(Debug)]
pub enum ErrorKind<S> {
    /// A `BuildError`
    Build(BuildError<S>),
    /// A `RunError`
    Run(RunError),
}

impl<S: fmt::Debug + fmt::Display> fmt::Display for ErrorKind<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Build(ref e) => {
                writeln!(fmt, "Error building component of render graph: {}", e)
            }
            ErrorKind::Run(ref e) => writeln!(fmt, "Error running render graph: {}", e),
        }
    }
}

impl<S: fmt::Debug + fmt::Display + 'static> error::Error for ErrorKind<S> {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// An error while building a `RenderGraph` or some component of it (such as when reloading shaders)
#[derive(Debug)]
pub enum BuildError<S> {
    /// Pass output access error
    PassOutput(PassOutputError),
    /// Framebuffer access error
    Framebuffer(FramebufferError),
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
            Framebuffer(e) => writeln!(fmt, "Error accessing framebuffer: {}", e),
            PipelineState(e) => writeln!(fmt, "Error building pipeline state: {}", e),
            BufferCreation(e) => writeln!(fmt, "Error creating buffer: {}", e),
            TextureCreation(e) => writeln!(fmt, "Error creating texture: {}", e),
            TargetCreation(e) => writeln!(fmt, "Error creating target: {}", e),
            ResourceCreation(e) => writeln!(fmt, "Error creating resource view: {}", e),
            Creation(e) => writeln!(
                fmt,
                "Error creating target, texture, or resource view: {}",
                e
            ),
            Image(e) => writeln!(fmt, "Image error: {}", e),
            Program(e) => writeln!(fmt, "Program linking error: {}", e),
            Io(e, path) => writeln!(fmt, "Io error (at path `{}`): {}", path, e),
            String(e) => writeln!(fmt, "Error: {}", e),
            Custom(e) => writeln!(fmt, "Custom `BuildError`: {}", e),
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

impl<S> From<FramebufferError> for BuildError<S> {
    fn from(e: FramebufferError) -> Self {
        BuildError::Framebuffer(e)
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
    /// A mapping error
    Mapping(gfx::mapping::Error),
    /// A memory copy error
    Copy(gfx::CopyError<usize, usize>),
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
            Mapping(e) => writeln!(fmt, "Error creating mapping: {}", e),
            Copy(e) => writeln!(fmt, "Error copying memory: {}", e),
            String(e) => writeln!(fmt, "Error: {}", e),
            Custom(e) => writeln!(fmt, "Custom `RunError`: {}", e),
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

impl From<gfx::CopyError<usize, usize>> for RunError {
    fn from(e: gfx::CopyError<usize, usize>) -> Self {
        RunError::Copy(e)
    }
}

impl From<glutin::ContextError> for RunError {
    fn from(e: glutin::ContextError) -> Self {
        RunError::ContextError(e)
    }
}

impl From<gfx::mapping::Error> for RunError {
    fn from(e: gfx::mapping::Error) -> Self {
        RunError::Mapping(e)
    }
}

impl From<String> for RunError {
    fn from(e: String) -> Self {
        RunError::String(e)
    }
}
