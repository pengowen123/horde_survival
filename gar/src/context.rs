use {gfx, gfx_window_glutin, glutin};

use std::marker::PhantomData;

use {options, render, backend};

/// Graphics context
///
/// Should only be created once and references should be handed out to functions that draw things
///
/// See the module-level documentation for more
pub struct Context<F, R, C, D, W>
    where F: gfx::Factory<R>,
          R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device
{
    /// Context options
    options: options::ContextOptions,
    /// Window
    window: W,
    /// Renderer
    renderer: render::Renderer<F, R, C, D>,

    _marker: PhantomData<R>,
}

impl<F, R, C, D, W> Context<F, R, C, D, W>
    where F: gfx::Factory<R> + render::CreateCommandBuffer<CommandBuffer = C>,
          R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
          W: backend::WindowOptions
{
    /// Initializes and returns a new `Context`
    ///
    /// Uses the provided options to configure the context, and the init function to initialize the
    /// renderer.
    pub fn new<I>(options: options::Options, init: I) -> Self
        where I: Fn(W::Options) -> render::RendererParts<W, D, F, R>
    {
        let (renderer, window) = render::Renderer::new(options.renderer_options, init);

        Context {
            options: options.context_options,
            window: window,
            renderer: renderer,
            _marker: PhantomData,
        }
    }
    /// Returns a reference to the window
    pub fn window(&self) -> &W {
        &self.window
    }
}
