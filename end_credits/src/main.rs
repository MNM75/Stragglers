// An "end credits" slideshow of images, one from each team member, displayed for 3s each

use bevy::{prelude::*, window::PresentMode};
#[derive(Component, Deref, DerefMut)]
struct SwitchTimer(Timer);


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "End Credits".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, change_image)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("car.png"),
        ..default()
    });
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Rachel.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("helenf.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("littleguy2.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("Vivibutcompressed.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(12., TimerMode::Once)));
}

fn change_image(time: Res<Time>, mut switch: Query<(&mut SwitchTimer, &mut Transform)>) {
    for (mut timer, mut transform) in switch.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = timer.duration().as_secs_f32();
        }
    }
}