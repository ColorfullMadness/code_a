use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
     if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
     }
}

pub fn build_main_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
            style: MAIN_MENU_STYLE,
            ..default()
        },
        MainMenu{},
    )
    )
    .with_children(|parent| {
        //title
        parent.spawn(
            NodeBundle {
                style: TITLE_STYLE,
                ..default()
            }
        ).with_children(|parent|{
            // Text
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Code A",
                                get_title_text_style(asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );

        });
        //play button
        parent.spawn(
            (
                ButtonBundle {
                    style: NORMAL_BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton{}
            )
        )
        .with_children(|parent|{
            parent.spawn(
                TextBundle{
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play!",
                                get_button_text_style(asset_server)
                                )
                        ],
                        alignment: TextAlignment::Center,   
                        ..default()
                    },
                    ..default()
                }
            );
        });
        //quit button
        parent.spawn(
            (
                ButtonBundle {
                    style: NORMAL_BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton{}
            )
        )
        .with_children(|parent|{
            parent.spawn(
                TextBundle{
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server)
                                )
                        ],
                        alignment: TextAlignment::Center,   
                        ..default()
                    },
                    ..default()
                }
            );
        });
        parent.spawn(
            TextBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Find Oscilloscope to win.",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            }
                        )
                    ],
                    alignment: TextAlignment::Left,
                    ..default()
                },
                ..default()
            }
        );
    })
    .id();

    main_menu_entity
}

