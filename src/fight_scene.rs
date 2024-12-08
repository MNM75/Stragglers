use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use std::time::Duration;

use crate::GameState;
use crate::events::EnemyCollisionEvent;
use crate::enemy::EnemyStats;
use crate::enemy::Enemy;

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
struct MagicSprite;



#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

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


static mut player_health: f32 =  1.0; 
static mut enemy_health: f32 =  1.0; 


impl Plugin for FightScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_battle_ui);
        app.add_systems(PostStartup, hide_battle_ui);
        app.add_systems(OnEnter(GameState::BattleMode), show_battle_ui);
        app.add_systems(OnExit(GameState::BattleMode), hide_battle_ui);
        app.add_systems(Update, execute_animations); 
        app.add_systems(Update, trigger_animation::<PlayerSprite>.run_if(input_just_pressed(KeyCode::Digit1)));
        app.add_systems(Update, trigger_animation::<MagicSprite>.run_if(input_just_pressed(KeyCode::Digit2)));
        app.add_systems(Update, init_upon_collision);
        app.add_systems(Update, update_enemy_health_bar);
    }
}
fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
    let mut animation = query.single_mut();
    // create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
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
            GameState::Welcome => next_state.set(GameState::BattleMode),
            GameState::InGame => next_state.set(GameState::BattleMode),
            GameState::BattleMode => next_state.set(GameState::BattleMode),
            GameState::SkillTreeMenu => next_state.set(GameState::BattleMode),
            GameState::EndCredits => next_state.set(GameState::EndCredits),
            GameState::DefeatScreen => next_state.set(GameState::DefeatScreen),
        }
    }
}


// Set up battle scene UI
fn setup_battle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,
) {
        

    let bg_texture_handle = asset_server.load("fightBackground.png");

    let player_texture_handle = asset_server.load("L_swordAnimation.png");
    let LSword_layout = TextureAtlasLayout::from_grid(UVec2::new(216, 202), 8, 1, None, None);
    let Lsword_layout_handle = texture_atlases.add(LSword_layout);
    let animation_config_1 = AnimationConfig::new(1, 7, 24);

    let magic_texture_handle = asset_server.load("magic_animation.png");
    let magic_layout = TextureAtlasLayout::from_grid(UVec2::new(216, 202), 8, 1, None, None);
    let magic_layout_handle = texture_atlases.add(magic_layout);
    let animation_config_2 = AnimationConfig::new(0, 7, 24);
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
            transform: Transform::from_scale(Vec3::splat(2.5))
                .with_translation(Vec3::new(-400.0, -100.0, 0.9)),
            texture: player_texture_handle.clone(),
            ..default()
        },
        TextureAtlas {
            layout: Lsword_layout_handle.clone(),
            index: animation_config_1.first_sprite_index,
        },
        
        PlayerSprite,
        animation_config_1,
        FightSprites,
        FightScene
    ));
    //magic sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(2.5))
                .with_translation(Vec3::new(-400.0, -100.0, 1.0)),
            texture: magic_texture_handle.clone(),
            ..default()
        },
        TextureAtlas {
            layout: magic_layout_handle.clone(),
            index: animation_config_2.first_sprite_index,
        },
        
        MagicSprite,
        animation_config_2,
        FightSprites,
        FightScene
    ));

    // enemy sprite
    commands.spawn((
        SpriteBundle {
            texture: enemy_texture_handle,
            transform: Transform {
                translation: Vec3::new(400., -100., 0.8), // position enemy sprite
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
                translation: Vec3::new(-400.0-(240.0*(1.0-unsafe { player_health })), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * unsafe { player_health }, 0.1, 1.0), // scale based on player's health
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
                translation: Vec3::new(400.0-(240.0*(1.0-unsafe { enemy_health })), 200., 2.), // position health bar above the background
                scale: Vec3::new(1.0 * unsafe { enemy_health }, 0.1, 1.0), // scale based on enemy's health
                ..default()
            },
            ..default()
        },
        EnemyHealthBar,
        FightSprites,
        FightScene
    ));
}



