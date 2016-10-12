// Windows
#[cfg(target_os="windows")]
pub const VERTEX_SHADER: &'static [u8] = include_bytes!("./include/vertex.glsl");
#[cfg(target_os="windows")]
pub const FRAGMENT_SHADER: &'static [u8] = include_bytes!("./include/fragment.glsl");

#[cfg(target_os="windows")]
pub const NEWLINE: &'static str = "\r\n";

// Linux
// FIXME: `out` is not available in GLSL 1.1, but using gl_FragColor leads to gfx not registering
//         the output of the fragment shader
#[cfg(target_os="linux")]
pub const VERTEX_SHADER: &'static [u8] = include_bytes!("./include/vertex_110.glsl");
#[cfg(target_os="linux")]
pub const FRAGMENT_SHADER: &'static [u8] = include_bytes!("./include/fragment_110.glsl");

#[cfg(target_os="linux")]
pub const NEWLINE: &'static str = "\n";
