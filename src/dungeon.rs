use bevy::prelude::*;
use rand::prelude::*;
use crate::enemy::spawn_enemy;
const TILE_SIZE: u32 = 144;
const DOOR_SIZE: u32 = 296;
const GRID_WIDTH: usize = 8; // Width of the grid
const GRID_HEIGHT: usize = 8; // Height of the grid

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Door;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Wall,
    Tile,
    Path(Direction), // This stores the direction when it's part of the path
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, Debug)]
struct GridCell {
    cell_type: Cell,
    marked: bool,
    direction: Option<Direction>, // New field to store direction for Tiles
}

impl GridCell {
    fn new(cell_type: Cell) -> Self {
        GridCell {
            cell_type,
            direction: None, // No direction by default
            marked: false,   // Default unmarked
        }
    }
}

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_dungeon);
    }
}

fn create_dungeon( //main function that calls all other spawining functions
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    commands.spawn((Camera2dBundle::default(),));

    //starting room
    let room1_start_position = Vec3::new(
        -2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        3,
        room1_start_position,
        0,
    );

    //battle rooms
    let room2_start_position = Vec3::new(
        8.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        1,
        room2_start_position,
        2,
    );

    let hallway1_start_position = Vec3::new(
        3.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_hallway(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        hallway1_start_position,
    );

    let maze1_start_position = Vec3::new(
        3.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        16.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );

    spawn_maze(

    );
    
        
}

fn spawn_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>, 
    door_position: usize, //1 = left door, 2 = top door, 3 = right door, 4 = bottom door
    start_position: Vec3,
    enemy: u32,

){
    let tile_sheet_handle: Handle<Image> = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle: Handle<Image> = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    let x_bound = start_position.x;
    let y_bound = start_position.y;

    let mut i = 0;
    let mut y: usize = 0;
    let mut t = Vec3::new(x_bound, y_bound, 0.);


    //while 6 rows are not filled, apply a tile to each column in a row
    while (y as f32) < (5 as f32) {
        //if current row is filled, move to next row up
        if i == 6 {
           t += Vec3::new(-6.0 * TILE_SIZE as f32, TILE_SIZE as f32, 0.); // Changing the transform value
           i = 0;
           y += 1;
       }

       // while a row has less than 6 tiles, keep adding
       while (i as f32) * (TILE_SIZE as f32) < 6.0 * TILE_SIZE as f32 {
            //determine if this tile should be a wall
            let mut is_wall = false;
            if(door_position == 1) {
                is_wall = y == 0 || y == 5 || i == 5 || (i == 0 && y != 2 && y != 3); // opening in the left wall
            }else if(door_position == 2) {
                is_wall = y == 0 || i == 5 || i == 0 || (y == 5 && i != 2 && i != 3); // opening in the top wall
            }else if(door_position == 3) {
                is_wall = y == 0 || y == 5 || i == 0 || (i == 5 && y != 2 && y != 3); // opening in the right wall
            }else if(door_position == 4) {
                is_wall = i == 0 || y == 5 || i == 5 || (y == 0 && i != 2 && i != 3); // opening in the bottom wall
            }
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
   ////// spawning enemy at a point in room ////// 
   if(enemy != 0){
    let new_rand: usize = random();
    let random_x = start_position.x + 2.0 * TILE_SIZE as f32;
    let random_y = start_position.y + 2.0 * TILE_SIZE as f32;
    
    let enemy_position = Vec3::new(random_x, random_y, 1.0);
    spawn_enemy(commands, asset_server, texture_atlases, enemy_position, enemy);
   }
   


}

fn spawn_hallway(
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

    Vec3::new(start_position.x + (HALLWAY_COLUMNS as f32 * TILE_SIZE as f32), t.y, 0.0)
}

fn spawn_maze(

){

}
