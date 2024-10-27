// An "end credits" slideshow of images, one from each team member, displayed for 3s each

use bevy::prelude::*;
use crate::GameState;
use crate::events::EndGameEvent;

#[derive(Component, Deref, DerefMut)]
struct SwitchTimer(Timer);

pub struct EndCreditsPlugin;

impl Plugin for EndCreditsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_systems(OnEnter(GameState::EndCredits), setup)
        .add_systems(OnEnter(GameState::EndCredits), replace_camera)
        .add_systems(Update, change_image.run_if(in_state(GameState::EndCredits)))
        .add_systems(Update, init_upon_game_end);
    }
}

fn init_upon_game_end(
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventReader<EndGameEvent>,
){
    for _event in collision_events.read() {
        next_state.set(GameState::EndCredits);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("end_credit_assets/EndScene.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(0.1, TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("end_credit_assets/car.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(5., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("end_credit_assets/Rachel.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(8., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("end_credit_assets/helenf.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(SwitchTimer(Timer::from_seconds(11., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("end_credit_assets/Miko.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(14., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("end_credit_assets/Vivibutcompressed.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(17., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("end_credit_assets/Leo Liang.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(20., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("end_credit_assets/Andre.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(23., TimerMode::Once)));
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("end_credit_assets/dungeonHall.PNG"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    })
    .insert(SwitchTimer(Timer::from_seconds(26., TimerMode::Once)));
}

fn change_image(time: Res<Time>, mut switch: Query<(&mut SwitchTimer, &mut Transform)>) {
    for (mut timer, mut transform) in switch.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = timer.duration().as_secs_f32();
        }
    }
}

fn replace_camera(
    mut commands: Commands,
    query: Query<Entity, With<Camera>>,
) {
    for camera_entity in query.iter() {
        commands.entity(camera_entity).despawn();
     }
}