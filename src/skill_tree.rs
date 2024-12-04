use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*
};

use crate::GameState;
use crate::player::{PlayerStats, BonusStats, Player, init_player};
use crate::{WIN_W, WIN_H};
use crate::player::{LEVEL_W, LEVEL_H};

#[derive(Component)]
struct SkillTreeUIBackground;
#[derive(Component)]
struct SkillTreeUISkeleton;
#[derive(Component)]
struct SkillTreeUIDetails;
#[derive(Component)]
struct SkillTreeUINode {
    unlocked: bool,
    index: u32,
}
#[derive(Component)]
struct SkillTreeUIComponent;

#[derive(Component)]
struct StatText {
    stat_type: StatType,
}

enum StatType {
    Atk,
    Def,
    Matk,
    Mdef,
    Spd,
    MaxHp,
    Hp,

    SkillPoints,
    AbilityPoints,

    Strength,
    Magic,
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
        app.add_systems(Update, unlock_skill_tree_nodes.run_if(in_state(GameState::SkillTreeMenu)));
        app.add_systems(Update, spend_ability_point.run_if(in_state(GameState::SkillTreeMenu)));
        app.add_systems(OnEnter(GameState::SkillTreeMenu), show_skill_tree_ui);
        app.add_systems(OnExit(GameState::SkillTreeMenu), hide_skill_tree_ui);
    }
}

