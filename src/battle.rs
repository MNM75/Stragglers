use bevy::prelude::*;
use rand::prelude::*;
use crate::GameState;

use crate::player::PlayerStats;

use crate::enemy::EnemyStats;


use crate::player::Player;
use crate::enemy::Enemy;
use crate::enemy::despawn_closest_enemy;

pub struct BattlePlugin;


impl Plugin for BattlePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, battle_input.run_if(in_state(GameState::BattleMode)));
    }
}

// turn tracker function goes here
/*
fn turn_tracker(
    // get game state
    // get turn state
) {
}
*/

fn battle_input(
    /* for input */
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,

    mut player_stat_query: Query<&mut PlayerStats, With<Player>>,
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,

    // later: get the turn state

    /* for enemy despawn */
    commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // later: check it's the player's turn

        if input.just_pressed(KeyCode::Digit1) { //later on we can just if the different attacks there are and query player stats.
            if let Ok(mut enemy_stats) = enemy_stat_query.get_single_mut() {
                if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
   
                    // Simple attack that deals 10 damage to the enemy
                    let attack_dmg = physical_attack(5, player_stats.attack, enemy_stats.mgkatk);
                    enemy_stats.hp = enemy_stats.hp.saturating_sub(attack_dmg);
                    
                    info!("Enemy was attacked with sword for {} damage! Enemy HP is now: {}", attack_dmg, enemy_stats.hp);

                    if enemy_stats.hp <= 0 {
                        info!("Enemy defeated!");
                        despawn_closest_enemy(commands, enemy_query, player_query);  // Despawn the enemy if defeated
                        next_state.set(GameState::InGame);

                    } else {
                        enemy_attack(player_stat_query, enemy_stat_query);
                    }
                }
            }
            
        }
        else if input.just_pressed(KeyCode::Digit2) {
            //info!("magic attacked! but it had no effect...");
            if let Ok(mut enemy_stats) = enemy_stat_query.get_single_mut() {
                if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
   
                    // Simple attack that deals 10 damage to the enemy
                    let attack_dmg = magic_attack(5, player_stats.magic, enemy_stats.mgkdef);
                    enemy_stats.hp = enemy_stats.hp.saturating_sub(attack_dmg);
                    
                    info!("Enemy was attacked with magic for {} damage! Enemy HP is now: {}", attack_dmg, enemy_stats.hp);

                    if enemy_stats.hp <= 0 {
                        info!("Enemy defeated!");
                        despawn_closest_enemy(commands, enemy_query, player_query);  // Despawn the enemy if defeated
                        next_state.set(GameState::InGame);

                    } else {
                        enemy_attack(player_stat_query, enemy_stat_query);
                    }
                }
            }
           
        }
        else if input.just_pressed(KeyCode::Digit3) {
            battle_heal(player_stat_query);
            info!("press 5 to end turn");
            // later: change turn state here
            //enemy_attack(player_stat_query, enemy_stat_query);

        }
        else if input.just_pressed(KeyCode::Digit4) {
            info!("ran away");
            /* change game state to over world */
            match state.get() {
                GameState::BattleMode => next_state.set(GameState::InGame),
                GameState::InGame => next_state.set(GameState::InGame),
                GameState::SkillTreeMenu => next_state.set(GameState::SkillTreeMenu), // no op?
                GameState::EndCredits => next_state.set(GameState::EndCredits),
            }
            despawn_closest_enemy(commands, enemy_query, player_query);
        /* else do nothing until player selects a valid battle option */
        } else if input.just_pressed(KeyCode::Digit5){
            enemy_attack(player_stat_query, enemy_stat_query);
        }

     
    }
        


fn battle_heal(
    mut player_stat_query: Query<&mut PlayerStats, With<Player>>,
) {
    if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
        let current_hp = player_stats.hp;
        let max_hp = player_stats.max_hp;
        let heal_amt = heal(4, player_stats.magic); // get the heal amount (just a flat 5 hp for now)
        player_stats.hp = current_hp + heal_amt.clamp(0, max_hp - current_hp);
        info!("Player healed! Player hp is now: {}", player_stats.hp);
    }
}

fn enemy_attack(
    mut player_stat_query: Query<&mut PlayerStats, With<Player>>,
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,
) {
    let rand: usize = random();
    let attack = rand %3; 
    //info!("attack value: {}", attack);
    let mut enemy_damage = 0;
    if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
        if let Ok(mut enemy_stats) = enemy_stat_query.get_single_mut() {
            if (attack == 0){
                enemy_damage = physical_attack(5, enemy_stats.physatk, player_stats.physDefense);
                player_stats.hp = player_stats.hp.saturating_sub(enemy_damage);
                info!("Enemy bit you for {} damage! Player HP is now: {}",enemy_damage, player_stats.hp);
            } else if (attack == 1){
                enemy_damage = magic_attack(5, enemy_stats.mgkatk, player_stats.mgkDefense);
                player_stats.hp = player_stats.hp.saturating_sub(enemy_damage);
                info!("Enemy hit you with a psychic force for {} damage! Player HP is now: {}",enemy_damage, player_stats.hp);
            } else if (attack == 2){
                enemy_heal(enemy_stat_query);
                //player_stats.hp = player_stats.hp.saturating_sub(enemy_damage);
                
            }
        }       
        
    }
}

fn enemy_heal(
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,
) {
    if let Ok(mut enemy_stats) = enemy_stat_query.get_single_mut() {
        let current_hp = enemy_stats.hp;
        let max_hp = enemy_stats.max_hp;
        let heal_amt = heal(4, enemy_stats.mgkatk); // get the heal amount (just a flat 5 hp for now)
        enemy_stats.hp = current_hp + heal_amt.clamp(0, max_hp - current_hp);
        info!("Enemy healed! Enemy hp is now: {}", enemy_stats.hp);
    }
}

fn physical_attack(base_damage: u32,physical_attack: u32, physical_defense: u32) -> u32{
    let mut final_dmg: u32 = 0;
    let num = rand::thread_rng().gen_range(75..125);
    //attack
    final_dmg = ((base_damage as f64)*(((num as f64)/100.0)*(1.0+(physical_attack as f64)/10.0))) as u32;
    //defend
    final_dmg =((final_dmg as f64)*(1.0+0.5*((physical_defense as f64)/10.0))) as u32;

    return final_dmg;
}

fn magic_attack(base_damage: u32,magic_attack: u32, magic_defense: u32) -> u32{
    let mut final_dmg: u32 = 0;

    //defend
    let num = rand::thread_rng().gen_range(0..100);
    let magic_contest =(((magic_attack-magic_defense+10) as f64)*5.0+25.0) as u32; 

    if(num<magic_contest){
        final_dmg = ((base_damage as f64)*(1.0+(magic_attack as f64)/10.0)) as u32;
    }

    return final_dmg;
}

fn heal(base_heal: u32,magic_attack: u32) -> u32{
    let final_heal: u32 = ((base_heal as f64)*(1.0+((magic_attack as f64)/10.0)))as u32;

    return final_heal;
}