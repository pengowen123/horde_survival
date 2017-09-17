//! OBJ loading

use gfx::traits::FactoryExt;
use gfx::{self, handle, format};
use image_utils;
use obj;
use genmesh;

use std::io::{self, BufReader};
use std::path::Path;

use graphics::draw::{Vertex, Drawable, Material};
use super::utils;

/// Loads an OBJ file from the provided path, and creates a `Drawable` component from it
pub fn create_drawable_from_obj_file<R, F>(
    factory: &mut F,
    name: &str,
    material: Material,
) -> Result<Drawable<R>, ObjError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    // Read data from the file
    let data = utils::read_bytes(super::get_model_file_path(name, ".obj"))?;

    let mut buf_reader = BufReader::new(data.as_slice());
    let data = obj::Obj::load_buf(&mut buf_reader)?;

    let mut vertices = Vec::new();

    for object in &data.objects {
        for shape in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match *shape {
                genmesh::Polygon::PolyTri(genmesh::Triangle { x: a, y: b, z: c }) => {
                    for v in &[a, b, c] {
                        let pos = data.position[v.0];
                        let uv = v.1.map(|i| data.texture[i]).unwrap_or([0.0; 2]);
                        let normal = v.2.map(|i| data.normal[i]).unwrap_or([0.0; 3]);

                        vertices.push(Vertex::new(pos, uv, normal));
                    }
                }
                p @ _ => {
                    println!("unknown polygon: {:?}", p);
                }
            }
        }
    }

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

    let diffuse = load_texture::<_, image_utils::Srgba8, _, _>(
        factory,
        &super::get_model_file_path(name, "_diffuse.png"),
    )?;

    let specular = load_texture::<_, image_utils::Srgba8, _, _>(
        factory,
        &super::get_model_file_path(name, "_specular.png"),
    )?;

    Ok(Drawable::new(vbuf, slice, diffuse, specular, material))
}

quick_error! {
    /// An error while loading an OBJ file
    #[derive(Debug)]
    pub enum ObjError {
        Io(err: io::Error) {
            display("IO error: {}", err)
            from()
        }
        Texture(err: image_utils::TextureError) {
            display("Texture creation error: {}", err)
            from()
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
    let data = utils::read_bytes(path)?;

    image_utils::load_texture::<_, _, CF>(factory, &data, image_utils::PNG).map_err(|e| e.into())
}
