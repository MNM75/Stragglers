use std::collections::HashSet;

use bevy::prelude::*;
use rand::prelude::*;

const TILE_SIZE: u32 = 144;
const DOOR_SIZE: u32 = 296;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct Wall;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_room);
    }
}

fn create_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures and create texture atlases
    let tile_sheet_handle = asset_server.load("mossTiles.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    commands.spawn((Camera2dBundle::default(),));

    const ROOM_WIDTH: usize = 10; // width in tiles
    const ROOM_HEIGHT: usize = 9; // height in tiles

    let mut grid = vec![vec![false; ROOM_WIDTH]; ROOM_HEIGHT];

    // Randomly choose a starting point
    let start_x = random::<usize>() % ROOM_WIDTH;
    let start_y = random::<usize>() % ROOM_HEIGHT;

    // List of cells making up the maze
    let mut maze: HashSet<(usize, usize)> = HashSet::new();
    maze.insert((start_x, start_y));
    grid[start_y][start_x] = true;

    let mut rng = thread_rng();

    // Get neighbors of a cell
    fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        directions.iter().filter_map(|&(dx, dy)| {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < width && ny < height {
                Some((nx, ny))
            } else {
                None
            }
        }).collect()
    }

    // Wilson's algorithm
    while maze.len() < ROOM_WIDTH * ROOM_HEIGHT {
        let (new_x, new_y) = loop {
            let random_x = rng.gen_range(0..ROOM_WIDTH);
            let random_y = rng.gen_range(0..ROOM_HEIGHT);
            if !maze.contains(&(random_x, random_y)) {
                break (random_x, random_y);
            }
        };

        let mut current_cell = (new_x, new_y);
        let mut path = vec![current_cell];

        // Random walk until we reach a visited cell
        loop {
            let neighbors = get_neighbors(current_cell.0, current_cell.1, ROOM_WIDTH, ROOM_HEIGHT);
            if neighbors.is_empty() {
                break; // This should not happen in a properly connected maze
            }

            current_cell = *neighbors.choose(&mut rng).unwrap();
            path.push(current_cell);

            if maze.contains(&current_cell) {
                break;
            }
        }

        for cell in path {
            maze.insert(cell);
            grid[cell.1][cell.0] = true;
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &is_part_of_maze) in row.iter().enumerate() {
            let position = Vec3::new(
                x as f32 * TILE_SIZE as f32,
                -(y as f32 * TILE_SIZE as f32),
                0.0,
            );

            if is_part_of_maze {
                // Add a tile
                let rand_index: usize = rng.gen_range(0..tile_layout_len);
                commands.spawn((
                    SpriteBundle {
                        texture: tile_sheet_handle.clone(),
                        transform: Transform { translation: position, ..default() },
                        ..default()
                    },
                    TextureAtlas {
                        index: rand_index,
                        layout: tile_layout_handle.clone(),
                    },
                    Tile,
                ));
            } else {
                // Add a wall
                commands.spawn((
                    SpriteBundle {
                        texture: wall_sheet_handle.clone(),
                        transform: Transform { translation: position, ..default() },
                        ..default()
                    },
                    TextureAtlas {
                        index: 0,
                        layout: wall_layout_handle.clone(),
                    },
                    Wall,
                ));
            }
        }
    }
}

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugins(MazePlugin)
        .run();
}
