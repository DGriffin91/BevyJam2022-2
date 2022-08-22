use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

use super::Named;

pub struct TriggerEnterEvent {
    pub trigger_name: Option<String>,
    pub trigger_entity: Entity,
}

impl Named for TriggerEnterEvent {
    fn name(&self) -> Option<&str> {
        self.trigger_name.as_deref()
    }
}

pub struct TriggerExitEvent {
    pub trigger_name: Option<String>,
    pub trigger_entity: Entity,
}

impl Named for TriggerExitEvent {
    fn name(&self) -> Option<&str> {
        self.trigger_name.as_deref()
    }
}

/// A trigger which emits [`TriggerEnterEvent`] and [`TriggerExitEvent`] events when the player enters the region.
#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct Trigger {
    pub enabled: bool,
}

impl Default for Trigger {
    fn default() -> Self {
        Self { enabled: true }
    }
}

spawn_from_scene!(trigger, Trigger, |cmds, _entity, _trigger| {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
});

pub(super) fn trigger_collision_events(
    player: Query<Entity, With<LogicalPlayer>>,
    mut trigger_entered_events: EventWriter<TriggerEnterEvent>,
    mut trigger_exit_events: EventWriter<TriggerExitEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<(Option<&Name>, &Trigger)>,
) {
    for player in player.iter() {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        let trigger = if *e1 == player {
                            triggers
                                .get(*e2)
                                .map(|(name, trigger)| (name, trigger, *e2))
                                .ok()
                        } else if *e2 == player {
                            triggers
                                .get(*e1)
                                .map(|(name, trigger)| (name, trigger, *e1))
                                .ok()
                        } else {
                            None
                        };

                        if let Some((name, trigger, entity)) = trigger {
                            if trigger.enabled {
                                debug!(name = ?name, "Enter trigger");
                                trigger_entered_events.send(TriggerEnterEvent {
                                    trigger_name: name.map(|name| name.to_string()),
                                    trigger_entity: entity,
                                });
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        let trigger = if *e1 == player {
                            triggers
                                .get(*e2)
                                .map(|(name, trigger)| (name, trigger, *e2))
                                .ok()
                        } else if *e2 == player {
                            triggers
                                .get(*e1)
                                .map(|(name, trigger)| (name, trigger, *e1))
                                .ok()
                        } else {
                            None
                        };

                        if let Some((name, trigger, entity)) = trigger {
                            if trigger.enabled {
                                debug!(name = ?name, "Exit trigger");
                                trigger_exit_events.send(TriggerExitEvent {
                                    trigger_name: name.map(|name| name.to_string()),
                                    trigger_entity: entity,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
