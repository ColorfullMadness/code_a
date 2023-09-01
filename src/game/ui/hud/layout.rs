use bevy::prelude::*;

use crate::game::ui::hud::*;
use crate::game::ui::hud::styles::*;

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
        //image
        parent.spawn(
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(32.0),Val::Px(32.0)),
                    ..default()
                },
                image: asset_server.load("granade.png").into(),
                ..default()
            }
        );
        parent.spawn((
            TextBundle{
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Play!",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            }
                            )
                    ],
                    alignment: TextAlignment::Center,   
                    ..default()
                },
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            //ScoreText{},
        ));
    }).id();

    hud_entity
}