use std::time::Duration;

use bevy::prelude::*;


pub struct GraphicsPlugin;

#[derive(Resource, Debug)]
pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>, 
    pub run_animation: [usize; 4],
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer, 
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(Self::load_graphics).add_system(Self::frame_animation);
    }
}

pub fn spawn_player_sprite(
    commands: &mut Commands,
    characters: &CharacterSheet, 
    translation: Vec3
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(characters.run_animation[0]);
    sprite.custom_size = Some(Vec2 { x: 32.0, y: 32.0 });

    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: characters.handle.clone(), 
        transform: Transform { translation: translation,
        ..Default::default()
     },
     ..Default::default()
    }).insert(FrameAnimation{
        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        frames: characters.run_animation.to_vec(),
        current_frame: 0
    })
    .id()
}

impl GraphicsPlugin {
    fn load_graphics (
        mut commands: Commands, 
        assets: Res<AssetServer>, 
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ){
        let image: Handle<Image> = assets.load("Players/buddie0.png");
        let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.0), 8, 30, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            run_animation: [13*8, 13*8+1, 13*8+2, 13*8+3],
        });
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>, 
    ){
        for (mut sprite, mut animation) in  sprites_query.iter_mut(){
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}