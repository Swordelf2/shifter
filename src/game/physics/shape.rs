//! 2D shapes: circles and convex polygons
use std::borrow::Borrow;

use bevy::math::Vec2;
use bevy::transform::components::Transform;

use crate::util::Vec2Ext;

use super::util::{update_max_point, update_min_point};

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
}

// TODO check if it's a convex polygon
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct PolyShape {
    pub points: Vec<Vec2>,
}

/// Shape that can shift. Current shape is always `transform` * `original_shape`,
/// where `transform` is the last transform given in `update()`.
#[derive(Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct ShiftedShape {
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
            }

            _ => unreachable!(),
        }
        (min_point, max_point)
    }
}

impl Borrow<Shape> for ShiftedShape {
    fn borrow(&self) -> &Shape {
        &self.shape
    }
}
