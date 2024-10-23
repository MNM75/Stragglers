use bevy::prelude::*;
use crate::GameState;
use crate::events::EnemyCollisionEvent;

use crate::player::Player;
use crate::WIN_W;
use crate::WIN_H;
use crate::player::LEVEL_W;
use crate::player::LEVEL_H;

#[derive(Component)]
struct FightScene;

#[derive(Component)]
struct FightSprites;

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
        app.add_systems(Startup, setup_battle_ui);
        app.add_systems(PostStartup, hide_battle_ui);
        app.add_systems(OnEnter(GameState::BattleMode), show_battle_ui);
        app.add_systems(OnExit(GameState::BattleMode), hide_battle_ui);        
        app.add_systems(Update, init_upon_collision);
    }
}

// open battle scene upon enemy collision (does not close upon re-collision)
fn init_upon_collision(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventReader<EnemyCollisionEvent>,
){
    for _event in collision_events.read() {
        match state.get() {
            GameState::InGame => next_state.set(GameState::BattleMode),
            GameState::BattleMode => next_state.set(GameState::BattleMode),
            GameState::SkillTreeMenu => next_state.set(GameState::BattleMode),
        }
    }
}

// Set up battle scene UI
fn setup_battle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_health = 0.75; 
    let enemy_health = 0.5;    

    let bg_texture_handle = asset_server.load("fightBackground.png");
    let player_texture_handle = asset_server.load("L_static.png");
    let enemy_texture_handle = asset_server.load("enemyPlaceHolder.png");
    let healthbar_background_handle = asset_server.load("healthbarBackground.png");
    let healthbar_handle = asset_server.load("healthbar.png");

    // background
    commands.spawn((
        SpriteBundle {
            texture: bg_texture_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.), // position background
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        BattleBackground,
        FightSprites,
        FightScene
    ));

    // player sprite
    commands.spawn((
        SpriteBundle {
            texture: player_texture_handle,
            transform: Transform {
                translation: Vec3::new(-400., -100., 1.), // position player sprite
                scale: Vec3::new(2.5, 2.5, 2.5),         // adjust player sprite size
                ..default()
            },
            ..default()
        },
        PlayerSprite,
        FightSprites,
        FightScene
    ));

    // enemy sprite
    commands.spawn((
        SpriteBundle {
            texture: enemy_texture_handle,
            transform: Transform {
                translation: Vec3::new(400., -100., 1.), // position enemy sprite
                scale: Vec3::new(2.5, 2.5, 2.5),        // adjust enemy sprite size
                ..default()
            },
            ..default()
        },
        EnemySprite,
        FightSprites,
        FightScene
    ));

    // menu text
    // commands.spawn((
    //     TextBundle {
    //         text: Text::from_section(
    //             "1: Attack\n2: Run",
    //             TextStyle {
    //                 font_size: 40.0,
    //                 color: Color::WHITE,
    //                 ..default()
    //             },
    //         ),
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             left: Val::Px(50.),
    //             bottom: Val::Px(50.),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     BattleMenuText,
    //     FightScene
    // ));

    // player health bar background
    commands.spawn((
        SpriteBundle {
            texture: healthbar_background_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-400., 200., 1.), // position health bar background
                scale: Vec3::new(1.0, 0.1, 1.0),        // scale the background as needed
                ..default()
            },
            ..default()
        },
        PlayerHealthBarBackground,
        FightSprites,
        FightScene
    ));

    // player health bar: scales based on health
    commands.spawn((
        SpriteBundle {
            texture: healthbar_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-400.0-(240.0*(1.0-player_health)), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * player_health, 0.1, 1.0), // scale based on player's health
                ..default()
            },
            ..default()
        },
        PlayerHealthBar,
        FightSprites,
        FightScene
    ));

    // enemy health bar background
    commands.spawn((
        SpriteBundle {
            texture: healthbar_background_handle.clone(),
            transform: Transform {
                translation: Vec3::new(400., 200., 1.), // position health bar background
                scale: Vec3::new(1.0, 0.1, 1.0),       // scale the background as needed
                ..default()
            },
            ..default()
        },
        EnemyHealthBarBackground,
        FightSprites,
        FightScene
    ));

    // enemy health bar: scales based on health
    commands.spawn((
        SpriteBundle {
            texture: healthbar_handle.clone(),
            transform: Transform {
                translation: Vec3::new(400.0-(240.0*(1.0-enemy_health)), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * enemy_health, 0.1, 1.0), // scale based on enemy's health
                ..default()
            },
            ..default()
        },
        EnemyHealthBar,
        FightSprites,
        FightScene
    ));
}

// Hide
fn show_battle_ui(
    mut commands: Commands,
    query: Query<Entity, With<FightScene>>,
    mut background: Query<&mut Transform, (With<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
    mut player_sp: Query<&mut Transform, (With<PlayerSprite>, Without<BattleBackground>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
    mut enemy_sp: Query<&mut Transform, (With<EnemySprite>, Without<BattleBackground>, Without<PlayerSprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
    mut player_hb: Query<&mut Transform, (With<PlayerHealthBar>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
    mut player_hbb: Query<&mut Transform, (With<PlayerHealthBarBackground>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
    mut enemy_hb: Query<&mut Transform, (With<EnemyHealthBar>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBarBackground>)>,
    mut enemy_hbb: Query<&mut Transform, (With<EnemyHealthBarBackground>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>)>,
    player: Query<&Transform, (With<Player>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,
) {
    let player_health = 0.75;  
    let enemy_health = 0.5;   

    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    let pt = player.single();
    //for sprite in sprites.iter() {

    let mut bg = background.single_mut();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;

    bg.translation.x = pt.translation.x.clamp(-x_bound, x_bound);   // same logic as camera/player movement
    bg.translation.y = pt.translation.y.clamp(-y_bound, y_bound);   // same logic as camera/player movement
    bg.translation.z = pt.translation.z + 1.;

    let mut ps = player_sp.single_mut();

    ps.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-400.0;   // same logic as camera/player movement
    ps.translation.y = pt.translation.y.clamp(-y_bound, y_bound)-100.0;   // same logic as camera/player movement
    ps.translation.z = pt.translation.z + 1.;

    let mut es = enemy_sp.single_mut();

    es.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0;   // same logic as camera/player movement
    es.translation.y = pt.translation.y.clamp(-y_bound, y_bound)-100.0;   // same logic as camera/player movement
    es.translation.z = pt.translation.z + 1.;

    let mut phb = player_hb.single_mut();

    phb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-400.0-(240.0*(1.0-player_health));   // same logic as camera/player movement
    phb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    phb.translation.z = pt.translation.z + 1.;

    let mut phbb = player_hbb.single_mut();

    phbb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-400.0;   // same logic as camera/player movement
    phbb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    phbb.translation.z = pt.translation.z + 1.;

    let mut ehb = enemy_hb.single_mut();

    ehb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0-(240.0*(1.0-enemy_health));   // same logic as camera/player movement
    ehb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    ehb.translation.z = pt.translation.z + 1.;

    let mut ehbb = enemy_hbb.single_mut();

    ehbb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0;   // same logic as camera/player movement
    ehbb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    ehbb.translation.z = pt.translation.z + 1.;
    //}/**/
}

fn hide_battle_ui(
    mut commands: Commands,
    query: Query<Entity, With<FightScene>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}