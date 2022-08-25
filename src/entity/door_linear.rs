use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_kira_audio::{AudioControl, DynamicAudioChannels};
use interpolation::lerp;
use serde::{Deserialize, Serialize};

use crate::{assets::SoundAssets, audio::AudioComponent, spawn_from_scene};

pub struct DoorFullyClosedEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

pub struct DoorFullyOpenedEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct NamedDoorStatus {
    pub entity: Entity,
    pub is_open: bool,
}

#[derive(Default)]
pub struct NamedDoorStatuses(HashMap<String, NamedDoorStatus>);

/// A door which moves linearly based on [`move_to`].
#[derive(Clone, Copy, Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct DoorLinear {
    pub speed: f32,
    pub move_to: [f32; 3],
    pub origin: Vec3,
    pub state: DoorState,
}

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub enum DoorState {
    FullyOpen,
    FullyClosed,
    /// 0 means closed, 1 means open
    Opening(f32),
    /// 0 means open, 1 means closed
    Closing(f32),
}

impl DoorState {
    pub fn toggle(&mut self) {
        if self.is_open() || self.is_opening() {
            self.close();
        } else if self.is_closed() || self.is_closing() {
            self.open();
        }
    }

    pub fn open(&mut self) {
        match self {
            DoorState::FullyOpen => {}
            DoorState::FullyClosed => {
                *self = DoorState::Opening(0.0);
            }
            DoorState::Opening(_progress) => {}
            DoorState::Closing(progress) => {
                *self = DoorState::Opening(1.0 - *progress);
            }
        }
    }

    pub fn close(&mut self) {
        match self {
            DoorState::FullyOpen => {
                *self = DoorState::Closing(0.0);
            }
            DoorState::FullyClosed => {}
            DoorState::Opening(progress) => {
                *self = DoorState::Closing(1.0 - *progress);
            }
            DoorState::Closing(_progress) => {}
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self, DoorState::FullyOpen)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, DoorState::FullyClosed)
    }

    pub fn is_opening(&self) -> bool {
        matches!(self, DoorState::Opening(_))
    }

    pub fn is_closing(&self) -> bool {
        matches!(self, DoorState::Closing(_))
    }
}

impl Default for DoorState {
    fn default() -> Self {
        DoorState::FullyClosed
    }
}

spawn_from_scene!(door_linear, DoorLinear, |_cmds, entity, door_linear| {
    let trans = entity.get::<Transform>().unwrap();
    door_linear.origin = trans.translation;
});

pub(super) fn door_sounds(
    mut cmds: Commands,
    doors: Query<(Entity, &DoorLinear, Option<&AudioComponent>), Changed<DoorLinear>>,
    mut open_doors: Local<HashSet<Entity>>,
    sound_assets: Res<SoundAssets>,
    mut channels: ResMut<DynamicAudioChannels>,
) {
    for (entity, door, audio_comp) in doors.iter() {
        if door.state.is_opening() {
            if open_doors.insert(entity) {
                AudioComponent::get_or_insert(&mut cmds, entity, audio_comp, &mut channels)
                    .channel(&channels)
                    .play(sound_assets.door_open.clone());
            }
        } else {
            open_doors.remove(&entity);
        }
    }
}

pub(super) fn update_door(
    time: Res<Time>,
    mut doors: Query<(Entity, &mut Transform, &mut DoorLinear, Option<&Name>)>,
    mut door_fully_closed_events: EventWriter<DoorFullyClosedEvent>,
    mut door_fully_opened_events: EventWriter<DoorFullyOpenedEvent>,
    mut named_door_statuses: ResMut<NamedDoorStatuses>,
) {
    named_door_statuses.0 = HashMap::new();

    for (entity, mut trans, mut door, name) in doors.iter_mut() {
        // TODO: player can get stuck in doors.
        // Workaround is to make sure trigger to open doors overlaps doors.
        let tween_result = tween_transform(
            time.delta_seconds(),
            door.speed,
            door.origin,
            door.move_to.into(),
            &mut door.state,
            &mut trans,
        );
        match tween_result {
            TweenResult::JustClosed => {
                door_fully_closed_events.send(DoorFullyClosedEvent {
                    name: name.map(|name| name.to_string()),
                    entity,
                });
                if let Some(name) = name {
                    named_door_statuses.0.insert(
                        name.to_string(),
                        NamedDoorStatus {
                            entity,
                            is_open: false,
                        },
                    );
                }
            }
            TweenResult::JustOpened => {
                door_fully_opened_events.send(DoorFullyOpenedEvent {
                    name: name.map(|name| name.to_string()),
                    entity,
                });
                if let Some(name) = name {
                    named_door_statuses.0.insert(
                        name.to_string(),
                        NamedDoorStatus {
                            entity,
                            is_open: true,
                        },
                    );
                }
            }
            _ => {}
        }
    }
}

// TODO move elsewhere
// Note: this teleports the transform,
// can result in things getting stuck
pub fn tween_transform(
    delta_s: f32,
    speed: f32,
    origin: Vec3,
    destination: Vec3,
    state: &mut DoorState,
    trans: &mut Transform,
) -> TweenResult {
    let (result, progress) = match *state {
        DoorState::Opening(mut progress) => {
            progress = f32::min(progress + speed * delta_s, 1.0);
            if progress >= 1.0 {
                *state = DoorState::FullyOpen;
                (TweenResult::JustOpened, Some(1.0))
            } else {
                *state = DoorState::Opening(progress);
                (TweenResult::Progress, Some(progress))
            }
        }
        DoorState::Closing(mut progress) => {
            progress = f32::max(progress + speed * delta_s, 0.0);
            if progress >= 1.0 {
                *state = DoorState::FullyClosed;
                (TweenResult::JustClosed, Some(0.0))
            } else {
                *state = DoorState::Closing(progress);
                (TweenResult::Progress, Some(1.0 - progress))
            }
        }
        _ => (TweenResult::Idle, None),
    };

    if let Some(progress) = progress {
        let local_dest = origin + trans.compute_matrix().transform_vector3(destination);
        trans.translation = lerp::<[f32; 3]>(&origin.into(), &local_dest.into(), &progress).into();
    }

    result
}

pub enum TweenResult {
    Idle,
    JustClosed,
    JustOpened,
    Progress,
}
