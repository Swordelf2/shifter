use bevy::prelude::*;

pub fn test_system(
    query: Query<&TestComponent>
) {
    println!("Hello, world!");
}
