use bevy::prelude::*;
const TILE_SIZE: u32 = 144;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct Wall;

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
    // Load textures and create texture atlases
    let tile_sheet_handle = asset_server.load("tileProto.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let tile_layout_len = tile_layout.textures.len();
    let tile_layout_handle = texture_atlases.add(tile_layout);

    let wall_sheet_handle = asset_server.load("wall.png");
    let wall_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let wall_layout_len = wall_layout.textures.len();
    let wall_layout_handle = texture_atlases.add(wall_layout);

    commands.spawn((Camera2dBundle::default(),));

    // Starting point is x = -5 tiles, y = -5 tiles (to create an 8x8 room with an additional 1 tile wall)
    let x_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;
    let y_bound = (5. * TILE_SIZE as f32) - (TILE_SIZE as f32) / 2.;

    let mut i = 0;
    let mut y: usize = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);

    // While 10 rows are not filled, apply a tile to each column in a row
    while (y as f32) < (9 as f32) {
        // If current row is filled, move to next row up
        if i == 10 {
            t += Vec3::new(-10.0 * TILE_SIZE as f32, TILE_SIZE as f32, 0.); // Changing the transform value
            i = 0;
            y += 1;
        }

        // While a row has less than 10 tiles, keep adding
        while (i as f32) * (TILE_SIZE as f32) < 10.0 * TILE_SIZE as f32 {
            // Determine if this tile should be a wall
            let is_wall = y == 0 || y == 9 || i == 0 || (i == 9 && y != 4 && y != 5); // Opening in the right wall at y == 4 and y == 5

            if is_wall {
                // Add wall tile
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
                // Add regular tile
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

    const OFFSET: f32 = 500.0; // Change this value to raise/lower the hallway

    // Start the hallway at the opening position (y = 4), raise it higher
    let hallway_start_position = Vec3::new(
        (5.0 * TILE_SIZE as f32) + TILE_SIZE as f32 / 2.0, // Right edge of the room
        -((4.0 * TILE_SIZE as f32) - TILE_SIZE as f32 / 2.0) + OFFSET, // Adjusted y position with an offset
        0.0,
    );

    create_hallway(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        hallway_start_position,
    );
}

fn create_hallway(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    start_position: Vec3,
) {
    let hallway_length = 10; // Length of the hallway
    let mut t = start_position;

    // Load tile texture
    let tile_sheet_handle = asset_server.load("tileProto.png");
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 1, 1, None, None);
    let tile_layout_handle = texture_atlases.add(tile_layout);

    for _ in 0..hallway_length {
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
                index: 0, // You can change this based on your tile layout
                layout: tile_layout_handle.clone(),
            },
            Tile,
        ));
        t += Vec3::new(TILE_SIZE as f32, 0.0, 0.0); // Move to the right for the next tile
    }
}
