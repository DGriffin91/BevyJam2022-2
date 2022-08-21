use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{interact::InteractEvent, spawn_from_scene};

/// A teleport component with a destination and activation radius.
#[derive(Clone, Copy, Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct DoorLinear {
    pub distance: f32,
    pub angle: f32,
    pub is_open: bool,
}

spawn_from_scene!(door_linear, DoorLinear);

pub(super) fn interact_open_door(
    doors: Query<&mut DoorLinear>,
    mut interact_events: EventReader<InteractEvent>,
) {
    for InteractEvent(entity) in interact_events.iter() {
        println!("Interaction {entity:?}");

        if let Ok(door) = doors.get(*entity) {
            println!("Open door! {door:?}");
        }
    }
}
