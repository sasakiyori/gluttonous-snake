pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::{HighestScore, Score};
use systems::update_highest_score;

use crate::util::resources::GameOver;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .insert_resource(HighestScore(0))
            .add_event::<GameOver>()
            .add_system(update_highest_score);
    }
}
