use bevy::{prelude::*, ui::FocusPolicy, window::WindowResized};

use bevy_fps_controller::controller::FpsController;
use iyes_loopless::prelude::*;

use crate::{
    assets::{FontAssets, GameState},
    get_display_scale,
    materials::post_process::PostProcessingMaterial,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameSettings::default());
        app.add_enter_system(GameState::RunLevel, create_menu_ui);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(resize_menu_ui)
                .with_system(render_menu)
                .with_system(update_menu)
                .into(),
        );
    }
}

#[derive(Component)]
struct MenuUiContainer;

#[derive(Component)]
pub struct MenuText(pub String);

fn create_menu_ui(mut cmds: Commands, windows: Res<Windows>, font_assets: Res<FontAssets>) {
    let window = windows.get_primary().unwrap();

    let scale = get_display_scale(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(scale.x), Val::Px(scale.y)),
            justify_content: JustifyContent::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(0.0),
                left: Val::Percent(0.0),
                ..default()
            },
            ..default()
        },
        color: Color::NONE.into(),
        focus_policy: FocusPolicy::Pass,
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn_bundle(
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "",
                    TextStyle {
                        font: font_assets.fira_mono_medium.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_LEFT)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Percent(10.0),
                        left: Val::Percent(3.0),
                        ..default()
                    },
                    ..default()
                }),
            )
            .insert(MenuText(String::from("")));
    })
    .insert(MenuUiContainer);
}

fn resize_menu_ui(
    mut ui: Query<&mut Style, With<MenuUiContainer>>,
    mut window_resized_events: EventReader<WindowResized>,
) {
    if let Some(event) = window_resized_events.iter().last() {
        for mut style in ui.iter_mut() {
            let scale = get_display_scale(event.width, event.height);
            style.size = Size::new(Val::Px(scale.x), Val::Px(scale.y));
        }
    }
}

pub struct GameSettings {
    sensitivity: f32,
    monitor_fx: bool,
    sel: i32,
    max: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            sensitivity: 1.0,
            monitor_fx: true,
            sel: 0,
            max: 1,
        }
    }
}

fn add_item(s: &mut String, pos: i32, sel: i32, text: &str, value: &str) {
    if sel == pos {
        s.push_str("> ")
    } else {
        s.push_str("  ")
    }
    s.push_str(text);
    s.push_str(" ");
    s.push_str(value);
    s.push_str("\n");
}

impl GameSettings {
    fn render(&self) -> String {
        let mut s = String::from("");
        add_item(
            &mut s,
            0,
            self.sel,
            "Mouse Speed",
            &format!("{:.2}", self.sensitivity),
        );
        add_item(
            &mut s,
            1,
            self.sel,
            "Monitor FX",
            &format!("{}", self.monitor_fx),
        );
        s
    }
}

fn update_menu(
    mut windows: ResMut<Windows>,
    mut game_settings: ResMut<GameSettings>,
    keys: Res<Input<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    let window = windows.primary_mut();
    if window.cursor_visible() {
        let next = keys.just_pressed(KeyCode::Down) || keys.just_pressed(KeyCode::S);
        let prev = keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::W);
        let inc = keys.just_pressed(KeyCode::Right) || keys.just_pressed(KeyCode::D);
        let dec = keys.just_pressed(KeyCode::Left) || keys.just_pressed(KeyCode::A);
        if next {
            game_settings.sel += 1;
        }
        if prev {
            game_settings.sel -= 1;
        }
        game_settings.sel = game_settings.sel.clamp(0, game_settings.max);
        if game_settings.sel == 0 {
            if inc {
                game_settings.sensitivity += 0.1;
            } else if dec {
                game_settings.sensitivity -= 0.1;
            }
            game_settings.sensitivity = game_settings.sensitivity.clamp(0.0, 10.0);
        }
        if game_settings.sel == 1 {
            if inc {
                game_settings.monitor_fx = true;
            } else if dec {
                game_settings.monitor_fx = false;
            }
        }

        for mut controller in &mut controllers {
            controller.sensitivity = 0.001 * game_settings.sensitivity;
        }

        let monitor_fx = game_settings.monitor_fx as u32 as f32;

        for (_, mat) in post_processing_materials.iter_mut() {
            if mat.monitor_fx != monitor_fx {
                mat.monitor_fx = monitor_fx;
            }
        }
    }
}

fn render_menu(
    mut texts: Query<(&mut Text, &mut MenuText)>,
    mut windows: ResMut<Windows>,
    game_settings: Res<GameSettings>,
) {
    let window = windows.primary_mut();
    if window.cursor_visible() {
        if let Some((mut text, mut menu_text)) = texts.iter_mut().next() {
            let new_menu_text = game_settings.render();
            if menu_text.0 != new_menu_text {
                menu_text.0 = new_menu_text;
                if let Some(section) = text.sections.iter_mut().next() {
                    section.value = menu_text.0.clone();
                }
            }
        }
    } else {
        if let Some((mut text, mut menu_text)) = texts.iter_mut().next() {
            if menu_text.0.len() > 0 {
                menu_text.0 = String::from("");
                if let Some(section) = text.sections.iter_mut().next() {
                    section.value = menu_text.0.clone();
                }
            }
        }
    }
}
