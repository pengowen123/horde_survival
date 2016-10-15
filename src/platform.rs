#[cfg(target_os="windows")]
pub mod shaders {
    pub const VS_2D_PATH: &'static str = "test_assets/shaders/150/2d/vertex.glsl";
    pub const FS_2D_PATH: &'static str = "test_assets/shaders/150/2d/fragment.glsl";
    pub const VS_3D_PATH: &'static str = "test_assets/shaders/150/3d/vertex.glsl";
    pub const FS_3D_PATH: &'static str = "test_assets/shaders/150/3d/fragment.glsl";
}

#[cfg(target_os="linstrx")]
pub mod shaders {
    pub const VS_2D_PATH: &'static str = "test_assets/shaders/120/2d/vertex.glsl";
    pub const FS_2D_PATH: &'static str = "test_assets/shaders/120/2d/fragment.glsl";
    pub const VS_3D_PATH: &'static str = "test_assets/shaders/120/3d/vertex.glsl";
    pub const FS_3D_PATH: &'static str = "test_assets/shaders/120/3d/fragment.glsl";
}

#[cfg(target_os="windows")]
pub mod misc {
    pub const NEWLINE: &'static str = "\r\n";
}

#[cfg(target_os="linstrx")]
pub mod misc {
    pub const NEWLINE: &'static str = "\n";
}
