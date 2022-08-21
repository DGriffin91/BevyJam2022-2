#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use assets::{get_verts_indices, ModelAssets, MyStates};
use bevy::{asset::AssetServerSettings, math::vec3, prelude::*, render::camera::Projection};

use bevy_rapier3d::prelude::*;

use bevy_asset_loader::prelude::*;

use bevy_fps_controller::controller::*;
use editor::GameEditorPlugin;
use entity::EntityPlugin;
use interact::InteractPlugin;
use scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use sidecar_asset::SidecarAssetPlugin;

use iyes_loopless::prelude::*;

mod assets;
mod editor;
mod entity;
mod interact;
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
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FpsControllerPlugin)
        .add_plugin(GameEditorPlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(InteractPlugin)
        .add_enter_system(MyStates::Next, setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(MyStates::Next)
                .with_system(sun_follow_camera)
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
            scene: model_assets.test_area.clone(),
            ..default()
        },
        hook: SceneHook::new(|entity, world, cmds| {
            if let Some(name) = entity.get::<Name>() {
                if name.contains("(C-SENS)") {
                    // Cuboid sensor, will use scale/rotation from gltf
                    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
                }

                // Triggering with ball Sensor seems inconsistent. Cuboid seems much better
                // if name.contains("(S-SENS)") {
                //     // Sphere sensor, will use scale/rotation from gltf
                //     cmds.insert(Collider::ball(1.0)).insert(Sensor);
                // }
            }

            if let Some(parent) = entity.get::<Parent>() {
                if let Some(parent_name) = world.get::<Name>(parent.get()) {
                    if parent_name.contains("(C)") {
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
