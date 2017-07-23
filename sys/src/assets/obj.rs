//! OBJ loading

use std::path::Path;
use std::io::{self, BufReader, Read};
use std::fs::File;
use gfx::traits::FactoryExt;
use gfx::{self, texture};
use obj;
use genmesh;

use graphics::draw::{Vertex, Drawable};

/// Loads an OBJ file from the provided path, and creates a `Drawable` component from it
pub fn create_drawable_from_obj_file<P, R, F>(
    path: P,
    factory: &mut F,
) -> Result<Drawable<R>, ObjError>
where
    P: AsRef<Path>,
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    // Read data from the file
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

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
                _ => unimplemented!(),
            }
        }
    }

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());
    let texels = [0xFF, 0x00, 0x00, 0xFF];
    let (_, texture_view) = factory
        .create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single),
            &[&[texels]],
        )
        .unwrap();

    Ok(Drawable::new(texture_view, vbuf, slice))
}

quick_error! {
    #[derive(Debug)]
    pub enum ObjError {
        Io(err: io::Error) {
            display("IO error: {}", err)
            from()
        }
    }
}
