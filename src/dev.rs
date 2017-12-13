//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use specs;
use gfx;
use cgmath::*;
use na::{self, Translation3};
use na::geometry::TranslationBase;
use common::nphysics3d::object::RigidBody;
use common::nphysics3d::math::Isometry;
use common::ncollide::shape::Cuboid;
use common::*;

use graphics::draw::{self, Material};
use graphics::draw::components::*;
use physics::components::*;
use control::Control;
use assets::obj;
use math::functions::dir_vec_to_quaternion;
use math::convert;
use player;

pub fn add_test_entities<R, F>(world: &mut specs::World, factory: &mut F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let body_init = || {
        let geom = Cuboid::new(na::Vector3::new(1.0, 1.0, 2.0));
        let mut body = RigidBody::new_dynamic(geom, 100.0, 0.0, 100.0);
        body.append_translation(&Translation3::new(0.0, 0.0, 100.0));
        body
    };

    let physics = Physics::new(Box::new(body_init), true);
    let space = Position(Point3::new(0.0, 0.0, 0.0));
    let direction = Direction::default();
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

    create_test_entity(
        world,
        factory,
        "test_map",
        [0.0; 3],
        Direction::default(),
        1.0,
        Material::new(32.0),
        Some(GenericProperties(0.0, 100.0)),
        Box::new(|_| {}),
    );

    // Create test entities
    {
        //let mut cube = |pos, size, dir| {
        //let _ = create_test_entity(
        //world,
        //factory,
        //"box",
        //pos,
        //dir,
        //size,
        //Material::new(32.0),
        //Some(GenericProperties(0.0, 100.0)),
        //);
        //};

        //cube([-4.0, 0.0, 5.0], 2.0, Direction::default());
        //cube([2.0, 4.0, 1.0], 2.0, Direction::default());
        //cube([3.0, -1.0, 3.0], 2.0, Direction::default());
        //cube(
        //[5.0, 5.0, 3.5],
        //2.0,
        //Direction(dir_vec_to_quaternion([1.0, 1.0, 1.0])),
        //);
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
            #[allow(unused)]
            let mut dir_light = |x, y, z, pos| {
                let _ = create_dir_light(
                    world,
                    [x, y, z],
                    pos,
                    light_color,
                    40.0,
                    ShadowSettings::Disabled,
                );
            };

            dir_light(1.0, -1.0, -1.0, [-10.0, 10.0, 5.0]);
        }

        // Create point lights
        {
            #[allow(unused)]
            let mut point_light = |x, y, z| {
                let _ = create_point_light(
                    world,
                    factory,
                    [x, y, z],
                    light_color,
                    LightAttenuation::new(1.0, 0.14, 0.07),
                    ShadowSettings::Enabled,
                    Box::new(|_| {}),
                );
            };

            //point_light(0.0, 0.0, 10.0);
            //point_light(-5.0, -5.0, 1.5);
            //point_light(5.0, 3.0, 6.5);
            //point_light(5.0, -5.0, 3.5);
            //point_light(-3.0, 7.0, 10.0);
        }

        // Create spot lights
        {
            #[allow(unused)]
            let mut spot_light = |pos, dir| {
                let _ = create_spot_light(
                    world,
                    factory,
                    pos,
                    dir,
                    light_color,
                    LightAttenuation::new(1.0, 0.14, 0.07),
                    Deg(30.0),
                    Deg(45.0),
                    ShadowSettings::Enabled,
                    Box::new(|_| {}),
                );
            };

            //spot_light([-4.0, -4.0, 10.0], [1.0, 1.0, -1.0]);
            //spot_light([0.0, 0.0, 65.0], [0.0, 0.0, -1.0]);
        }

    }
}

#[derive(Clone, Copy)]
struct GenericProperties(::Float, ::Float);

type MapEntity = Box<Fn(specs::EntityBuilder)>;

fn create_test_entity<'a, R, F, P>(
    world: &'a mut specs::World,
    factory: &mut F,
    name: &str,
    pos: P,
    dir: Direction,
    scale: f32,
    material: Material,
    properties: Option<GenericProperties>,
    map: MapEntity,
) where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: Into<Option<[::Float; 3]>>,
{
    let pos = pos.into();
    let space = pos.map(|p| Position(Point3::new(p[0], p[1], p[2])));
    let scale = Scale::new(scale);
    let objects = obj::load_obj(factory, name, material).unwrap();
    let shader_param = draw::ShaderParam::default();

    for (drawable, mesh) in objects {
        let mut entity = world
            .create_entity()
            .with(scale)
            .with(drawable)
            .with(shader_param)
            .with(dir);

        if let Some(s) = space {
            entity = entity.with(s);
        }

        let mut mesh_opt = Some(mesh);
        if let Some(p) = properties {
            let body_init = move || {
                let m = mesh_opt.take().unwrap();

                let pos = if let Some(s) = space {
                    convert::to_na_vector(s.0.to_vec())
                } else {
                    na::Vector3::zero()
                };

                let dir = convert::to_na_quaternion(dir.0);

                let mut rb = RigidBody::new_static(m, p.0, p.1);

                let transform = Isometry::from_parts(TranslationBase::from_vector(pos), dir);

                rb.set_transformation(transform);

                rb
            };

            let physics = Physics::new(Box::new(body_init), false);
            entity = entity.with(physics).with(PhysicsTiedPosition).with(
                PhysicsTiedDirection,
            );
        }

        map(entity);
    }
}

fn create_dir_light<'a>(
    world: &'a mut specs::World,
    direction: [::Float; 3],
    pos: [::Float; 3],
    color: LightColor,
    ortho_size: f32,
    shadow_settings: ShadowSettings,
) -> specs::EntityBuilder<'a> {
    let direction = dir_vec_to_quaternion(direction);

    world
        .create_entity()
        .with(DirectionalLight::new(
            color,
            shadow_settings,
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
        .with(Position(pos.into()))
}

fn create_point_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [::Float; 3],
    color: LightColor,
    attenuation: LightAttenuation,
    shadow_settings: ShadowSettings,
    map: MapEntity,
) where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    create_test_entity(
        world,
        factory,
        "light",
        pos,
        Direction::default(),
        0.2,
        Material::new(0.0),
        None,
        Box::new(move |e| {
            let e = e.with(PointLight::new(
                color,
                shadow_settings,
                attenuation,
                ProjectionData::new(1.0, 50.0),
            ));
            map(e);
        }),
    )
}

fn create_spot_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [::Float; 3],
    direction: [::Float; 3],
    color: LightColor,
    attenuation: LightAttenuation,
    angle: Deg<f32>,
    outer_angle: Deg<f32>,
    shadow_settings: ShadowSettings,
    map: MapEntity,
) where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let direction = dir_vec_to_quaternion(direction);

    create_test_entity(
        world,
        factory,
        "light",
        pos,
        Direction(direction),
        0.5,
        Material::new(0.0),
        None,
        Box::new(move |e| {
            let e = e.with(
                SpotLight::new(
                    color,
                    shadow_settings,
                    angle.into(),
                    outer_angle.into(),
                    attenuation,
                    ProjectionData::new(1.0, 50.0),
                ).unwrap(),
            );

            map(e);
        }),
    );
}
