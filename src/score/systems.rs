use std::cmp::max;

use bevy::prelude::*;

use super::resources::{HighestScore, Score};

use crate::util::resources::GameOver;

pub fn update_highest_score(
    mut event_reader: EventReader<GameOver>,
    current_score: Res<Score>,
    mut highest_score: ResMut<HighestScore>,
) {
    for _ in event_reader.iter() {
        highest_score.0 = max(highest_score.0, current_score.0);
        println!("highest score: {}", highest_score.0);
    }
}
