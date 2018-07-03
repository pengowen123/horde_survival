//! Utilities for graphics

/// Returns a quad that fills the entire screen
///
/// Requires a function that creates a vertex given its position and UV coordinates.
pub fn create_screen_quad<F, V>(f: F) -> [V; 6]
where
    F: Fn([f32; 2], [f32; 2]) -> V,
{
    [
        f([-1.0, -1.0], [0.0, 0.0]),
        f([1.0, -1.0], [1.0, 0.0]),
        f([1.0, 1.0], [1.0, 1.0]),
        f([-1.0, -1.0], [0.0, 0.0]),
        f([1.0, 1.0], [1.0, 1.0]),
        f([-1.0, 1.0], [0.0, 1.0]),
    ]
}
