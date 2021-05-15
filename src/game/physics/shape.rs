//! 2D shapes: circles and convex polygons
use std::f32::consts::PI;

use bevy::math::Vec2;
use bevy::transform::components::Transform;

use crate::util::{iter, Vec2Ext};

use super::util::{segments_intersection, update_max_point, update_min_point};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub enum Shape {
    Circle(CircleShape),
    Poly(PolyShape),
}

// TODO probably should remove this Default, it's only here for the inspector
impl Default for Shape {
    fn default() -> Self {
        Self::Poly(Default::default())
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct CircleShape {
    pub radius: f32,
    pub center: Vec2,
    /// Current normal, used internally
    normal: Vec2,
}

impl CircleShape {
    pub fn new(radius: f32, center: Vec2) -> Self {
        Self {
            radius,
            center,
            normal: Vec2::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
/// Points and their edges are ordered counter-clockwise.
pub struct PolyShape {
    points: Vec<Vec2>,
    edges: Vec<Vec2>,
    normals: Vec<Vec2>,
}

impl PolyShape {
    /// Create a poly shape while checking if it's a convex polygon.
    pub fn new(points: Vec<Vec2>) -> Self {
        assert!(!points.is_empty());
        let mut poly = PolyShape {
            edges: vec![Vec2::default(); points.len()],
            normals: vec![Vec2::default(); points.len()],
            points,
        };

        poly.update_edges();

        // Check if it's a convex polygon
        if !poly.is_convex() {
            // Try to reverse the points
            poly.points.reverse();
            poly.update_edges();
            assert!(poly.is_convex(), "Not a convex polygon");
        }

        poly
    }

    /// Must update edges before calling
    fn is_convex(&self) -> bool {
        let angle_sum: f32 = iter::pairs(self.edges.iter())
            .map(|(edge1, edge2)| {
                let mut angle = Vec2::angle_between(*edge1, *edge2);
                if angle < 0.0 {
                    angle += 2.0 * PI;
                }
                angle
            })
            .sum();
        const ANGLE_EPS: f32 = 1e-6;
        (angle_sum - 2.0 * PI).abs() < ANGLE_EPS
    }

    fn update_edges(&mut self) {
        for ((&start, &end), edge) in
            iter::pairs(self.points.iter()).zip(self.edges.iter_mut())
        {
            *edge = end - start;
        }
    }

    fn update_normals(&mut self) {
        for (&edge, normal) in self.edges.iter().zip(self.normals.iter_mut()) {
            *normal = edge.rotate_clockwise_90().normalize();
        }
    }
}

/// Shape that can shift. Current shape is always `transform` * `original_shape`,
/// where `transform` is the last transform given in `update()`.
///
/// Only this struct is used by the collision system in [Collider], not [Shape].
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub(super) struct ShiftedShape {
    original_shape: Shape,
    shape: Shape,
}

impl ShiftedShape {
    pub fn from_original_shape(original_shape: Shape) -> Self {
        Self {
            shape: original_shape.clone(),
            original_shape,
        }
    }

    /// Updates the shape with `transform`. Returns min and max points of the new shape.
    pub fn update(&mut self, transform: &Transform) -> (Vec2, Vec2) {
        // Bottom left of the bounding box
        let mut min_point: Vec2 = Vec2::new(f32::INFINITY, f32::INFINITY);
        // Top right of the bounding box
        let mut max_point: Vec2 =
            Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        match (&self.original_shape, &mut self.shape) {
            (
                Shape::Circle(original_circle_shape),
                Shape::Circle(circle_shape),
            ) => {
                assert!(
                    transform.scale.x == transform.scale.y,
                    "Scaling a circle into\\
                    an ellipse is not implemented yet"
                );
                circle_shape.radius =
                    original_circle_shape.radius * transform.scale.x;
                circle_shape.center =
                    original_circle_shape.center.apply_transform(transform);
                update_min_point(
                    &mut min_point,
                    circle_shape.center - Vec2::splat(circle_shape.radius),
                );
                update_max_point(
                    &mut max_point,
                    circle_shape.center + Vec2::splat(circle_shape.radius),
                );
            }
            (Shape::Poly(original_poly_shape), Shape::Poly(poly_shape)) => {
                assert!(
                    original_poly_shape.points.len() != 0
                        && original_poly_shape.points.len()
                            == poly_shape.points.len()
                );
                for (original_point, point) in original_poly_shape
                    .points
                    .iter()
                    .zip(poly_shape.points.iter_mut())
                {
                    *point = original_point.apply_transform(transform);
                    update_min_point(&mut min_point, *point);
                    update_max_point(&mut max_point, *point);
                }

                // Update the edges and normals of the poly shape
                poly_shape.update_edges();
                poly_shape.update_normals();
            }

            _ => unreachable!(),
        }
        (min_point, max_point)
    }

    /// Puts all normalized projection normals for this [Shape]
    /// with respect to `other_shape`
    /// into `normals_buf`.
    /// Clears `normals_buf` beforehand
    fn get_normals(
        &self,
        other_shape: &ShiftedShape,
        normals_buf: &mut Vec<Vec2>,
    ) {
        normals_buf.clear();
        match &self.shape {
            Shape::Circle(self_circle) => match &other_shape.shape {
                Shape::Circle(other_circle) => {
                    let normal =
                        (other_circle.center - self_circle.center).normalize();
                    normals_buf.push(normal);
                }
                Shape::Poly(other_poly) => {
                    for &point in &other_poly.points {
                        normals_buf
                            .push((point - self_circle.center).normalize());
                    }
                }
            },
            Shape::Poly(self_poly) => {
                normals_buf.extend_from_slice(&self_poly.normals)
            }
        }
    }

    /// Return projection of the shape onto the normalized `normal` as a segment.
    fn project(&self, normal: Vec2) -> (f32, f32) {
        assert!(
            normal.is_normalized(),
            "Not normalized normal = {:?}",
            normal
        );
        match &self.shape {
            Shape::Circle(circle) => {
                let center_projected = circle.center.dot(normal);
                (
                    center_projected - circle.radius,
                    center_projected + circle.radius,
                )
            }
            Shape::Poly(poly) => {
                let mut min = std::f32::INFINITY;
                let mut max = std::f32::NEG_INFINITY;
                for &point in &poly.points {
                    let proj = point.dot(normal);
                    if proj < min {
                        min = proj;
                    }
                    if proj > max {
                        max = proj
                    }
                }
                (min, max)
            }
        }
    }

    #[inline]
    fn update_mpv(cur_mpv: &mut Vec2, mpv_dir: Vec2, mpv_len: f32) {
        if mpv_len * mpv_len < cur_mpv.length_squared() {
            *cur_mpv = mpv_dir * mpv_len;
        }
    }

    /// Returns `Some(mpv) if two shapes are colliding and `None` otherwise,
    /// where `mpv` is the Minimum Push Vector to push `other` out of `self`
    ///
    /// TODO: maybe optimize the Circle to Circle case
    pub(super) fn process_collision(
        &self,
        other: &ShiftedShape,
        normal_buf: &mut Vec<Vec2>,
    ) -> Option<Vec2> {
        let mut mpv = Vec2::new(std::f32::INFINITY, std::f32::INFINITY);

        // Iterate over collision normals
        let (shape1, shape2) = (self, other);
        shape1.get_normals(shape2, normal_buf);
        for normal in normal_buf.iter() {
            // Project both shapes onto the normal
            let seg1 = shape1.project(*normal);
            let seg2 = shape2.project(*normal);
            // Determine if `seg1` and `seg2` intersect
            let intersection = segments_intersection(seg1, seg2);
            if let Some(intersection_len) = intersection {
                // Update the mpv
                Self::update_mpv(&mut mpv, *normal, intersection_len);
            } else {
                // No intersection on a projection means these shapes are not colliding
                return None;
            }
        }

        // Do the same for the normals of the second shape
        // Iterate over collision normals
        shape2.get_normals(shape1, normal_buf);
        for normal in normal_buf.iter() {
            // Project both shapes onto the normal
            let seg1 = shape1.project(*normal);
            let seg2 = shape2.project(*normal);
            // Determine if `seg1` and `seg2` intersect
            let intersection = segments_intersection(seg1, seg2);
            if let Some(intersection_len) = intersection {
                // Note that we invert the direction of the normal
                Self::update_mpv(&mut mpv, -*normal, intersection_len);
            } else {
                // No intersection on a projection means these shapes are not colliding
                return None;
            }
        }

        // If we got here, that means there is no seperating axis,
        // so the shapes are collidding
        Some(mpv)
    }
}
