use bevy::prelude::*;
use crate::GameState;
use crate::map::Wall;

#[derive(Component)]
struct Enemy;

pub struct EnemyPlugin;
    
impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, init_enemy)
        .add_systems(Update, enemy_pace.run_if(in_state(GameState::InGame)));
    }

}


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
}

fn enemy_pace(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>,
){
    for mut transform in query.iter_mut() {
        transform.translation.x += 5.0 * time.delta_seconds();
        transform.translation.z = 900.0
    }
}