#[cfg(target_os="windows")]
pub mod shaders {
    pub const VS_2D_PATH: &'static str = "test_assets/shaders/150/2d/vertex.glsl";
    pub const FS_2D_PATH: &'static str = "test_assets/shaders/150/2d/fragment.glsl";
    pub const VS_3D_PATH: &'static str = "test_assets/shaders/150/3d/vertex.glsl";
    pub const FS_3D_PATH: &'static str = "test_assets/shaders/150/3d/fragment.glsl";
    pub const VS_GUI_PATH: &'static str = "test_assets/shaders/150/gui/vertex.glsl";
    pub const FS_GUI_PATH: &'static str = "test_assets/shaders/150/gui/fragment.glsl";
}

#[cfg(target_os="linux")]
pub mod shaders {
    pub const VS_2D_PATH: &'static str = "test_assets/shaders/120/2d/vertex.glsl";
    pub const FS_2D_PATH: &'static str = "test_assets/shaders/120/2d/fragment.glsl";
    pub const VS_3D_PATH: &'static str = "test_assets/shaders/120/3d/vertex.glsl";
    pub const FS_3D_PATH: &'static str = "test_assets/shaders/120/3d/fragment.glsl";
    pub const VS_GUI_PATH: &'static str = "test_assets/shaders/120/gui/vertex.glsl";
    pub const FS_GUI_PATH: &'static str = "test_assets/shaders/120/gui/fragment.glsl";
}

#[cfg(target_os="windows")]
pub mod misc {
    pub const NEWLINE: &'static str = "\r\n";
}

#[cfg(target_os="linux")]
pub mod misc {
    pub const NEWLINE: &'static str = "\n";
}
