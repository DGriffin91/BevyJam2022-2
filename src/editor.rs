use bevy::prelude::*;
use bevy_editor_pls::{
    controls::{self, EditorControls},
    default_windows::hierarchy::HierarchyWindow,
    editor_window::{EditorWindow, EditorWindowContext},
    AddEditorWindow, EditorEvent, EditorPlugin, EditorState,
};
use bevy_fps_controller::controller::FpsController;

use iyes_loopless::prelude::*;

use crate::assets::MyStates;

pub struct GameEditorPlugin;
impl Plugin for GameEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin)
            .add_startup_system(set_cam3d_controls)
            .insert_resource(editor_controls())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(MyStates::Next)
                    .with_system(manage_cursor)
                    .into(),
            )
            .add_editor_window::<MyEditorWindow>();
    }
}

pub struct MyEditorWindow;
impl EditorWindow for MyEditorWindow {
    type State = ();
    const NAME: &'static str = "Another editor panel";

    fn ui(_world: &mut World, cx: EditorWindowContext, ui: &mut bevy_editor_pls::egui::Ui) {
        let _currently_inspected = &cx.state::<HierarchyWindow>().unwrap().selected;

        ui.label("Anything can go here");
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
        match &e {
            EditorEvent::Toggle { now_active } => {
                if *now_active {
                    set_cursor_mode = true;
                } else {
                    set_fps_mode = true;
                }
            }
            _ => (),
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
