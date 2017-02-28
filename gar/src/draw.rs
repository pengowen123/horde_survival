use gfx;

use render;

pub struct Draw<'a, F, R, C, D, W>
    where W: 'a,
          F: gfx::Factory<R> + 'a,
          R: gfx::Resources + 'a,
          C: gfx::CommandBuffer<R> + 'a,
          D: gfx::Device + 'a
{
    window: &'a mut W,
    renderer: &'a mut render::Renderer<F, R, C, D>,
}
