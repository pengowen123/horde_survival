//! Components and systems for physics simulation

pub mod handle;
mod utils;

use specs::{self, Join, DispatcherBuilder};
use nphysics3d::world::World;
use nphysics3d::object::RigidBodyHandle;
use nphysics3d::math::Rotation;
use na::{self, geometry};

use std::collections::HashMap;
use std::mem;

use world;
use self::handle::{BodyHandle, BodyId};
use delta;

pub struct System {
    world: World<::Float>,
    /// Contains handles to all physics bodies
    // The bool here is to help with determining which bodies to remove
    // It represents whether the entity has been visited this run of the system
    body_handles: HashMap<handle::BodyId, (RigidBodyHandle<::Float>, bool)>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // TODO: Find a way for controlled entities (by ai or player) to supply inertia to their body
    //       Prefer not to force other entities such as the map to have it if possible
    space: specs::WriteStorage<'a, world::Spatial>,
    physics: specs::WriteStorage<'a, world::Physics>,
    delta: specs::Fetch<'a, delta::Delta>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // TODO: There may be room for optimization here (remove extra loops)
        //       Profile before doing anything
        let mut physics = data.physics;
        let mut space = data.space;
        let delta = data.delta.to_float();

        for p in (&mut physics).join() {
            // Must be called for every entity so dead ones can have their body removed
            let _ = self.get_or_create_handle(p);
        }

        // Simulate the world for `delta` seconds
        self.world.step(delta);

        // TODO: Test that this works
        self.remove_dead_bodies();

        for (p, s) in (&mut physics, &mut space).join() {
            let handle = self.get_or_create_handle(p);

            // TODO: Maybe use try_borrow to avoid panics (but maybe it isn't necessary here)
            let pos = handle.borrow().position_center();
            s.position = utils::to_cgmath_point(pos);

            if p.lock_rotation() {
                // TODO: Fix this
                //handle
                //.borrow_mut()
                //.set_rotation(Rotation::from_quaternion(geometry::QuaternionBase::new(1.0,
                //0.0,
                //0.0,
                //0.0)));
            }
        }
    }
}

impl System {
    /// Returns a handle to the body belonging to the entity with the given `Physics` component
    /// Creates the body if it does not exist yet
    fn get_or_create_handle(&mut self,
                            physics_comp: &mut world::Physics)
                            -> RigidBodyHandle<::Float> {
        let handle = match physics_comp.handle {
            BodyHandle::Id(ref id) => {
                self.body_handles
                    .get_mut(id)
                    .expect("No body with the given id")
            }
            BodyHandle::New(body) => {
                // Create the rigid body
                let rigid_body = body();
                let handle = self.world.add_rigid_body(rigid_body);

                // Assign its ID
                let (id, handle) = BodyId::new(handle);
                physics_comp.handle = BodyHandle::Id(id.clone());

                // Add the rigid body to the list of handles
                self.body_handles.insert(id.clone(), (handle, true));
                self.body_handles
                    .get_mut(&id)
                    .expect("should be unreachable")
            }
        };

        handle.1 = true;
        handle.0.clone()
    }

    /// Removes physics bodies belonging to entities that were removed
    /// For this to work, `get_or_create_handle` must be called on every entity beforehand
    fn remove_dead_bodies(&mut self) {
        let handles = mem::replace(&mut self.body_handles, HashMap::new());
        self.body_handles = handles
            .into_iter()
            .filter_map(|(id, mut handle)| {
                let visited = handle.1;

                if visited {
                    // Reset the `visited` flag
                    handle.1 = false;
                    Some((id, handle))
                } else {
                    // Was not visited, so remove the body (the entity no longer exists)
                    self.world.remove_rigid_body(&handle.0);
                    None
                }
            })
            .collect();
    }
}

/// Initializes physics-related components and systems
pub fn init<'a, 'b>(world: &mut specs::World,
                    dispatcher: DispatcherBuilder<'a, 'b>)
                    -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<world::Physics>();

    // Initialize systems
    let mut physics_world = World::new();
    physics_world.set_gravity(na::Vector3::new(0.0, 0.0, -9.81));

    let system = System {
        world: physics_world,
        body_handles: HashMap::new(),
    };

    // Add systems
    dispatcher.add_thread_local(system)
}
