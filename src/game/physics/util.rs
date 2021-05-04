use bevy::math::Vec2;

#[inline]
pub fn update_min_point(min_point: &mut Vec2, point: Vec2) {
    if point.x < min_point.x {
        min_point.x = point.x;
    }
    if point.y < min_point.y {
        min_point.y = point.y;
    }
}

#[inline]
pub fn update_max_point(max_point: &mut Vec2, point: Vec2) {
    if point.x > max_point.x {
        max_point.x = point.x;
    }
    if point.y > max_point.y {
        max_point.y = point.y;
    }
}

/// Test whether segments [a1, b1] and [a2, b2] intersect
#[inline]
pub fn segments_intersect(a1: f32, b1: f32, a2: f32, b2: f32) -> bool {
    assert!(a1 <= b1 && a2 <= b2);
    if a1 < a2 {
        b1 >= a2
    } else {
        b2 >= a1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_intersect_test() {
        assert_eq!(segments_intersect(0.0, 0.0, 0.0, 0.0), true);
        assert_eq!(segments_intersect(0.0, 1.0, 0.5, 1.5), true);
        assert_eq!(segments_intersect(0.1, 0.4, 0.0, 0.09), false);
        assert_eq!(segments_intersect(-2.0, -1.0, -1.6, -1.6), true);
        assert_eq!(segments_intersect(2.5, 3.5, 2.5, 2.6), true);
    }
}
