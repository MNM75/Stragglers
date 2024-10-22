use bevy::{prelude::*, window::PresentMode};

mod fight_scene;
mod map;
mod player;
mod skill_tree;
mod text_box;

use fight_scene::FightScenePlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use skill_tree::SkillTreePlugin;
use text_box::TextboxPlugin;

const TITLE: &str = "main";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

// Global states for the game
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    InGame,
    SkillTreeMenu,
    BattleMode,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum TextState {
    #[default]
    TextShown,
    TextHidden,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MenuState {
    #[default]
    MainMenu,
    AttackMenu,
    Text,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_state::<TextState>()
        .init_state::<GameState>()
        .init_state::<MenuState>()
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SkillTreePlugin)
        .add_plugins(FightScenePlugin)
        .add_plugins(TextboxPlugin)
        /*
            add other plugins here
        */
        .run();
}
