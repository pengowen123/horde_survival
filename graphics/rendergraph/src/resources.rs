//! A type containing temporary resources required by render passes

use common::graphics::{Drawable, DrawableSkeletal, ParticleSource};
use common::{gfx, specs};

/// All the temporary resources required by render passes, which are usually component storage
// references
// Using a generic type in `Pass` instead of a concrete type is complicated, so unfortunately this
// must be declared in this crate
#[derive(Clone)]
pub struct TemporaryResources<'a, R: gfx::Resources> {
    /// The component storage for `Drawable`
    pub drawable: &'a specs::ReadStorage<'a, Drawable<R>>,
    /// The component storage for `DrawableSkeletal`
    pub drawable_skeletal: &'a specs::ReadStorage<'a, DrawableSkeletal<R>>,
    /// The component storage for `ParticleSource`
    pub particle_source: &'a specs::ReadStorage<'a, ParticleSource<R>>,
}

impl<'a, R: gfx::Resources> Copy for TemporaryResources<'a, R> {}

impl<'a, R: gfx::Resources> TemporaryResources<'a, R> {
    /// Returns a new `TemporaryResources` using the provided component storage references
    pub fn new(
        drawable: &'a specs::ReadStorage<'a, Drawable<R>>,
        drawable_skeletal: &'a specs::ReadStorage<'a, DrawableSkeletal<R>>,
        particle_source: &'a specs::ReadStorage<'a, ParticleSource<R>>,
    ) -> Self {
        Self {
            drawable,
            drawable_skeletal,
            particle_source,
        }
    }
}
