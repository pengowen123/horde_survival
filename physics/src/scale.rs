//! Scaling of rigid bodies

// TODO: Make sure the scale implementations are correct
use nphysics3d::object::{RigidBody, RigidBodyHandle};
use nphysics3d::volumetric::Volumetric;
use nphysics3d::math::AngularInertia;
use ncollide::shape;
use na;
use slog;
use common;

use std::sync::Arc;

/// Returns the provided rigid body scaled by the provided amount
// NOTE: This is implemented by creating a new rigid body, because rigid bodies can't be modified
pub fn scale_body(rb: &RigidBodyHandle<::Float>, scale: ::Float, log: &slog::Logger)
    -> RigidBody<::Float> {
    let rb = rb.borrow();
    let shape = rb.shape();

    let (new_shape, mass_properties) =
        try_all_shapes(&**shape, scale, rb.density()).unwrap_or_else(|| {
            error!(log, "Attempt to scale unknown shape";);
            panic!(common::CRASH_MSG);
        });

    rb.with_new_shape(new_shape, mass_properties)
}

/// Tries to cast a shape to each of the provided type, then creates a scaled version of the provided rigid
/// body
///
/// Returns `Some` from the current function if successful
///
/// If the macro invocation starts with `STATIC`, the returned mass properties will be `None`, and
/// density does not need to be supplied
///
/// # Examples
///
/// try_shapes!(DYNAMIC, rigid_body, shape, 3.0, [Cuboid3<f64>, Ball3<f64>]);
macro_rules! try_shapes {
    (DYNAMIC, $shape:expr, $scale:expr, $density:expr, [$($shape_type:path,)*]) => {{
        $(
            if let Some(s) = $shape.as_shape::<$shape_type>() {
                let new_shape = s.scale($scale)?;
                let mass_properties = $density.map(|d| new_shape.mass_properties(d));

                return Some((shape::ShapeHandle3::new(new_shape), mass_properties))
            }
        )*
    }};

    (STATIC, $shape:expr, $scale:expr, [$($shape_type:path,)*]) => {{
        $(
            if let Some(s) = $shape.as_shape::<$shape_type>() {
                let new_shape = s.scale($scale)?;

                return Some((shape::ShapeHandle3::new(new_shape), None))
            }
        )*
    }};
}

type MassProperties = (::Float, na::Point3<::Float>, AngularInertia<::Float>);

/// Tries to cast the provided shape to a each shape type and scales the result of the first
/// successful cast
///
/// Returns the scaled shape and the mass properties of the new shape, if any
fn try_all_shapes(
    shape: &shape::Shape<na::Point3<::Float>, na::Isometry3<::Float>>,
    scale: ::Float,
    density: Option<::Float>,
) -> Option<(shape::ShapeHandle3<::Float>, Option<MassProperties>)> {
    // Try all shapes that have no volume
    try_shapes!(
        STATIC,
        shape,
        scale,
        [
            shape::Plane3<::Float>,
            shape::Segment3<::Float>,
            shape::Triangle3<::Float>,
            shape::TriMesh3<::Float>,
        ]
    );

    // Try all shapes that have volume
    try_shapes!(
        DYNAMIC,
        shape,
        scale,
        density,
        [
            shape::Ball3<::Float>,
            shape::Compound3<::Float>,
            shape::Cone3<::Float>,
            shape::ConvexHull3<::Float>,
            shape::Cuboid3<::Float>,
            shape::Cylinder3<::Float>,
        ]
    );

    None
}

/// A trait implemented by all `nphysics` shape types
trait Scale: Sized {
    /// Returns `self` scaled by the provided amount
    fn scale(&self, scale: ::Float) -> Option<Self>;
}

impl Scale for shape::Ball3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.radius() * scale))
    }
}

// NOTE: This is useless because `Capsule` does not implement `Shape`
impl Scale for shape::Capsule3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

// Scale compound shapes by recursively scaling their sub-shapes
impl Scale for shape::Compound3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(
            self.shapes()
                .iter()
                .map(|&(transform, ref shape)| {
                    let new_transform = na::Isometry3::from_parts(
                        na::Translation3::from_vector(transform.translation.vector),
                        transform.rotation,
                    );
                    Some((
                        new_transform,
                        try_all_shapes(&**shape, scale, None)?.0,
                    ))
                })
                .collect::<Option<_>>()?,
        ))
    }
}

impl Scale for shape::Cone3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

impl Scale for shape::ConvexHull3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.points().iter().map(|p| p * scale).collect()))
    }
}

impl Scale for shape::Cuboid3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_extents() * scale))
    }
}

impl Scale for shape::Cylinder3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

// Planes can't be scaled
impl Scale for shape::Plane3<::Float> {
    fn scale(&self, _: ::Float) -> Option<Self> {
        Some(self.clone())
    }
}

impl Scale for shape::Polyline3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
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

        Some(Self::new(Arc::new(new_vertices), indices, None, None))
    }
}

impl Scale for shape::Segment3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.a() * scale, self.b() * scale))
    }
}

impl Scale for shape::Triangle3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.a() * scale, self.b() * scale, self.c() * scale))
    }
}

impl Scale for shape::TriMesh3<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
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

        Some(Self::new(Arc::new(new_vertices), indices, None, None))
    }
}
