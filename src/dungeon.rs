use std::thread::spawn;

use bevy::{gizmos::grid, prelude::*};
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

#[derive(Resource)]
struct MazeGrid {
    grid: Vec<Vec<GridCell>>,
}

impl MazeGrid {
    fn new(rows: usize, cols: usize) -> Self {
        let grid = create_grid(rows, cols);
        MazeGrid { grid }
    }
}

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MazeGrid::new(GRID_WIDTH, GRID_HEIGHT)) // Add the grid as a resource
            .add_systems(Startup, create_dungeon);
    }
}

fn create_dungeon( //main function that calls all other spawining functions
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut maze_grid: ResMut<MazeGrid>,

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
        11.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        15.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand2: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        4,
        room2_start_position,
        (rand2%2 +1) as u32,
    );

    let room3_start_position = Vec3::new(
        23.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        15.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand3: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        4,
        room3_start_position,
        (rand3%2 +1) as u32,
    );

    let room4_start_position = Vec3::new(
        31.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        15.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand4: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        4,
        room4_start_position,
        (rand4%2 +1) as u32,
    );

    let room5_start_position = Vec3::new(
        11.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -23.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand5: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        2,
        room5_start_position,
        (rand5%2 +1) as u32,
    );

    let room6_start_position = Vec3::new(
        19.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -23.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand6: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        2,
        room6_start_position,
        (rand6%2 +1) as u32,
    );

    let room7_start_position = Vec3::new(
        31.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -23.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand7: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        2,
        room7_start_position,
        (rand7%2 +1) as u32,
    );

    let room8_start_position = Vec3::new(
        40.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    let rand8: usize = random();
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        1,
        room8_start_position,
        (rand8%2 +1) as u32,
    );
    //boss room
    let room9_start_position = Vec3::new(
        40.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -10.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        5,
        room9_start_position,
        2, //CHANGE TO BOSS TYPE
    );
    //end room
    let room10_start_position = Vec3::new(
        51.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -10.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_room(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        1,
        room10_start_position,
        0,
    );

    let hallway1_start_position = Vec3::new(
        3.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        2.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    //hallways
    spawn_hallway(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        hallway1_start_position,
    );

    let hallway2_start_position = Vec3::new(
        46.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -6.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );
    spawn_hallway(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        hallway2_start_position,
    );
    //maze
    let maze1_start_position = Vec3::new(
        8.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        -17.0 * TILE_SIZE as f32 - TILE_SIZE as f32/2.0, 
        0.0,
    );

    let final_room_center = room10_start_position + Vec3::new(
        (6.0 * TILE_SIZE as f32) / 2.0, 
        (6.0 * TILE_SIZE as f32) / 2.0, 
        10.0,
    );
    spawn_door(&mut commands, &asset_server, &mut texture_atlases, final_room_center);
    generate_maze(maze_grid, commands, asset_server, texture_atlases, maze1_start_position);


  
    
        
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
            }else if(door_position == 5){
                is_wall = y == 0 || y == 5 || (i == 0 && y != 2 && y != 3) || (i == 5 && y != 2 && y != 3);
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



/// Wilson's Algorithm Functions ///
fn create_grid(rows: usize, cols: usize) -> Vec<Vec<GridCell>> {
    let mut grid = Vec::with_capacity(rows);

    for _ in 0..rows {
        let row = vec![GridCell::new(Cell::Wall); cols]; // Start with all walls
        grid.push(row);
    }

    grid
}

// Mark a cell as a wall
fn mark_wall(grid: &mut Vec<Vec<GridCell>>, row: usize, col: usize) {
    if row < grid.len() && col < grid[row].len() {
        grid[row][col].cell_type = Cell::Wall;
    }
}

// Mark a cell as visited and as a wall
fn add_to_UST(grid: &mut Vec<Vec<GridCell>>, row: usize, col: usize) {
    if row < grid.len() && col < grid[row].len() {
        // Mark the cell as visited
        grid[row][col].marked = true;
    }
}

// Check if a cell is visited
fn in_UST(grid: &Vec<Vec<GridCell>>, row: usize, col: usize) -> bool {
    if row < grid.len() && col < grid[row].len() {
        return grid[row][col].marked;
    }
    false
}

// Return a randomly selected cell that's unvisited (Not in UST)
fn get_random_unvisited_cell(grid: &Vec<Vec<GridCell>>) -> Option<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut unvisited_cells: Vec<(usize, usize)> = Vec::new();

    // Collect all unvisited cells
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if !in_UST(grid, row, col) {
                unvisited_cells.push((row, col));
            }
        }
    }

    // If there are unvisited cells, return one at random
    if !unvisited_cells.is_empty() {
        let rand_index = rng.gen_range(0..unvisited_cells.len());
        Some(unvisited_cells[rand_index])
    } else {
        None
    }
}

