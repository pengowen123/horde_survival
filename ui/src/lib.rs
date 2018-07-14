//! A UI system
//!
//! The UI is only calculated here; a draw list is sent to the graphics system to be rendered

extern crate specs;
extern crate shred;
#[macro_use]
extern crate shred_derive;
extern crate common;
#[macro_use]
extern crate conrod as conrod_macros;
extern crate window;
#[macro_use]
extern crate slog;

mod menus;
mod theme;
mod consts;

use common::conrod::{self, Ui, UiBuilder, Dimensions, render, gfx};
use common::glutin;
use common::UiState;
use window::window_event;

use std::sync::{Mutex, MutexGuard, mpsc};
use std::time::{Instant, Duration};

pub const UPS: u64 = 60;
const UPDATE_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / UPS);

/// A type that receives window events through a channel
pub type EventReceiver = mpsc::Receiver<glutin::Event>;

/// The UI represented as a list of objects to draw
pub struct UiDrawList(Option<render::OwnedPrimitives>);

impl UiDrawList {
    /// Returns a iterator-like object over the primitives in the draw list
    pub fn walk(&self) -> Option<render::WalkOwnedPrimitives> {
        self.0.as_ref().map(|p| p.walk())
    }
}

/// An image map resource for the UI
// NOTE: This is initialized by the graphics system because of the `R` type parameter
pub struct ImageMap<R: gfx::Resources>(
    Mutex<Map<R>>
);

/// The image map type used by the UI
pub type Map<R> = conrod::image::Map<(gfx::handle::ShaderResourceView<R, [f32; 4]>, (u32, u32))>;

impl<R: gfx::Resources> ImageMap<R> {
    pub fn new() -> Self {
        ImageMap(Mutex::new(Map::new()))
    }
    pub fn get(&self) -> MutexGuard<Map<R>> {
        self.0.lock().unwrap()
    }
}

pub struct System {
    ui: Ui,
    menus: menus::Menus,
    events: EventReceiver,
    // Used to limit the UPS of the UI
    last_run: Option<Instant>,
    reader_id: window_event::ReaderId,
}

impl System {
    fn new(
        window_dim: Dimensions,
        events: EventReceiver,
        log: &slog::Logger,
        reader_id: window_event::ReaderId,
    ) -> Self {
        let mut ui = UiBuilder::new(window_dim)
            .theme(theme::default_theme())
            .build();
        let menus = menus::Menus::new(ui.widget_id_generator());

        let font_path = format!("{}{}",
                                env!("CARGO_MANIFEST_DIR"),
                                "/../test_assets/fonts/NotoSans-Regular.ttf");

        ui.fonts.insert_from_file(&font_path).unwrap_or_else(|e| {
            error!(log, "Error loading font (at path `{}`): {}", font_path, e);
            panic!(common::CRASH_MSG);
        });

        Self {
            ui,
            menus,
            events,
            last_run: None,
            reader_id,
        }
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    ui_state: specs::FetchMut<'a, UiState>,
    draw_list: specs::FetchMut<'a, UiDrawList>,
    window: specs::Fetch<'a, window::Window>,
    event_channel: specs::FetchMut<'a, window_event::EventChannel>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Handle window resize events
        for event in data.event_channel.read(&mut self.reader_id) {
            if let window_event::Event::WindowResized(new_size) = *event {
                let (new_width, new_height): (u32, u32) =
                    new_size.to_physical(data.window.get_hidpi_factor()).into();

                self.ui.win_w = new_width as conrod::Scalar;
                self.ui.win_h = new_height as conrod::Scalar;
            }
        }

        // Limit UPS to the UPS constant
        if let Some(t) = self.last_run {
            if t.elapsed() < UPDATE_INTERVAL {
                return;
            }
        }
        self.last_run = Some(Instant::now());

        if let Some(primitives) = self.ui.draw_if_changed() {
            data.draw_list.0 = Some(primitives.owned());
        }

        while let Ok(event) = self.events.try_recv() {
            if let Some(event) = conrod::backend::winit::convert_event(event, data.window.window())
            {
                self.ui.handle_event(event);
            }
        }

        let rebuild_widgets =
            // Build widgets if the draw list is empty
            data.draw_list.0.is_none() ||
            // Rebuild widgets if an window event happened
            self.ui.global_input().events().next().is_some() ||
            // Rebuild widgets regardless of events if the in-game menu is active
            data.ui_state.is_in_game();

        if rebuild_widgets || true {
            let mut ui = self.ui.set_widgets();
            
            match *data.ui_state {
                UiState::MainMenu =>
                    self.menus.set_widgets_main_menu(&mut ui, &mut data.ui_state, &data.window),
                UiState::InGame =>
                    self.menus.set_widgets_in_game(&mut ui, &mut data.ui_state, &data.window),
                UiState::PauseMenu =>
                    self.menus.set_widgets_pause_menu(
                        &mut ui,
                        &mut data.ui_state,
                        &data.window,
                        &mut data.event_channel,
                    ),
                UiState::OptionsMenu =>
                    self.menus.set_widgets_options_menu(&mut ui, &mut data.ui_state, &data.window),
                UiState::Exit => {},
            }
        }
    }
}

/// Adds UI-related resources
pub fn add_resources(world: &mut specs::World) {
    world.add_resource(UiDrawList(None));
    world.add_resource(UiState::MainMenu);
}

/// Initializes UI-related systems
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
    window_dim: (u32, u32),
    events: EventReceiver,
) -> specs::DispatcherBuilder<'a, 'b> {
    let log = world.read_resource::<slog::Logger>();
    let window_dim: Dimensions = [window_dim.0.into(), window_dim.1.into()];
    let reader_id = world.write_resource::<window_event::EventChannel>().register_reader();
    let ui = System::new(window_dim, events, &log, reader_id);

    dispatcher.add(ui, "ui", &[])
}
