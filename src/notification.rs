use bevy::{prelude::*, ui::FocusPolicy, window::WindowResized};

use iyes_loopless::prelude::*;

use crate::{
    assets::{FontAssets, GameState},
    entity::trigger::NamedTriggerStatuses,
    get_display_scale,
    inventory::Icon,
    levels::level2_lobby::GarageOpened,
};

pub struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_enter_system(GameState::RunLevel, create_notification_ui);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(resize_notification_ui)
                .with_system(clear_notification)
                .with_system(fade_in_ending_white)
                .into(),
        );
    }
}

#[derive(Component)]
struct NotificationUiContainer;

#[derive(Component)]
pub struct NotificationText(pub f32);

#[derive(Component)]
pub struct EndText;

fn create_notification_ui(mut cmds: Commands, windows: Res<Windows>, font_assets: Res<FontAssets>) {
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
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font_assets.fira_mono_medium.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_LEFT)
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
            .insert(NotificationText(3.0));
        parent
            .spawn_bundle(
                TextBundle::from_section(
                    "subfuse",
                    TextStyle {
                        font: font_assets.fira_mono_medium.clone(),
                        font_size: 48.0,
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                }),
            )
            .insert(EndText);
    })
    .insert(NotificationUiContainer);
}

fn resize_notification_ui(
    mut ui: Query<&mut Style, With<NotificationUiContainer>>,
    mut window_resized_events: EventReader<WindowResized>,
) {
    if let Some(event) = window_resized_events.iter().last() {
        for mut style in ui.iter_mut() {
            let scale = get_display_scale(event.width, event.height);
            style.size = Size::new(Val::Px(scale.x), Val::Px(scale.y));
        }
    }
}

fn clear_notification(time: Res<Time>, mut texts: Query<(&mut Text, &mut NotificationText)>) {
    for (mut text, mut note) in &mut texts {
        if note.0 >= 0.0 {
            note.0 -= time.delta_seconds();
            if note.0 <= 0.0 {
                if let Some(section) = text.sections.iter_mut().next() {
                    section.value = String::from("");
                }
            }
        }
    }
}

fn fade_in_ending_white(
    time: Res<Time>,
    mut fade: Local<f32>,
    mut ui: Query<&mut UiColor, With<NotificationUiContainer>>,
    mut end_text: Query<&mut Text, With<EndText>>,
    triggers: Res<NamedTriggerStatuses>,
    mut player_entered_end: Local<bool>,
    garage_opened: Option<Res<GarageOpened>>,
    mut icons: Query<(&Icon, &mut Visibility)>,
) {
    if garage_opened.is_some() && triggers.is_changed() {
        if let Some(_status) = triggers.any("End Win Area") {
            *player_entered_end = true;
        }
    }
    if *player_entered_end {
        *fade += time.delta_seconds() * 0.1;
        for mut color in ui.iter_mut() {
            color.0 = Color::rgba(0.7, 0.7, 0.7, fade.clamp(0.0, 1.0));
        }
        for mut text in end_text.iter_mut() {
            if let Some(section) = text.sections.iter_mut().next() {
                section.style.color = Color::rgba(0.0, 0.0, 0.0, fade.clamp(0.0, 1.0));
            }
        }
        for (_, mut vis) in &mut icons {
            // Couldn't figure out a good way to show notification ui on top
            vis.is_visible = false;
        }
    }
}
