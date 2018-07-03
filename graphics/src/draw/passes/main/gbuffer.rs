//! Geometry-buffer creation for deferred shading

use gfx::{self, texture};

use draw::render_target::ViewPair;

/// A geometry buffer
///
/// Stores a buffer internally per type of data:
///
/// - Position
/// - Normal
/// - Color and specular (specular is stored in alpha channel)
#[derive(Clone)]
pub struct GeometryBuffer<R>
where
    R: gfx::Resources,
{
    pub position: ViewPair<R, GFormat>,
    pub normal: ViewPair<R, GFormat>,
    pub color: ViewPair<R, GFormat>,
}

pub type GFormat = [f32; 4];

impl<R: gfx::Resources> GeometryBuffer<R> {
    /// Creates and returns a geometry buffer with the provided dimensions
    pub fn new<F>(
        factory: &mut F,
        width: texture::Size,
        height: texture::Size,
    ) -> Result<GeometryBuffer<R>, gfx::CombinedError>
    where
        R: gfx::Resources,
        F: gfx::Factory<R>,
    {
        // NOTE: Replace this with RGB textures to save memory if necessary

        // Create buffers
        let position = ViewPair::new(factory, width, height)?;
        let normal = ViewPair::new(factory, width, height)?;
        let color = ViewPair::new(factory, width, height)?;

        Ok(GeometryBuffer {
            position,
            normal,
            color,
        })
    }
}
