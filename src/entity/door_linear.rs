use bevy::prelude::*;

use interpolation::lerp;
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

/// A door which moves linearly based on [`move_to`].
#[derive(Clone, Copy, Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct DoorLinear {
    pub speed: f32,
    pub move_to: [f32; 3],
    pub progress: f32,
    pub is_open: bool,
    pub origin: Vec3,
}

spawn_from_scene!(door_linear, DoorLinear, |_cmds, entity, door_linear| {
    let trans = entity.get::<Transform>().unwrap();
    door_linear.origin = trans.translation;
});

pub(super) fn update_door(time: Res<Time>, mut doors: Query<(&mut Transform, &mut DoorLinear)>) {
    for (mut trans, mut door) in doors.iter_mut() {
        // TODO: player can get stuck in doors.
        // Workaround is to make sure trigger to open doors overlaps doors.
        tween_transform(
            time.delta_seconds(),
            door.is_open,
            door.speed,
            door.origin,
            door.move_to.into(),
            &mut door.progress,
            &mut trans,
        );
    }
}

// TODO move elsewhere
// Note: this teleports the transform,
// can result in things getting stuck
pub fn tween_transform(
    delta_s: f32,
    state: bool,
    speed: f32,
    origin: Vec3,
    destination: Vec3,
    progress: &mut f32,
    trans: &mut Transform,
) {
    if state {
        if *progress < 1.0 {
            *progress += speed * delta_s;
        }
    } else if *progress > 0.0 {
        *progress -= speed * delta_s;
    }

    let local_dest = origin + trans.compute_matrix().transform_vector3(destination);

    trans.translation = lerp::<[f32; 3]>(&origin.into(), &local_dest.into(), progress).into();
}
