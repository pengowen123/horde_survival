//! OBJ loading

use assets;
use common::ncollide3d::shape;
use common::{self, na};
use genmesh::{self, Triangulate};
use gfx::traits::FactoryExt;
use gfx::{self, format, handle};
use image_utils;
use obj;
use slog;

use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use common::graphics::{Drawable, Material, Vertex};

type Polygon = genmesh::Polygon<obj::IndexTuple>;

/// Loads an OBJ file from the provided path, and creates a `Drawable` and `TriMesh` for each object
/// from it
pub fn load_obj<R, F>(
    assets: &assets::Assets,
    factory: &mut F,
    name: &str,
    material: Material,
    log: &slog::Logger,
) -> Result<Vec<(Drawable<R>, shape::TriMesh<::Float>)>, ObjError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    // Read data from the file
    let path = assets.get_model_path(name.to_owned() + ".obj");
    let data = assets::read_bytes(&path).map_err(|e| ObjError::Io(IoError(path.clone(), e)))?;

    let mut buf_reader = BufReader::new(data.as_slice());
    let mut obj =
        obj::Obj::load_buf(&mut buf_reader).map_err(|e| ObjError::Io(IoError(path, e)))?;

    for path in &mut obj.material_libs {
        *path = assets.get_model_path(&path).to_str().unwrap().to_string();
    }

    obj.load_mtls().map_err(|e| {
        let first_error = e.into_iter().next().unwrap();
        ObjError::MtlError(MtlError {
            path: first_error.0,
            err: first_error.1,
        })
    })?;

    obj.objects
        .iter()
        .flat_map(|o| {
            load_object(&obj, o, log)
                .into_iter()
                .map(|(vertices, diffuse_tex_path)| {
                    let tex_path = diffuse_tex_path.replace("_diffuse.png", "").to_owned();

                    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

                    let diffuse = load_texture::<_, image_utils::Srgba8, _, _>(
                        factory,
                        &assets.get_model_path(tex_path.clone() + "_diffuse.png"),
                    )?;

                    let specular = load_texture::<_, image_utils::Srgba8, _, _>(
                        factory,
                        &assets.get_model_path(tex_path + "_specular.png"),
                    )?;

                    let drawable = Drawable::new(vbuf, slice, diffuse, specular, material);

                    let mesh = {
                        // Collect vertices of the mesh
                        let mesh_vertices = vertices
                            .iter()
                            .map(|v| {
                                na::Point3::new(
                                    v.pos[0] as ::Float,
                                    v.pos[1] as ::Float,
                                    v.pos[2] as ::Float,
                                )
                            })
                            .collect::<Vec<_>>();

                        if mesh_vertices.is_empty() {
                            return Err(ObjError::EmptyObj);
                        }

                        // Collect indices of the mesh
                        let mut mesh_indices = Vec::new();
                        let mut i = 0;

                        while i < mesh_vertices.len() - 1 {
                            mesh_indices.push(na::Point3::new(i, i + 1, i + 2));
                            i += 3;
                        }

                        shape::TriMesh::new(
                            mesh_vertices,
                            mesh_indices,
                            None,
                        )
                    };

                    Ok((drawable, mesh))
                })
                // Collecting into a vector then immediately consuming it is necessary to avoid a
                // re-borrow lifetime error
                .collect::<Vec<_>>()
                .into_iter()
        }).collect()
}

// TODO: Use indices instead of cloning data to save memory
/// Loads the provided object from the `Obj`, and returns a list of tuples containing the vertices
/// of a mesh and their path
fn load_object<'a>(
    obj: &obj::Obj<'a, Polygon>,
    object: &obj::Object<'a, Polygon>,
    log: &slog::Logger,
) -> Vec<(Vec<Vertex>, String)> {
    let mut objects = Vec::new();

    // Create an object per group
    for group in &object.groups {
        let mut vertices = Vec::new();

        for tri in group.polys.iter().cloned().triangulate() {
            // Create vertices from the triangles
            for v in &[tri.x, tri.y, tri.z] {
                let pos = obj.position[v.0];
                let uv = v.1.map(|i| obj.texture[i]).unwrap_or([0.0; 2]);
                let normal = v.2.map(|i| obj.normal[i]).unwrap_or([1.0; 3]);

                let pos = transform_coords(pos);
                let normal = transform_coords(normal);

                vertices.push(Vertex::new(pos, uv, normal));
            }
        }

        let material = group.material.as_ref().unwrap_or_else(|| {
            error!(log, "Missing material for group `{}`", group.name;);
            // TODO: Just use a default material instead of crashing here
            panic!(common::CRASH_MSG);
        });

        objects.push((
            vertices,
            material.map_kd.clone().unwrap_or_else(|| {
                error!(log, "Material `{}` has no diffuse texture", material.name;);
                // TODO: Just use a default texture instead of crashing here
                panic!(common::CRASH_MSG);
            }),
        ));
    }

    objects
}

/// Applies a transformation to the coordinates to make the in-game model match the view in Blender
fn transform_coords(arr: [f32; 3]) -> [f32; 3] {
    [arr[0] * -1.0, arr[2], arr[1]]
}

/// A wrapper for `std::io::Error` that includes the file path
#[derive(Debug)]
pub struct IoError(pub PathBuf, pub io::Error);

impl IoError {
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn err(&self) -> &io::Error {
        &self.1
    }
}

#[derive(Debug)]
pub struct MtlError {
    path: String,
    err: io::Error,
}

quick_error! {
    /// An error while loading an OBJ file
    #[derive(Debug)]
    pub enum ObjError {
        Io(err: IoError) {
            display("IO error while reading `{}``: {}",
                    err.path().to_str().expect("Path contained invalid UTF-8"), err.err())
            from()
        }
        Texture(err: image_utils::TextureError) {
            display("Texture creation error: {}", err)
            from()
        }
        EmptyObj {
            display("OBJ with no vertices")
        }
        MtlError(err: MtlError) {
            display("Error loading material at path `{}`: {}", err.path, err.err)
        }
    }
}

/// Loads a texture from the file at the provided path
///
/// The file should be in the PNG format.
fn load_texture<P, CF, R, F>(
    factory: &mut F,
    path: P,
) -> Result<handle::ShaderResourceView<R, CF::View>, ObjError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: AsRef<Path>,
    CF: format::Formatted,
    CF::Channel: format::TextureChannel,
    CF::Surface: format::TextureSurface,
{
    let data = assets::read_bytes(&path)
        .map_err(|e| ObjError::Io(IoError(path.as_ref().to_owned(), e)))?;

    image_utils::load_texture::<_, _, CF>(factory, &data, image_utils::PNG).map_err(|e| e.into())
}
