use bevy::prelude::*;

mod maze; // Assuming your maze generation code is in a module named `map`

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(maze::MazePlugin) // Add your MapPlugin here
        .run();
}
