use gfx;
use shred::Resources;

use super::builder::GraphBuilder;

pub type SetupFn<R, C, F> = fn(&mut GraphBuilder<R, C, F>);

pub struct PassSetup<R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    setup: SetupFn<R, C, F>,
}

impl<R, C, F> PassSetup<R, C, F>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    pub fn new(setup: SetupFn<R, C, F>) -> Self {
        Self { setup }
    }

    pub fn setup(self, builder: &mut GraphBuilder<R, C, F>) {
        (self.setup)(builder)
    }
}

pub trait Pass<R: gfx::Resources, C: gfx::CommandBuffer<R>> {
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, resources: &mut Resources);
}
