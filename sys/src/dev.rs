//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use specs;
use gfx::{self, texture};
use gfx::traits::FactoryExt;
use cgmath::{self, Rotation3};
use na::{Vector3, Translation3};
use nphysics3d::object::RigidBody;
use ncollide::shape::{Cuboid, Plane};

use graphics::draw::{self, Vertex};
use world::components::*;
use physics::components::*;
use control::Control;
use assets::obj;
use player;

pub fn add_test_entities<R, F>(world: &mut specs::World, factory: &mut F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let body_init = || {
        let geom = Cuboid::new(Vector3::new(1.0, 1.0, 2.0));
        let mut body = RigidBody::new_dynamic(geom, 100.0, 0.0, 100.0);
        body.append_translation(&Translation3::new(5.0, 5.0, 15.0));
        body
    };

    let physics = Physics::new(body_init, true);
    let space = Spatial(cgmath::Point3::new(0.0, 0.0, 0.0));
    let direction = Direction(cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)));
    let control = Control::default();

    // Add player entity
    world
        .create_entity()
        .with(physics)
        .with(space)
        .with(direction)
        .with(control)
        .with(PhysicsTiedPosition)
        .with(player::components::Player);

    let body_init = || {
        let geom = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
        let mut body = RigidBody::new_dynamic(geom, 10.0, 0.0, 10.0);
        body.append_translation(&Translation3::new(0.0, 0.0, 10.0));
        body
    };

    let physics = Physics::new(body_init, true);

    let space = Spatial(cgmath::Point3::new(0.0, 0.0, 0.0));
    let direction = Direction(cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)));
    let control = Control::default();

    // TODO: make shader loading happen at runtime
    //       make lighting and shadows in shaders
    let draw = obj::create_drawable_from_obj_file(
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/models/suzanne.obj"),
        factory,
    ).unwrap();
    let shader_param = draw::ShaderParam::default();

    // Add test dummy entity
    world
        .create_entity()
        .with(draw)
        .with(shader_param)
        .with(physics)
        .with(space)
        .with(direction)
        .with(PhysicsTiedPosition)
        .with(PhysicsTiedDirection)
        .with(control);

    let body_init = || {
        let geom = Plane::new(Vector3::new(0.0 as ::Float, 0.0, 1.0));
        RigidBody::new_static(geom, 1.0, 1.0)
    };

    let physics = Physics::new(body_init, false);
    let space = Spatial(cgmath::Point3::new(0.0, 0.0, 0.0));

    let texels = [0x10, 0xC0, 0x20, 0xFF];
    let (vertices, indices) = create_plane();
    let draw = create_drawable(factory, vertices, indices, texels);
    let shader_param = draw::ShaderParam::default();

    // Add a plane to test physics on
    world
        .create_entity()
        .with(physics)
        .with(space)
        .with(draw)
        .with(shader_param);
}

fn create_drawable<R, F>(
    factory: &mut F,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texels: [u8; 4],
) -> draw::Drawable<R>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let (texture, diffuse, specular) = {
        let mut create_texture = |texels| {
            factory
                .create_texture_immutable::<gfx::format::Rgba8>(
                    texture::Kind::D2(1, 1, texture::AaMode::Single),
                    &[&[texels]],
                )
                .unwrap()
                .1
        };

        let texture = create_texture(texels);
        let diffuse = create_texture(texels);
        let specular = create_texture([128, 128, 128, 255]);

        (texture, diffuse, specular)
    };

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, indices.as_slice());

    draw::Drawable::new(vbuf, slice, texture, diffuse, specular)
}

fn create_plane() -> (Vec<Vertex>, Vec<u16>) {
    let normal = [0.0, 0.0, 1.0];
    let mut vertex_data = vec![
        Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0], normal),
        Vertex::new([-1.0, 1.0, 0.0], [0.0, 1.0], normal),
        Vertex::new([1.0, -1.0, 0.0], [1.0, 0.0], normal),
        Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0], normal),
    ];
    let scale = 15.0;
    vertex_data = vertex_data
        .into_iter()
        .map(|v| {
            Vertex::new(
                [v.pos[0] * scale, v.pos[1] * scale, v.pos[2] * scale],
                v.uv,
                Default::default(),
            )
        })
        .collect();

    let index_data = vec![0, 1, 2, 2, 3, 1];

    (vertex_data, index_data)
}
