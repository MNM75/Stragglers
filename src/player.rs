use bevy::prelude::*;
use crate::map::Wall;
use crate::GameState;
use crate::WIN_W;
use crate::WIN_H; 

const TILE_SIZE: u32 = 144;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

pub const LEVEL_W: f32 = 8000.;
pub const LEVEL_H: f32 = 1920.;

const ANIM_TIME: f32 = 0.2;
enum PlayerType {
    Character,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
struct Background;

pub struct Sides {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl From<Vec3> for Sides {
    fn from(pos: Vec3) -> Self {
        Self {
            top: pos.y + (TILE_SIZE as f32) / 2.,
            bottom: pos.y - (TILE_SIZE as f32) / 2.,
            left: pos.x - (TILE_SIZE as f32) / 2.,
            right: pos.x + (TILE_SIZE as f32) / 2.,
        }
    }
}

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}    
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, init_player)
        .add_systems(Update, move_player.run_if(in_state(GameState::InGame)))
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player).run_if(in_state(GameState::InGame)));
    }

}
    
fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    ///////////initializing player///////////
    let pc_sheet_handle = asset_server.load("L_Animation.png");
    let pc_layout = TextureAtlasLayout::from_grid(UVec2::new(82, 144), 4, 4, None, None);
    let pc_layout_len = pc_layout.textures.len();
    let pc_layout_handle = texture_atlases.add(pc_layout);
    commands.spawn((
        SpriteBundle {
            texture: pc_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: pc_layout_handle,
            index: 8,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(4),
        Velocity::new(),
        Player,
    ));
}

fn animate_player(
    time: Res<Time>,
    
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Player>,
    >,
) {
    let (v, mut texture_atlas, mut timer, frame_count) = player.single_mut();
    let mut counter: usize = 0;
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
        counter = counter +1;
        texture_atlas.index = (texture_atlas.index + counter) % **frame_count;
        }
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    //mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut TextureAtlas), (With<Player>, Without<Background>)>,
) {
    let (mut pt, mut pv, mut texture_atlas) = player.single_mut();

    let mut deltav = Vec2::splat(0.);
    //if input.just_pressed(KeyCode::KeyA){
        // texture_atlas.index = 4;
    //}

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        texture_atlas.index = 4;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
        //texture_atlas.index = 0;
    }

    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
        texture_atlas.index = 12;
    }

    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
        texture_atlas.index = 8;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    
    if new_pos.x >= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_W / 2. - (TILE_SIZE as f32) / 2.
    {
        //check collision
        if !check_wall_collision(new_pos, &wall_query){
            pt.translation = new_pos;
        }
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
         //check collision
         if !check_wall_collision(new_pos, &wall_query){
            pt.translation = new_pos;
        }
    }
}

fn check_wall_collision(
    new_pos: Vec3,
    collider_query: &Query<&Transform, (With<Wall>, Without<Player>)>,
) -> bool {
    for collider_transform in collider_query.iter() {
        let a: Sides = new_pos.into();
        let b: Sides = collider_transform.translation.into();
        if a.bottom <= b.top && a.top >= b.bottom && a.right >= b.left && a.left <= b.right {
            return true
        }
    }
    return false;
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
}    
