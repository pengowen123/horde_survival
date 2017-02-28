use gfx;
use gfx::traits::FactoryExt;

use {options, backend};

/// Color format used by gfx
pub type ColorFormat = gfx::format::Srgba8;
/// Depth format used by gfx
pub type DepthFormat = gfx::format::DepthStencil;

/// Render target view
pub type RTV<R> = gfx::handle::RenderTargetView<R, ColorFormat>;
/// Depth stencil view
pub type DSV<R> = gfx::handle::DepthStencilView<R, DepthFormat>;

pub type RendererParts<W, D, F, R> = (W, D, F, RTV<R>, DSV<R>);

/// A backend-agnostic renderer
pub struct Renderer<F, R, C, D>
    where F: gfx::Factory<R>,
          R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device
{
    /// Renderer options
    options: options::RendererOptions,
    /// Factory
    factory: F,
    /// Device
    device: D,
    /// Encoder
    encoder: gfx::Encoder<R, C>,
    /// Render target view
    rtv: RTV<R>,
    /// Depth stencil view
    dsv: DSV<R>,
}

impl<F, R, C, D> Renderer<F, R, C, D>
    where F: gfx::Factory<R> + CreateCommandBuffer<CommandBuffer = C>,
          R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device
{
    /// Initializes and returns a new `Renderer`, using the provided options
    pub fn new<I, W>(options: options::RendererOptions, init: I) -> (Self, W)
        where W: backend::WindowOptions,
              I: Fn(W::Options) -> RendererParts<W, D, F, R>
    {
        let window_options = W::from_renderer_options(&options);

        let (window, device, mut factory, rtv, dsv) = init(window_options);
        let encoder = factory.create_command_buffer();

        let renderer = Renderer {
            options: options,
            factory: factory,
            device: device,
            encoder: encoder.into(),
            rtv: rtv,
            dsv: dsv,
        };

        (renderer, window)
    }
}

/// A trait implemented by each backend's factory type, used for creating command buffers
pub trait CreateCommandBuffer {
    type CommandBuffer;
    fn create_command_buffer(&mut self) -> Self::CommandBuffer;
}
