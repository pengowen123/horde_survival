//! Setup for the player entity

use common::ncollide3d::shape::{self, ShapeHandle};
use common::nphysics3d::math::Isometry;
use common::nphysics3d::object;
use common::nphysics3d::volumetric::Volumetric;
use common::physics::{Physics, PhysicsTiedPosition};
use common::specs::{self, Builder};
use common::{cgmath, na, nphysics3d};
use common::{Direction, Player, Position};

use control::{Control, MovementForceGenerator, Spring};

/// The radius of the collider of the player physics body
pub const PLAYER_COLLIDER_RADIUS: ::Float = 0.5;

/// The friction applied to the player physics body
pub const PLAYER_FRICTION: ::Float = 0.85;

/// The collider margin for the player physics body
pub const COLLIDER_MARGIN: ::Float = 0.01;

/// The height of the spring used to prop up the player physics body
pub const PLAYER_SPRING_HEIGHT: ::Float = 5.0;

/// The stiffness of the spring
pub const PLAYER_SPRING_STIFFNESS: ::Float = 11000.0;

/// The friction of the spring
pub const PLAYER_SPRING_FRICTION: ::Float = 1300.0;

/// The maximum speed of the player physics body
const PLAYER_MAX_SPEED: ::Float = 10.0;

/// The acceleration rate of the player physics body
const PLAYER_ACCELERATION: ::Float = 5000.0;

/// The jump strength of the player physics body
const PLAYER_JUMP_STRENGTH: ::Float = 13.0;

pub fn add_player_entity(world: &mut specs::World) {
    let (physics, control) = {
        let mut phys_world = world.write_resource::<nphysics3d::world::World<::Float>>();

        let geom = ShapeHandle::new(shape::Ball::new(PLAYER_COLLIDER_RADIUS));

        let center_of_mass = geom.center_of_mass();
        // The density is normalized by the player collider size to keep forces consistent
        // regardless of it
        let density = 100.0 / (PLAYER_COLLIDER_RADIUS * PLAYER_COLLIDER_RADIUS);
        let inertia = geom.inertia(density);

        let handle = phys_world.add_rigid_body(
            Isometry::new(na::Vector3::new(-5.0, -5.0, 20.0), na::zero()),
            inertia,
            center_of_mass,
        );

        let collider = phys_world.add_collider(
            COLLIDER_MARGIN,
            geom,
            handle,
            Isometry::identity(),
            object::Material::new(0.0, 0.0),
        );

        let physics = Physics::new(handle, vec![], Some(collider), vec![]);
        let control = {
            let movement = MovementForceGenerator::new(
                PLAYER_ACCELERATION,
                PLAYER_MAX_SPEED,
                PLAYER_JUMP_STRENGTH,
            );

            let spring = Spring::new(
                PLAYER_SPRING_HEIGHT,
                PLAYER_SPRING_STIFFNESS,
                PLAYER_SPRING_FRICTION,
            );

            Control::new(handle, movement, spring, PLAYER_FRICTION, &mut phys_world)
        };

        (physics, control)
    };

    let space = Position(cgmath::Point3::new(0.0, 0.0, 0.0));
    let direction = Direction::default();

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