//NOT completely working, feel free to remove if needed
//  - green health bar only moves to the left of the screen after taking damage, going out of bounds instead of completely disappearing
fn update_enemy_health_bar(
    mut commands: Commands,
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,
    mut enemy_hb: Query<&mut Transform, (With<EnemyHealthBar>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBarBackground>)>,
    player: Query<&Transform, (With<Player>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>)>,

){
    if let Ok(enemy_stats) = enemy_stat_query.get_single(){
        unsafe { enemy_health = (&enemy_stats.hp/&enemy_stats.max_hp) as f32 };
    }

    
    let pt = player.single();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    let mut ehb = enemy_hb.single_mut();    //enemy health bar 

    ehb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0+(240.0*(1.0 - unsafe { enemy_health }));
    ehb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    ehb.translation.z = pt.translation.z + 1.2;

}


// Hide
fn show_battle_ui(
    mut commands: Commands,
    query: Query<Entity, With<FightScene>>,
    mut background: Query<&mut Transform, (With<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut player_sp: Query<&mut Transform, (With<PlayerSprite>, Without<BattleBackground>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut magic_sp: Query<&mut Transform, (With<MagicSprite>, Without<BattleBackground>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<PlayerSprite>)>,
    mut enemy_sp: Query<&mut Transform, (With<EnemySprite>, Without<BattleBackground>, Without<PlayerSprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut player_hb: Query<&mut Transform, (With<PlayerHealthBar>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut player_hbb: Query<&mut Transform, (With<PlayerHealthBarBackground>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut enemy_hb: Query<&mut Transform, (With<EnemyHealthBar>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
    mut enemy_hbb: Query<&mut Transform, (With<EnemyHealthBarBackground>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<MagicSprite>)>,
    player: Query<&Transform, (With<Player>, Without<BattleBackground>, Without<PlayerSprite>, Without<EnemySprite>, Without<PlayerHealthBar>, Without<PlayerHealthBarBackground>, Without<EnemyHealthBar>, Without<EnemyHealthBarBackground>, Without<MagicSprite>)>,
) { 

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
    bg.translation.z = pt.translation.z + 1.0;

    let mut ps = player_sp.single_mut();

    ps.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-200.0;   // same logic as camera/player movement
    ps.translation.y = pt.translation.y.clamp(-y_bound, y_bound)-100.0;   // same logic as camera/player movement
    ps.translation.z = pt.translation.z + 1.2;

    let mut ms = magic_sp.single_mut();

    ms.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-200.0;   // same logic as camera/player movement
    ms.translation.y = pt.translation.y.clamp(-y_bound, y_bound)-100.0;   // same logic as camera/player movement
    ms.translation.z = pt.translation.z + 1.3;

    let mut es = enemy_sp.single_mut();

    es.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+200.0;   // same logic as camera/player movement
    es.translation.y = pt.translation.y.clamp(-y_bound, y_bound)-100.0;   // same logic as camera/player movement
    es.translation.z = pt.translation.z + 1.1;

    let mut phb = player_hb.single_mut();

    phb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-400.0-(240.0*(1.0-unsafe { player_health }));   // same logic as camera/player movement
    phb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    phb.translation.z = pt.translation.z + 1.2;

    let mut phbb = player_hbb.single_mut();

    phbb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)-400.0;   // same logic as camera/player movement
    phbb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    phbb.translation.z = pt.translation.z + 1.1;
 
    let mut ehb = enemy_hb.single_mut();    //enemy health bar 

    ehb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0-(240.0*(1.0-unsafe { enemy_health }));   // same logic as camera/player movement
    ehb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    ehb.translation.z = pt.translation.z + 1.2;

    let mut ehbb = enemy_hbb.single_mut();  //enemy health bar background

    ehbb.translation.x = pt.translation.x.clamp(-x_bound, x_bound)+400.0;   // same logic as camera/player movement
    ehbb.translation.y = pt.translation.y.clamp(-y_bound, y_bound)+200.0;   // same logic as camera/player movement
    ehbb.translation.z = pt.translation.z + 1.1;
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