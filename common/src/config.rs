//! Configuration of Horde Survival

use gfx::texture;
use glutin;

use std::fmt;
use std::path::PathBuf;

pub const DEFAULT_SENSITIVITY: ::Float = 0.0035;

// A type that holds all command-line configuration options
/// A first-person, wave based game
#[derive(StructOpt, Debug)]
#[structopt(name = "horde_survival")]
pub struct CommandLineConfig {
    /// The path to the folder containing game assets
    #[structopt(long = "assets_path", parse(from_os_str))]
    assets_path: Option<PathBuf>,
}

/// A type that holds all configuration options that can be customized in the configuration file
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub graphics: GraphicsConfig,
    pub window: WindowConfig,
    pub camera: CameraConfig,
    pub bindings: BindConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub postprocessing: bool,
    pub shadows: bool,
    pub shadow_map_size: texture::Size,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraConfig {
    pub sensitivity: ::Float,
    pub fov: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindConfig {
    pub move_forward: Bind,
    pub move_backward: Bind,
    pub move_left: Bind,
    pub move_right: Bind,
    pub reload_shaders: Bind,
}

#[derive(Clone)]
pub enum BindName {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

impl BindConfig {
    pub fn set(&mut self, name: BindName, value: Bind) {
        *self.get_mut(name) = value;
    }

    pub fn get_mut(&mut self, name: BindName) -> &mut Bind {
        match name {
            BindName::MoveForward => &mut self.move_forward,
            BindName::MoveLeft => &mut self.move_left,
            BindName::MoveRight => &mut self.move_right,
            BindName::MoveBackward => &mut self.move_backward,
        }
    }

    /// Returns whether the provided `Bind` is already assigned to an action
    pub fn is_in_use(&self, bind: &Bind) -> bool {
        // NOTE: If new binds are added, add them here as well
        self.move_forward == *bind ||
            self.move_left == *bind ||
            self.move_right == *bind ||
            self.move_backward == *bind
    }
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            postprocessing: false,
            shadows: false,
            shadow_map_size: 1024,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
            vsync: false,
        }
    }
}
impl Default for BindConfig {
    fn default() -> Self {
        Self {
            move_forward: Bind {
                modifiers: Default::default(),
                key: Key::W,
            },
            move_backward: Bind {
                modifiers: Default::default(),
                key: Key::S,
            },
            move_left: Bind {
                modifiers: Default::default(),
                key: Key::A,
            },
            move_right: Bind {
                modifiers: Default::default(),
                key: Key::D,
            },
            reload_shaders: Bind {
                modifiers: Default::default(),
                key: Key::F1,
            },
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            sensitivity: DEFAULT_SENSITIVITY,
            fov: 45.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifiersState {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl Default for ModifiersState {
    fn default() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
        }
    }
}

impl From<glutin::ModifiersState> for ModifiersState {
    fn from(mods: glutin::ModifiersState) -> Self {
        Self {
            ctrl: mods.ctrl,
            shift: mods.shift,
            alt: mods.alt,
        }
    }
}

impl Into<glutin::ModifiersState> for ModifiersState {
    fn into(self) -> glutin::ModifiersState {
        glutin::ModifiersState {
            ctrl: self.ctrl,
            shift: self.shift,
            alt: self.alt,
            logo: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bind {
    pub key: Key,
    pub modifiers: ModifiersState,
}

impl Bind {
    pub fn new(key: Key, modifiers: ModifiersState) -> Self {
        Self {
            key,
            modifiers,
        }
    }
}

impl fmt::Display for Bind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        const CTRL_PREFIX: &str = "Ctrl+";
        const SHIFT_PREFIX: &str = "Shift+";
        const ALT_PREFIX: &str = "Alt+";

        let key_string = format!("{:?}", self.key);

        let mut string = String::with_capacity(
            key_string.len() +
            (CTRL_PREFIX.len() * self.modifiers.ctrl as usize) +
            (SHIFT_PREFIX.len() * self.modifiers.ctrl as usize) +
            (ALT_PREFIX.len() * self.modifiers.ctrl as usize)
        );

        if self.modifiers.ctrl {
            string.push_str("Ctrl+");
        }

        if self.modifiers.shift {
            string.push_str("Shift+");
        }

        if self.modifiers.alt {
            string.push_str("Alt+");
        }

        string.push_str(&key_string);

        writeln!(fmt, "{}", string)
    }
}

macro_rules! make_key_struct {
    ($($key:ident),*,) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Key {
            $(
                $key,
            )*
        }

        impl From<glutin::VirtualKeyCode> for Key {
            fn from(key: glutin::VirtualKeyCode) -> Self {
                match key {
                    $(
                        glutin::VirtualKeyCode::$key => Key::$key,
                    )*
                }
            }
        }

        impl Into<glutin::VirtualKeyCode> for Key {
            fn into(self) -> glutin::VirtualKeyCode {
                match self {
                    $(
                        Key::$key => glutin::VirtualKeyCode::$key,
                    )*
                }
            }
        }
    }
}

make_key_struct! {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}
