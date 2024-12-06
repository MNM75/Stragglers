use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;

use crate::GameState;


#[derive(Component)]
struct WelcomeSreen;

pub struct WelcomePlugin;

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_welcome);
        app.add_systems(Update, hide_welcome.run_if(input_just_pressed(KeyCode::Space)));
    }
}

fn show_welcome(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<WelcomeSreen>>,
){
    match state.get() {
        GameState::Welcome=> next_state.set(GameState::Welcome),
        GameState::InGame => next_state.set(GameState::Welcome),
        GameState::BattleMode => next_state.set(GameState::Welcome),
        GameState::SkillTreeMenu => next_state.set(GameState::Welcome),
        GameState::EndCredits => next_state.set(GameState::EndCredits),
        GameState::DefeatScreen => next_state.set(GameState::DefeatScreen),
    }
    let welcome_texture_handle: Handle<Image> = asset_server.load("welcomeScreen.png");
    
    commands.spawn((
        SpriteBundle {
            texture: welcome_texture_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 950.), // position screen
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        WelcomeSreen,
    ));

    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn hide_welcome(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<Entity, With<WelcomeSreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
    match state.get() {
        GameState::Welcome=> next_state.set(GameState::InGame),
        GameState::InGame => next_state.set(GameState::InGame),
        GameState::BattleMode => next_state.set(GameState::InGame),
        GameState::SkillTreeMenu => next_state.set(GameState::InGame),
        GameState::EndCredits => next_state.set(GameState::EndCredits),
        GameState::DefeatScreen => next_state.set(GameState::DefeatScreen),
    }
}
