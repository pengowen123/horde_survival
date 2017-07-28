//! Utilities for graphics

use super::pipeline::postprocessing::Vertex;

/// Returns a quad that fills the entire screen
pub fn create_screen_quad() -> [Vertex; 6] {
    [
        Vertex::new([-1.0, -1.0], [0.0, 0.0]),
        Vertex::new([1.0, -1.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0], [1.0, 1.0]),
        Vertex::new([-1.0, -1.0], [0.0, 0.0]),
        Vertex::new([1.0, 1.0], [1.0, 1.0]),
        Vertex::new([-1.0, 1.0], [0.0, 1.0]),
    ]
}
