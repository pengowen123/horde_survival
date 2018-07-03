//! Types for managing off-screen render targets

use gfx::{self, format, handle, texture};

/// A render target and a view of that target as a texture
#[derive(Clone)]
pub struct ViewPair<R, T>
where
    R: gfx::Resources,
    T: format::Formatted,
{
    resource: handle::ShaderResourceView<R, T::View>,
    target: handle::RenderTargetView<R, T>,
}

impl<R, T> ViewPair<R, T>
where
    R: gfx::Resources,
    T: format::Formatted,
    <T as format::Formatted>::Surface: format::RenderSurface + format::TextureSurface,
    <T as format::Formatted>::Channel: format::RenderChannel + format::TextureChannel,
{
    /// Creates a render target using the provided factory, and returns a `ViewPair`
    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        width: texture::Size,
        height: texture::Size,
    ) -> Result<Self, gfx::CombinedError> {

        let (_, srv, rtv) = factory.create_render_target(width, height)?;

        Ok(ViewPair {
            resource: srv,
            target: rtv,
        })
    }

    /// Returns a reference to the view pair's render target
    pub fn rtv(&self) -> &handle::RenderTargetView<R, T> {
        &self.target
    }

    /// Returns a reference to the view pair's shader resource view (a view of the render target as
    /// a texture)
    pub fn srv(&self) -> &handle::ShaderResourceView<R, T::View> {
        &self.resource
    }
}
