//! Components and systems to pass additional parameters (such as rotation) to the shaders

pub mod translation;
pub mod rotation;
pub mod scale;

use common::cgmath::{self, One};
use specs::{self, DispatcherBuilder, Join};
use gfx;

use self::translation::Translation;
use self::rotation::Rotation;
use self::scale::Scale;
use draw::components;

/// A type that stores all individual parameters to pass the the graphics system
#[derive(Clone, Copy, Debug)]
pub struct ShaderParam {
    translation: Translation,
    rotation: Rotation,
    scale: Scale,
}

impl ShaderParam {
    pub fn new(translation: Translation, rotation: Rotation, scale: Scale) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Returns the model matrix, created from the stored translation, rotation, and scale matrices
    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        self.translation * self.rotation * self.scale
    }
}

impl Default for ShaderParam {
    fn default() -> Self {
        // Identity transformations (zero translation, zero rotation)
        ShaderParam::new(One::one(), One::one(), One::one())
    }
}

impl specs::Component for ShaderParam {
    type Storage = specs::VecStorage<Self>;
}

pub struct System<R>(::std::marker::PhantomData<R>);

impl<R: gfx::Resources> System<R> {
    // The argument here is only to tie the `R` type parameter
    pub fn new<F: gfx::Factory<R>>(_: &F) -> Self {
        System(::std::marker::PhantomData)
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::WriteStorage<'a, components::Drawable<R>>,
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
