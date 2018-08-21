//! Setup for the player entity

use common::ncollide3d::shape::{self, ShapeHandle};
use common::nphysics3d::volumetric::Volumetric;
use common::nphysics3d::object;
use common::nphysics3d::math::Isometry;
use common::{Player, Position, Direction};
use common::physics::{Physics, PhysicsTiedPosition};
use common::specs::{self, Builder};
use common::{cgmath, na, nphysics3d};

use control::Control;

/// The collider margin for the player physics body
pub const COLLIDER_MARGIN: ::Float = 0.01;

pub fn add_player_entity(world: &mut specs::World) {
    let physics = {
        let mut phys_world = world.write_resource::<nphysics3d::world::World<::Float>>();

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

    let space = Position(cgmath::Point3::new(0.0, 0.0, 0.0));
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
}
