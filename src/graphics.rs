use bevy::{prelude::*, reflect::Enum};

pub struct GraphicsPlugin;

#[derive(Resource, Debug)]
pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>, 
    pub run_animation: [usize; 4],
    pub talk_animation: [usize; 3],
    pub shoot: [usize; 2],
    pub idle: [usize; 1]
}

//TODO add animation enum for easier code readabiliyt
#[derive(Component, Debug)]
pub struct Animations {
    pub animations: Vec<FrameAnimation>,
    pub current_animation: usize,
}

#[derive(Component, Debug, Clone, Default)]
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
            run_animation: [14*8, 14*8+1, 14*8+2, 14*8+3],
            talk_animation: [10*8, 10*8+1, 10*8+2],
            shoot: [25*8, 7*8+1],
            idle: [7*8]
        });
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut Animations)>,
        time: Res<Time>, 
    ){
        for (mut sprite, mut animations) in  sprites_query.iter_mut(){
            //dbg!(&animations);
            //let mut animation = animations.animations[animations.current_animation];
            let current = animations.current_animation;
            animations.animations[current].timer.tick(time.delta());
            if animations.animations[current].timer.just_finished() {
                animations.animations[current].current_frame = (animations.animations[current].current_frame + 1) % animations.animations[current].frames.len();
                sprite.index = animations.animations[current].frames[animations.animations[current].current_frame];
            }
        }
    }


}