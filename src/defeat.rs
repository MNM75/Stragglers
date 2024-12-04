use bevy::prelude::*;
use crate::GameState;
use crate::player::PlayerStats;
use crate::player::Player;
use crate::WIN_W;
use crate::WIN_H;
use crate::player::LEVEL_W;
use crate::player::LEVEL_H;

pub struct DefeatScreenPlugin;

#[derive(Component)]
struct DefeatScreenElement;

impl Plugin for DefeatScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DefeatScreen), spawn_defeat_screen);
        app.add_systems(OnExit(GameState::DefeatScreen), despawn_defeat_screen);
        app.add_systems(Update, handle_respawn_button.run_if(in_state(GameState::DefeatScreen)));
    }
}

fn spawn_defeat_screen(
    mut commands: Commands, asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>
) {

    let pt = player.single();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;

    let x_player = pt.translation.x.clamp(-x_bound, x_bound);   // same logic as camera/player movement
    let y_player = pt.translation.y.clamp(-y_bound, y_bound);   // same logic as camera/player movement
    let z_player = pt.translation.z;

    commands.spawn((
        DefeatScreenElement,
        SpriteBundle {
        texture: asset_server.load("defeat.png"),
        transform: Transform::from_xyz(x_player, y_player, z_player+1.0),
        ..default()
        }
    ));

    // Spawn the Respawn button
    commands.spawn((
        DefeatScreenElement,
        ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        }))
        .with_children(|parent| {
            parent.spawn((
                DefeatScreenElement,
                TextBundle {
                text: Text::from_section(
                    "Respawn",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            }));
        });
}

fn handle_respawn_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut player_stat_query: Query<&mut PlayerStats, With<Player>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
                    player_stats.hp = player_stats.max_hp;
                }
                
                if let Ok(mut transform) = player.get_single_mut() {
                    transform.translation.x = 0.0;
                    transform.translation.y = 0.0;
                }
                // Transition back to gameplay
                state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(136.0, 136.0, 136.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(255.0, 112.0, 112.0));
            }
        }
    }
}

fn despawn_defeat_screen(mut commands: Commands, query: Query<Entity, With<DefeatScreenElement>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}