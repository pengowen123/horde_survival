extern crate rendergraph;
extern crate common;
#[macro_use]
extern crate gfx;

use common::{gfx_window_glutin, glutin, shred};
use glutin::{EventsLoop, GlContext};
use gfx::{
    format::{Srgba8, DepthStencil},
    traits::FactoryExt,
};

use std::sync::Arc;

const VS_DATA: &[u8] = b"
#version 150 core

in vec4 a_Color;
in vec2 a_Pos;

out vec4 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}";

const FS_DATA: &[u8] = b"
#version 150 core

in vec4 v_Color;

out vec4 Target0;

void main() {
    Target0 = v_Color;
}";

gfx_defines! {
    vertex Vertex {
        color: [f32; 4] = "a_Color",
        pos: [f32; 2] = "a_Pos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<Srgba8> = "Target0",
    }
}

impl Vertex {
    fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
        Self { pos, color }
    }
}

struct TrianglePass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> TrianglePass<R> {
    fn setup<C, F>(builder: &mut rendergraph::builder::GraphBuilder<R, C, F, Srgba8, DepthStencil>)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R>,
    {
        let main_color = builder.main_color().clone();
        let bundle = {
            let factory = builder.factory();
            
            let pso = factory.create_pipeline_simple(VS_DATA, FS_DATA, pipe::new()).unwrap();
            let data = pipe::Data {
                vbuf: factory.create_vertex_buffer(&[
                    Vertex::new([-0.5, -0.5], [1.0, 0.0, 0.0, 1.0]),
                    Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0, 1.0]),
                    Vertex::new([0.0, 0.5], [0.0, 0.0, 1.0, 1.0]),
                ]),
                out_color: main_color,
            };
            let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

            gfx::Bundle::new(slice, pso, data)
        };

        let pass = TrianglePass { bundle };

        builder.add_pass(pass);
    }
}

impl<R: gfx::Resources, C: gfx::CommandBuffer<R>> rendergraph::pass::Pass<R, C> for TrianglePass<R> {
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, res: &mut shred::Resources) {
        encoder.clear(&self.bundle.data.out_color, [1.0; 4]);
        self.bundle.encode(encoder);
    }
}

fn setup_passes<R, C, F>(
    builder: &mut rendergraph::builder::GraphBuilder<R, C, F, Srgba8, DepthStencil>
)
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let triangle_module = rendergraph::module::Module::new()
        .add_pass(TrianglePass::setup as rendergraph::pass::SetupFn<_, _, _, _, _>);

    triangle_module.setup_passes(builder);
}

fn main() {
    let (w, h) = (800, 600);
    let events = EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Horde Survival")
        .with_min_dimensions(w, h);

    // Initialize gfx structs
    let (window, device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<Srgba8, DepthStencil>(
            window_builder,
            context_builder,
            &events,
        );

    unsafe {
        window.make_current().unwrap();
    }

    let window = Arc::new(window);

    let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut builder = rendergraph::builder::GraphBuilder::new(&mut factory, main_color, main_depth);

    setup_passes(&mut builder);

    let mut rendergraph = builder.build(device, encoder, window.clone());

    loop {
        rendergraph.execute_passes();
    }
}