fn load_skill_tree_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
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

        // skill tree skeleton
        commands.spawn((
            SkillTreeUISkeleton,
            SkillTreeUIComponent,
            SpriteBundle {
            texture: asset_server.load("skillTreeSkeleton.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            }
        ));

        // skill tree node details
        commands.spawn((
            SkillTreeUIDetails,
            SkillTreeUIComponent,
            SpriteBundle {
            texture: asset_server.load("skillTreeDetails.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            }
        ));

        // skill tree nodes, indexed 0 - 17
        let node_sheet_handle = asset_server.load("skillTreeNodeSheet.png");
        let node_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 2, 1, None, None);
        let node_layout_handle = texture_atlases.add(node_layout);

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 0,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 1,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 2,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 3,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 4,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 5,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 6,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 7,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 8,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 9,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 10,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 11,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 12,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 13,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 14,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 15,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 16,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        commands.spawn((
            SkillTreeUINode {
                unlocked: false,
                index: 17,
            },
            SkillTreeUIComponent,
            SpriteBundle {
            texture: node_sheet_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
            },
            TextureAtlas {
                layout: node_layout_handle.clone(),
                index: 0,
            },
        ));

        // Ability Points
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::AbilityPoints },
            TextBundle::from_section(
                player_stats.ability_points.to_string(),
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
                player_stats.skill_points.to_string(),
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
            StatText { stat_type: StatType::Strength },
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
            StatText { stat_type: StatType::Magic },
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
                top: Val::Px(255.0),
                left: Val::Px(220.0),
                ..default()
            })
        ));
        // Agility
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::Agility },
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
            StatText { stat_type: StatType::Health },
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
            StatText { stat_type: StatType::Atk },
            TextBundle::from_section(
                player_stats.atk.to_string(),
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
        // phys def
        commands.spawn((
            SkillTreeUIComponent,
            StatText { stat_type: StatType::Def },
            TextBundle::from_section(
                player_stats.def.to_string(),
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
            StatText { stat_type: StatType::Spd },
            TextBundle::from_section(
                player_stats.spd.to_string(),
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
            StatText { stat_type: StatType::Matk },
            TextBundle::from_section(
                player_stats.matk.to_string(),
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
            StatText { stat_type: StatType::Mdef },
            TextBundle::from_section(
                player_stats.mdef.to_string(),
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

        // exit text  ------------------------------------------------------------
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                "Press 'Q' to exit",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(WHITE),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            })
        ));

        // temporary descriptive text  ------------------------------------------------------------
        commands.spawn((
            SkillTreeUIComponent,
            TextBundle::from_section(
                "Press 'F, G, H, or J' to upgrade strength, magic, agility, or health\nup to a base total of 7\nClick on a node to unlock bonus stats",
                TextStyle {
                    font_size: 20.0,
                    color: bevy::prelude::Color::Srgba(WHITE),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            })
        ));
    }
}

fn show_skill_tree_ui(
    mut commands: Commands,
    query: Query<Entity, With<SkillTreeUIComponent>>,
    mut skeleton: Query<&mut Transform, (With<SkillTreeUISkeleton>, Without<SkillTreeUIBackground>, Without<SkillTreeUIDetails>)>,
    mut details: Query<&mut Transform, (With<SkillTreeUIDetails>, Without<SkillTreeUIBackground>, Without<SkillTreeUISkeleton>)>,
    mut background: Query<&mut Transform, (With<SkillTreeUIBackground>, Without<SkillTreeUISkeleton>, Without<SkillTreeUIDetails>)>,
    mut nodes: Query<&mut Transform, (With<SkillTreeUINode>, Without<SkillTreeUIBackground>, Without<SkillTreeUISkeleton>, Without<SkillTreeUIDetails>)>,
    player: Query<&Transform, (With<Player>, Without<SkillTreeUIBackground>, Without<SkillTreeUISkeleton>, Without<SkillTreeUINode>, Without<SkillTreeUIDetails>)>,)
    // an &Transform with <...> would not have <SkillTreeUI...> applied by user logic, but the Without<T> is included to not cause a panic and crash the game
{
    // makes the skill tree UI visible
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }

    // centers the skill tree on the center of the screen (based on player location)
    let pt = player.single();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;

    let x_player = pt.translation.x.clamp(-x_bound, x_bound);   // same logic as camera/player movement
    let y_player = pt.translation.y.clamp(-y_bound, y_bound);   // same logic as camera/player movement
    let z_player = pt.translation.z;

    let mut bt = background.single_mut();
    let mut st = skeleton.single_mut();
    let mut dt = details.single_mut();

    bt.translation.x = x_player;
    bt.translation.y = y_player;
    bt.translation.z = z_player + 1.;

    st.translation.x = x_player + 120.;
    st.translation.y = y_player;
    st.translation.z = bt.translation.z + 1.;

    dt.translation.x = x_player + 120.;
    dt.translation.y = y_player;
    dt.translation.z = st.translation.z + 2.;

    // loads the nodes...
    let mut i = 0;
    let mut tier1 = true;
    let mut tier2 = false;
    let mut tier3 = false;

    for mut nt in nodes.iter_mut() {
        if tier1 == true {
            nt.translation.x = st.translation.x - 410. + (i as f32)%3. * 120.;
            nt.translation.y = st.translation.y;
            nt.translation.z = st.translation.z + 1.;
            i += 1;
            if i > 2 {
                tier1 = false;
                tier2 = true;
            }
        }
        else if tier2 == true {
            if 2 < i && i <= 5 {
                nt.translation.x = st.translation.x - 112.5 + (i as f32)%3. * 120.;
                nt.translation.y = st.translation.y + 86.;
                nt.translation.z = st.translation.z + 1.;
                i += 1;
            }
            else if i > 5 {
                nt.translation.x = st.translation.x - 112.5 + (i as f32)%3. * 120.;
                nt.translation.y = st.translation.y - 86.;
                nt.translation.z = st.translation.z + 1.;
                i += 1;
                if i > 8 {
                    tier2 = false;
                    tier3 = true;
                }
            }
        }
        else if tier3 == true {
            if 8 < i && i <= 11 {
                nt.translation.x = st.translation.x + 185. + (i as f32)%3. * 120.;
                nt.translation.y = st.translation.y + 172.;
                nt.translation.z = st.translation.z + 1.;
                i += 1;
            }
            else if 11 < i && i <= 14 {
                nt.translation.x = st.translation.x + 185. + (i as f32)%3. * 120.;
                nt.translation.y = st.translation.y;
                nt.translation.z = st.translation.z + 1.;
                i += 1;
            }
            else if i > 14 {
                nt.translation.x = st.translation.x + 190. + (i as f32)%3. * 120.;
                nt.translation.y = st.translation.y - 172.;
                nt.translation.z = st.translation.z + 1.;
                i += 1;
                if i > 17 {
                    tier3 = false;
                }
            }
        }
    }
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
                GameState::EndCredits => next_state.set(GameState::EndCredits),
            }
        }
}

