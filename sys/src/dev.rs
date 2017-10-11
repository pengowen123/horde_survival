//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use specs;
use gfx;
use cgmath::*;
use na::{self, Translation3};
use nphysics3d::object::RigidBody;
use ncollide::shape::{Cuboid, Plane};

use graphics::draw::{self, Material};
use graphics::draw::components::*;
use world::components::*;
use physics::components::*;
use control::Control;
use assets::obj;
use math::functions::dir_vec_to_quaternion;
use player;

pub fn add_test_entities<R, F>(world: &mut specs::World, factory: &mut F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let body_init = || {
        let geom = Cuboid::new(na::Vector3::new(1.0, 1.0, 2.0));
        let mut body = RigidBody::new_dynamic(geom, 100.0, 0.0, 100.0);
        body.append_translation(&Translation3::new(0.0, 0.0, 15.0));
        body
    };

    let physics = Physics::new(body_init, true);
    let space = Spatial(Point3::new(0.0, 0.0, 0.0));
    let direction = Direction(Quaternion::from_angle_y(Deg(0.0)));
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

    // Add a plane to test physics on
    let body_init = || {
        let geom = Plane::new(na::Vector3::new(0.0 as ::Float, 0.0, 1.0));
        RigidBody::new_static(geom, 1.0, 1.0)
    };

    let physics = Physics::new(body_init, false);

    create_test_entity(world, factory, "floor", [0.0; 3], 15.0, Material::new(32.0)).with(physics);

    // Create test entities
    {
        let mut cube = |pos, size| {
            let _ = create_test_entity(world, factory, "box", pos, size, Material::new(32.0));
        };

        cube([-4.0, 0.0, 5.0], 2.0);
        cube([2.0, 4.0, 1.0], 2.0);
        cube([3.0, -1.0, 3.0], 2.0);
    }

    // Create some lights
    {
        let light_color = LightColor::new(
            [0.1, 0.1, 0.1, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.5, 0.5, 0.5, 1.0],
        );

        // Create directional lights
        {
            let mut dir_light = |x, y, z, pos| {
                let _ = create_dir_light(world, [x, y, z], pos, light_color, 20.0);
            };

            //dir_light(1.0, -1.0, -1.0, [-10.0, 10.0, 5.0]);
        }

        // Create point lights
        {
            let mut point_light = |x, y, z| {
                let _ = create_point_light(
                    world,
                    factory,
                    [x, y, z],
                    light_color,
                    LightAttenuation::new(1.0, 0.14, 0.07),
                );
            };

            point_light(0.0, 0.0, 10.0);
            //point_light(-5.0, -5.0, 1.5);
            //point_light(5.0, 3.0, 6.5);
            //point_light(5.0, -5.0, 3.5);
            //point_light(-3.0, 7.0, 10.0);
        }

        // Create spot lights
        {
            let mut spot_light = |pos, dir| {
                let _ =
                    create_spot_light(world, factory, pos, dir, light_color, Deg(12.5), Deg(17.5));
            };

            //spot_light([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]);
        }

    }
}

fn create_test_entity<'a, R, F, P>(
    world: &'a mut specs::World,
    factory: &mut F,
    name: &str,
    pos: P,
    scale: f32,
    material: Material,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: Into<Option<[::Float; 3]>>,
{
    let pos = pos.into();
    let space = pos.map(|p| Spatial(Point3::new(p[0], p[1], p[2])));
    let scale = draw::components::Scale(scale);
    let drawable = obj::create_drawable_from_obj_file(factory, name, material).unwrap();
    let shader_param = draw::ShaderParam::default();

    let mut entity = world.create_entity().with(scale).with(drawable).with(
        shader_param,
    );

    if let Some(s) = space {
        entity = entity.with(s);
    }

    entity
}

fn create_dir_light<'a>(
    world: &'a mut specs::World,
    direction: [f64; 3],
    pos: [f64; 3],
    color: LightColor,
    ortho_size: f32,
) -> specs::EntityBuilder<'a> {
    let direction = dir_vec_to_quaternion(direction);

    world
        .create_entity()
        .with(DirectionalLight::new(
            color,
            ShadowSettings::Enabled,
            Ortho {
                left: -ortho_size,
                right: ortho_size,
                bottom: -ortho_size,
                top: ortho_size,
                near: 1.0,
                far: ortho_size * 10.0,
            },
        ))
        .with(Direction(direction))
        .with(Spatial(pos.into()))
}

fn create_point_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [f64; 3],
    color: LightColor,
    attenuation: LightAttenuation,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    create_test_entity(world, factory, "light", pos, 0.2, Material::new(0.0)).with(PointLight::new(
        color,
        ShadowSettings::Enabled,
        attenuation,
        ProjectionData::new(
            0.15,
            25.0,
        ),
    ))
}

fn create_spot_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [f64; 3],
    direction: [f64; 3],
    color: LightColor,
    angle: Deg<f32>,
    outer_angle: Deg<f32>,
) -> specs::EntityBuilder<'a>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let direction = dir_vec_to_quaternion(direction);

    create_test_entity(world, factory, "light", pos, 0.5, Material::new(0.0))
        .with(
            SpotLight::new(
                color,
                ShadowSettings::Enabled,
                angle.into(),
                outer_angle.into(),
            ).unwrap(),
        )
        .with(Direction(direction))
}
