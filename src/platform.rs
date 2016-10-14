#[cfg(target_os="windows")]
pub mod shaders {
    pub const VERTEX_SHADER_2D: &'static [u8] = include_bytes!("./include/150/2d/vertex.glsl");
    pub const FRAGMENT_SHADER_2D: &'static [u8] = include_bytes!("./include/150/2d/fragment.glsl");
    pub const VERTEX_SHADER_3D: &'static [u8] = include_bytes!("./include/150/3d/vertex.glsl");
    pub const FRAGMENT_SHADER_3D: &'static [u8] = include_bytes!("./include/150/3d/fragment.glsl");
}

#[cfg(target_os="linux")]
pub mod shaders {
    pub const VERTEX_SHADER_2D: &'static [u8] = include_bytes!("./include/120/2d/vertex.glsl");
    pub const FRAGMENT_SHADER_2D: &'static [u8] = include_bytes!("./include/120/2d/fragment.glsl");
    pub const VERTEX_SHADER_3D: &'static [u8] = include_bytes!("./include/120/3d/vertex.glsl");
    pub const FRAGMENT_SHADER_3D: &'static [u8] = include_bytes!("./include/120/3d/fragment.glsl");
}

#[cfg(target_os="windows")]
pub mod misc {
    pub const NEWLINE: &'static str = "\r\n";
}

#[cfg(target_os="linux")]
pub mod misc {
    pub const NEWLINE: &'static str = "\n";
}
