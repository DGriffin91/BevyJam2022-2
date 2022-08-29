#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy_kira_audio::{prelude::Audio, AudioControl, AudioTween};
use std::{f32::consts::PI, time::Duration};

use assets::{abs_transform, copy_names};
use bevy::{
    asset::AssetServerSettings,
    diagnostic::LogDiagnosticsPlugin,
    math::{vec2, vec3, vec4},
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
    window::{PresentMode, WindowMode, WindowResizeConstraints, WindowResized},
};
use bevy_asset_loader::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use levels::level3_chair::RingsSetup;
use menu::MenuPlugin;
use notification::NotificationPlugin;

use crate::assets::{FontAssets, GameState, ImageAssets, ModelAssets, SoundAssets};
use crate::audio::AudioComponentPlugin;
#[cfg(debug_assertions)]
use crate::editor::GameEditorPlugin;
use crate::entity::EntityPlugin;
use crate::inventory::InventoryPlugin;
use crate::levels::{Level, LevelsPlugin};
use crate::materials::{
    general::GeneralMaterial, post_process::PostProcessingMaterial, rings::RingsMaterial,
    swap_materials,
};
use crate::scene_hook::HookPlugin;
use crate::sidecar_asset::SidecarAssetPlugin;

mod assets;
mod audio;
#[cfg(debug_assertions)]
mod editor;
mod entity;
mod inventory;
mod levels;
mod macros;
mod materials;
mod menu;
mod notification;
mod scene_hook;
mod sidecar_asset;

fn main() {
    let mut app = App::new();

    app.add_loopless_state(GameState::AssetLoading)
        .add_loopless_state(Level::None)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::RunLevel)
                .with_collection::<FontAssets>()
                .with_collection::<ImageAssets>()
                .with_collection::<ModelAssets>()
                .with_collection::<SoundAssets>(),
        )
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            title: "Subfuse".to_string(),
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints {
                min_width: 256.0,
                min_height: 256.0,
                ..Default::default()
            },
            scale_factor_override: Some(1.0), //Needed for some mobile devices, but disables scaling
            present_mode: PresentMode::AutoVsync,
            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(AudioComponentPlugin)
        .add_plugin(HookPlugin)
        .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        .add_plugin(MaterialPlugin::<GeneralMaterial>::default())
        .add_plugin(MaterialPlugin::<RingsMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(SidecarAssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FpsControllerPlugin)
        .insert_resource(RingsSetup::default());

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    #[cfg(debug_assertions)]
    app.add_plugin(GameEditorPlugin);

    app.add_plugin(LevelsPlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(NotificationPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(MenuPlugin)
        .add_system(window_resized)
        .add_enter_system(GameState::RunLevel, hide_mouse)
        .add_enter_system(GameState::RunLevel, setup_player)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .label("pre_process")
                .with_system(copy_names)
                .with_system(abs_transform)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .after("pre_process")
                .with_system(sun_follow_camera)
                .with_system(toggle_mouse)
                .with_system(swap_materials)
                .into(),
        )
        .run();
}

pub fn get_display_scale(window_width: f32, window_height: f32) -> Vec3 {
    let window_width = window_width as u32;
    let window_height = window_height as u32;

    let scale = (window_height / 512).max(2);

    let width = (window_width / scale).max(256);
    let height = (window_height / scale).max(256);
    vec3(width as f32, height as f32, scale as f32)
}

fn setup_player(
    mut cmds: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut windows: ResMut<Windows>,
    mut rapier_debug: ResMut<DebugRenderContext>,
    audio: Res<Audio>,
    sound_assets: Res<SoundAssets>,
) {
    audio
        .play(sound_assets.drone1.clone())
        .looped()
        .fade_in(AudioTween::linear(Duration::from_secs(5)))
        .with_volume(0.4);

    rapier_debug.enabled = false; //Can't disable by default

    let window = windows.get_primary_mut().unwrap();

    //Trigger rescale?
    window.set_resolution(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    let scale = get_display_scale(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    let size = Extent3d {
        width: scale.x as u32,
        height: scale.y as u32,
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
        .insert(Ccd { enabled: true }) // Prevent clipping when going fast
        .insert(LogicalPlayer(0))
        .insert(FpsControllerInput {
            pitch: 0.0,
            yaw: 110.0f32.to_radians(),
            ..default()
        })
        .insert(FpsController {
            run_speed: 8.0,
            forward_speed: 8.0,
            max_air_speed: 8.0,
            walk_speed: 4.0,
            air_acceleration: 800.0, // bhop :D
            jump_speed: 6.0,
            key_jump: None,
            ..default()
        })
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: false },
            transform: Transform::from_translation(vec3(-36.0, 3.0, 67.0)),
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
    .insert(UiCameraConfig { show_ui: true })
    .insert(RenderPlayer(0))
    .insert(PlayerCamera);

    // cmds.spawn_bundle(Camera2dBundle {
    //     camera: Camera {
    //         target: RenderTarget::Image(image_handle.clone()),
    //         priority: 0,
    //         ..default()
    //     },
    //     ..default()
    // })
    // .insert(UiCameraConfig { show_ui: true });

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
        monitor_fx: vec4(1.0, 0.0, 0.0, 0.0),
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
    .insert(UiCameraConfig { show_ui: false })
    .insert(post_processing_pass_layer);
}

fn hide_mouse(mut windows: ResMut<Windows>) {
    let primary_win = windows.primary_mut();
    primary_win.set_cursor_visibility(false);
    primary_win.set_cursor_lock_mode(true);
    primary_win.set_cursor_position(vec2(0.0, 0.0));
}

fn toggle_mouse(
    mut windows: ResMut<Windows>,
    keys: Res<Input<KeyCode>>,
    mut fps_controller: Query<&mut FpsController>,
    btn: Res<Input<MouseButton>>,
) {
    let window = windows.primary_mut();
    let mut fps_controller = fps_controller.single_mut();
    if keys.just_pressed(KeyCode::Tab) {
        let is_locked = window.cursor_locked();
        if is_locked {
            // Unlock
            fps_controller.enable_input = false;
            window.set_cursor_visibility(true);
            window.set_cursor_lock_mode(false);
        } else {
            // Lock
            fps_controller.enable_input = true;
            window.set_cursor_visibility(false);
            window.set_cursor_lock_mode(true);
            window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));
        }
    }
    if keys.just_pressed(KeyCode::Escape)
        || (!window.cursor_locked() && fps_controller.enable_input)
    {
        // Unlock
        fps_controller.enable_input = false;
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }

    if btn.just_pressed(MouseButton::Left) {
        // Lock
        fps_controller.enable_input = true;
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
        window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));
    }
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
        let scale = get_display_scale(event.width, event.height);

        if let Some((_, mat)) = post_processing_materials.iter_mut().next() {
            let image = images.get_mut(&mat.source_image).unwrap();
            image.resize(Extent3d {
                width: scale.x as u32,
                height: scale.y as u32,
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