// Function to get the next randomly selected cell based on the direction
fn get_next_cell(row: usize, col: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row.wrapping_sub(1), col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col.wrapping_sub(1)),
        Direction::Right => (row, col + 1),
    }
}

// Assign a cell with a path direction
fn mark_with_direction(
    grid: &mut Vec<Vec<GridCell>>,
    row: usize,
    col: usize,
    direction: Direction,
) {
    if row < grid.len() && col < grid[row].len() {
        // Mark the cell as part of the path with direction
        grid[row][col].cell_type = Cell::Path(direction);
        grid[row][col].direction = Some(direction); // Also set the direction explicitly
    }
}

// Function to randomly pick one of the four directions
fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();
    let direction_index: usize = rng.gen_range(0..4); // Generate a random index (0 to 3)

    match direction_index {
        // 0, 1, 2, or 3
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => Direction::Right,
    }
}

// Function to check if a cell is within the grid bounds
fn is_within_bounds(grid: &Vec<Vec<GridCell>>, row: usize, col: usize) -> bool {
    row < grid.len() && col < grid[row].len()
}

// Create a path from a given cell that connects to UST
fn create_path(grid: &mut Vec<Vec<GridCell>>, row: usize, col: usize) {
    let mut current_row = row;
    let mut current_col = col;

    // If the current cell is part of the UST, stop processing
    if in_UST(&grid, row, col) {
        println!("Found a path at ({}, {})", current_row, current_col);
        return; // Stop if we find a Tile
    }

    // Assuming `current_row` and `current_col` are the current cell coordinates

    let mut direction = random_direction(); // Start with a random direction

    loop {
        // Get the new cell coordinates based on the current position and direction
        let (new_row, new_col) = get_next_cell(current_row, current_col, direction);

        // Check if the new cell is within bounds
        if is_within_bounds(grid, new_row, new_col) {
            // Mark the current cell with the chosen direction
            mark_with_direction(grid, current_row, current_col, direction);

            // Recursively move to the new cell
            create_path(grid, new_row, new_col);

            add_to_UST(grid, current_row, current_col); // Add current cell to UST
            break;
        } else {
            // If out of bounds, pick a new random direction and try again
            direction = random_direction();
        }
    }
}

fn print_grid(grid: &Vec<Vec<GridCell>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            match &cell.cell_type {
                Cell::Tile => print!("T "), // Tile is represented by 'T'
                Cell::Wall => print!("W "), // Wall is represented by 'W'
                Cell::Path(direction) => {
                    // Print the direction with its arrow
                    match direction {
                        Direction::Up => print!("↑ "),    // Path going up
                        Direction::Down => print!("↓ "),  // Path going down
                        Direction::Left => print!("← "),  // Path going left
                        Direction::Right => print!("→ "), // Path going right
                    }
                }
            }
        }
        println!(); // Move to the next line after each row
    }
}

////////////


fn generate_maze(
    mut maze_grid: ResMut<MazeGrid>,  
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    start_position: Vec3,
) {
    /////////////////////  Generate a maze blueprint using Wilson's Algo /////////////////////

    let mut grid = &mut maze_grid.grid; // Use the grid from the resource

    // Randomly select a cell
    let mut rng = rand::thread_rng();
    let random_row = rng.gen_range(0..GRID_HEIGHT);
    let random_col = rng.gen_range(0..GRID_WIDTH);

    // Mark the randomly selected cell as visited (mutable borrow)
    add_to_UST(&mut grid, random_row, random_col);
    grid[random_row][random_col].cell_type = Cell::Tile; // Mark it as a Tile

    // Now that the cell is marked, we can access it immutably
    println!(
        "First randomly selected and marked cell at ({}, {}): {:?}",
        random_row, random_col, grid[random_row][random_col]
    );

    // Continue finding and visiting random unvisited cells until all cells are visited
    while let Some((row, col)) = get_random_unvisited_cell(&grid) {
        println!("Randomly selected new unvisited cell at ({}, {})", row, col);

        let cell = &grid[row][col]; // Access the cell in the grid
        if let Cell::Tile = cell.cell_type {
            // Skip cells part of UST (Tile)
            println!("The selected cell is a Tile at ({}, {})", row, col);
            continue;
        } else {
            if is_within_bounds(&mut grid, row, col) {
                // Only execute this block if the cell is a Wall
                println!("The selected cell is a Wall at ({}, {})", row, col);

                // Call `create_path` to explore from this unvisited cell
                create_path(&mut grid, row, col);
            }
        }
    }
    print_grid(&grid);

    let actual_grid = blueprint_to_grid(&mut commands, &asset_server, &mut texture_atlases, &grid);
    let doubled_grid = double_grid(&mut commands, &asset_server, &mut texture_atlases, &actual_grid);
   // print_grid(&doubled_grid);
    
    spawn_maze(&mut commands, &asset_server, &mut texture_atlases, &doubled_grid, start_position);
}

