use std::collections::LinkedList;

use bevy::prelude::Component;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct SnakePiece(pub Direction);

#[derive(Component)]
pub struct Snake {
    pub body: LinkedList<SnakePiece>,
}
