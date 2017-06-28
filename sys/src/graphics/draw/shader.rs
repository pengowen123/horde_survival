//! Declaration of the graphics pipeline, and a component for each entity containing the
//! information needed to draw it

use specs;
use gfx;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type TextureView<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;
pub type VertexBuffer<R> = gfx::handle::Buffer<R, Vertex>;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
        position: [f32; 4] = "u_Pos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        texture: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            pos: [pos[0], pos[1], pos[2], 1.0],
            uv: uv,
        }
    }
}

/// A component that stores the information needed to draw an entity
pub struct Drawable<R: gfx::Resources> {
    texture: TextureView<R>,
    vertex_buffer: VertexBuffer<R>,
    slice: gfx::Slice<R>,
}

impl<R: gfx::Resources> Drawable<R> {
    pub fn new(texture: TextureView<R>,
               vertex_buffer: VertexBuffer<R>,
               slice: gfx::Slice<R>)
               -> Self {
        Self {
            texture,
            vertex_buffer,
            slice,
        }
    }

    pub fn texture(&self) -> &TextureView<R> {
        &self.texture
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer<R> {
        &self.vertex_buffer
    }

    pub fn slice(&self) -> &gfx::Slice<R> {
        &self.slice
    }
}

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::VecStorage<Self>;
}