// function that takes maze blueprint as input and returns a spawned maze as output
fn spawn_maze(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: &Vec<Vec<GridCell>>,
    start_position: Vec3,
) {
    let tile_sheet_handle = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);


    ///////////////////// spawning the maze /////////////////////
    const MAZE_WIDTH: usize = GRID_WIDTH + 1; // Include more rows for walls
    const MAZE_HEIGHT: usize = GRID_HEIGHT + 1; // Include more columns for walls

    let x_bound = start_position.x;
    let y_bound = start_position.y;

    let mut t = Vec3::new(x_bound, y_bound, 0.);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut is_wall = false;
            //creating doors
            if(y == 18 && x == 0 || y == 17 && x == 0 //start room door
            || y == 31 && x == 5 || y == 31 && x == 6 //room 2 door
            || y == 31 && x == 17 || y == 31 && x == 18
            || y == 31 && x == 25 || y == 31 && x == 26
            || y == 0 && x == 5 || y == 0 && x == 6 
            || y == 0 && x == 13 || y == 0 && x == 14
            || y == 0 && x == 25 || y == 0 && x == 26
            || y == 21 && x == 31 || y == 22 && x == 31
            || y == 9 && x == 31 || y == 10 && x == 31 ){ //boss room\
                is_wall = false;
            } else{
                is_wall = matches!(cell.cell_type, Cell::Wall);
            }

            if is_wall {
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
                            index: (x + y) % wall_layout_len,
                            layout: wall_layout_handle.clone(),
                        },
                        Wall,
                    ))
                    .insert(Background);
            } else {
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
            
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
        }
        t += Vec3::new(
            -(grid[0].len() as f32) * TILE_SIZE as f32,
            TILE_SIZE as f32,
            0.,
        );
    }
}

// takes correct maze paths blueprint and returns a maze grid (arrows to walls and tiles)
fn blueprint_to_grid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: &Vec<Vec<GridCell>>,
) -> Vec<Vec<GridCell>> {
    // expand each GridCell into a 3x3 block in the 'actual' grid
    let rows = grid.len();
    let cols = grid[0].len();

    // formula is 2x + 1 since the 3x3 "blocks" should overlap
    let row_actual = rows * 2 + 1;
    let cols_actual = cols * 2 + 1;

    let mut grid_actual = create_grid(row_actual, cols_actual);
    let mut tile_count = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            // match cell to cell in actual grid
            let r_actual = r * 2 + 1;
            let c_actual = c * 2 + 1;

            match &cell.cell_type {
                Cell::Path(direction) => {
                    // open cell in actual grid
                    grid_actual[r_actual][c_actual] = GridCell::new(Cell::Tile);
                    tile_count += 1;

                    // other cells in the 3x3 are opened based on direction
                    // the cell the path points towards should be an open tile

                    match direction {
                        Direction::Up => {
                            grid_actual[r_actual - 1][c_actual] = GridCell::new(Cell::Tile);
                            tile_count += 1;
                        },    // Path going up
                        Direction::Down => {
                            grid_actual[r_actual + 1][c_actual] = GridCell::new(Cell::Tile);
                            tile_count += 1;
                        },  // Path going down
                        Direction::Left => {
                            grid_actual[r_actual][c_actual - 1] = GridCell::new(Cell::Tile);
                            tile_count += 1;
                        },  // Path going left
                        Direction::Right => {
                            grid_actual[r_actual][c_actual + 1] = GridCell::new(Cell::Tile);
                            tile_count += 1;
                        }, // Path going right
                    }
                }
                Cell::Tile => {
                    // open cell in actual grid
                    grid_actual[r_actual][c_actual] = GridCell::new(Cell::Tile);
                    tile_count += 1;
                }
                _ => {}
            }
            
        }
    }
    grid_actual
}

// scales hallways to be 2x
fn double_grid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: &Vec<Vec<GridCell>>,
) -> Vec<Vec<GridCell>> {
    // expand each GridCell into a 2x2 block in the scaled-up version
    let rows = grid.len();
    let cols = grid[0].len();

    let row_actual = rows * 2;
    let cols_actual = cols * 2;

    let mut grid_actual = Vec::new();

    for row in grid.iter() {
        let mut row1 = vec![];
        let mut row2 = vec![];

        for cell in row {
            row1.push(cell.clone());
            row1.push(cell.clone());

            row2.push(cell.clone());
            row2.push(cell.clone());
        }
        grid_actual.push(row1);
        grid_actual.push(row2);
    }

    // make exterior walls thinner
    for row in &mut grid_actual {
        row.remove(0);
        row.pop();
    }
    grid_actual.remove(0);
    grid_actual.pop();

    grid_actual
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