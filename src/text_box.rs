//use std::os::windows::thread; //does not compile with this line on mac

use bevy::prelude::*;
use crate::GameState;
//use crate::TextState;
use crate::MenuState;

use crate::player::Player;
use crate::player::init_player;
use crate::WIN_W;
use crate::WIN_H;
use crate::player::LEVEL_W;
use crate::player::LEVEL_H;

use crate::player::PlayerStats; // }
use crate::enemy::EnemyStats;   // }for player and enemy hp displays
use crate::enemy::Enemy;        // }
use crate::enemy::find_closest_enemy;

#[derive(Component)]    //All UI's in battle screen have this component
struct Textbox;

#[derive(Component)]    //Used to identify battle options UI
struct Battleoptions;

#[derive(Component)]    //Used to identify Player HP UI
struct Playerhp;

#[derive(Component)]    //Used to identify Enemy HP UI
struct Enemyhp;

#[derive(Component)]
struct TextboxBackground;

#[derive(Component)]
struct BattleLogTag;

#[derive(Component)]
pub struct BattleDialogue{
    pub dialogue1: String,
    pub dialogue2: String,
    pub dialogue3: String,
}

impl BattleDialogue{
    pub fn new() -> Self {
        Self {
            dialogue1: String::from(""),
            dialogue2: String::from(""),
            dialogue3: String::from(">Battle Start"),
        }
    }

    pub fn change(&mut self, text: String){
        self.dialogue1 = self.dialogue2.clone();
        self.dialogue2 = self.dialogue3.clone();
        self.dialogue3 = text.clone();
    }
    
}

pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_textbox.after(init_player));
       // app.add_systems(Startup, steup_player_health.after(init_player));
        app.add_systems(PostStartup, hide_textbox);
        app.add_systems(OnEnter(GameState::BattleMode), show_textbox);
        app.add_systems(OnExit(GameState::BattleMode), hide_textbox);        
        //app.add_systems(Update, toggle_textbox);
        app.add_systems(Update, menu_interaction);
        app.add_systems(Update, update_playerhp.after(menu_interaction));
        app.add_systems(Update, update_enemyhp.after(menu_interaction));
        app.add_systems(Update, update_battledialogue.after(menu_interaction));

    }
}
// toggle the textbox with 'T'
// fn toggle_textbox(
//     state: Res<State<TextState>>,
//     mut next_state: ResMut<NextState<TextState>>,
//     input: Res<ButtonInput<KeyCode>>,
// ) {
//     if input.just_pressed(KeyCode::KeyT) {
//         match state.get() {
//             TextState::TextHidden => next_state.set(TextState::TextShown),
//             TextState::TextShown => next_state.set(TextState::TextHidden),
//         }
//     }
// }



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

    // Battle Options display
    commands.spawn((
        Textbox,
        Battleoptions,
        TextBundle {
            text: Text::from_section(
                "What will you do?\n1. Attack\n2. Magic\n3. Heal\n4. Run",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ).with_justify(JustifyText::Left),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(80.0),
            ..default()
        })
    ));
    // Player HP Display
    commands.spawn((
        Textbox,
        Playerhp,
        TextBundle {
            text: Text::from_section(
                "10/10",        //will immediately be changed by update_playerhp to reflect current hp values 
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(500.0),
            left: Val::Px(100.0),
            ..default()
        })
    ));
    // Enemy HP display
    commands.spawn((
        Textbox,
        Enemyhp,
        TextBundle {
            text: Text::from_section(
                "10/10",        //will immediately be changed by update_enemyhp to reflect current hp values 
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(500.0),
            right: Val::Px(100.0),
            ..default()
        })
    ));
    //battle dialogue display
    commands.spawn((
        BattleDialogue::new(),
    ));
    let mut bd= BattleDialogue::new();
    
    commands.spawn((    //top of dialogue, bd[0]
        Textbox,
        BattleLogTag,
        TextBundle {
            text: Text::from_section(
                bd.dialogue1.clone(),        
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(40.0),
            ..default()
        }),
    ));
    commands.spawn((    //middle of dialogue, bd[1]
        Textbox,
        BattleLogTag,
        TextBundle {
            text: Text::from_section(
                bd.dialogue2.clone(),       
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            right: Val::Px(40.0),
            ..default()
        }),
    ));
    commands.spawn((    //bottom of dialogue, bd[2]
        Textbox,
        BattleLogTag,
        TextBundle {
            text: Text::from_section(
                bd.dialogue3.clone(),        
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(40.0),
            ..default()
        }),
    ));
}

