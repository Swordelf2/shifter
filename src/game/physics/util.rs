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

/// Tests whether segments `[a.0, a.1] and [b.0, b.1] intersect
#[inline]
pub fn segments_intersect(a: (f32, f32), b: (f32, f32)) -> bool {
    assert!(a.0 <= a.1 && b.0 <= b.1);

    if a.0 < b.0 {
        a.1 >= b.0
    } else {
        b.1 >= a.0
    }
}

/// Tests whether segments `[a.0, a.1] and [b.0, b.1] intersect, and if they do
/// returns the length of the intersection.
///
/// *Note*: works incorrectly if one segment contains another.
#[inline]
pub fn segments_intersection(a: (f32, f32), b: (f32, f32)) -> Option<f32> {
    assert!(a.0 <= a.1 && b.0 <= b.1);

    let intersection_len = if a.0 < b.0 { a.1 - b.0 } else { b.1 - a.0 };
    if intersection_len >= 0.0 {
        Some(intersection_len)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn segments_intersect_test() {
        assert_eq!(segments_intersect((0.0, 0.0), (0.0, 0.0)), true);
        assert_eq!(segments_intersect((0.0, 1.0), (0.5, 1.5)), true);
        assert_eq!(segments_intersect((0.1, 0.4), (0.0, 0.09)), false);
        assert_eq!(segments_intersect((-2.0, -1.0), (-1.6, -1.6)), true);
        assert_eq!(segments_intersect((2.5, 3.5), (2.5, 2.6)), true);
    }
}
