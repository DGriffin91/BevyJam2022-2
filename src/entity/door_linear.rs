use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

/// A teleport component with a destination and activation radius.
#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize)]
#[serde(default)]
pub struct DoorLinear {
    pub distance: f32,
    pub angle: f32,
    pub is_open: bool,
}

spawn_from_scene!(door_linear, DoorLinear);