fn menu_interaction(
    mut query: Query<&mut Text, With<Battleoptions>>,
    input: Res<ButtonInput<KeyCode>>,  
    menu_state: Res<State<MenuState>>,  
    mut next_menu_state: ResMut<NextState<MenuState>>, 
    //mut next_text_state: ResMut<NextState<TextState>>,  
) {
    for mut text in query.iter_mut() {
        match menu_state.get() { 
            MenuState::MainMenu => {    //main attack options menu
                if input.just_pressed(KeyCode::Digit1) {
                    text.sections[0].value = "Attacked!".to_string();   
                    next_menu_state.set(MenuState::Text);
                    //next_menu_state.set(MenuState::AttackMenu);
                    //text.sections[0].value = "Choose your attack:\n1. Attack1\n2. Attack2\n3. Attack3".to_string();
                } else if input.just_pressed(KeyCode::Digit2) {
                    text.sections[0].value = "Magic Attacked!".to_string();                    
                    next_menu_state.set(MenuState::Text);
                } else if input.just_pressed(KeyCode::Digit3) {
                    text.sections[0].value = "1hp restored!".to_string();   //<-------- in future change to variable # of hp resotred
                    next_menu_state.set(MenuState::Text);
                } else if input.just_pressed(KeyCode::Digit3) {
                    text.sections[0].value = "You ran away!".to_string();
                    next_menu_state.set(MenuState::Text);
                }
            }
            MenuState::AttackMenu => {  //sub menu containing other attacks?
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
            MenuState::Text => {    //puts main attack menu back up after action text was displayed
                //for _key in input.get_just_pressed() {
                    //probably need to put in a delay here so the above action texts gets displayed?
                    text.sections[0].value = "What will you do?\n1. Attack\n2. Magic\n3. Heal\n4. Run".to_string();
                    next_menu_state.set(MenuState::MainMenu);
                //}
            }
        }
    }

}


fn update_playerhp(
    mut playerhpquery: Query<&mut Text, With<Playerhp>>,        //to change hp textbox
    player_stat_query: Query<&mut PlayerStats, With<Player>>,   //to get hp and hp_max values
){
    if let Ok(player_stat) = player_stat_query.get_single(){
        for mut text in &mut playerhpquery.iter_mut(){
            text.sections[0].value = player_stat.hp.to_string() + "/"+ &player_stat.max_hp.to_string();
        }
    }
}
fn update_enemyhp(
    mut enemyhpquery: Query<&mut Text, With<Enemyhp>>,          //to change hp textbox
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,      //to get hp and hp_max values
    commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
){
    if let Some(closest_enemy) = find_closest_enemy(&commands, &enemy_query, &player_query){
        if let Ok(mut enemy_stat) = enemy_stat_query.get_mut(closest_enemy) {
            for mut text in &mut enemyhpquery.iter_mut(){
                text.sections[0].value = enemy_stat.hp.to_string() + "/"+ &enemy_stat.max_hp.to_string();
            }
        }
    }
}

fn update_battledialogue(
    mut battle_log_query: Query<&mut Text, With<BattleLogTag>>,          //to access the dialogue box
    battle_dialogue_query: Query<&mut BattleDialogue>,                   //to get the actual dialogue strings
){
    if let Ok(battle_dialogue) = battle_dialogue_query.get_single(){
        let mut counter = 1;
        for mut text in &mut battle_log_query.iter_mut(){
            if(counter == 1){
                //text.sections[0].value = battle_dialogue.dialogue1.clone();
                text.sections[0].value = battle_dialogue.dialogue1.clone();
                text.sections[0].style.color = Color::srgb(1.0, 1.0, 1.0);
            }
            else if(counter == 2){
                text.sections[0].value = battle_dialogue.dialogue2.clone();
                text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.7);
            }
            else if(counter == 3){
                text.sections[0].value = battle_dialogue.dialogue3.clone();
                text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.0);
            }
            counter += 1;   //should only interate 3 times since there are only 3 entites tagged wiith BattleLogTag
        }
    }
}

fn reset_battle_dialogue(
    mut battle_log_query: Query<&mut Text, With<BattleLogTag>>,          //to access the dialogue box
){
    for mut text in &mut battle_log_query.iter_mut(){
        //text.sections[0].value = battle_dialogue.dialogue1.clone();
        text.sections[0].value = "".to_string();
        text.sections[0].style.color = Color::srgb(1.0, 1.0, 1.0);
        text.sections[0].value = "".to_string();
        text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.7);
        text.sections[0].value = ">Battle Start".to_string();
        text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.0);
    }
}

// Show all textboxes
fn show_textbox(
    mut commands: Commands,
    query: Query<Entity, With<Textbox>>,
    // mut background: Query<&mut Transform, With<TextboxBackground>>,
    // player: Query<&Transform, (With<Player>, Without<TextboxBackground>)>,
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

// Hide all textbox
fn hide_textbox(
    mut commands: Commands,
    query: Query<Entity, With<Textbox>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}