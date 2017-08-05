//! OBJ loading

use gfx::traits::FactoryExt;
use gfx;
use image_utils;
use obj;
use genmesh;

use std::io::{self, BufReader};

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
    let data = utils::read_bytes(get_path(name, ".obj"))?;

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

    let mut load_texture = |path| {
        let data = utils::read_bytes(path)?;
        let result: Result<_, ObjError> =
            image_utils::load_texture::<_, _, image_utils::Rgba8>(factory, &data, image_utils::PNG)
                .map_err(|e| e.into());
        result
    };

    let diffuse = load_texture(get_path(name, "_diffuse.png"))?;
    let specular = load_texture(get_path(name, "_specular.png"))?;

    Ok(Drawable::new(vbuf, slice, diffuse, specular, material))
}

quick_error! {
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

/// Returns the path to the file given the name of a model and a suffix to attach to it
fn get_path(name: &str, suffix: &str) -> String {
    format!(
        "{}/assets/models/{}{}",
        env!("CARGO_MANIFEST_DIR"),
        name,
        suffix
    )
}
