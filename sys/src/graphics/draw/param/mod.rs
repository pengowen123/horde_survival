//! Components and systems to pass additional parameters (such as rotation) to the shaders

pub mod translation;
pub mod rotation;

use cgmath::One;
use specs::{self, DispatcherBuilder, Join};
use gfx;

use self::translation::Translation;
use self::rotation::Rotation;
use graphics::draw::shader;

/// A type that stores all individual parameters to pass the the graphics system
#[derive(Clone, Copy, Debug)]
pub struct ShaderParam {
    translation: Translation,
    rotation: Rotation,
}

impl ShaderParam {
    pub fn new(translation: Translation, rotation: Rotation) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn translation(&self) -> &Translation {
        &self.translation
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }
}

impl Default for ShaderParam {
    fn default() -> Self {
        // Identity transformations (zero translation, zero rotation)
        ShaderParam::new(One::one(), One::one())
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
    drawable: specs::WriteStorage<'a, shader::Drawable<R>>,
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
        .add(translation::System, "shader-param-translation", &[])
        .add(rotation::System, "shader-param-rotation", &[])
}
