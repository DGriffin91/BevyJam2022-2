use bevy::prelude::*;

use bevy_fps_controller::controller::LogicalPlayer;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};

pub struct OverlapPlugin;

impl Plugin for OverlapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OverlapStartEvent>();
        app.add_event::<OverlapStopEvent>();
        app.add_system(overlap);
    }
}

pub struct OverlapStartEvent(pub Entity);
pub struct OverlapStopEvent(pub Entity);

pub fn overlap(
    player: Query<Entity, With<LogicalPlayer>>,
    mut overlap_start_events: EventWriter<OverlapStartEvent>,
    mut overlap_stop_events: EventWriter<OverlapStopEvent>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for player in player.iter() {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            overlap_start_events.send(OverlapStartEvent(*e2));
                        } else if *e2 == player {
                            overlap_start_events.send(OverlapStartEvent(*e1));
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, flags) => {
                    if flags.contains(CollisionEventFlags::SENSOR) {
                        if *e1 == player {
                            overlap_stop_events.send(OverlapStopEvent(*e2));
                        } else if *e2 == player {
                            overlap_stop_events.send(OverlapStopEvent(*e1));
                        }
                    }
                }
            }
        }
    }
}
