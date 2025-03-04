use bevy::prelude::*;
use rand::prelude::*;
use crate::enemy::spawn_enemy;
const TILE_SIZE: u32 = 144;
const DOOR_SIZE: u32 = 296;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Door;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_room);
    }
}

fn create_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
     ///////////creating an 8x8 tile background (centered at window origin), with wall//////////
    
    let tile_sheet_handle = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    commands.spawn((Camera2dBundle::default(),));

     //starting point is x = -5 tiles, y = -5 tiles (to create an 8x8 room with an additional 1 tile wall)
    let x_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut i = 0;
    let mut y: usize = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);

     //while 10 rows are not filled, apply a tile to each column in a row
     while (y as f32) < (9 as f32) {
         //if current row is filled, move to next row up
         if i == 10 {
            t += Vec3::new(-10.0 * TILE_SIZE as f32, TILE_SIZE as f32, 0.); // Changing the transform value
            i = 0;
            y += 1;
        }

        // while a row has less than 10 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 10.0 * TILE_SIZE as f32 {
             //determine if this tile should be a wall
            let is_wall = y == 0 || y == 9 || i == 0 || (i == 9 && y != 4 && y != 5); // opening in the right wall at y == 4 and y == 5

            if is_wall {
                // add wall tile
                commands
                    .spawn((
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
                    ))
                    .insert(Background);
            } else {
                // add regular tile
                let rand: usize = random();
                commands
                    .spawn((
                        SpriteBundle {
                            texture: tile_sheet_handle.clone(),
                            transform: Transform {
                                translation: t,
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: rand % tile_layout_len,
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

    const OFFSET: f32 = 720.0; // change this value to raise/lower the hallway

   
    // create the hallway and get the end position
    let hallway_start_position = Vec3::new(
        (5.0 * TILE_SIZE as f32) + TILE_SIZE as f32 / 2.0,
        -((4.0 * TILE_SIZE as f32) - TILE_SIZE as f32 / 2.0) + OFFSET,
        0.0,
    );

    let end_of_hallway = create_hallway(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        hallway_start_position,
    );

    // spawn second room to the right of the hallway
    let second_room_start_position = Vec3::new(
        end_of_hallway.x, // the end of the hallway
        end_of_hallway.y -288.0, // adjust the y value to match the hallway exit
        0.0,
    );

    //create the second room
    create_second_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        second_room_start_position,
    );

    let second_room_center = second_room_start_position + Vec3::new(
        (10.0 * TILE_SIZE as f32) / 2.0, 
        (10.0 * TILE_SIZE as f32) / 2.0, 
        10.0,
    );

    spawn_door(
        &mut commands, 
        &asset_server, 
        &mut texture_atlases, 
        second_room_center,
    );
}

fn create_hallway(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    start_position: Vec3,
) -> Vec3 {
    const HALLWAY_ROWS: usize = 4; // ttal rows, including the walls
    const HALLWAY_COLUMNS: usize = 5; // columns for the hallway
    let mut t = start_position;

    // load tile texture
    let tile_sheet_handle: Handle<Image> = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    // load wall texture
    let wall_sheet_handle: Handle<Image> = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_handle = texture_atlases.add(wall_layout);

    
    for row in 0..HALLWAY_ROWS {
        for column in 0..HALLWAY_COLUMNS {
            if row == 0 || row == HALLWAY_ROWS - 1 {
                // outer rows are walls
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
                        index: 0, // wall texture index
                        layout: wall_layout_handle.clone(),
                    },
                    Wall,
                ));
            } else {
                // inner rows are tiles
                let rand: usize = random();
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
                        index: rand % tile_layout_len, // tile texture index
                        layout: tile_layout_handle.clone(),
                    },
                    Tile,
                ));
            }
            t += Vec3::new(TILE_SIZE as f32, 0.0, 0.0); // move to the right for the next tile/wall
        }
        // reset the translation and move down
        t.x = start_position.x; // reset x to the starting x position
        t.y -= TILE_SIZE as f32; // move down for the next row
    }

    ////// spawning enemy at random point in hallway ////// 
    let new_rand: usize = random();
    let random_x = start_position.x + random::<f32>() * ((start_position.x + HALLWAY_COLUMNS as f32 * TILE_SIZE as f32) - start_position.x);
    let random_y = start_position.y + random::<f32>() * ((start_position.y - (HALLWAY_ROWS - 1) as f32 * TILE_SIZE as f32) - start_position.y);
    let enemy_position = Vec3::new(random_x, random_y, 1.0);
    spawn_enemy(commands, asset_server, texture_atlases, enemy_position, 1);

    Vec3::new(start_position.x + (HALLWAY_COLUMNS as f32 * TILE_SIZE as f32), t.y, 0.0)
}

