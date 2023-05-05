use std::collections::LinkedList;

use bevy::prelude::{Component, Entity};

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct SnakePiece;

#[derive(Component)]
pub struct Snake {
    pub body: LinkedList<Entity>,
    pub direction: Direction,
}
