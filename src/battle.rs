use bevy::prelude::*;
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
            // Simple attack that deals 10 damage to the enemy
            let base_damage = 10;
            enemy_stats.hp = enemy_stats.hp.saturating_sub(base_damage);
            
            info!("Enemy attacked for {} damage! Enemy HP is now: {}", base_damage, enemy_stats.hp);

            if enemy_stats.hp <= 0 {
                info!("Enemy defeated!");
                despawn_closest_enemy(commands, enemy_query, player_query);  // Despawn the enemy if defeated
            }
        }

    if input.just_pressed(KeyCode::Digit1) {
        info!("attacked! but it had no effect...");
        /* attack */

        // later: change turn state here
    }
    else if input.just_pressed(KeyCode::Digit2) {
        info!("magic attacked! but it had no effect...");
        /* magic */
        // later: change turn state here
    }
    else if input.just_pressed(KeyCode::Digit3) {
        info!("healing 1 hp");
        battle_heal(player_stat_query);
        // later: change turn state here
    }
    else if input.just_pressed(KeyCode::Digit4) {
        info!("ran away");
        /* change game state to over world */
        match state.get() {
            GameState::BattleMode => next_state.set(GameState::InGame),
            GameState::InGame => next_state.set(GameState::InGame),
            GameState::SkillTreeMenu => next_state.set(GameState::SkillTreeMenu), // no op?
        }
        despawn_closest_enemy(commands, enemy_query, player_query);
    /* else do nothing until player selects a valid battle option */
    }
}
}

fn battle_heal(
    mut player_stat_query: Query<&mut PlayerStats, With<Player>>,
) {
    if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
        let current_hp = player_stats.hp;
        let max_hp = player_stats.max_hp;
        let heal_amt = 1; // get the heal amount (just a flat 1 hp for now)
        player_stats.hp = current_hp + heal_amt.clamp(0, max_hp - current_hp);
    }
}