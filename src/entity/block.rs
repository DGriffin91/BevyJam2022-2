use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::spawn_from_scene;

#[derive(Clone, Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Block {}

spawn_from_scene!(block, Block, |cmds, _entity, _collider| {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0));
});
