use bevy::prelude::*;
use crate::GameState;
use crate::map::Wall;

const TILE_SIZE: u32 = 144;
const ENEMY_SIZE: u32 = 144;
const ENEMY_SPEED: f32 = 50.0;
const PACE_BOUNDARY: usize = 2;

#[derive(Component)]
pub struct Enemy{
    direction: i32,
    left_boundary: f32, 
    right_boundary: f32,
}

#[derive(Component)]
pub struct EnemyStats {
    pub attack: u32,
    pub magic: u32,
    pub speed: u32,
    pub max_hp: u32,
    pub hp: u32,
}

impl EnemyStats {
    pub fn new() -> Self {
        Self {
            attack: 1,
            magic: 1,
            speed: 1,
            max_hp: 10,
            hp: 10,
        }
    }
}

pub struct EnemyPlugin;
    
impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
/*         app.add_systems(Startup, init_enemy);
 */        app.add_systems(Update, enemy_pace.run_if(in_state(GameState::InGame)));
    }

}

/*
fn init_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let texture_handle = asset_server.load("enemyPlaceHolder.png");
    println!("Loading texture: {:?}", texture_handle);
    
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..default()
        },
        ..default()
    })
    .insert(Enemy {});
}*/

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    position: Vec3,
) {
    // load textures and create texture atlases
    let enemy_texture_handle = asset_server.load("enemyPlaceHolder.png");
    let enemy_layout = TextureAtlasLayout::from_grid(UVec2::splat(ENEMY_SIZE), 1, 1, None, None);
    let enemy_layout_handle = texture_atlases.add(enemy_layout);

    let left_boundary = position.x - (TILE_SIZE as f32 * PACE_BOUNDARY as f32);
    let right_boundary = position.x + (TILE_SIZE as f32 * PACE_BOUNDARY as f32);

    commands.spawn((
        SpriteBundle {
            texture: enemy_texture_handle.clone(),
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: 0, 
            layout: enemy_layout_handle.clone(),
        },
        Enemy  {
            direction: 1,
            left_boundary,
            right_boundary,
        },
        EnemyStats::new(),
    ));
}

fn enemy_pace(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Enemy)>,
){
    for (mut transform, mut enemy) in query.iter_mut() {
        transform.translation.x += enemy.direction as f32 * ENEMY_SPEED * time.delta_seconds();
        //turn if needed
        if transform.translation.x > enemy.right_boundary {
            enemy.direction = -1; 
        } else if transform.translation.x < enemy.left_boundary {
            enemy.direction = 1;
        }
        transform.translation.z = 900.0
    }
}