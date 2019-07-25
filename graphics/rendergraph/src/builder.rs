//! A `GraphBuilder` type that is used to build a `RenderGraph`

use assets;
use gfx::{self, handle};
use shred;

use std::any::Any;
use std::collections::HashMap;
use std::{error, fmt};

use super::pass::Pass;
use super::RenderGraph;

/// An error while accessing the output of a pass
#[derive(Debug)]
pub struct PassOutputError {
    name: String,
    kind: PassOutputErrorKind,
}

impl PassOutputError {
    /// Returns a new `PassOutputError`, with the provided name and kind
    pub fn new(name: String, kind: PassOutputErrorKind) -> Self {
        Self { name, kind }
    }

    /// Returns the name of the pass
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
/// The kind of of `PassOutputError`
pub enum PassOutputErrorKind {
    /// The output of a pass was not found
    NotFound,
    /// The output of a pass was found, but it did not have the requested type
    DowncastError,
}

impl fmt::Display for PassOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            PassOutputErrorKind::NotFound => {
                writeln!(f, "The output of the pass `{}` was not found", self.name)
            }
            PassOutputErrorKind::DowncastError => writeln!(
                f,
                "The output of the pass `{}` was not of the expected type",
                self.name
            ),
        }
    }
}

impl error::Error for PassOutputError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// A `GraphBuilder`
pub struct GraphBuilder<'a, R, C, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R> + 'a,
    F: gfx::Factory<R> + 'a,
{
    /// A mutable reference to the `Factory` for asset creation
    pub factory: &'a mut F,
    /// A reference to the `Assets` for asset path calculation
    pub assets: &'a assets::Assets,
    passes: Vec<Box<Pass<R, C, F, CF, DF>>>,
    pass_outputs: HashMap<String, Box<Any>>,
    resources: shred::Resources,
    main_color: handle::RenderTargetView<R, CF>,
    main_depth: handle::DepthStencilView<R, DF>,
}

impl<'a, R, C, F, CF, DF> GraphBuilder<'a, R, C, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    /// Returns a new `GraphBuilder`, using `factory` to create resources, `assets` to calculate
    /// asset paths, and `main_color` and `main_depth` as the main targets (these should be acquired
    /// from a backend crate such as `gfx_window_glutin`).
    pub fn new(
        factory: &'a mut F,
        assets: &'a assets::Assets,
        main_color: handle::RenderTargetView<R, CF>,
        main_depth: handle::DepthStencilView<R, DF>,
    ) -> Self {
        Self {
            passes: Vec::new(),
            pass_outputs: HashMap::new(),
            factory,
            resources: shred::Resources::new(),
            main_color,
            main_depth,
            assets,
        }
    }

    /// Adds the output of a pass for other passes to use
    pub fn add_pass_output<S, O>(&mut self, name: S, pass_output: O)
    where
        S: Into<String>,
        O: Any,
    {
        let name = name.into();
        let pass_output: Box<O> = pass_output.into();
        assert!(
            self.pass_outputs
                .insert(name.clone(), pass_output)
                .is_none(),
            "A pass output with this name has already been added: {}",
            name
        );
    }

    /// Adds the provided pass to the `GraphBuilder`
    pub fn add_pass<P>(&mut self, pass: P)
    where
        P: Pass<R, C, F, CF, DF> + 'static,
    {
        let pass: Box<P> = pass.into();
        self.passes.push(pass);
    }

    /// Adds the resource to the `GraphBuilder`
    pub fn add_resource<Res: shred::Resource>(&mut self, resource: Res) {
        self.resources.insert(resource);
    }

    /// Returns a reference to the `GraphBuilder`'s resources
    pub fn get_resources(&self) -> &shred::Resources {
        &self.resources
    }

    /// Returns a mutable reference to the `GraphBuilder`'s resources
    pub fn get_mut_resources(&mut self) -> &mut shred::Resources {
        &mut self.resources
    }

    /// Returns the resource with the provided name
    ///
    /// If the resource does not exist or it doesn't have the expected type, an error is returned
    /// instead.
    pub fn get_pass_output<T: 'static>(&self, name: &str) -> Result<&T, PassOutputError> {
        self.pass_outputs
            .get(name)
            .ok_or(PassOutputError::new(
                name.to_string(),
                PassOutputErrorKind::NotFound,
            )).and_then(|o| {
                o.downcast_ref().ok_or(PassOutputError::new(
                    name.to_string(),
                    PassOutputErrorKind::DowncastError,
                ))
            })
    }

    /// Returns a reference to the window's color output
    pub fn main_color(&self) -> &handle::RenderTargetView<R, CF> {
        &self.main_color
    }

    /// Returns a reference to the window's depth output
    pub fn main_depth(&self) -> &handle::DepthStencilView<R, DF> {
        &self.main_depth
    }

    /// Builds the `RenderGraph`, using the `GraphBuilder` and some additional types needed for
    /// executing passes
    pub fn build<D>(
        self,
        device: D,
        encoder: gfx::Encoder<R, C>,
    ) -> RenderGraph<R, C, D, F, CF, DF>
    where
        D: gfx::Device<Resources = R, CommandBuffer = C>,
    {
        RenderGraph::new(
            self.passes,
            self.resources,
            encoder,
            device,
            self.main_color,
            self.main_depth,
        )
    }
}
