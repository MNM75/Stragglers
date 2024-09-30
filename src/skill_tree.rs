use bevy::{
    color::palettes::css::BLACK,
    prelude::*
};

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
        app.add_systems(Startup, init_skill_tree_ui);
    }
}


fn init_skill_tree_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // bring in asset for skill tree ui
    commands.spawn(SpriteBundle {
        texture: asset_server.load("skillTreeUI.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });

    // Ability Points
    commands.spawn(
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
        })
    );

    // Skill Points
    commands.spawn(
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
    );

    // Ability Scores ------------------------------------------------------------
    // Strength
    commands.spawn(
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
    );
    // Magic
    commands.spawn(
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
    );
    // Agility
    commands.spawn(
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
    );
    // Health
    commands.spawn(
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
    );

    // Player Stats ------------------------------------------------------------
    // hp
    commands.spawn(
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
    );
    // atk
    commands.spawn(
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
    );
    // def
    commands.spawn(
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
    );
    // spd
    commands.spawn(
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
    );
    // matk
    commands.spawn(
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
    );
    // mdef
    commands.spawn(
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
        })
    );
}