use std::f32::consts::{PI, TAU};

use assets::{get_verts_indices, ModelAssets, MyStates};
use bevy::{asset::AssetServerSettings, math::vec3, prelude::*, render::camera::Projection};

use bevy_rapier3d::prelude::*;

use bevy_asset_loader::prelude::*;

use bevy_fps_controller::controller::*;
use editor::GameEditorPlugin;
use scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use sidecar_asset::SidecarAssetPlugin;

use iyes_loopless::prelude::*;

mod assets;
mod editor;
mod scene_hook;
mod sidecar_asset;

fn main() {
    App::new()
        .add_loopless_state(MyStates::AssetLoading)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .with_collection::<ModelAssets>(),
        )
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HookPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration::default())
        .add_plugin(SidecarAssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FpsControllerPlugin)
        .add_plugin(GameEditorPlugin)
        .add_enter_system(MyStates::Next, setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(MyStates::Next)
                .with_system(sun_follow_camera)
                .with_system(teleport_player)
                .into(),
        )
        .run();
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
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
            pitch: -TAU / 12.0,
            yaw: TAU * 5.0 / 8.0,
            ..default()
        })
        .insert(FpsController { ..default() })
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: false },
            transform: Transform::from_translation(vec3(-75.0, 1.0, -40.0)),
            ..default()
        });
    let mut camera_3d_bundle = Camera3dBundle { ..default() };
    camera_3d_bundle.projection = Projection::Perspective(PerspectiveProjection {
        fov: PI / 3.0,
        ..default()
    });
    cmds.spawn_bundle(camera_3d_bundle)
        .insert(RenderPlayer(0))
        .insert(MainCamera);

    // sun
    cmds.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -100.0,
                right: 100.0,
                bottom: -100.0,
                top: 100.0,
                near: -500.0,
                far: 500.0,
                scale: 1.0,
                ..default()
            },
            //shadow_depth_bias: 0.1,
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0f32.to_radians(),
            45.0f32.to_radians(),
            0.0,
        )),
        ..default()
    })
    .insert(Sun);

    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.map.clone(),
            ..default()
        },
        hook: SceneHook::new(|entity, world, cmds| {
            if let Some(name) = entity.get::<Name>() {
                if name.contains("(POS) parking garage elevator") {
                    cmds.insert(TeleportLocations::ParkingGarageElevator);
                } else if name.contains("(POS) lobby elevator") {
                    cmds.insert(TeleportLocations::LobbyElevator);
                }
            }
            if let Some(parent) = entity.get::<Parent>() {
                if let Some(name) = world.get::<Name>(parent.get()) {
                    if name.contains("(C)") {
                        if let Some(mesh) = entity.get::<Handle<Mesh>>() {
                            let meshes = world.get_resource::<Assets<Mesh>>().unwrap();
                            let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
                            cmds.insert(Collider::trimesh(vertices, indices));
                        }
                    }
                }
            }
        }),
    });
}

fn teleport_player(
    time: Res<Time>,
    mut player: Query<(&LogicalPlayer, &mut Transform), Without<TeleportLocations>>,
    teleports: Query<(&TeleportLocations, &Transform), Without<LogicalPlayer>>,
    mut cooldown: Local<f32>,
) {
    let since_startup = time.seconds_since_startup() as f32;

    if let Some((_player, mut player_trans)) = player.iter_mut().next() {
        for (tele, tele_trans) in &teleports {
            let dist = tele_trans.translation.distance(player_trans.translation);

            //Need to be out of range for 1 second
            if dist < 3.0 {
                if since_startup - *cooldown > 1.0 {
                    match tele {
                        TeleportLocations::ParkingGarageElevator => {
                            for (t, p) in &teleports {
                                if let TeleportLocations::LobbyElevator = t {
                                    player_trans.translation = p.translation;
                                    *cooldown = since_startup;
                                    return;
                                }
                            }
                        }
                        TeleportLocations::LobbyElevator => {
                            for (t, p) in &teleports {
                                if let TeleportLocations::ParkingGarageElevator = t {
                                    player_trans.translation = p.translation;
                                    *cooldown = since_startup;
                                    return;
                                }
                            }
                        }
                    }
                }
                //we didn't return, so restart cooldown
                *cooldown = since_startup;
            }
        }
    }
}

#[derive(Component)]
enum TeleportLocations {
    ParkingGarageElevator,
    LobbyElevator,
}

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct MainCamera;

fn sun_follow_camera(
    camera: Query<&Transform, (With<MainCamera>, Without<Sun>)>,
    mut sun: Query<&mut Transform, (With<Sun>, Without<MainCamera>)>,
) {
    for mut sun in &mut sun {
        for camera in &camera {
            sun.translation = camera.translation;
        }
    }
}
