use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::GameState;

pub struct Level1GaragePlugin;
impl Plugin for Level1GaragePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(ConditionSet::new().run_in_state(GameState::RunLevel).into());
    }
}
