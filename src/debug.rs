use bevy::prelude::*;

use crate::TestComponent;

pub fn test_system(
    query: Query<&TestComponent>
) {
    for component in query.iter() {
        dbg!(component);
    }
}
