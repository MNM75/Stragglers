use bevy::{
    color::palettes::css::BLACK,
    prelude::*
};

use crate::GameState;
use crate::player::PlayerStats;
use crate::player::Player;
use crate::player::init_player;
use crate::WIN_W;
use crate::WIN_H;
use crate::player::LEVEL_W;
use crate::player::LEVEL_H;

#[derive(Component)]
struct SkillTreeUIBackground;
#[derive(Component)]
struct SkillTreeUIComponent;

#[derive(Component)]
struct StatText {
    stat_type: StatType,
}

enum StatType {
    Attack,
    Magic,
    Speed,
    MaxHp,
    Hp,
    SkillPoints,
    AbilityPoints,
    Strength,
    Mgk,
    Agility,
    Health,
}

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_skill_tree_ui.after(init_player));
        app.add_systems(PostStartup, hide_skill_tree_ui);
        app.add_systems(Update, toggle_skill_tree_ui);
        app.add_systems(Update, update_skill_tree_ui);
        app.add_systems(Update, spend_ability_point.run_if(in_state(GameState::SkillTreeMenu)));
        app.add_systems(Update, spend_skill_point.run_if(in_state(GameState::SkillTreeMenu)));
        app.add_systems(OnEnter(GameState::SkillTreeMenu), show_skill_tree_ui);
        app.add_systems(OnExit(GameState::SkillTreeMenu), hide_skill_tree_ui);
    }
}

fn load_skill_tree_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&PlayerStats, With<Player>>,
) {
    // Retrieve player stats
    if let Ok(player_stats) = player_query.get_single() {
        // bring in asset for skill tree ui
        commands.spawn((
            SkillTreeUIBackground,
            SkillTreeUIComponent,
            SpriteBundle {
            texture: asset_server.load("skillTreeUI.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            }
        ));

        // Ability Points
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::AbilityPoints },
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(30.0),
                left: Val::Px(225.0),
                ..default()
            }),
            
        ));

        // Skill Points
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::SkillPoints },
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                left: Val::Px(225.0),
                ..default()
            })
        ));

        // Ability Scores ------------------------------------------------------------
        // Strength
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.strength.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(215.0),
                left: Val::Px(220.0),
                ..default()
            })
        ));
        // Magic
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.mgk.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(255.0),
                left: Val::Px(220.0),
                ..default()
            })
        ));
        // Agility
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.agility.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(295.0),
                left: Val::Px(220.0),
                ..default()
            })
        ));
        // Health
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.health.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(335.0),
                left: Val::Px(220.0),
                ..default()
            })
        ));

        // Player Stats ------------------------------------------------------------
        // hp
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::Hp },
            TextBundle::from_section(
                format!("{}/{}", player_stats.hp, player_stats.max_hp),  // Dynamic current/max HP
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(185.0),
                left: Val::Px(80.0),
                ..default()
            })
        ));
        // atk
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.attack.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(135.0),
                left: Val::Px(105.0),
                ..default()
            })
        ));
        // def
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(85.0),
                left: Val::Px(105.0),
                ..default()
            })
        ));
        // spd
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.speed.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(185.0),
                left: Val::Px(225.0),
                ..default()
            })
        ));
        // matk
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                player_stats.magic.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(135.0),
                left: Val::Px(225.0),
                ..default()
            })
        ));
        // mdef
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(BLACK),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(85.0),
                left: Val::Px(225.0),
                ..default()
            }),
        ));
    }
}

fn show_skill_tree_ui(
    mut commands: Commands,
    query: Query<Entity, With<SkillTreeUIComponent>>,
    mut background: Query<&mut Transform, With<SkillTreeUIBackground>>,
    player: Query<&Transform, (With<Player>, Without<SkillTreeUIBackground>)>,) // an &Transform with <Player> would not have <SkillTreeUIBackground> applied by user logic, but the Without<T> is included to not cause a panic and crash the game
{
    // makes the skill tree UI visible
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }

    // centers the skill tree on the center of the screen (based on player location)
    let pt = player.single();
    let mut bt = background.single_mut();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;

    bt.translation.x = pt.translation.x.clamp(-x_bound, x_bound);   // same logic as camera/player movement
    bt.translation.y = pt.translation.y.clamp(-y_bound, y_bound);   // same logic as camera/player movement
    bt.translation.z = pt.translation.z + 1.;
}