fn update_skill_tree_ui(
    mut player_query: Query<&mut PlayerStats, With<Player>>,
    mut stat_query: Query<(&StatText, &mut Text), With<SkillTreeUIComponent>>,
    mut bonus_query:  Query<&mut BonusStats>,
) {
    // update the player stats before updating the stats text
    if let Ok(bonus) = bonus_query.get_single_mut(){
        if let Ok(mut player_stats) = player_query.get_single_mut() {
            player_stats.update_stats(bonus);
        }
    }
    // update the stats text
    if let Some(player_stats) = player_query.iter().next() {
        for (stat_text, mut text) in stat_query.iter_mut() {
            match stat_text.stat_type {
                StatType::Atk => {
                    text.sections[0].value = player_stats.atk.to_string();
                }
                StatType::Def => {
                    text.sections[0].value = player_stats.def.to_string();
                }
                StatType::Matk => {
                    text.sections[0].value = player_stats.matk.to_string();
                }
                StatType::Mdef => {
                    text.sections[0].value = player_stats.mdef.to_string();
                }
                StatType::Spd => {
                    text.sections[0].value = player_stats.spd.to_string();
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
                    text.sections[0].value = player_stats.strength.to_string();
                }
                StatType::Magic => {
                    text.sections[0].value = player_stats.magic.to_string();
                }
                StatType::Agility => {
                    text.sections[0].value = player_stats.agility.to_string();
                }
                StatType::Health => {
                    text.sections[0].value = player_stats.health.to_string();
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
                player_stats.strength += 1;
            } else if input.just_pressed(KeyCode::KeyG) { //g to upgrade magic
                player_stats.magic += 1;
            } else if input.just_pressed(KeyCode::KeyH) { //h to upgrade speed
                player_stats.agility += 1;
            } else if input.just_pressed(KeyCode::KeyJ) { //j to upgrade max_hp all temporary for now
                player_stats.health += 1;
                player_stats.heal(10);
            } else {
                return; // No valid key pressed, exit early
            }
            // Deduct an ability point for every successful upgrade
            player_stats.ability_points -= 1;
        }
    }
}

fn spend_skill_points(
    mut player_stats: Mut<'_, PlayerStats>,
    points: u32,
) {
        // Check if there are skill points available to spend
        if player_stats.skill_points > 0 {
            // Deduct points for every successful upgrade
            player_stats.skill_points -= points;
        }
}

fn unlock_skill_tree_nodes(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    mut sprites: Query<(&Transform, &Handle<Image>, &mut TextureAtlas, &mut SkillTreeUINode), (With<Sprite>, With<SkillTreeUINode>)>,
    assets: Res<Assets<Image>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<&mut PlayerStats, With<Player>>,
    mut bonus_query:  Query<&mut BonusStats>,
) {
    // array that tracks node unlock values
    let mut node_array: [bool; 18] = [false; 18];
    let mut n = 0;
    for (_transform, _image_handle, _texture_atlas, node) in &mut sprites {
        node_array[n] = node.unlocked;
        n += 1;
    }

    let (camera, position) = cameras.single();

    // get all matching entities
    for (transform, image_handle, texture_atlas, node) in &mut sprites {
        // get the (rectangular) bounds of the sprite
        let image_size = assets.get(image_handle).unwrap().size_f32();
        let scaled = image_size * transform.scale.truncate();
        let bounds = Rect::from_center_size(
            transform.translation.truncate(),
            scaled
        );

        // get the cursor location in world coordinates
        if let Some(p) = window.single().cursor_position()
            .and_then(|cursor| camera.viewport_to_world(position, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // if the cursor location is within the (rectangular) bounds of one of the nodes and the mouse is clicked...
            if p.x > bounds.min.x 
            && p.x < bounds.max.x 
            && p.y > bounds.min.y 
            && p.y < bounds.max.y 
            && buttons.just_pressed(MouseButton::Left)
            {
                if let Ok(mut player_stats) = player_query.get_single_mut() {
                    if let Ok(mut bonus_stats) = bonus_query.get_single_mut() {
                        if node.unlocked != true {
                            let curr_sp = player_stats.skill_points;
                            let i = node.index;
                            // left
                            if i == 0 && curr_sp >= 1 {
                                // unlock the node by changing its unlocked value and sprite using texture atlas index
                                unlock_node(texture_atlas, node, node_array);
                                // edit relevant values
                                bonus_stats.max_hp += 10;
                                player_stats.heal(10);
                                spend_skill_points(player_stats, 1);
                            }
                            else if i == 1 && node_array[(i-1) as usize] == true && curr_sp >= 1 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.atk += 3;
                                spend_skill_points(player_stats, 1);
                            }
                            else if i == 2 && node_array[(i-1) as usize] == true && curr_sp >= 1{
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.matk += 3;
                                player_stats.update_stats(bonus_stats);
                                spend_skill_points(player_stats, 1);
                            }

                            // middle
                            // top
                            else if i == 3 && node_array[(i-1) as usize] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.def += 6;
                                spend_skill_points(player_stats, 2);
                            }
                            else if i == 4 && node_array[(i-1) as usize] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.atk += 6;
                                spend_skill_points(player_stats, 2);
                            }
                            else if i == 5 && node_array[(i-1) as usize] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.max_hp += 15;
                                player_stats.heal(15);
                                spend_skill_points(player_stats, 2);
                            }
                            // bottom
                            else if i == 6 && node_array[2] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.mdef += 9;
                                spend_skill_points(player_stats, 2);
                            }
                            else if i == 7 && node_array[(i-1) as usize] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.spd += 1;
                                spend_skill_points(player_stats, 2);
                            }
                            else if i == 8 && node_array[(i-1) as usize] == true && curr_sp >= 2 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.matk += 6;
                                spend_skill_points(player_stats, 2);
                            }

                            // right
                            // top
                            else if i == 9 && node_array[5] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.atk += 9;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 10 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.def += 12;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 11 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                player_stats.strength += 1;
                                spend_skill_points(player_stats, 3);
                            }
                            // middle
                            else if i == 12 && (node_array[5] == true || node_array[8] == true) && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.spd += 1;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 13 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                player_stats.ability_points += 1;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 14 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.max_hp += 25;
                                player_stats.heal(25);
                                spend_skill_points(player_stats, 3);
                            }
                            // bottom
                            else if i == 15 && node_array[8] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.matk += 9;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 16 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                bonus_stats.mdef += 9;
                                spend_skill_points(player_stats, 3);
                            }
                            else if i == 17 && node_array[(i-1) as usize] == true && curr_sp >= 3 {
                                unlock_node(texture_atlas, node, node_array);
                                player_stats.magic += 1;
                                spend_skill_points(player_stats, 3);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn unlock_node(
    mut texture_atlas: Mut<TextureAtlas>,
    mut node: Mut<'_, SkillTreeUINode>,
    mut node_array: [bool; 18]
) {
    texture_atlas.index = 1;
    node.unlocked = true;
    node_array[node.index as usize] = true;
}