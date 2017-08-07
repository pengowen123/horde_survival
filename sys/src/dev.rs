//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use specs;
use gfx;
use cgmath::{self, Rotation3, InnerSpace};
use na::{Vector3, Translation3};
use nphysics3d::object::RigidBody;
use ncollide::shape::{Cuboid, Plane};

use graphics::draw::{self, Material};
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

    // Create test entities
    create_test_entity(
        world,
        factory,
        "box",
        [0.0, 0.0, 1.0],
        2.0,
        Material::new(32.0),
    );
    create_test_entity(
        world,
        factory,
        "box",
        [3.0, 2.0, 5.0],
        2.0,
        Material::new(32.0),
    );
    let direction = Direction(cgmath::Quaternion::from_axis_angle(
        cgmath::vec3(1.0, 1.0, 1.0).normalize(),
        cgmath::Deg(60.0),
    ));
    create_test_entity(
        world,
        factory,
        "box",
        [5.0, -3.0, 1.66],
        2.0,
        Material::new(32.0),
    ).with(direction);

    // Add a plane to test physics on
    let body_init = || {
        let geom = Plane::new(Vector3::new(0.0 as ::Float, 0.0, 1.0));
        RigidBody::new_static(geom, 1.0, 1.0)
    };

    let physics = Physics::new(body_init, false);

    create_test_entity(world, factory, "floor", [0.0; 3], 15.0, Material::new(32.0)).with(physics);
}

fn create_test_entity<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    name: &str,
    pos: [::Float; 3],
    scale: f32,
    material: Material,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let space = Spatial(cgmath::Point3::new(pos[0], pos[1], pos[2]));
    let scale = draw::components::Scale(scale);
    let drawable = obj::create_drawable_from_obj_file(factory, name, material).unwrap();
    let shader_param = draw::ShaderParam::default();

    world
        .create_entity()
        .with(space)
        .with(scale)
        .with(drawable)
        .with(shader_param)
}
