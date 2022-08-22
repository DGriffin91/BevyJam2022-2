use bevy::prelude::*;

use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::interact::debug_event_print;

pub struct OverlapPlugin;

impl Plugin for OverlapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OverlapEvent>();
        app.add_system(overlap);
        app.add_system(overlap_debug);
        app.init_resource::<OverlapDebugMode>();
    }
}
#[derive(Default)]
pub struct OverlapDebugMode(pub bool);
pub struct OverlapEvent {
    pub start: bool, // else stop
    pub entity: Entity,
}

pub fn overlap(
    player: Query<Entity, With<LogicalPlayer>>,
    mut overlap_events: EventWriter<OverlapEvent>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for player in player.iter() {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            overlap_events.send(OverlapEvent {
                                entity: *e2,
                                start: true,
                            });
                        } else if *e2 == player {
                            overlap_events.send(OverlapEvent {
                                entity: *e1,
                                start: true,
                            });
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            overlap_events.send(OverlapEvent {
                                entity: *e2,
                                start: false,
                            });
                        } else if *e2 == player {
                            overlap_events.send(OverlapEvent {
                                entity: *e1,
                                start: false,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn overlap_debug(
    debug_mode: Res<OverlapDebugMode>,
    mut overlap_events: EventReader<OverlapEvent>,
    transforms: Query<&Transform>,
    names: Query<&Name>,
) {
    if debug_mode.0 {
        for event in overlap_events.iter() {
            debug_event_print(
                &format!(
                    "OverlapEvent {} ",
                    if event.start { "start" } else { "stop" }
                ),
                event.entity,
                &transforms,
                &names,
            );
        }
    }
}
