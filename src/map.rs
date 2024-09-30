use bevy::prelude::*;
const TILE_SIZE: u32 = 144;


#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Background;


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
}