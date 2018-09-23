//! Pipeline declaration for shadows from directional lights

use assets;
use cgmath::{Matrix4, SquareMatrix};
use common::config;
use gfx::traits::FactoryExt;
use gfx::{self, handle, state, texture};
use rendergraph::error::{BuildError, RunError};
use rendergraph::framebuffer::Framebuffers;
use rendergraph::pass::Pass;
use shred;
use specs::Join;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use draw::glsl::Mat4;
use draw::passes::main::geometry_pass;
use draw::passes::shadow;
use draw::{passes, types, DrawableStorageRef};

pub struct Output<R: gfx::Resources> {
    pub srv: handle::ShaderResourceView<R, [f32; 4]>,
}

gfx_defines! {
    pipeline pipe {
        vbuf: gfx::VertexBuffer<geometry_pass::Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    constant Locals {
        light_space_matrix: Mat4 = "lightSpaceMatrix",
        model: Mat4 = "model",
    }
}

pub struct DirectionalShadowPass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
    enabled: bool,
}

impl<R: gfx::Resources> DirectionalShadowPass<R> {
    fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
        shadow_map_size: texture::Size,
        enabled: bool,
    ) -> Result<(Self, Output<R>), BuildError<String>> {
        // Make a 1x1 shadow map if shadows are disabled
        let shadow_map_size = if enabled { shadow_map_size } else { 1 };

        let (_, srv, dsv) = factory.create_depth_stencil(shadow_map_size, shadow_map_size)?;

        let vbuf = factory.create_vertex_buffer(&[]);
        let slice = gfx::Slice::new_match_vertex_buffer(&vbuf);

        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            out_depth: dsv,
        };

        let pso = Self::load_pso(factory, assets)?;
        let pass = Self {
            bundle: gfx::Bundle::new(slice, pso, data),
            enabled,
        };

        let output = Output { srv };

        Ok((pass, output))
    }

    fn load_pso<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>> {
        passes::load_pso(
            assets,
            factory,
            "dir_shadow_vertex.glsl",
            "dir_shadow_fragment.glsl",
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
            HashMap::new(),
        )
    }
}

pub fn setup_pass<R, C, F>(
    builder: &mut types::GraphBuilder<R, C, F>,
) -> Result<(), BuildError<String>>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    let (enabled, shadow_map_size) = {
        let config = builder.get_resources().fetch::<config::GraphicsConfig>();

        (config.shadows, config.shadow_map_size)
    };

    let (pass, output) =
        DirectionalShadowPass::new(builder.factory, builder.assets, shadow_map_size, enabled)?;

    builder.add_pass(pass);
    builder.add_pass_output("dir_shadow_map", output);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for DirectionalShadowPass<R>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "directional_light_shadows"
    }

    fn execute_pass(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        resources: &mut shred::Resources,
    ) -> Result<(), RunError> {
        if !self.enabled {
            return Ok(());
        }

        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);

        let drawable = resources.fetch::<DrawableStorageRef<R>>();
        let drawable = unsafe { &*drawable.get() };

        let shadow_source = resources
            .fetch::<Arc<Mutex<shadow::DirShadowSource>>>()
            .clone();
        let light_space_matrix = match shadow_source.lock().unwrap().light_space_matrix() {
            Some(m) => m,
            // If there is no shadow source, just return (depth buffer was already cleared)
            None => return Ok(()),
        };
        let mut locals = Locals {
            light_space_matrix: light_space_matrix.into(),
            model: Matrix4::identity().into(),
        };

        for d in drawable.join() {
            let model = d.param().get_model_matrix();
            locals.model = model.into();

            encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

            self.bundle.data.vbuf = d.vertex_buffer().clone();
            self.bundle.slice = d.slice().clone();
            self.bundle.encode(encoder);
        }

        Ok(())
    }

    fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets)?;
        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        _: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
        _: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        // If the shadows setting was disabled, set the shadow map to a dummy texture to save memory
        if !config.shadows && self.enabled {
            let (_, dummy_srv, dummy_dsv) = factory.create_depth_stencil(1, 1)?;
            self.bundle.data.out_depth = dummy_dsv.clone();

            framebuffers.add_framebuffer("dir_shadow_map", Output { srv: dummy_srv });
        }

        let (w, h, _, _) = self.bundle.data.out_depth.get_dimensions();
        assert_eq!(w, h);
        // The height is always the same as the width, so just the width is used
        let current_shadow_map_size = w;

        // If the shadows setting was enabled, make a new shadow map
        let mut make_shadow_map = config.shadows && !self.enabled;

        // If the shadow size setting was changed and shadows are enabled, make a new shadow map
        if (config.shadow_map_size != current_shadow_map_size) && config.shadows {
            make_shadow_map = true;
        }

        if make_shadow_map {
            let (_, srv, dsv) =
                factory.create_depth_stencil(config.shadow_map_size, config.shadow_map_size)?;
            self.bundle.data.out_depth = dsv.clone();

            framebuffers.add_framebuffer("dir_shadow_map", Output { srv });
        }

        self.enabled = config.shadows;

        Ok(())
    }
}
