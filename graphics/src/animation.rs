//! A system for updating animation states

use common::{self, gfx};
use common::specs::{self, Join};

pub struct System<R: gfx::Resources>(::std::marker::PhantomData<R>);

impl<R: gfx::Resources> System<R> {
    pub fn new<F: gfx::Factory<R>>(_: &F) -> Self {
        Self(Default::default())
    }
}

#[derive(SystemData)]
pub struct SystemData<'a, R: gfx::Resources> {
    delta: specs::ReadExpect<'a, common::Delta>,
    drawable_skeletal: specs::WriteStorage<'a, common::graphics::DrawableSkeletal<R>>,
}

impl<'a, R: gfx::Resources> specs::System<'a> for System<R> {
    type SystemData = SystemData<'a, R>;

    fn run(&mut self, mut data: Self::SystemData) {
        let delta = data.delta.to_float();

        for d in (&mut data.drawable_skeletal).join() {
            d.update_animation_controller(delta);
        }
    }
}
