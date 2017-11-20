//! Scaling of rigid bodies

use nphysics3d::object::{RigidBody, RigidBodyHandle};
use ncollide::shape;
use na;

use std::sync::Arc;

/// Returns the provided rigid body scaled by the provided amount
// NOTE: This is implemented by creating a new rigid body, because rigid bodies can't be modified
pub fn scale_body(rb: &RigidBodyHandle<::Float>, scale: ::Float) -> RigidBody<::Float> {
    let rb = rb.borrow();
    let shape = rb.shape();

    try_all_shapes(&*rb, &**shape, scale).expect("Unknown shape")
}

/// Tries to cast a shape to each of the provided type, then creates a scaled version of the provided rigid
/// body
///
/// Returns `Ok` from the current function if successful
///
/// # Examples
///
/// try_shapes!(rigid_body, shape, 3.0, [Cuboid3<f64>, Ball3<f64>]);
macro_rules! try_shapes {
    ($rigid_body:expr, $shape:expr, $scale:expr, [$($shape_type:path,)*]) => {{
        $(
            if let Some(s) = $shape.as_shape::<$shape_type>() {
                let new_shape = s.scale($scale);

                if $rigid_body.can_move() {
                    return Ok($rigid_body.with_new_shape_dynamic(new_shape));
                } else {
                    return Ok($rigid_body.with_new_shape_static(new_shape));
                }
            }
        )*
    }}

}

/// Tries to cast the provided shape to a each shape type and scales the result of the first
/// successful cast
fn try_all_shapes(
    rb: &RigidBody<::Float>,
    shape: &shape::Shape<na::Point3<::Float>, na::Isometry3<::Float>>,
    scale: ::Float,
) -> Result<RigidBody<::Float>, ()> {
    // TriMesh can't be used as a dynamic body so it is special-cased
    if let Some(s) = shape.as_shape::<shape::TriMesh3<::Float>>() {
        let new_shape = s.scale(scale);

        return Ok(rb.with_new_shape_static(new_shape));
    }

    try_shapes!(
        rb,
        shape,
        scale,
        [
            shape::Cuboid3<::Float>,
            // TODO: Add the rest of the shapes here and implement Scale for them
        ]
    );

    Err(())
}

/// A trait implemented by all `nphysics` shape types
trait Scale {
    /// Returns `self` scaled by the provided amount
    fn scale(&self, scale: ::Float) -> Self;
}

impl Scale for shape::TriMesh3<::Float> {
    fn scale(&self, scale: ::Float) -> Self {
        // Vertices are copied because they must be modified
        let old_vertices = (&**self.vertices()).iter().cloned();
        let new_vertices: Vec<_> = old_vertices
            .map(|v| {
                let new = v * scale;
                new
            })
            .collect();
        // Indices are shared between original and scaled mesh
        let indices: Arc<Vec<_>> = self.indices().clone();

        Self::new(Arc::new(new_vertices), indices, None, None)
    }
}

impl Scale for shape::Cuboid3<::Float> {
    fn scale(&self, scale: ::Float) -> Self {
        Self::new(self.half_extents() * scale)
    }
}
