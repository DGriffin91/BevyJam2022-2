use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::Projection};
use bevy_editor_pls::{
    controls::{self, EditorControls},
    default_windows::hierarchy::HierarchyWindow,
    editor_window::{EditorWindow, EditorWindowContext},
    egui::Slider,
    AddEditorWindow, EditorEvent, EditorPlugin,
};
use bevy_editor_pls_default_windows::cameras::EditorCamera;
use bevy_fps_controller::controller::{FpsController, LogicalPlayer};
use bevy_rapier3d::render::DebugRenderContext;
use iyes_loopless::prelude::*;

use crate::{assets::GameState, PlayerCamera};

pub struct GameEditorPlugin;
impl Plugin for GameEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin)
            .add_startup_system(set_cam3d_controls)
            .insert_resource(editor_controls())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::RunLevel)
                    .with_system(sync_editor_free_camera)
                    .into(),
            )
            .add_editor_window::<SettingsWindow>();
    }
}

pub struct SettingsWindow;
impl EditorWindow for SettingsWindow {
    type State = ();
    const NAME: &'static str = "Settings";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        let _currently_inspected = &cx.state::<HierarchyWindow>().unwrap().selected;

        // FOV
        let mut projection = world
            .query_filtered::<&mut Projection, With<PlayerCamera>>()
            .single_mut(world);
        let fov = match projection.as_mut() {
            Projection::Perspective(perspective) => &mut perspective.fov,
            Projection::Orthographic(_) => panic!("Orthographic not supported"),
        };

        ui.label("FOV");
        ui.add(Slider::new(fov, PI / 2.0..=PI / 4.0));

        // Sensitivity
        let mut fps_controller = world
            .query_filtered::<&mut FpsController, With<LogicalPlayer>>()
            .single_mut(world);
        let mut sensitivity = fps_controller.sensitivity * 10_000.0;

        ui.label("Sensitivity");
        ui.add(Slider::new(&mut sensitivity, 1.0..=50.0));
        fps_controller.sensitivity = sensitivity / 10_000.0;

        // Debugging
        if let Some(mut debug) = world.get_resource_mut::<DebugRenderContext>() {
            ui.checkbox(&mut debug.enabled, "Draw Rapier Debug");
        }
    }
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn set_cam3d_controls(
    mut query: Query<
        &mut bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls,
    >,
) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::E;
    controls.key_down = KeyCode::Q;
}

fn sync_editor_free_camera(
    mut d3_cam: Query<&mut Transform, (With<EditorCamera>, Without<PlayerCamera>)>,
    player_cam: Query<&Transform, With<PlayerCamera>>,
    mut editor_events: EventReader<EditorEvent>,
) {
    for editor_event in editor_events.iter() {
        if let EditorEvent::Toggle { now_active } = editor_event {
            if *now_active {
                for mut cam in d3_cam.iter_mut() {
                    *cam = *player_cam.single();
                }
            }
        }
    }
}
