//! Components to pass additional parameters to the shaders

// TODO: Add a system to write params to Drawable, to allow entities to have no ShaderParam
//       component
pub mod translation;
pub mod rotation;

use cgmath::One;
use specs::{self, DispatcherBuilder};

use self::translation::Translation;
use self::rotation::Rotation;

/// A type that stores all individual parameters to pass the the graphics system
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
        ShaderParam::new(Translation::one(), Rotation::one())
    }
}

impl specs::Component for ShaderParam {
    type Storage = specs::VecStorage<Self>;
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
