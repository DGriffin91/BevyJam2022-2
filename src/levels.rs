use bevy::prelude::*;

pub mod elevator_level;
pub mod level1_garage;
pub mod level2_lobby;
pub mod test_area;

#[derive(Component)]
pub enum Levels {
    Level1Garage,
    Level2Lobby,
    ElevatorLevel,
}
