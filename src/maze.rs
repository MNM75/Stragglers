use bevy::prelude::*;
use rand::prelude::*;

const TILE_SIZE: u32 = 144;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct Wall;

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

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_room);
    }
}

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

const GRID_WIDTH: usize = 9; // Width of the grid
const GRID_HEIGHT: usize = 9; // Height of the grid

fn create_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    ///////////creating an 8x8 tile background (centered at window origin), with wall//////////
    /*
       let tile_sheet_handle = asset_server.load("mossTiles.png");
       let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
       let tile_layout_len = tile_layout.textures.len();
       let tile_layout_handle = texture_atlases.add(tile_layout);

       let wall_sheet_handle = asset_server.load("wall.png");
       let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
       let wall_layout_len = wall_layout.textures.len();
       let wall_layout_handle = texture_atlases.add(wall_layout);

       commands.spawn((Camera2dBundle::default(),));
    */
    //starting point is x = -5 tiles, y = -5 tiles (to create an 8x8 room with an additional 1 tile wall)
    let x_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut i = 0;
    let mut y: usize = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);

    ///////////////////// Attempt at Wilson's Algo /////////////////////

    let mut grid = create_grid(GRID_WIDTH, GRID_HEIGHT);

    // Randomly select a cell
    let mut rng = rand::thread_rng();
    let random_row = rng.gen_range(0..GRID_HEIGHT);
    let random_col = rng.gen_range(0..GRID_WIDTH);

    // Now we have a randomly selected cell
    let random_cell = &grid[random_row][random_col];

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
    ///////////////////////////////////////////////////////////////////////

    /*
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
    */
    // Print the grid for debugging
    print_grid(&grid);

    // Call spawn_maze with the generated grid
    spawn_maze(&mut commands, &asset_server, &mut texture_atlases, &grid);
}

// function that takes maze blueprint as input and returns a spawned maze as output
fn spawn_maze(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: &Vec<Vec<GridCell>>,
) {
    let tile_sheet_handle = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    commands.spawn((Camera2dBundle::default(),));

    ///////////////////// Attempt at spawning the maze /////////////////////
    const MAZE_WIDTH: usize = GRID_WIDTH + 1; // Include more rows for walls
    const MAZE_HEIGHT: usize = GRID_HEIGHT + 1; // Include more columns for walls

    let x_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut t = Vec3::new(-x_bound, -y_bound, 0.);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let is_wall = matches!(cell.cell_type, Cell::Wall);

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
