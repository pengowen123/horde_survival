//! Types for managing off-screen render targets

use gfx::{self, format, handle, texture};

use std::mem;

/// The color format used by the render targets managed by `RenderTargets`
pub type ColorFormat = format::Rgba8;

/// A render target and a view of that target as a texture
#[derive(Clone)]
pub struct ViewPair<R, T>
where
    R: gfx::Resources,
    T: format::Formatted,
{
    resource: handle::ShaderResourceView<R, T::View>,
    target: handle::RenderTargetView<R, T>,
    pub id: usize,
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
        id: usize,
    ) -> Result<Self, gfx::CombinedError> {

        let (_, srv, rtv) = factory.create_render_target(width, height)?;

        Ok(ViewPair {
            resource: srv,
            target: rtv,
            id,
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

/// Returns this view pair's id
    pub fn id(&self) -> usize {
        self.id
    }
}

/// A type for managing off-screen render targets
pub struct RenderTargets<R: gfx::Resources> {
    active: ViewPair<R, ColorFormat>,
    extra: ViewPair<R, ColorFormat>,
}

impl<R: gfx::Resources> RenderTargets<R> {
    /// Creates all required render targets using the provided factory, and returns a
    /// `RenderTargets`
    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        width: texture::Size,
        height: texture::Size,
    ) -> Result<Self, gfx::CombinedError> {

        let mut active = ViewPair::new(factory, width, height, 0)?;
        let mut extra = ViewPair::new(factory, width, height, 1)?;

        // This is so the view pair returns by `get_active` returns a render target to write to, and
        // the texture from the other render target to read from
        mem::swap(&mut active.resource, &mut extra.resource);

        Ok(RenderTargets { active, extra })
    }

    /// Returns a reference to the active view pair
    pub fn get_active(&self) -> &ViewPair<R, ColorFormat> {
        &self.active
    }

    /// Sets the provided RTV and SRV to the active pair
    pub fn use_active(
        &self,
        rtv: &mut handle::RenderTargetView<R, ColorFormat>,
        srv: &mut handle::ShaderResourceView<R, <ColorFormat as format::Formatted>::View>,
    ) {
        let pair = self.get_active();
        *rtv = pair.rtv().clone();
        *srv = pair.srv().clone();
    }

    /// Returns a reference to all render targets, in a tuple
    pub fn get_all_render_targets(
        &self,
    ) -> (&handle::RenderTargetView<R, ColorFormat>, &handle::RenderTargetView<R, ColorFormat>) {
        (self.active.rtv(), self.extra.rtv())
    }

    /// Swaps the active render target with the inactive one
    pub fn swap_render_targets(&mut self) {
        mem::swap(&mut self.active, &mut self.extra);
    }
}
