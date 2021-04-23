use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemLabel {
    Input,
    Physics,
}
