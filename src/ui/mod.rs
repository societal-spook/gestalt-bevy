use bevy::{prelude::*, window::CursorGrabMode};

// use crate::interact::Focus;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            //     .add_system(look_at_ui)
            .add_system(toggle_cursor);
    }
}

fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let mut window = windows.single_mut();

        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

#[derive(Component)]
pub struct CrosshairText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(14.0), Val::Px(14.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                        ..default()
                    });
                    // .with_children(|parent| {
                    //     parent.spawn((
                    //         TextBundle::from_section(
                    //             "Text Example",
                    //             TextStyle {
                    //                 font: asset_server.load("fonts/CastoroTitling-Regular.ttf"),
                    //                 font_size: 20.0,
                    //                 color: Color::WHITE,
                    //             },
                    //         )
                    //         .with_style(Style {
                    //             position: UiRect {
                    //                 top: Val::Px(20.0),
                    //                 left: Val::Px(20.0),
                    //                 ..default()
                    //             },
                    //             ..Default::default()
                    //         }),
                    //         CrosshairText,
                    //     ));
                    // });
                });
        });
}

// fn look_at_ui(mut query: Query<&mut Text>, looking_at: Query<&Focus>) {
//     if query.iter().len() != 1 {
//         return;
//     }

//     let mut text = query.single_mut();

//     if let Some(section) = text.sections.first_mut() {
//         if looking_at.iter().len() == 1 {
//             section.value = String::from("INTERACT");
//         } else {
//             section.value = String::from("");
//         }
//     }
// }
