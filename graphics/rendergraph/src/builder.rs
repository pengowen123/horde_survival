use gfx::{self, format, handle};
use shred::{self, Resources};
use glutin;

use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;

use super::pass::Pass;
use super::RenderGraph;

pub type MainColor<R> = handle::RenderTargetView<R, format::Srgba8>;
pub type MainDepth<R> = handle::DepthStencilView<R, format::DepthStencil>;

#[derive(Debug)]
pub enum PassOutputError {
    NotFound(String),
    DowncastError,
}

pub struct GraphBuilder<'a, R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R> + 'a,
      F: gfx::Factory<R> + 'a,
{
    passes: Vec<Box<Pass<R, C>>>,
    pass_outputs: HashMap<String, Box<Any>>,
    resources: Resources,
    factory: &'a mut F,
    encoder: gfx::Encoder<R, C>,
    main_color: MainColor<R>,
    main_depth: MainDepth<R>,
}

impl<'a, R, C, F> GraphBuilder<'a, R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    pub fn new(
        factory: &'a mut F,
        encoder: gfx::Encoder<R, C>,
        main_color: MainColor<R>,
        main_depth: MainDepth<R>,
    ) -> Self {
        Self {
            passes: Vec::new(),
            pass_outputs: HashMap::new(),
            factory,
            encoder,
            resources: Resources::new(),
            main_color,
            main_depth,
        }
    }

    pub fn add_pass<P, S, O>(&mut self, name: S, pass: P, pass_output: O)
        where P: Pass<R, C> + 'static,
              S: Into<String>,
              O: Any
    {
        self.add_pass_no_output(pass);

        let pass_output: Box<O> = pass_output.into();
        self.pass_outputs.insert(name.into(), pass_output);
    }

    pub fn add_pass_no_output<P>(&mut self, pass: P)
        where P: Pass<R, C> + 'static,
    {
        let pass: Box<P> = pass.into();
        self.passes.push(pass);
    }

    pub fn add_resource<Res: shred::Resource>(&mut self, resource: Res) {
        self.resources.add(resource);
    }

    pub fn get_pass_output<T: 'static>(&self, name: &str) -> Result<&T, PassOutputError> {
        self.pass_outputs
            .get(name)
            .ok_or(PassOutputError::NotFound(name.to_string()))
            .and_then(|o| o.downcast_ref().ok_or(PassOutputError::DowncastError))
    }

    pub fn main_color(&self) -> &MainColor<R> {
        &self.main_color
    }

    pub fn main_depth(&self) -> &MainDepth<R> {
        &self.main_depth
    }

    pub fn factory(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn encoder(&mut self) -> &mut gfx::Encoder<R, C> {
        &mut self.encoder
    }

    pub fn build<D>(
        self,
        device: D,
        window: Arc<glutin::GlWindow>
    ) -> RenderGraph<R, C, D>
        where D: gfx::Device<Resources = R, CommandBuffer = C>
    {
        RenderGraph::new(self.passes, self.resources, self.encoder, device, window)
    }
}
