//! A system to apply configuration changes to the window

use common::{config, glutin, specs};

use window_event;

pub struct System {
    reader_id: window_event::ReaderId,
}

impl System {
    pub fn new(reader_id: window_event::ReaderId) -> Self {
        Self { reader_id }
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    window: specs::ReadExpect<'a, ::Window>,
    config: specs::ReadExpect<'a, config::Config>,
    event_channel: specs::ReadExpect<'a, window_event::EventChannel>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for e in data.event_channel.read(&mut self.reader_id) {
            match e {
                window_event::Event::ConfigChanged(window_event::ChangedConfig::Window) => {
                    let config = &data.config.window;
                    let new_window_size =
                        glutin::dpi::LogicalSize::new(config.width.into(), config.height.into());
                    data.window.set_min_dimensions(Some(new_window_size));
                    data.window.set_inner_size(new_window_size);
                    let fullscreen = if config.fullscreen {
                        Some(data.window.get_current_monitor())
                    } else {
                        None
                    };
                    data.window.set_fullscreen(fullscreen);
                }
                _ => {}
            }
        }
    }
}
