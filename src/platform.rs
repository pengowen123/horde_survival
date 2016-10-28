#[cfg(target_os="windows")]
pub mod misc {
    pub const NEWLINE: &'static str = "\r\n";
}

#[cfg(not(target_os="windows"))]
pub mod misc {
    pub const NEWLINE: &'static str = "\n";
}
