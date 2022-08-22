use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::Projection};
use bevy_editor_pls::{
    controls::{self, EditorControls},
    default_windows::hierarchy::HierarchyWindow,
    editor_window::{EditorWindow, EditorWindowContext},
    egui::Slider,
    AddEditorWindow, EditorEvent, EditorPlugin, EditorState,
};
use bevy_fps_controller::controller::{FpsController, LogicalPlayer};

use bevy_rapier3d::render::DebugRenderContext;
use iyes_loopless::prelude::*;

use crate::{
    assets::MyStates, interact::InteractDebugMode, overlap::OverlapDebugMode, PlayerCamera,
};

pub struct GameEditorPlugin;
impl Plugin for GameEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin)
            .add_startup_system(set_cam3d_controls)
            .insert_resource(editor_controls())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(MyStates::RunLevel)
                    .with_system(manage_cursor)
                    .into(),
            )
            .add_editor_window::<MyEditorWindow>();
    }
}

pub struct MyEditorWindow;
impl EditorWindow for MyEditorWindow {
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

        if let Some(mut debug) = world.get_resource_mut::<DebugRenderContext>() {
            ui.checkbox(&mut debug.enabled, "Draw Rapier Debug");
        }
        if let Some(mut debug) = world.get_resource_mut::<InteractDebugMode>() {
            ui.checkbox(&mut debug.0, "Interact Debug Mode");
        }
        if let Some(mut debug) = world.get_resource_mut::<OverlapDebugMode>() {
            ui.checkbox(&mut debug.0, "Overlap Debug Mode");
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

pub fn manage_cursor(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
    mut editor_events: EventReader<EditorEvent>,
    editor_state: Res<EditorState>,
) {
    let mut set_fps_mode = false;
    let mut set_cursor_mode = false;
    let window = windows.get_primary_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) && !editor_state.active {
        set_fps_mode = true;
    }
    if key.just_pressed(KeyCode::Escape) {
        set_cursor_mode = true;
    }
    for e in editor_events.iter() {
        if let EditorEvent::Toggle { now_active } = &e {
            if *now_active {
                set_cursor_mode = true;
            } else {
                set_fps_mode = true;
            }
        }
    }
    if set_fps_mode {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
        for mut controller in &mut controllers {
            controller.enable_input = true;
        }
    }
    if set_cursor_mode {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
        for mut controller in &mut controllers {
            controller.enable_input = false;
        }
    }
}
