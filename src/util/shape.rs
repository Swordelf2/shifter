//! 2D shapes: circles and convex polygons

use bevy::math::Vec2;

#[derive(Debug)]
pub enum Shape {
    Circle(CircleShape),
    Poly(PolyShape),
}

#[derive(Debug, Default)]
pub struct CircleShape {
    pub radius: f32,
    pub center: Vec2,
}

// TODO check if it's a convex polygon
#[derive(Debug, Default)]
pub struct PolyShape {
    pub points: Vec<Vec2>,
}
