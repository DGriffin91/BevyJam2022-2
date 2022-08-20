use std::f32::consts::TAU;

use bevy::{asset::AssetServerSettings, math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FpsControllerPlugin)
        .add_startup_system(setup)
        .add_system(manage_cursor)
        .add_system(sun_follow_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    commands
        .spawn()
        .insert(Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero())
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(GravityScale(0.0))
        .insert(Ccd { enabled: true }) // Prevent clipping when going fast
        .insert(Transform::from_xyz(0.0, 3.0, 0.0))
        .insert(LogicalPlayer(0))
        .insert(FpsControllerInput {
            pitch: -TAU / 12.0,
            yaw: TAU * 5.0 / 8.0,
            ..default()
        })
        .insert(FpsController { ..default() });
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert(RenderPlayer(0))
        .insert(MainCamera);

    // Floor
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -40.0,
                max_x: 40.0,
                min_y: -0.25,
                max_y: 0.25,
                min_z: -40.0,
                max_z: 40.0,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_translation(vec3(0.0, -0.25, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(40.0, 0.25, 40.0))
        .insert(RigidBody::Fixed)
        .insert(Transform::identity());

    // Cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -1.0,
                max_x: 1.0,
                min_y: -1.0,
                max_y: 1.0,
                min_z: -1.0,
                max_z: 1.0,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(4.0, 1.0, 4.0),
            ..default()
        })
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(RigidBody::Fixed);

    // Cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -2.0,
                max_x: 2.0,
                min_y: -8.0,
                max_y: 8.0,
                min_z: -2.0,
                max_z: 2.0,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(45.0, 1.0, 4.0),
            ..default()
        })
        .insert(Collider::cuboid(2.0, 8.0, 2.0))
        .insert(RigidBody::Fixed);

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // sun
    commands
        .spawn_bundle(DirectionalLightBundle {
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
                shadow_depth_bias: 0.1,
                shadows_enabled: true,
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

pub fn manage_cursor(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
) {
    let window = windows.get_primary_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
        for mut controller in &mut controllers {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
        for mut controller in &mut controllers {
            controller.enable_input = false;
        }
    }
}