fn hide_skill_tree_ui(
    mut commands: Commands,
    query: Query<Entity, With<SkillTreeUIComponent>>)
{
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn toggle_skill_tree_ui(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
        if input.just_pressed(KeyCode::KeyQ) {
            match state.get() {
                GameState::InGame => next_state.set(GameState::SkillTreeMenu),
                GameState::SkillTreeMenu => next_state.set(GameState::InGame),
                GameState::BattleMode => next_state.set(GameState::BattleMode),
            }
        }
}

fn update_skill_tree_ui(
    player_query: Query<&PlayerStats, With<Player>>,
    mut stat_query: Query<(&StatText, &mut Text), With<SkillTreeUIComponent>>,
) {
    if let Some(player_stats) = player_query.iter().next() {
        for (stat_text, mut text) in stat_query.iter_mut() {
            match stat_text.stat_type {
                StatType::Attack => {
                    text.sections[0].value = player_stats.attack.to_string();
                }
                StatType::Magic => {
                    text.sections[0].value = player_stats.magic.to_string();
                }
                StatType::Speed => {
                    text.sections[0].value = player_stats.speed.to_string();
                }
                StatType::MaxHp => {
                    text.sections[0].value = player_stats.max_hp.to_string();
                }
                StatType::Hp => {
                    text.sections[0].value = format!("{}/{}", player_stats.hp, player_stats.max_hp);
                }
                StatType::SkillPoints => {
                    text.sections[0].value = player_stats.skill_points.to_string();
                }
                StatType::AbilityPoints => {
                    text.sections[0].value = player_stats.ability_points.to_string();
                }
                StatType::Strength => {
                    text.sections[0].value = player_stats.attack.to_string();
                }
                StatType::Mgk => {
                    text.sections[0].value = player_stats.attack.to_string();
                }
                StatType::Agility => {
                    text.sections[0].value = player_stats.attack.to_string();
                }
                StatType::Health => {
                    text.sections[0].value = player_stats.attack.to_string();
                }
            }
        }
    }
}

fn spend_ability_point(
    input: Res<ButtonInput<KeyCode>>, // Directly read input for key presses
    mut player_query: Query<&mut PlayerStats, With<Player>>,
) {
    if let Ok(mut player_stats) = player_query.get_single_mut() {
        // Check if there are ability points available to spend
        if player_stats.ability_points > 0 {
            // Check for key presses and upgrade the appropriate stat
            if input.just_pressed(KeyCode::KeyF) { //press f to upgrade respect *attack
                player_stats.attack += 1;
            } else if input.just_pressed(KeyCode::KeyG) { //g to upgrade magic
                player_stats.magic += 1;
            } else if input.just_pressed(KeyCode::KeyH) { //h to upgrade speed
                player_stats.speed += 1;
            } else if input.just_pressed(KeyCode::KeyJ) { //j to upgrade max_hp all temporary for now
                player_stats.max_hp += 10;
            } else {
                return; // No valid key pressed, exit early
            }
            // Deduct an ability point for every successful upgrade
            player_stats.ability_points -= 1;
        }
    }
}

fn spend_skill_point(
    input: Res<ButtonInput<KeyCode>>, // Directly read input for key presses
    mut player_query: Query<&mut PlayerStats, With<Player>>,
) {
    if let Ok(mut player_stats) = player_query.get_single_mut() {
        // Check if there are skill points available to spend
        if player_stats.skill_points > 0 {
            // Check for key presses and upgrade the appropriate stat
            if input.just_pressed(KeyCode::KeyR) {
                player_stats.strength += 1;
            } else if input.just_pressed(KeyCode::KeyT) {
                player_stats.mgk += 1;
            } else if input.just_pressed(KeyCode::KeyY) {
                player_stats.agility += 1;
            } else if input.just_pressed(KeyCode::KeyU) {
                player_stats.health += 10;
            } else {
                return; // No valid key pressed, exit early
            }
            // Deduct an ability point for every successful upgrade
            player_stats.skill_points -= 1;
        }
    }
}
