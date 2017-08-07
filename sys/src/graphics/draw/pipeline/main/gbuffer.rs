//! Geometry-buffer creation for deferred shading

use gfx::{self, texture, format, handle};

use graphics::draw::types;

/// A geometry buffer
///
/// Stores a buffer internally per type of data:
///
/// - Position
/// - Normal
/// - Color and specular (specular is stored in alpha channel)
///
/// Also stores a depth buffer.
pub struct GeometryBuffer<R>
where
    R: gfx::Resources,
{
    pub position: ViewPair<R, GFormat>,
    pub normal: ViewPair<R, GFormat>,
    pub color: ViewPair<R, GFormat>,
    pub depth: handle::DepthStencilView<R, types::DepthFormat>,
}

pub struct ViewPair<R, T>
where
    R: gfx::Resources,
    T: format::Formatted,
{
    pub resource: handle::ShaderResourceView<R, T::View>,
    pub target: handle::RenderTargetView<R, T>,
}

pub type GFormat = [f32; 4];

/// Creates and returns a geometry buffer with the provided dimensions
pub fn create_geometry_buffer<F, R>(
    factory: &mut F,
    width: texture::Size,
    height: texture::Size,
) -> GeometryBuffer<R>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    // NOTE: Replace this with RGB textures to save memory if necessary
    let (position, normal, color) = {
        let mut create_buffer = || {
            let (_, srv, rtv) = factory.create_render_target(width, height).unwrap();

            ViewPair {
                resource: srv,
                target: rtv,
            }
        };

        // Create buffers
        let position = create_buffer();
        let normal = create_buffer();
        let color = create_buffer();

        (position, normal, color)
    };

    // Create depth target
    let dsv = factory
        .create_depth_stencil_view_only(width, height)
        .unwrap();

    GeometryBuffer {
        position,
        normal,
        color,
        depth: dsv,
    }
}
