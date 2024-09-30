use bevy::{prelude::*, window::PresentMode};

mod map;
mod player;
mod skill_tree;

use map::MapPlugin;
use player::PlayerPlugin;
use skill_tree::SkillTreePlugin;

const TITLE: &str = "main";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main(){
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
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SkillTreePlugin) // comment this out to remove the skill tree UI
        /*
            add other plugins here
        */
        .run();

}

