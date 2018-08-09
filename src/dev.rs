//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use slog;
use common::gfx;
use common::specs::{self, Builder};
use common::cgmath::*;
use common::na::{self, Translation3};
use common::nphysics3d::math::{Inertia, Isometry};
use common::ncollide3d::shape::{self, ShapeHandle};
use common::nphysics3d::volumetric::Volumetric;
use common::nphysics3d::world::World;
use common::nphysics3d::object::{self, BodyMut, BodyStatus};
use common::*;
use common::physics::*;
use math::functions::dir_vec_to_quaternion;
use math::convert;
use control::Control;
use graphics::obj_loading;
use graphics::draw::{self, Material, LightSpaceMatrix};
use graphics::draw::components::*;
use assets::Assets;

use std::sync::Arc;

const COLLIDER_MARGIN: ::Float = 0.01;

pub fn add_test_entities<R, F>(world: &mut specs::World, factory: &mut F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let physics = {
        let mut phys_world = world.write_resource::<World<::Float>>();

        let geom = ShapeHandle::new(shape::Ball::new(1.0));

        let center_of_mass = geom.center_of_mass();
        let density = 100.0;
        let inertia = geom.inertia(density);

        let handle = phys_world.add_rigid_body(
            Isometry::new(na::Vector3::new(0.0, 0.0, 100.0), na::zero()),
            inertia,
            center_of_mass,
        );

        let collider = phys_world.add_collider(
            COLLIDER_MARGIN,
            geom,
            handle,
            Isometry::identity(),
            object::Material::new(0.0, 100.0),
        );

        Physics::new(handle, vec![], Some(collider), vec![])
    };

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
        .with(Player)
        .build();

    create_test_entity(
        world,
        factory,
        "test_map",
        [0.0; 3],
        Direction::default(),
        1.0,
        Material::new(32.0),
        Some(object::Material::new(0.0, 100.0)),
        Box::new(|e| e),
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
        //Some(object::Material::new(0.0, 100.0)),
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
            let mut dir_light = |x, y, z, shadows| {
                create_dir_light(
                    world,
                    [x, y, z],
                    light_color,
                    shadows,
                ).build();
            };

            let ortho_size = 60.0;
            let proj = Ortho {
                left: -ortho_size,
                right: ortho_size,
                bottom: -ortho_size,
                top: ortho_size,
                near: 1.0,
                far: ortho_size * 5.0,
            };
            let dir = cgmath::Vector3::new(1.0, -1.0, -1.0);
            let pos = cgmath::Point3::new(-10.0, 10.0, 5.0);
            let lsm = LightSpaceMatrix::from_components(proj, pos, dir.cast());
            dir_light(dir.x, dir.y, dir.z, Some(lsm));
        }

        // Create point lights
        {
            #[allow(unused)]
            let mut point_light = |x, y, z| {
                create_point_light(
                    world,
                    factory,
                    [x, y, z],
                    light_color,
                    LightAttenuation::new(1.0, 0.14, 0.07),
                    Box::new(|e| e),
                );
            };

            //point_light(0.0, 0.0, 10.0);
            //point_light(-5.0, -5.0, 1.5);
            //point_light(5.0, 3.0, 6.5);
            //point_light(5.0, -5.0, 3.5);
        }

        // Create spot lights
        {
            #[allow(unused)]
            let mut spot_light = |pos, dir| {
                create_spot_light(
                    world,
                    factory,
                    pos,
                    dir,
                    light_color,
                    LightAttenuation::new(1.0, 0.14, 0.07),
                    Deg(30.0),
                    Deg(45.0),
                    Box::new(|e| e),
                );
            };

            //spot_light([-4.0, -4.0, 10.0], [1.0, 1.0, -1.0]);
            //spot_light([0.0, 0.0, 65.0], [0.0, 0.0, -1.0]);
        }

    }
}

type MapEntity = Box<Fn(specs::EntityBuilder) -> specs::EntityBuilder>;

fn create_test_entity<'a, R, F, P>(
    world: &'a mut specs::World,
    factory: &mut F,
    name: &str,
    pos: P,
    dir: Direction,
    scale: f32,
    material: Material,
    properties: Option<object::Material<::Float>>,
    map: MapEntity,
) where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: Into<Option<[::Float; 3]>>,
{
    let pos = pos.into();
    let space = pos.map(|p| Position(Point3::new(p[0], p[1], p[2])));
    let scale = Scale::new(scale);
    let objects = obj_loading::load_obj(
        &world.read_resource::<Arc<Assets>>(),
        factory,
        name,
        material,
        &world.read_resource::<slog::Logger>(),
    ).unwrap();
    let shader_param = draw::ShaderParam::default();

    for (drawable, mesh) in objects {
        let physics = properties.clone().map(|props| {
            let mut phys_world = world.write_resource::<World<::Float>>();
            let pos = if let Some(s) = space {
                convert::to_na_point(s.0)
            } else {
                na::Point3::origin()
            };
            let pos_vec = if let Some(s) = space {
                convert::to_na_vector(s.0.to_vec())
            } else {
                na::zero()
            };

            let dir = convert::to_na_quaternion(dir.0);

            let isometry = Isometry::from_parts(Translation3::from_vector(pos_vec), dir);
            let handle = phys_world.add_rigid_body(
                isometry,
                Inertia::zero(),
                pos,
            );

            let collider = phys_world.add_collider(
                COLLIDER_MARGIN,
                ShapeHandle::new(mesh),
                handle,
                Isometry::identity(),
                props,
            );

            if let BodyMut::RigidBody(rb) = phys_world.body_mut(handle) {
                rb.set_status(BodyStatus::Static);
            }

            Physics::new(handle, Vec::new(), Some(collider), Vec::new())
        });

        let mut entity = world
            .create_entity()
            .with(scale)
            .with(drawable)
            .with(shader_param)
            .with(dir);

        if let Some(s) = space {
            entity = entity.with(s);
        }

        if let Some(physics) = physics {
            entity = entity
                .with(physics)
                .with(PhysicsTiedPosition)
                .with(PhysicsTiedDirection);
        }

        map(entity).build();
    }
}

fn create_dir_light<'a>(
    world: &'a mut specs::World,
    direction: [::Float; 3],
    color: LightColor,
    lsm: Option<LightSpaceMatrix>,
) -> specs::EntityBuilder<'a> {
    let direction = dir_vec_to_quaternion(direction);

    world
        .create_entity()
        .with(DirectionalLight::new(
            color,
            lsm,
        ))
        .with(Direction(direction))
}

fn create_point_light<'a, R, F>(
    world: &'a mut specs::World,
    factory: &mut F,
    pos: [::Float; 3],
    color: LightColor,
    attenuation: LightAttenuation,
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
        0.5,
        Material::new(0.0),
        None,
        Box::new(move |e| {
            let e = e.with(PointLight::new(
                color,
                attenuation,
            ));
            map(e)
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
                    angle.into(),
                    outer_angle.into(),
                    attenuation,
                ).unwrap(),
            );

            map(e)
        }),
    );
}
