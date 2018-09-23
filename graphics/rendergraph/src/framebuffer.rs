//! A `Framebuffers` type that stores framebuffers so that they can be accessed by multiple render
//! passes

use gfx::{self, handle};

use std::any::Any;
use std::collections::HashMap;
use std::{error, fmt};

/// An error while accessing a framebuffer
#[derive(Debug)]
pub struct FramebufferError {
    name: String,
    kind: FramebufferErrorKind,
}

impl FramebufferError {
    /// Returns a new `FramebufferError`, with the provided name and kind
    pub fn new(name: String, kind: FramebufferErrorKind) -> Self {
        Self { name, kind }
    }

    /// Returns the name of the framebuffer
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
/// The kind of of `FramebufferError`
pub enum FramebufferErrorKind {
    /// A framebuffer was not found
    NotFound,
    /// A framebuffer, but it did not have the requested type
    DowncastError,
}

impl fmt::Display for FramebufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FramebufferErrorKind::NotFound => writeln!(f, "Framebuffer not found: {}", self.name),
            FramebufferErrorKind::DowncastError => {
                writeln!(f, "Framebuffer was not of the expected type: {}", self.name)
            }
        }
    }
}

impl error::Error for FramebufferError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// A list of framebuffers
pub struct Framebuffers<R: gfx::Resources, CF, DF> {
    main_color: handle::RenderTargetView<R, CF>,
    main_depth: handle::DepthStencilView<R, DF>,
    framebuffers: HashMap<String, Box<Any>>,
}

impl<R: gfx::Resources, CF, DF> Framebuffers<R, CF, DF> {
    /// Returns a new `Framebuffers`, with the provided main color and depth targets
    pub fn new(
        main_color: handle::RenderTargetView<R, CF>,
        main_depth: handle::DepthStencilView<R, DF>,
    ) -> Self {
        Framebuffers {
            main_color,
            main_depth,
            framebuffers: HashMap::new(),
        }
    }

    /// Returns the main color framebuffer
    pub fn get_main_color(&self) -> &handle::RenderTargetView<R, CF> {
        &self.main_color
    }

    /// Returns the main depth framebuffer
    pub fn get_main_depth(&self) -> &handle::DepthStencilView<R, DF> {
        &self.main_depth
    }

    /// Adds a framebuffer
    pub fn add_framebuffer<S, F>(&mut self, name: S, framebuffer: F)
    where
        S: Into<String>,
        F: Any,
    {
        let name = name.into();
        let framebuffer: Box<F> = framebuffer.into();
        assert!(
            self.framebuffers
                .insert(name.clone(), framebuffer)
                .is_none(),
            "A framebuffer with this name has already been added: {}",
            name
        );
    }

    /// Returns the framebuffer with the provided name
    ///
    /// If the framebuffer does not exist or it doesn't have the expected type, an error is returned
    /// instead.
    pub fn get_framebuffer<T: 'static>(&self, name: &str) -> Result<&T, FramebufferError> {
        self.framebuffers
            .get(name)
            .ok_or(FramebufferError::new(
                name.to_string(),
                FramebufferErrorKind::NotFound,
            )).and_then(|o| {
                o.downcast_ref().ok_or(FramebufferError::new(
                    name.to_string(),
                    FramebufferErrorKind::DowncastError,
                ))
            })
    }
}
