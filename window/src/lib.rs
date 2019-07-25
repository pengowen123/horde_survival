//! Systems and components to abstract the use of the game window and its events

extern crate common;
#[macro_use]
extern crate shred_derive;
use common::specs;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate slog;

pub mod config;
pub mod info;
pub mod input;
pub mod window_event;

use common::{shred, glutin, Float};
use common::takeable_option::Takeable;

use std::ops::Deref;

/// The window type
pub struct Window {
    inner: Takeable<glutin::WindowedContext<glutin::NotCurrent>>,
}

impl Window {
    pub fn new(wrapper: glutin::WindowedContext<glutin::NotCurrent>) -> Self {
        Self {
            inner: Takeable::new(wrapper),
        }
    }

    pub fn get_window(&self) -> &glutin::Window {
        self.inner.window()
    }

    /// Tries to make the stored context current and returns a reference to it
    ///
    /// `ContextWrapperRef::make_not_current` or `ContextWrapperRef::treat_as_not_current` must be
    /// called on the returned reference after it is done being used.
    pub unsafe fn get_current_context_wrapper(
        &mut self,
    ) -> Result<ContextWrapperRef, glutin::ContextError> {
        ContextWrapperRef::new(&mut self.inner)
    }
}

/// A reference to a `ContextWrapper`
pub struct ContextWrapperRef<'a> {
    wrapper: &'a mut Takeable<glutin::WindowedContext<glutin::NotCurrent>>,
    current_wrapper: glutin::WindowedContext<glutin::PossiblyCurrent>,
    /// Whether `ContextWrapper::made_current` was called when this reference was created
    made_current: bool,
}

impl<'a> ContextWrapperRef<'a> {
    // Unsafe because it tries to make a context the current one, which requires FFI
    unsafe fn new(
        wrapper: &'a mut Takeable<glutin::WindowedContext<glutin::NotCurrent>>,
    ) -> Result<Self, glutin::ContextError> {
        let (current_wrapper, made_current) = {
            if wrapper.is_current() {
                (Takeable::take(wrapper).treat_as_current(), false)
            } else {
                (
                    Takeable::take(wrapper).make_current().map_err(|e| e.1)?,
                    true
                )
            }
        };

        Ok(Self {
            wrapper,
            current_wrapper,
            made_current,
        })
    }
}

impl<'a> Deref for ContextWrapperRef<'a> {
    type Target = glutin::WindowedContext<glutin::PossiblyCurrent>;

    fn deref(&self) -> &Self::Target {
        &self.current_wrapper
    }
}

impl<'a> ContextWrapperRef<'a> {
    /// Makes the current context not current if there was a previously current context
    pub unsafe fn make_not_current(self) -> Result<(), glutin::ContextError> {
        if self.made_current {
            Takeable::insert(
                self.wrapper,
                self.current_wrapper.make_not_current().map_err(|e| e.1)?);
        } else {
            return Ok(self.treat_as_not_current())
        }

        Ok(())
    }

    /// Treats the current context as not current
    pub unsafe fn treat_as_not_current(self) {
        Takeable::insert(self.wrapper, self.current_wrapper.treat_as_not_current());
    }
}

/// Registers all components and systems in this crate
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
) -> specs::DispatcherBuilder<'a, 'b> {
    world.add_resource(info::WindowInfo::default());
    world.add_resource(window_event::EventChannel::new());

    let mut event_channel = world.write_resource::<window_event::EventChannel>();
    let reader_id = event_channel.register_reader();

    let config_system = config::System::new(reader_id);
    // NOTE: These systems will be added to the graphics dispatcher, if other systems are added here
    //       in the future the main dispatcher must be added as an argument to this function
    dispatcher
        .with(info::System, "window-info", &[])
        .with(config_system, "window-config", &[])
}
