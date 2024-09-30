/* 
    Originally map.rs, renamed file to player.rs and created a seperate plugin file for map named map.rs 
*/

//use bevy::{prelude::*, window::PresentMode};
use bevy::prelude::*;

const TITLE: &str = "player";   
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: u32 = 144;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const LEVEL_W: f32 = 1920.;
const LEVEL_H: f32 = 1080.;
enum PlayerType {
    Character,
}

#[derive(Component)]
struct Player;
/*          Moved to map.rs
#[derive(Component)]
struct Tile;
*/
#[derive(Component)]
struct Background;


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
        .add_systems(Update, move_player)
        .add_systems(Update, move_camera.after(move_player));
    }

}
/*                  Moved to main.rs
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, move_camera.after(move_player))
        .run();
}
*/

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
/*                          Moved to new map.rs file
    ///////////creating an 8x8 tile background (centered at window origin)///////////

    let tile_sheet_handle = asset_server.load("tileProto.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    commands.spawn(Camera2dBundle::default());

    //starting point is x = -4 tiles, y = -4 tiles
    let x_bound =  (4. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (4. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut i = 0;
    let mut y:usize = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    
    //while 8 rows are not filled, apply a tile to each column in a row
    while (y as f32)< (8 as f32) {
        //if current row is filled, move to next row up
        if (i == 8){
            t += Vec3::new((-8.0 *TILE_SIZE as f32),  TILE_SIZE as f32, 0.); //changing the transform value
            i=0;
            y+=1;
        }
        //while a row has less than 8 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 8.0*TILE_SIZE as f32 {
            commands.spawn((
                SpriteBundle {
                    texture: tile_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: i % tile_layout_len,
                    layout: tile_layout_handle.clone(),
                },
                Tile,
            ))
            .insert(Background);

            i += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
        }
    }
*/
    ///////////initializing player///////////
    let pc_sheet_handle = asset_server.load("characterProto.png");
    let pc_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
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
            layout: pc_layout_handle.clone(),
            index: PlayerType::Character as usize,
        },
        Velocity::new(),
        Player,
    ));

}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
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
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }
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


