//! Temporary hacks to set the game up for testing
//! Will be removed when no longer needed

use assets::Assets;
use common::cgmath::*;
use common::gfx::{self, handle, format};
use common::graphics::{Material, Particle, ParticleSource, ShaderParam, SpawnParticleFn};
use common::na::{self, Translation3};
use common::ncollide3d::shape::ShapeHandle;
use common::nphysics3d::math::{Inertia, Isometry};
use common::nphysics3d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use common::nphysics3d::world::World;
use common::nphysics3d::material;
use common::physics::*;
use common::specs::{self, Builder};
use common::*;
use control::FloorHandle;
use graphics::draw::components::*;
use graphics::draw::LightSpaceMatrix;
use graphics::obj_loading;
use math::convert;
use math::functions::dir_vec_to_quaternion;
use physics::scale::Scale as ScaleTrait;
use player::{self, COLLIDER_MARGIN};
use slog;
use image_utils;

use std::sync::Arc;

pub fn add_test_entities<R, F>(world: &mut specs::World, factory: &mut F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let (floor_body, floor_collider) = create_test_entity(
        world,
        factory,
        "player_controller_playground",
        [0.0, 0.0, 0.0],
        Direction::default(),
        5.0,
        Material::new(32.0),
        Some(material::BasicMaterial::new(0.0, 0.0)),
        Box::new(|e| e),
    ).into_iter().nth(0).unwrap();

    // Set the floor collider handle to the test map's first collision object
    world
        .write_resource::<FloorHandle>()
        .set_floor(floor_body, floor_collider);

    player::add_player_entity(world);

    // Create test entities
    {
        {
            let mut cube = |pos, size, dir| {
                let _ = create_test_entity(
                    world,
                    factory,
                    "box",
                    pos,
                    dir,
                    size,
                    Material::new(32.0),
                    Some(material::BasicMaterial::new(0.0, 0.0)),
                    Box::new(|e| e),
                );
            };

            cube([0.0, 0.0, 15.0], 1.0, Direction::default());
            //cube([2.0, 4.0, 1.0], 2.0, Direction::default());
            //cube([3.0, -1.0, 3.0], 2.0, Direction::default());
            //cube(
            //[5.0, 5.0, 3.5],
            //2.0,
            //Direction(dir_vec_to_quaternion([1.0, 1.0, 1.0])),
            //);
        }

        let particle_source = ParticleSource::new(
            15,
            5.0,
            Box::new(|pos: &Point3<f32>| {
                Particle::new(
                    1.0,
                    0.4,
                    *pos,
                    Vector3::new(2.0, 2.0, 5.0),
                    -1.0,
                    3.0,
                )
            }) as SpawnParticleFn,
            load_texture(
                factory,
                &world.read_resource::<Arc<Assets>>(),
                "test_particle.png",
                image_utils::PNG,
            ),
        ).unwrap();

        world
            .create_entity()
            .with(Position(Point3::new(5.0, 5.0, 5.0)))
            .with(particle_source)
            .build();
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
                create_dir_light(world, [x, y, z], light_color, shadows).build();
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
            let lsm = LightSpaceMatrix::from_components(proj, pos, dir.cast().unwrap());
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
    properties: Option<material::BasicMaterial<::Float>>,
    map: MapEntity,
) -> Vec<(BodyHandle, ColliderDesc<::Float>)>
where
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
    let shader_param = ShaderParam::default();

    let mut body_handles = Vec::new();

    for (drawable, mesh) in objects {
        let physics = properties.clone().map(|props| {
            let mut phys_world = world.write_resource::<World<::Float>>();

            let pos_vec = if let Some(s) = space {
                convert::to_na_vector(s.0.to_vec())
            } else {
                na::zero()
            };

            let dir = convert::to_na_quaternion(dir.0);

            let scaled_mesh = mesh
                .scale(scale.get().into())
                .expect(&format!("Failed to scale mesh for entity: `{}`", name));

            let shape_handle = ShapeHandle::new(scaled_mesh);
            let make_collider_desc = || ColliderDesc::new(shape_handle.clone())
                .margin(COLLIDER_MARGIN)
                .material(material::MaterialHandle::new(props));

            let collider_desc = make_collider_desc();

            let isometry = Isometry::from_parts(Translation3::from(pos_vec), dir);
            let rb = RigidBodyDesc::new()
                .collider(&collider_desc)
                .position(isometry)
                .local_inertia(Inertia::zero())
                .set_status(BodyStatus::Static)
                .build(&mut phys_world);

            let handle = rb.handle();

            body_handles.push((handle, collider_desc));

            Physics::new(handle, Vec::new(), Some(make_collider_desc()), Vec::new())
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

    body_handles
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
        .with(DirectionalLight::new(color, lsm))
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
    let _ = create_test_entity(
        world,
        factory,
        "light",
        pos,
        Direction::default(),
        0.5,
        Material::new(0.0),
        None,
        Box::new(move |e| {
            let e = e.with(PointLight::new(color, attenuation));
            map(e)
        }),
    );
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

    let _ = create_test_entity(
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
                SpotLight::new(color, angle.into(), outer_angle.into(), attenuation).unwrap(),
            );

            map(e)
        }),
    );
}

fn load_texture<R: gfx::Resources, F: gfx::Factory<R>>(
    factory: &mut F,
    assets: &Assets,
    path: &str,
    format: image_utils::ImageFormat,
) -> handle::ShaderResourceView<R, [f32; 4]> {
    let texture_path = assets.get_model_path(path);
    let data = assets::read_bytes(texture_path).unwrap();
    image_utils::load_texture::<_, _, format::Srgba8>(factory, &data, format).unwrap()
}
