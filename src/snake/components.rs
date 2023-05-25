use bevy::prelude::Component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn is_crossing(&self, other: &Direction) -> bool {
        match self {
            Direction::Left | Direction::Right => match other {
                Direction::Up | Direction::Down => true,
                _ => false,
            },
            Direction::Up | Direction::Down => match other {
                Direction::Left | Direction::Right => true,
                _ => false,
            },
            Direction::None => true,
        }
    }
}

#[derive(Component)]
pub struct Snake(pub Direction);
