use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

use super::teleport_destination::TeleportDestination;

/// A teleport component which teleports the player to a destination.
#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct Teleport {
    pub destination: String,
    pub enabled: bool,
}

impl Default for Teleport {
    fn default() -> Self {
        Self {
            destination: Default::default(),
            enabled: true,
        }
    }
}

spawn_from_scene!(teleport, Teleport, |cmds, _entity, _teleport| {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
});

/// Teleport a player if they are within the radius of any active teleport.
pub(super) fn teleport_player(
    mut player: Query<(Entity, &mut Transform), With<LogicalPlayer>>,
    teleports: Query<&Teleport, Without<LogicalPlayer>>,
    destinations: Query<(&Transform, &Name), (With<TeleportDestination>, Without<LogicalPlayer>)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for (player_entity, mut player_trans) in player.iter_mut() {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        let teleport = if *e1 == player_entity {
                            teleports.get(*e2).ok()
                        } else if *e2 == player_entity {
                            teleports.get(*e1).ok()
                        } else {
                            None
                        };

                        if let Some(teleport) = teleport {
                            if teleport.enabled {
                                let destination =
                                    destinations.iter().find_map(|(transform, name)| {
                                        if **name == teleport.destination {
                                            Some(transform)
                                        } else {
                                            None
                                        }
                                    });

                                match destination {
                                    Some(transform) => {
                                        debug!(destination = ?teleport.destination, "Player teleport");
                                        player_trans.translation = transform.translation;
                                    }
                                    None => {
                                        warn!(destination = %teleport.destination, "Attempted to teleport to unknown destination");
                                    }
                                }
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(..) => {}
            }
        }
    }
}
