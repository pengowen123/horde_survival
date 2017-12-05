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

/// Returns a cube that represents a skybox
///
/// Requires a function that creates a vertex given its position.
pub fn create_skybox_cube<F, V>(f: F) -> [V; 36]
where
    F: Fn([f32; 3]) -> V,
{
    [
        f([-1.0, 1.0, -1.0]),
        f([-1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, 1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),

        f([-1.0, -1.0, 1.0]),
        f([-1.0, -1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),
        f([-1.0, 1.0, -1.0]),
        f([-1.0, 1.0, 1.0]),
        f([-1.0, -1.0, 1.0]),

        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, -1.0]),
        f([1.0, -1.0, -1.0]),

        f([-1.0, -1.0, 1.0]),
        f([-1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, -1.0, 1.0]),
        f([-1.0, -1.0, 1.0]),

        f([-1.0, 1.0, -1.0]),
        f([1.0, 1.0, -1.0]),
        f([1.0, 1.0, 1.0]),
        f([1.0, 1.0, 1.0]),
        f([-1.0, 1.0, 1.0]),
        f([-1.0, 1.0, -1.0]),

        f([-1.0, -1.0, -1.0]),
        f([-1.0, -1.0, 1.0]),
        f([1.0, -1.0, -1.0]),
        f([1.0, -1.0, -1.0]),
        f([-1.0, -1.0, 1.0]),
        f([1.0, -1.0, 1.0]),
    ]
}

/// Clears all render targets if `COLOR` is the first argument. Clears all depth targets if `DEPTH`
/// is the first argument. This macro can only be used in a method on `draw::System`.
///
/// # Examples
///
/// This will use `self.encoder` to clear the depth of `data.depth`, and `other_data.depth`:
/// ```rust
/// clear_targets!(DEPTH, self, data.depth, other_data.depth);
/// ```
macro_rules! clear_targets {
    (COLOR, $self_:ident, $($target:expr),*,) => {
        $(
            $self_.encoder.clear(&$target, CLEAR_COLOR);
        )*
    };
    (DEPTH, $self_:ident, $($target:expr),*,) => {
        $(
            $self_.encoder.clear_depth(&$target, 1.0);
        )*
    }
}
