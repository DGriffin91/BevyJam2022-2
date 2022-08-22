use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

use super::Named;

pub struct TriggerEnterEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

impl Named for TriggerEnterEvent {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

pub struct TriggerExitEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

impl Named for TriggerExitEvent {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
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

#[derive(Clone, Copy, Debug)]
pub struct NamedTriggerStatus {
    pub entity: Entity,
    pub exit_enter: bool,
}

#[derive(Default)]
pub struct NamedTriggerStatuses(HashMap<String, NamedTriggerStatus>);

impl NamedTriggerStatuses {
    pub fn any(&self, pat: &str) -> Option<NamedTriggerStatus> {
        for (name, status) in &self.0 {
            if name.contains(pat) {
                return Some(*status);
            }
        }
        None
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
    mut named_trigger_statuses: ResMut<NamedTriggerStatuses>,
) {
    named_trigger_statuses.0 = HashMap::new();
    if let Some(player) = player.iter().next() {
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
                                let name = name.map(|name| name.to_string());
                                debug!(name = ?name, "Enter trigger");
                                trigger_entered_events.send(TriggerEnterEvent {
                                    name: name.clone(),
                                    entity,
                                });
                                if let Some(name) = name {
                                    named_trigger_statuses.0.insert(
                                        name,
                                        NamedTriggerStatus {
                                            entity,
                                            exit_enter: true,
                                        },
                                    );
                                }
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
                                let name = name.map(|name| name.to_string());
                                debug!(name = ?name, "Exit trigger");
                                trigger_exit_events.send(TriggerExitEvent {
                                    name: name.clone(),
                                    entity,
                                });
                                if let Some(name) = name {
                                    named_trigger_statuses.0.insert(
                                        name,
                                        NamedTriggerStatus {
                                            entity,
                                            exit_enter: false,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
