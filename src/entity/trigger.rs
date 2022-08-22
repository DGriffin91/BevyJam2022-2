use bevy::{
    ecs::{system::EntityCommands, world::EntityRef},
    prelude::*,
};
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

pub struct TriggerEnterEvent {
    pub trigger_name: String,
    pub trigger_entity: Entity,
}

pub struct TriggerExitEvent {
    pub trigger_name: String,
    pub trigger_entity: Entity,
}

#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct Trigger {
    pub name: String,
    pub enabled: bool,
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            name: Default::default(),
            enabled: true,
        }
    }
}

spawn_from_scene!(trigger, Trigger, spawn_hook);

fn spawn_hook(cmds: &mut EntityCommands, _entity: &EntityRef, _trigger: &mut Trigger) {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
}

pub(super) fn trigger_collision_events(
    player: Query<Entity, With<LogicalPlayer>>,
    mut trigger_entered_events: EventWriter<TriggerEnterEvent>,
    mut trigger_exit_events: EventWriter<TriggerExitEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Trigger>,
) {
    for player in player.iter() {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            if let Ok(trigger) = triggers.get(*e2) {
                                if trigger.enabled {
                                    debug!(name = ?trigger.name, "Enter trigger!");
                                    trigger_entered_events.send(TriggerEnterEvent {
                                        trigger_name: trigger.name.clone(),
                                        trigger_entity: *e2,
                                    });
                                }
                            }
                        } else if *e2 == player {
                            if let Ok(trigger) = triggers.get(*e1) {
                                if trigger.enabled {
                                    debug!(name = ?trigger.name, "Enter trigger!");
                                    trigger_entered_events.send(TriggerEnterEvent {
                                        trigger_name: trigger.name.clone(),
                                        trigger_entity: *e1,
                                    });
                                }
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            if let Ok(trigger) = triggers.get(*e2) {
                                if trigger.enabled {
                                    debug!(name = ?trigger.name, "Exit trigger!");
                                    trigger_exit_events.send(TriggerExitEvent {
                                        trigger_name: trigger.name.clone(),
                                        trigger_entity: *e2,
                                    });
                                }
                            }
                        } else if *e2 == player {
                            if let Ok(trigger) = triggers.get(*e1) {
                                if trigger.enabled {
                                    debug!(name = ?trigger.name, "Exit trigger!");
                                    trigger_exit_events.send(TriggerExitEvent {
                                        trigger_name: trigger.name.clone(),
                                        trigger_entity: *e1,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
