use bevy::prelude::*;

use crate::game::ui::hud::*;

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let hud_entity = build_hud(&mut commands, &asset_server);
}

pub fn build_hud(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let hud_entity = commands.spawn(
        (NodeBundle {
            style: Style{
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: UiRect::new(Val::Px(18.0), Val::Px(18.0), Val::Px(18.0), Val::Px(18.0)),
                ..default()
            },
            ..default()
        },
        Hud{},
    )).with_children(|parent|{
        parent.spawn(
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(30.),
                        right: Val::Px(70.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Ammo: 0",
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
        ).insert(AmmoCountText);
        parent.spawn(
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(30.),
                        left: Val::Px(10.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Health: 0",
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
        ).insert(HealthCountText);
        parent.spawn(
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(90.0),Val::Px(50.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(22.5),
                        left: Val::Px(135.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                image: asset_server.load("oscilloscope.png").into(),
                ..default()
            },
        );

    }).id();
    hud_entity
}