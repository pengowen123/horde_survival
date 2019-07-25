//! Scaling of `ncollide3d` shapes

// TODO: Make sure the scale implementations are correct
use na;
use ncollide3d::shape::{self, ShapeHandle};

/// Returns a shape that is `shape` scaled by a factor of `scale_factor`
#[inline]
pub fn scale_shape(
    shape: &ShapeHandle<::Float>,
    scale_factor: ::Float,
) -> Option<ShapeHandle<::Float>> {
    try_all_shapes(&**shape, scale_factor)
}

/// Tries to cast a shape to each of the provided types, then creates a scaled version it
///
/// Returns `Some` from the current function if successful
///
/// # Examples
///
/// try_shapes!(shape, 3.0, [Cuboid<f64>, Ball<f64>]);
macro_rules! try_shapes {
    ($shape:expr, $scale:expr, [$($shape_type:path,)*]) => {{
        $(
            if let Some(s) = $shape.as_shape::<$shape_type>() {
                let new_shape = s.scale($scale)?;

                return Some(shape::ShapeHandle::new(new_shape))
            }
        )*
    }};
}

/// Tries to cast the provided shape to a each shape type and scales the result of the first
/// successful cast
///
/// Returns the scaled shape and the mass properties of the new shape, if any
fn try_all_shapes(
    shape: &shape::Shape<::Float>,
    scale: ::Float,
) -> Option<shape::ShapeHandle<::Float>> {
    try_shapes!(
        shape,
        scale,
        [
            shape::Plane<::Float>,
            shape::Segment<::Float>,
            shape::Triangle<::Float>,
            shape::TriMesh<::Float>,
            shape::Ball<::Float>,
            shape::Compound<::Float>,
            shape::ConvexHull<::Float>,
            shape::Cuboid<::Float>,
        ]
    );

    None
}

/// A trait implemented by all `nphysics` shape types
pub trait Scale: Sized {
    /// Returns `self` scaled by the provided amount
    fn scale(&self, scale: ::Float) -> Option<Self>;
}

impl Scale for shape::Ball<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.radius() * scale))
    }
}

// NOTE: Some of these `impl`s are useless because the type they are implementing for do not
//       implement `Shape`
impl Scale for shape::Capsule<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

// Scale compound shapes by recursively scaling their sub-shapes
impl Scale for shape::Compound<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(
            self.shapes()
                .iter()
                .map(|&(transform, ref shape)| {
                    let new_transform = na::Isometry3::from_parts(
                        na::Translation3::from(transform.translation.vector),
                        transform.rotation,
                    );
                    Some((new_transform, try_all_shapes(&**shape, scale)?))
                }).collect::<Option<_>>()?,
        ))
    }
}

impl Scale for shape::Cone<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

impl Scale for shape::ConvexHull<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        let points: Vec<_> = self.points().iter().map(|p| p * scale).collect();

        Some(Self::try_from_points(&points).unwrap())
    }
}

impl Scale for shape::Cuboid<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_extents() * scale))
    }
}

impl Scale for shape::Cylinder<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.half_height() * scale, self.radius() * scale))
    }
}

// Planes can't be scaled
impl Scale for shape::Plane<::Float> {
    fn scale(&self, _: ::Float) -> Option<Self> {
        Some(self.clone())
    }
}

impl Scale for shape::Polyline<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        let mut new = self.clone();

        new.scale_by(&na::Vector3::new(scale, scale, scale));

        Some(new)
    }
}

impl Scale for shape::Segment<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(self.a() * scale, self.b() * scale))
    }
}

impl Scale for shape::Triangle<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        Some(Self::new(
            self.a() * scale,
            self.b() * scale,
            self.c() * scale,
        ))
    }
}

impl Scale for shape::TriMesh<::Float> {
    fn scale(&self, scale: ::Float) -> Option<Self> {
        // NOTE: This removes the use of indices from the previous mesh for now until ncollide#299
        //       is fixed
        let new_points = self.points()
            .iter()
            .map(|point| point * scale)
            .collect::<Vec<_>>();

        let mut new_indices = Vec::new();
        let mut i = 0;

        while i < new_points.len() - 1 {
            new_indices.push(na::Point3::new(i, i + 1, i + 2));
            i += 3;
        }

        let uvs = self.uvs().map(ToOwned::to_owned);

        Some(Self::new(
            new_points,
            new_indices,
            uvs,
        ))
    }
}
