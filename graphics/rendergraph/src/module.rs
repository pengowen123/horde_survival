use gfx;

use super::pass::PassSetup;
use super::builder::GraphBuilder;

pub struct Module<R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    passes: Vec<PassSetup<R, C, F>>,
}

impl<R, C, F> Module<R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
        }
    }

    pub fn add_pass(mut self, pass: PassSetup<R, C, F>) -> Self {
        self.passes.push(pass);
        self
    }

    pub fn add_module(mut self, module: Module<R, C, F>) -> Self {
        self.passes.extend(module.passes);
        self
    }

    pub fn setup_passes(self, builder: &mut GraphBuilder<R, C, F>) {
        for pass in self.passes {
            pass.setup(builder);
        }
    }
}
