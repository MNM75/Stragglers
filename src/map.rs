use bevy::prelude::*;
const TILE_SIZE: u32 = 144;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;


#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Collider;

pub struct MapPlugin;

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, create_room);
    }
}

fn create_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    ///////////creating an 8x8 tile background (centered at window origin), with wall//////////

    let tile_sheet_handle = asset_server.load("tileProto.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    commands.spawn(Camera2dBundle::default());

    //starting point is x = -5 tiles, y = -5 tiles (to create an 8x8 room with an additional 1 tile wall)
    let x_bound =  (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut i = 0;
    let mut y:usize = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    
    //while 10 rows are not filled, apply a tile to each column in a row
    while (y as f32)< (9 as f32) {
        //if current row is filled, move to next row up
        if (i == 10){
            t += Vec3::new((-10.0 *TILE_SIZE as f32),  TILE_SIZE as f32, 0.); //changing the transform value
            i=0;
            y+=1; 
        }
        //while a row has less than 10 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 10.0*TILE_SIZE as f32 {
            //determine if this tile should be a wall
            let is_wall = y == 0 || y == 9 || i == 0 || i == 9;
            //println!("spawning ({}, {}), {}", t.x, t.y, is_wall);

            if is_wall { //add wall tile
                commands.spawn((
                    SpriteBundle {
                        texture: wall_sheet_handle.clone(),
                        transform: Transform {
                            translation: t,
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        index: i % wall_layout_len,
                        layout: wall_layout_handle.clone(),
                    },
                    Wall,
                    Collider,
                ))
                .insert(Background);
            } else { //add regular tile
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
            }

            i += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
        }
    }

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
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
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
        //check collision
        if (!check_wall_collision(new_pos, &wall_query)){
            pt.translation = new_pos;
        }
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
         //check collision
         if (!check_wall_collision(new_pos, &wall_query)){
            pt.translation = new_pos;
        }
    }
}

fn check_wall_collision(
    new_pos: Vec3,
    collider_query: &Query<&Transform, (With<Wall>, Without<Player>)>,
) -> bool {
    for collider_transform in collider_query.iter() {
        if new_pos.x + (TILE_SIZE as f32) / 2. > collider_transform.translation.x - (TILE_SIZE as f32) / 2.
        && new_pos.x - (TILE_SIZE as f32) / 2. < collider_transform.translation.x + (TILE_SIZE as f32) / 2.
        && new_pos.y - (TILE_SIZE as f32) / 2. < collider_transform.translation.y + (TILE_SIZE as f32) / 2.
        && new_pos.y + (TILE_SIZE as f32) / 2. > collider_transform.translation.y - (TILE_SIZE as f32) / 2.
        {
            return true;
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