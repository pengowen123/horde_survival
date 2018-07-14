//! The `Pass` trait and a `PassSetup` type for lazily initializing passes and resources

use gfx;
use shred::Resources;

use builder::GraphBuilder;
use error::{RunError, BuildError};
use framebuffer::Framebuffers;

/// A function used to setup a pass and its resources
pub type SetupFn<R, C, F, CF, DF> =
    fn(&mut GraphBuilder<R, C, F, CF, DF>) -> Result<(), BuildError<String>>;

/// A type for lazily initializing a pass and its resources
pub struct PassSetup<R, C, F, CF, DF>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    setup: SetupFn<R, C, F, CF, DF>,
}

impl<R, C, F, CF, DF> From<SetupFn<R, C, F, CF, DF>> for PassSetup<R, C, F, CF, DF>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    fn from(setup: SetupFn<R, C, F, CF, DF>) -> Self {
        Self { setup }
    }
}

impl<R, C, F, CF, DF> PassSetup<R, C, F, CF, DF>
where R: gfx::Resources,
      C: gfx::CommandBuffer<R>,
      F: gfx::Factory<R>,
{
    /// Returns a new `PassSetup` that will use the provided setup function
    pub fn new(setup: SetupFn<R, C, F, CF, DF>) -> Self {
        setup.into()
    }

    /// Calls the contained setup function, adding passes and resources to the `GraphBuilder`
    pub fn setup(
        self,
        builder: &mut GraphBuilder<R, C, F, CF, DF>
    ) -> Result<(), BuildError<String>> {
        (self.setup)(builder)
    }
}

/// A trait that represents a rendering pass
pub trait Pass<R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>, CF, DF> {
    /// Executes the pass, adding graphics commands to the `Encoder`
    ///
    /// The pass has access to the `RenderGraph`'s resources.
    fn execute_pass(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        resources: &mut Resources,
    ) -> Result<(), RunError>;
    /// Reloads the shaders for the pass
    fn reload_shaders(&mut self, factory: &mut F)
        -> Result<(), BuildError<String>>;
    /// Handles the window being resized
    fn handle_window_resize(
        &mut self,
        new_dimensions: (u16, u16),
        framebuffers: &mut Framebuffers<R, CF, DF>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>>;
}
