use bevy::{
    color::palettes::css::BLACK,
    prelude::*
};

use crate::GameState;

#[derive(Component)]
struct SkillTreeUIComponent;

// temporary holding spot
// #[derive(Component)]
// struct Points {
//     ability_points: u32,
//     skill_points: u32,
// }

// #[derive(Component)]
// struct AbilityScores {
//     strength: u32,
//     magic: u32,
//     agility: u32,
//     health: u32,
// }

// #[derive(Component)]
// struct Stats{
//     atk: u32,
//     matk: u32,
//     def: u32,
//     mdef: u32,
//     spd: u32,
//     hp: u32,
// }

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_skill_tree_ui);
        app.add_systems(PostStartup, hide_skill_tree_ui);
        app.add_systems(Update, toggle_skill_tree_ui);
        app.add_systems(OnEnter(GameState::SkillTreeMenu), show_skill_tree_ui);
        app.add_systems(OnExit(GameState::SkillTreeMenu), hide_skill_tree_ui);
    }
}

fn load_skill_tree_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // bring in asset for skill tree ui
    commands.spawn((
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
            "0",
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
            "0",
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
            "0",
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
            "0",
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
            bottom: Val::Px(185.0),
            left: Val::Px(105.0),
            ..default()
        })
    ));
    // atk
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
            "0",
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
            "0",
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

fn show_skill_tree_ui(mut commands: Commands, query: Query<Entity, With<SkillTreeUIComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    
}

fn hide_skill_tree_ui(mut commands: Commands, query: Query<Entity, With<SkillTreeUIComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn toggle_skill_tree_ui(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,) {
        if input.just_pressed(KeyCode::KeyQ) {
            match state.get() {
                GameState::InGame => next_state.set(GameState::SkillTreeMenu),
                GameState::SkillTreeMenu => next_state.set(GameState::InGame),
            }
        }
}