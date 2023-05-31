use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource)]
pub struct HighestScore(pub u32);

impl Default for HighestScore {
    fn default() -> Self {
        Self(0)
    }
}
