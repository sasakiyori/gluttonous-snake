use bevy::prelude::Component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Snake(pub Direction);
