//! Controls system to let players control their entity

use specs::{self, Join, DispatcherBuilder};

use std::sync::mpsc;

use {world, player};
use math::direction;
use event;

/// An event sent by a player, for example when a player presses a key an event will be generated
pub enum Event {
    /// A player rotated the camera (the direction will be added to the player entity's direction)
    RotateCamera(direction::Direction),
}

pub type EventReceiver = mpsc::Receiver<Event>;

pub struct System {
    input: EventReceiver,
    rotate_direction: Option<direction::Direction>,
}

impl System {
    pub fn new(input: EventReceiver) -> Self {
        Self {
            input: input,
            rotate_direction: None,
        }
    }

    fn check_input(&mut self) {
        while let Ok(e) = self.input.try_recv() {
            match e {
                Event::RotateCamera(direction) => {
                    self.rotate_direction = Some(direction);
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    _player: specs::ReadStorage<'a, player::Player>,
    direction: specs::WriteStorage<'a, world::Direction>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        self.check_input();

        for direction in (&mut data.direction).join() {
            if let Some(rotation) = self.rotate_direction {
                direction.0 += rotation;
            }
        }
    }
}

/// Initializes controls-related components and systems
pub fn init<'a, 'b>(dispatcher: DispatcherBuilder<'a, 'b>)
                    -> (DispatcherBuilder<'a, 'b>, event::SenderHub) {

    let (snd, recv) = event::SenderHub::new();
    let control = System::new(recv.into_receiver());
    let dispatcher = dispatcher.add(control, "control", &[]);

    (dispatcher, snd)
}
