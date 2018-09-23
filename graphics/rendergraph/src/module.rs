//! A `Module` type that is used to create groups of passes lazily

use gfx;

use builder::GraphBuilder;
use error::BuildError;
use pass::PassSetup;

/// A `Module`
pub struct Module<R, C, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    passes: Vec<PassSetup<R, C, F, CF, DF>>,
}

impl<R, C, F, CF, DF> Module<R, C, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    /// Returns a new `Module`
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    /// Adds the provided pass to the `Module`
    ///
    /// The pass will not be setup until `Module::setup_passes` is run.
    pub fn add_pass<P>(mut self, pass: P) -> Self
    where
        P: Into<PassSetup<R, C, F, CF, DF>>,
    {
        self.passes.push(pass.into());
        self
    }

    /// Adds the passes from `module` to the end of this one, in the order they were added to
    /// `module`
    pub fn add_module(mut self, module: Self) -> Self {
        self.passes.extend(module.passes);
        self
    }

    /// Calls `PassSetup::setup` on each of this `Module`'s passes
    pub fn setup_passes(
        self,
        builder: &mut GraphBuilder<R, C, F, CF, DF>,
    ) -> Result<(), BuildError<String>> {
        for pass in self.passes {
            pass.setup(builder)?;
        }

        Ok(())
    }
}
