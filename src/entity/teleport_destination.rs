use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

/// A marker component for teleport destinations.
#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize)]
pub struct TeleportDestination {}

spawn_from_scene!(teleport_destination, TeleportDestination);
