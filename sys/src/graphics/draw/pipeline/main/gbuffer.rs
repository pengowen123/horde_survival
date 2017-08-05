//! Geometry-buffer creation for deferred shading

use gfx::{self, texture, format, handle};
use gfx::format::ChannelSource;

/// A geometry buffer
///
/// Stores a buffer internally per type of data:
///
/// - Position
/// - Normal
/// - Color and specular (specular is stored in alpha channel)
pub struct GeometryBuffer<R>
where
    R: gfx::Resources,
{
    position: ViewPair<R, GFormat>,
    normal: ViewPair<R, GFormat>,
    color: ViewPair<R, GFormat>,
    depth: (handle::ShaderResourceView<R, [f32; 4]>, handle::DepthStencilView<R, format::Depth>),
}

pub struct ViewPair<R, T>
where
    R: gfx::Resources,
    T: format::Formatted,
{
    resource: handle::ShaderResourceView<R, T::View>,
    target: handle::RenderTargetView<R, T>,
}

pub type GFormat = [f32; 4];

// Custom depth format to view depth as `[f32; 4]`
pub struct DepthFormat;

impl format::Formatted for DepthFormat {
    type Surface = format::D24;
    type Channel = format::Unorm;
    type View = [f32; 4];

    fn get_format() -> format::Format {
        format::Format(format::SurfaceType::D24, format::ChannelType::Unorm)
    }
}

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

    // Create depth target
    let (texture, _, depth_rtv) = factory.create_depth_stencil(width, height).unwrap();
    // Create a custom SRV with swizzling
    let swizzle = format::Swizzle(
        ChannelSource::X,
        ChannelSource::X,
        ChannelSource::X,
        ChannelSource::X,
    );
    let depth_srv = factory
        .view_texture_as_shader_resource::<DepthFormat>(&texture, (0, 0), swizzle)
        .unwrap();

    GeometryBuffer {
        position,
        normal,
        color,
        depth: (depth_srv, depth_rtv),
    }
}
