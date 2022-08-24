#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use assets::SoundAssets;
use audio::AudioComponentPlugin;
use bevy::{
    asset::AssetServerSettings,
    diagnostic::FrameTimeDiagnosticsPlugin,
    math::vec3,
    prelude::*,
    render::{
        camera::{Projection, RenderTarget},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler, ImageSettings},
        view::RenderLayers,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    window::{PresentMode, WindowResized},
};
use bevy_asset_loader::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use levels::map::MapLevelPlugin;
use materials::{general::GeneralMaterial, post_process::PostProcessingMaterial};

use crate::assets::{GameState, ModelAssets};
use crate::editor::GameEditorPlugin;
use crate::entity::EntityPlugin;
use crate::scene_hook::HookPlugin;
use crate::sidecar_asset::SidecarAssetPlugin;

mod assets;
mod audio;
mod editor;
mod entity;
mod levels;
mod macros;
mod materials;
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
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            title: "BevyJam 2022 - 2".to_string(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(AudioComponentPlugin)
        .add_plugin(HookPlugin)
        .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        .add_plugin(MaterialPlugin::<GeneralMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration::default())
        .add_plugin(SidecarAssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FpsControllerPlugin)
        .add_plugin(GameEditorPlugin)
        .add_plugin(MapLevelPlugin)
        .add_plugin(EntityPlugin)
        .add_enter_system(GameState::RunLevel, setup_player)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(sun_follow_camera)
                .with_system(window_resized)
                .into(),
        )
        .run();
}

fn setup_player(
    mut cmds: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    let window_width = window.physical_width() as u32;
    let window_height = window.physical_height() as u32;

    let scale = (window_height / 512).max(2);

    let width = (window_width / scale).max(256);
    let height = (window_height / scale).max(256);

    let size = Extent3d {
        width,
        height,
        ..default()
    };
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        sampler_descriptor: ImageSampler::nearest(),
        ..default()
    };
    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);

    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    cmds.spawn()
        //TODO why so short?
        .insert(Collider::capsule(Vec3::Y * 0.0, Vec3::Y * 0.7, 0.5))
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
            air_acceleration: 800.0, // bhop :D
            ..default()
        })
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: false },
            transform: Transform::from_translation(vec3(14.6, 1.0, 2.0)),
            ..default()
        });
    cmds.spawn_bundle(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: PI / 3.0,
            ..default()
        }),
        ..default()
    })
    .insert(RenderPlayer(0))
    .insert(PlayerCamera);

    //----- POST PROCESS -----

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    ))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        source_image: image_handle,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    cmds.spawn_bundle(MaterialMesh2dBundle {
        mesh: quad_handle.into(),
        material: material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            ..default()
        },
        ..default()
    })
    .insert(post_processing_pass_layer)
    .insert(PostProcessMesh);

    // The post-processing pass camera.
    cmds.spawn_bundle(Camera2dBundle {
        camera: Camera {
            // renders after the first main camera which has default value: 0.
            priority: 1,
            ..default()
        },
        ..Camera2dBundle::default()
    })
    .insert(post_processing_pass_layer);
}

#[derive(Component)]
struct PostProcessMesh;

fn window_resized(
    mut window_resized_events: EventReader<WindowResized>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    post_process_mesh: Query<&Mesh2dHandle, With<PostProcessMesh>>,
    mut image_events: EventWriter<AssetEvent<Image>>,
) {
    if let Some(event) = window_resized_events.iter().last() {
        let window_width = event.width as u32;
        let window_height = event.height as u32;

        let scale = (window_height / 512).max(2);

        let width = (window_width / scale).max(256);
        let height = (window_height / scale).max(256);

        dbg!(scale, width, height);

        if let Some((_, mat)) = post_processing_materials.iter_mut().next() {
            let image = images.get_mut(&mat.source_image).unwrap();
            image.resize(Extent3d {
                width,
                height,
                ..default()
            });
            image_events.send(AssetEvent::Modified {
                handle: mat.source_image.clone(),
            });

            // Resize Mesh
            for mesh in post_process_mesh.iter() {
                let quad = Mesh::from(shape::Quad::new(Vec2::new(event.width, event.height)));
                let _ = meshes.set(mesh.0.clone(), quad);
            }
        }
    }
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
