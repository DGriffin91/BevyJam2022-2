use bevy::{prelude::*, ui::FocusPolicy, window::WindowResized};

use iyes_loopless::prelude::*;

use crate::{
    assets::{FontAssets, GameState},
    get_display_scale,
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
                .into(),
        );
    }
}

#[derive(Component)]
struct NotificationUiContainer;

#[derive(Component)]
pub struct NotificationText(pub f32);

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
        color: Color::NONE.into(), // Color::GRAY
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
            .insert(NotificationText(3.0));
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