// second room that opens on the left side to connect the hallway, similar to the first room 
fn create_second_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>, 
    start_position: Vec3,
) {
    // load textures and create texture atlases
    let tile_sheet_handle = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    let mut t = start_position; // use the given starting position

    let mut i = 0;
    let mut y: usize = 0;

    // while 10 rows are not filled, apply a tile to each column in a row
    while (y as f32) < (9 as f32) {
        // if current row is filled, move to next row up
        if i == 10 {
            t += Vec3::new(-10.0 * TILE_SIZE as f32, TILE_SIZE as f32, 0.); // move up for the next row
            i = 0;
            y += 1;
        }

        // while a row has less than 10 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 10.0 * TILE_SIZE as f32 {
            // determine if this tile should be a wall
            let is_wall = y == 0 || y == 9 || i == 9 || (i == 0 && y != 4 && y != 5); // opening on the left side of new room

            if is_wall {
                // add wall tile
                let rand: usize = random();
                commands
                    .spawn(( 
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
                    ))
                    .insert(Background);
            } else {
                // add regular tile
                let rand: usize = random();
                commands
                    .spawn(( 
                        SpriteBundle {
                            texture: tile_sheet_handle.clone(),
                            transform: Transform {
                                translation: t,
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: rand % tile_layout_len,
                            layout: tile_layout_handle.clone(),
                        },
                        Tile,
                    ))
                    .insert(Background);
            }

            i += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.); // move to the right for the next tile
        }
    }
}

fn spawn_door(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    position: Vec3,
){
    // load textures and create texture atlases
    let door_texture_handle = asset_server.load("enddoor.png");
    let door_layout = TextureAtlasLayout::from_grid(UVec2::splat(DOOR_SIZE), 1, 1, None, None);
    let door_layout_handle = texture_atlases.add(door_layout);

    commands.spawn((
        SpriteBundle {
            texture: door_texture_handle.clone(),
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: 0, 
            layout: door_layout_handle.clone(),
        },
        Door
    ));
}


// utility function to create small battle rooms
fn create_battle_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    start_position: Vec3,
    entrance_side: &str,
) {
    const ROOM_SIZE: usize = 6;
    const HALLWAY_LENGTH: usize = 2;
    let mut t = start_position;

    // load tile texture
    let tile_sheet_handle: Handle<Image> = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    // load wall texture
    let wall_sheet_handle: Handle<Image> = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    let mut t = start_position; // use the given starting position

    let mut i = 0;
    let mut y: usize = 0;

    // while 10 rows are not filled, apply a tile to each column in a row
    while (y as f32) < (9 as f32) {
        // if current row is filled, move to next row up
        if i == 10 {
            t += Vec3::new(-10.0 * TILE_SIZE as f32, TILE_SIZE as f32, 0.); // move up for the next row
            i = 0;
            y += 1;
        }

        // while a row has less than 10 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 10.0 * TILE_SIZE as f32 {
            // determine if this tile should be a wall
            let is_wall = y == 0 || y == 9 || i == 9 || (i == 0 && y != 4 && y != 5); // opening on the left side of new room

            if is_wall {
                // add wall tile
                let rand: usize = random();
                commands
                    .spawn(( 
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
                    ))
                    .insert(Background);
            } else {
                // add regular tile
                let rand: usize = random();
                commands
                    .spawn(( 
                        SpriteBundle {
                            texture: tile_sheet_handle.clone(),
                            transform: Transform {
                                translation: t,
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: rand % tile_layout_len,
                            layout: tile_layout_handle.clone(),
                        },
                        Tile,
                    ))
                    .insert(Background);
            }

            i += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.); // move to the right for the next tile
        }
    }

    // entrance hallway
    match entrance_side {
        "top" => {
            let hallway_start = t + Vec3::new(
                ROOM_SIZE as f32 / 2.0 * TILE_SIZE as f32, 
                TILE_SIZE as f32, 
                0.0);
            create_hallway(commands, asset_server, texture_atlases, hallway_start);
        }
        "bottom" => {
            let hallway_start = t + Vec3::new(
                ROOM_SIZE as f32 / 2.0 * TILE_SIZE as f32, 
                -(ROOM_SIZE as f32 * TILE_SIZE as f32),
                 0.0);
            create_hallway(commands, asset_server, texture_atlases, hallway_start);
        }
        "left" => {
            let hallway_start = t + Vec3::new(
                -(TILE_SIZE as f32), 
                -(ROOM_SIZE as f32 / 2.0 * TILE_SIZE as f32), 
                0.0);
            create_hallway(commands, asset_server, texture_atlases, hallway_start);
        }
        "right" => {
            let hallway_start = t + Vec3::new(
                ROOM_SIZE as f32 * TILE_SIZE as f32, 
                -(ROOM_SIZE as f32 / 2.0 * TILE_SIZE as f32), 
                0.0);
            create_hallway(commands, asset_server, texture_atlases, hallway_start);
        }
        _ => panic!("Invalid entrance side! Must be one of: 'top', 'bottom', 'left', 'right'."),
    };

     ////// spawning enemy at random point in room ////// 
     let new_rand: usize = random();
     let random_x = start_position.x + random::<f32>() * ((start_position.x + ROOM_SIZE as f32 * TILE_SIZE as f32) - start_position.x);
     let random_y = start_position.y + random::<f32>() * ((start_position.y - (ROOM_SIZE - 1) as f32 * TILE_SIZE as f32) - start_position.y);
     let enemy_position = Vec3::new(random_x, random_y, 1.0);
     spawn_enemy(commands, asset_server, texture_atlases, enemy_position, 1);
}