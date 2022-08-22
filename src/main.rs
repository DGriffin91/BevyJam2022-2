#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use assets::SoundAssets;
use bevy::{asset::AssetServerSettings, math::vec3, prelude::*, render::camera::Projection};
use bevy_asset_loader::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::{GameState, ModelAssets};
use crate::editor::GameEditorPlugin;
use crate::entity::EntityPlugin;
use crate::levels::test_area::TestAreaLevelPlugin;
use crate::scene_hook::HookPlugin;
use crate::sidecar_asset::SidecarAssetPlugin;

mod assets;
mod editor;
mod entity;
mod levels;
mod macros;
mod scene_hook;
mod sidecar_asset;

fn main() {
    App::new()
        .add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::RunLevel)
                .with_collection::<ModelAssets>()
                .with_collection::<SoundAssets>(),
        )
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(HookPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration::default())
        .add_plugin(SidecarAssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FpsControllerPlugin)
        .add_plugin(GameEditorPlugin)
        .add_plugin(TestAreaLevelPlugin)
        .add_plugin(EntityPlugin)
        .add_enter_system(GameState::RunLevel, setup_player)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(sun_follow_camera)
                .into(),
        )
        .run();
}

fn setup_player(mut cmds: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    cmds.spawn()
        //TODO why so short?
        .insert(Collider::capsule(Vec3::Y * 0.0, Vec3::Y * 0.5, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero())
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(GravityScale(0.0))
        .insert(Ccd { enabled: false }) // Prevent clipping when going fast
        .insert(LogicalPlayer(0))
        .insert(FpsControllerInput {
            pitch: 0.0,
            yaw: -PI / 2.0,
            ..default()
        })
        .insert(FpsController {
            run_speed: 12.0,
            forward_speed: 12.0,
            max_air_speed: 12.0,
            walk_speed: 6.0,
            ..default()
        })
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: false },
            transform: Transform::from_translation(vec3(-78.0, 1.0, -40.0)),
            ..default()
        });
    let mut camera_3d_bundle = Camera3dBundle { ..default() };
    camera_3d_bundle.projection = Projection::Perspective(PerspectiveProjection {
        fov: PI / 3.0,
        ..default()
    });
    cmds.spawn_bundle(camera_3d_bundle)
        .insert(RenderPlayer(0))
        .insert(PlayerCamera);
}

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct PlayerCamera;

fn sun_follow_camera(
    camera: Query<&Transform, (With<PlayerCamera>, Without<Sun>)>,
    mut sun: Query<&mut Transform, (With<Sun>, Without<PlayerCamera>)>,
) {
    for mut sun in &mut sun {
        for camera in &camera {
            sun.translation = camera.translation;
        }
    }
}
