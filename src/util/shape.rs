//! 2D shapes: circles and convex polygons

use bevy::math::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub enum Shape {
    Circle(CircleShape),
    Poly(PolyShape),
}

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
