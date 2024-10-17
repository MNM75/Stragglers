use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct EnemySprite;

#[derive(Component)]
struct BattleBackground;

#[derive(Component)]
struct BattleMenuText;

#[derive(Component)]
struct PlayerHealthBar;

#[derive(Component)]
struct EnemyHealthBar;

#[derive(Component)]
struct PlayerHealthBarBackground;

#[derive(Component)]
struct EnemyHealthBarBackground;

pub struct FightScenePlugin;

impl Plugin for FightScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BattleMode), setup_battle_ui);
        app.add_systems(OnExit(GameState::BattleMode), despawn_battle_ui);
        app.add_systems(Update, toggle_battle_scene);
    }
}

// toggle the battle scene with 'P'
fn toggle_battle_scene(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::BattleMode),
            GameState::BattleMode => next_state.set(GameState::InGame),
            GameState::SkillTreeMenu => next_state.set(GameState::BattleMode),
        }
    }
}

// Set up battle scene UI
fn setup_battle_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_health = 0.75;
    let enemy_health = 0.5;

    let bg_texture_handle = asset_server.load("fightBackground.png");
    let player_texture_handle = asset_server.load("fightPlayer.png");
    let enemy_texture_handle = asset_server.load("fightEnemy.png");
    let healthbar_background_handle = asset_server.load("healthbarBackground.png");
    let healthbar_handle = asset_server.load("healthbar.png");

    // background
    commands.spawn((
        SpriteBundle {
            texture: bg_texture_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.), // position background
                scale: Vec3::new(1.0, 1.0, 1.0),    // scale as needed
                ..default()
            },
            ..default()
        },
        BattleBackground,
    ));

    // player sprite
    commands.spawn((
        SpriteBundle {
            texture: player_texture_handle,
            transform: Transform {
                translation: Vec3::new(-400., -100., 1.), // position player sprite
                scale: Vec3::new(0.5, 0.5, 1.0),          // adjust player sprite size
                ..default()
            },
            ..default()
        },
        PlayerSprite,
    ));

    // enemy sprite
    commands.spawn((
        SpriteBundle {
            texture: enemy_texture_handle,
            transform: Transform {
                translation: Vec3::new(400., -100., 1.), // position enemy sprite
                scale: Vec3::new(0.5, 0.5, 1.0),         // adjust enemy sprite size
                ..default()
            },
            ..default()
        },
        EnemySprite,
    ));

    // menu text
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "1: Attack\n2: Run",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                bottom: Val::Px(50.),
                ..default()
            },
            ..default()
        },
        BattleMenuText,
    ));

    // player health bar background
    commands.spawn((
        SpriteBundle {
            texture: healthbar_background_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-400., 200., 1.), // position health bar background
                scale: Vec3::new(1.0, 0.1, 1.0),         // scale the background as needed
                ..default()
            },
            ..default()
        },
        PlayerHealthBarBackground,
    ));

    // player health bar: scales based on health
    commands.spawn((
        SpriteBundle {
            texture: healthbar_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-400.0 - (240.0 * (1.0 - player_health)), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * player_health, 0.1, 1.0), // scale based on player's health
                ..default()
            },
            ..default()
        },
        PlayerHealthBar,
    ));

    // enemy health bar background
    commands.spawn((
        SpriteBundle {
            texture: healthbar_background_handle.clone(),
            transform: Transform {
                translation: Vec3::new(400., 200., 1.), // position health bar background
                scale: Vec3::new(1.0, 0.1, 1.0),        // scale the background as needed
                ..default()
            },
            ..default()
        },
        EnemyHealthBarBackground,
    ));

    // enemy health bar: scales based on health
    commands.spawn((
        SpriteBundle {
            texture: healthbar_handle.clone(),
            transform: Transform {
                translation: Vec3::new(400.0 - (240.0 * (1.0 - enemy_health)), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * enemy_health, 0.1, 1.0), // scale based on enemy's health
                ..default()
            },
            ..default()
        },
        EnemyHealthBar,
    ));
}

// Despawn battle UI when exiting the battle mode

fn despawn_battle_ui(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<BattleBackground>,
            With<PlayerSprite>,
            With<EnemySprite>,
            With<PlayerHealthBar>,
            With<PlayerHealthBarBackground>,
            With<EnemyHealthBar>,
            With<EnemyHealthBarBackground>,
            With<BattleMenuText>,
        )>,
    >,
) {
    // Despawn all battle-related entities
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
