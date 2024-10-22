use bevy::prelude::*;
use crate::GameState;
use crate::player::Player;

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

pub struct EnemyPlugin;
    
impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
         app.add_systems(Update, enemy_pace.run_if(in_state(GameState::InGame)));
    }

}

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

// despawning an enemy
pub fn despawn_enemy(
    commands: &mut Commands,
    enemy_entity: Entity,
) {
    commands.entity(enemy_entity).despawn();
}

pub fn despawn_closest_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    let mut closest_enemy: Option<(Entity, f32)> = None;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let distance = player_transform.translation.distance(enemy_transform.translation);
        if let Some((_, closest_distance)) = closest_enemy {
            if distance < closest_distance {
                closest_enemy = Some((enemy_entity, distance));
            }
        } else {
            closest_enemy = Some((enemy_entity, distance));
        }
    }

    if let Some((closest_enemy_entity, _)) = closest_enemy {
        commands.entity(closest_enemy_entity).despawn();
    }
}