//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use specs;
use gfx;
use cgmath::{self, Rotation3, InnerSpace};
use na::{Vector3, Translation3};
use nphysics3d::object::RigidBody;
use ncollide::shape::{Cuboid, Plane};

use graphics::draw::{self, Material};
use graphics::draw::components::*;
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

    let light_color = LightColor::new(
        [0.1, 0.1, 0.1, 1.0],
        [1.0, 1.0, 1.0, 1.0],
        [0.5, 0.5, 0.5, 1.0],
    );

    // Create some lights
    create_point_light(
        world,
        factory,
        [5.0, 3.0, 6.5],
        light_color,
        1.0,
        0.14,
        0.07,
    );

    create_point_light(
        world,
        factory,
        [-5.0, -5.0, 1.5],
        light_color,
        1.0,
        0.1,
        0.04,
    );

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

fn create_dir_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [f64; 3],
    direction: [f32; 3],
    color: LightColor,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let direction = cgmath::Quaternion::from_axis_angle(direction.into(), cgmath::Deg(0.0));
    let direction = cgmath::Quaternion::from_sv(direction.s as f64, direction.v.cast());
    create_test_entity(world, factory, "sphere", pos, 0.5, Material::new(0.0))
        .with(DirectionalLight::new(color))
        .with(Direction(direction))
}

fn create_point_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [f64; 3],
    color: LightColor,
    constant: f32,
    linear: f32,
    quadratic: f32,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    create_test_entity(world, factory, "light", pos, 0.5, Material::new(0.0))
        .with(PointLight::new(color, constant, linear, quadratic))
}

fn create_spot_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [f64; 3],
    direction: [f32; 3],
    color: LightColor,
    angle: cgmath::Deg<f32>,
    outer_angle: cgmath::Deg<f32>,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let direction = cgmath::Quaternion::from_axis_angle(direction.into(), cgmath::Deg(0.0));
    let direction = cgmath::Quaternion::from_sv(direction.s as f64, direction.v.cast());
    create_test_entity(world, factory, "sphere", pos, 0.5, Material::new(0.0))
        .with(SpotLight::new(color, angle.into(), outer_angle.into()))
        .with(Direction(direction))
}
