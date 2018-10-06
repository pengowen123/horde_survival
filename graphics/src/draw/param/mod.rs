//! Systems to update the `ShaderParam` component of entities

pub mod rotation;
pub mod scale;
pub mod translation;

use common::graphics::ShaderParam;
use gfx;
use specs::{self, DispatcherBuilder, Join};

use common::graphics::Drawable;

pub struct System<R>(::std::marker::PhantomData<R>);

impl<R: gfx::Resources> System<R> {
    // The argument here is only to tie the `R` type parameter
    pub fn new<F: gfx::Factory<R>>(_: &F) -> Self {
        System(::std::marker::PhantomData)
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::WriteStorage<'a, Drawable<R>>,
    param: specs::ReadStorage<'a, ShaderParam>,
}

impl<'a, R: gfx::Resources> specs::System<'a> for System<R> {
    type SystemData = Data<'a, R>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (d, p) in (&mut data.drawable, &data.param).join() {
            d.set_shader_param(*p);
        }
    }
}

/// Initializes shader parameter-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    // Register components
    world.register::<ShaderParam>();

    // Add systems
    dispatcher
        .with(translation::System, "shader-param-translation", &[])
        .with(rotation::System, "shader-param-rotation", &[])
        .with(scale::System, "shader-param-scale", &[])
}
