//! Utilities for graphics

/// Returns a quad that fills the entire screen
///
/// Requires a function that creates a vertex given its position and UV coordinates.
pub fn create_screen_quad<F, V>(f: F) -> [V; 6]
where
    F: Fn([f32; 2], [f32; 2]) -> V,
{
    [
        f([-1.0, -1.0], [0.0, 0.0]),
        f([1.0, -1.0], [1.0, 0.0]),
        f([1.0, 1.0], [1.0, 1.0]),
        f([-1.0, -1.0], [0.0, 0.0]),
        f([1.0, 1.0], [1.0, 1.0]),
        f([-1.0, 1.0], [0.0, 1.0]),
    ]
}

/// Returns a cube that represents a skybox
///
/// Requires a function that creates a vertex given its position.
pub fn create_skybox_cube<F, V>(f: F) -> [V; 36]
where
    F: Fn([f32; 3]) -> V,
{
    [
        f([-1.0, 1.0, -1.0]),
        f([-1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, 1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),

        f([-1.0, -1.0, 1.0]),
        f([-1.0, -1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),
        f([-1.0, 1.0, 1.0]),
        f([-1.0, -1.0, 1.0]),

        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, -1.0]),
        f([1.0, -1.0, -1.0]),

        f([-1.0, -1.0, 1.0]),
        f([-1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, -1.0, 1.0]),
        f([-1.0, -1.0, 1.0]),

        f([-1.0, 1.0, -1.0]),
        f([1.0, 1.0, -1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([-1.0, 1.0, 1.0]),
        f([-1.0, 1.0, -1.0]),

        f([-1.0, -1.0, -1.0]),
        f([-1.0, -1.0, 1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([-1.0, -1.0, 1.0]),
        f([1.0, -1.0, 1.0]),
    ]
}

/// Clears all render targets if `COLOR` is the first argument. Clears all depth targets if `DEPTH`
/// is the first argument. This macro can only be used in a method on `draw::System`.
///
/// # Examples
///
/// This will use `self.encoder` to clear the depth of `data.depth`, and `other_data.depth`:
/// ```rust
/// clear_targets!(DEPTH, self, data.depth, other_data.depth);
/// ```
macro_rules! clear_targets {
    (COLOR, $self_:ident, $($target:expr),*,) => {
        $(
            $self_.encoder.clear(&$target, CLEAR_COLOR);
        )*
    };
    (DEPTH, $self_:ident, $($target:expr),*,) => {
        $(
            $self_.encoder.clear_depth(&$target, 1.0);
        )*
    }
}

/// Expands to a pipeline declaration for the provided light type, a type alias for the pipeline,
/// and a constructor for the pipeline. The `gfx_defines` macro doesn't support generics, so this is
/// used to avoid code duplication for different light types.
macro_rules! create_light_pipeline {
    ($name:ident, $alias_name:ident, $constructor_name:ident, $light_type:path) => {
        gfx_defines! {
            pipeline $name {
                vbuf: gfx::VertexBuffer<Vertex> = (),
                locals: gfx::ConstantBuffer<Locals> = "u_Locals",
                material: gfx::ConstantBuffer<Material> = "u_Material",
                // The light to process
                light: gfx::ConstantBuffer<$light_type> = "u_Light",
                // G-buffer textures
                g_position: gfx::TextureSampler<Vec4> = "t_Position",
                g_normal: gfx::TextureSampler<Vec4> = "t_Normal",
                g_color: gfx::TextureSampler<Vec4> = "t_Color",
                target_color: gfx::TextureSampler<Vec4> = "t_Target",
                // Output color (note that depth is not needed here)
                out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
            }
        }

        pub type $alias_name<R> = pipeline::Pipeline<R, $name::Data<R>>;

        impl<R: gfx::Resources> $alias_name<R> {
            /// Returns a new lighting `Pipeline`, created from the provided shaders and pipeline initialization
            /// data
            pub fn $constructor_name<F, P>(
                factory: &mut F,
                srv_pos: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_normal: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_color: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_previous: handle::ShaderResourceView<R, [f32; 4]>,
                rtv: handle::RenderTargetView<R, format::Rgba8>,
                vs_path: P,
                fs_path: P,
            ) -> Result<Self, PsoError>
            where
                F: gfx::Factory<R>,
                P: AsRef<Path>,
            {
                let rasterizer = state::Rasterizer { ..state::Rasterizer::new_fill() };

                let pso = pipeline::load_pso(
                    factory,
                    vs_path,
                    fs_path,
                    gfx::Primitive::TriangleList,
                    rasterizer,
                    $name::new(),
                )?;

                // Create a screen quad
                let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
                let vbuf = factory.create_vertex_buffer(&vertices);

                // Create texture sampler info
                let sampler_info =
                    texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

                let data = $name::Data {
                    vbuf: vbuf,
                    material: factory.create_constant_buffer(1),
                    locals: factory.create_constant_buffer(1),
                    light: factory.create_constant_buffer(1),
                    g_position: (srv_pos, factory.create_sampler(sampler_info)),
                    g_normal: (srv_normal, factory.create_sampler(sampler_info)),
                    g_color: (srv_color, factory.create_sampler(sampler_info)),
                    target_color: (srv_previous, factory.create_sampler(sampler_info)),
                    out_color: rtv,
                };

                Ok(Pipeline::new(pso, data))
            }
        }
    }
}
