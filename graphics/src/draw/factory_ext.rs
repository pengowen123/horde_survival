//! Extensions to the `Factory` trait from `gfx`.

use gfx::memory::Bind;
use gfx::{self, format, handle, memory, texture, Factory};

pub trait FactoryExtension<R: gfx::Resources>: Factory<R> {
    /// Creates a render target with the provided anti-aliasing mode enabled
    fn create_render_target_with_aa<RF>(
        &mut self,
        width: texture::Size,
        height: texture::Size,
        aa_mode: texture::AaMode,
    ) -> Result<
        (
            handle::Texture<R, RF::Surface>,
            handle::ShaderResourceView<R, RF::View>,
            handle::RenderTargetView<R, RF>,
        ),
        gfx::CombinedError,
    >
    where
        RF: format::RenderFormat + format::TextureFormat,
    {
        // Get target info
        let kind = texture::Kind::D2(width, height, aa_mode);
        let levels = 1;
        let channel_type = <RF::Channel as format::ChannelTyped>::get_channel_type();

        // Create render target texture
        let texture = self.create_texture(
            kind,
            levels,
            Bind::SHADER_RESOURCE | Bind::RENDER_TARGET,
            memory::Usage::Data,
            Some(channel_type),
        )?;
        // Get the texture as a shader resource
        let resource = self.view_texture_as_shader_resource::<RF>(
            &texture,
            (0, levels - 1),
            format::Swizzle::new(),
        )?;
        // Get the texture as a render target
        let target = self.view_texture_as_render_target(&texture, 0, None)?;

        Ok((texture, resource, target))
    }

    /// Creates a depth stencil with the provided anti-aliasing mode enabled
    fn create_depth_stencil_view_only_with_aa<DF>(
        &mut self,
        width: texture::Size,
        height: texture::Size,
        aa_mode: texture::AaMode,
    ) -> Result<handle::DepthStencilView<R, DF>, gfx::CombinedError>
    where
        DF: format::DepthFormat + format::TextureFormat,
    {
        // Get target info
        let kind = texture::Kind::D2(width, height, aa_mode);
        let levels = 1;
        let channel_type = <DF::Channel as format::ChannelTyped>::get_channel_type();

        // Create render target texture
        let texture = self.create_texture(
            kind,
            levels,
            Bind::SHADER_RESOURCE | Bind::DEPTH_STENCIL,
            memory::Usage::Data,
            Some(channel_type),
        )?;
        // Get the texture as a render target
        let target = self.view_texture_as_depth_stencil_trivial(&texture)?;

        Ok(target)
    }

    /// Creates a depth stencil with a cubemap texture
    fn create_depth_stencil_cubemap<DF>(
        &mut self,
        size: texture::Size,
    ) -> Result<
        (
            handle::ShaderResourceView<R, DF::View>,
            handle::DepthStencilView<R, DF>,
        ),
        gfx::CombinedError,
    >
    where
        DF: format::DepthFormat + format::TextureFormat,
    {
        // Get texture info
        let kind = texture::Kind::Cube(size);
        let levels = 1;
        let bind = Bind::DEPTH_STENCIL | Bind::SHADER_RESOURCE;
        let channel_type = <DF::Channel as format::ChannelTyped>::get_channel_type();

        // Create texture
        let texture =
            self.create_texture(kind, levels, bind, memory::Usage::Data, Some(channel_type))?;

        // Get the texture as a shader resource
        let srv =
            self.view_texture_as_shader_resource::<DF>(&texture, (0, 0), format::Swizzle::new())?;

        // Get the texture as a depth stencil
        let dsv = self.view_texture_as_depth_stencil_trivial(&texture)?;

        Ok((srv, dsv))
    }
}

impl<R, T> FactoryExtension<R> for T
where
    R: gfx::Resources,
    T: Factory<R>,
{
}
