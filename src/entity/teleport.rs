use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

use super::teleport_destination::TeleportDestination;

/// A teleport component with a destination and activation radius.
#[derive(Clone, Debug, Component, Serialize, Deserialize)]
#[serde(default)]
pub struct Teleport {
    pub destination: String,
    pub enabled: bool,
    // TODO: use a box instead of radius
    pub radius: f32,
}

impl Default for Teleport {
    fn default() -> Self {
        Self {
            destination: Default::default(),
            enabled: true,
            radius: 3.0,
        }
    }
}

spawn_from_scene!(teleport, Teleport);

/// Teleport a player if they are within the radius of any active teleport.
pub(super) fn teleport_player(
    mut player_transforms: Query<&mut Transform, With<LogicalPlayer>>,
    teleports: Query<(&Transform, &Teleport), Without<LogicalPlayer>>,
    destinations: Query<(&Transform, &Name), (With<TeleportDestination>, Without<LogicalPlayer>)>,
) {
    for mut player_transform in player_transforms.iter_mut() {
        for (teleport_transform, teleport) in teleports.iter() {
            if teleport.enabled {
                let dist = teleport_transform
                    .translation
                    .distance(player_transform.translation);

                if dist < teleport.radius {
                    // Player is in range of teleport, teleport player
                    let destination = destinations.iter().find_map(|(transform, name)| {
                        if **name == teleport.destination {
                            Some(transform)
                        } else {
                            None
                        }
                    });

                    match destination {
                        Some(transform) => {
                            player_transform.translation = transform.translation;
                        }
                        None => {
                            warn!(destination = %teleport.destination, "Attempted to teleport to unknown destination");
                        }
                    }
                }
            }
        }
    }
}
