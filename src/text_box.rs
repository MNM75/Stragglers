use std::os::windows::thread;

use bevy::prelude::*;
use crate::TextState;
use crate::MenuState;

use crate::player::Player;
use crate::WIN_W;
use crate::WIN_H;
use crate::player::LEVEL_W;
use crate::player::LEVEL_H;

#[derive(Component)]
struct Textbox;

#[derive(Component)]
struct TextboxBackground;

pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_textbox);
        //app.add_systems(PostStartup, hide_textbox);
        app.add_systems(OnEnter(TextState::TextShown), show_textbox);
        app.add_systems(OnExit(TextState::TextShown), hide_textbox);        
        app.add_systems(Update, toggle_textbox);
        app.add_systems(Update, menu_interaction);
    }
}

// toggle the textbox with 'T'
fn toggle_textbox(
    state: Res<State<TextState>>,
    mut next_state: ResMut<NextState<TextState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        match state.get() {
            TextState::TextHidden => next_state.set(TextState::TextShown),
            TextState::TextShown => next_state.set(TextState::TextHidden),
        }
    }
}

// Set up textbox
fn setup_textbox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /*let text_box_background = asset_server.load("fightPlayer.png");
    // Background for the textbox
    commands.spawn((
        SpriteBundle {
            texture: text_box_background,
            transform: Transform {
                translation: Vec3::new(400., -100., 1.), // position enemy sprite
                scale: Vec3::new(0.5, 0.5, 1.0),        // adjust enemy sprite size
                ..default()
            },
            ..default()
        },
        Textbox,
        TextboxBackground
    ));*/

    // Text display
    commands.spawn((
        Textbox,
        TextBundle {
            text: Text::from_section(
                "What will you do?\n1. Attack\n2. Run",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(100.0),
            ..default()
        })
    ));
}

fn menu_interaction(
    mut query: Query<&mut Text, With<Textbox>>,
    input: Res<ButtonInput<KeyCode>>,  
    menu_state: Res<State<MenuState>>,  
    mut next_menu_state: ResMut<NextState<MenuState>>, 
    mut next_text_state: ResMut<NextState<TextState>>,  
) {
    for mut text in query.iter_mut() {
        match menu_state.get() { 
            MenuState::MainMenu => {
                if input.just_pressed(KeyCode::Digit1) {
                    next_menu_state.set(MenuState::AttackMenu);
                    text.sections[0].value = "Choose your attack:\n1. Attack1\n2. Attack2\n3. Attack3".to_string();
                } else if input.just_pressed(KeyCode::Digit2) {
                    text.sections[0].value = "You ran away!".to_string();                    
                    next_menu_state.set(MenuState::Text);
                }
            }
            MenuState::AttackMenu => {
                if input.just_pressed(KeyCode::Digit1) {
                    text.sections[0].value = "You chose Attack1!".to_string();
                    next_menu_state.set(MenuState::Text);
                } else if input.just_pressed(KeyCode::Digit2) {
                    text.sections[0].value = "You chose Attack2!".to_string();
                    next_menu_state.set(MenuState::Text);
                } else if input.just_pressed(KeyCode::Digit3) {
                    text.sections[0].value = "You chose Attack3!".to_string();
                    next_menu_state.set(MenuState::Text);
                }
            }
            MenuState::Text => {
                for key in input.get_just_pressed() {
                    text.sections[0].value = "What will you do?\n1. Attack\n2. Run".to_string();
                    next_menu_state.set(MenuState::MainMenu);
                }
            }
        }
    }
}
// Show textbox
fn show_textbox(
    mut commands: Commands,
    query: Query<Entity, With<Textbox>>,
    mut background: Query<&mut Transform, With<TextboxBackground>>,
    player: Query<&Transform, (With<Player>, Without<TextboxBackground>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }

    /*let pt = player.single();
    let mut bt = background.single_mut();
    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;

    bt.translation.x = pt.translation.x.clamp(-x_bound, x_bound);   // same logic as camera/player movement
    bt.translation.y = pt.translation.y.clamp(-y_bound, y_bound);   // same logic as camera/player movement
    bt.translation.z = pt.translation.z + 1.; */
}

// Hide textbox
fn hide_textbox(
    mut commands: Commands,
    query: Query<Entity, With<Textbox>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}