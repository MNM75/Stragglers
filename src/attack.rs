use bevy::prelude::*;
use rand::prelude::*;
use crate::GameState;

use crate::player::Player;
use crate::enemy::Enemy;

use crate::player::PlayerStats;
use crate::enemy::EnemyStats;


pub fn choose_attack(
    player_stat_query: &mut Query<&mut PlayerStats, With<Player>>,
    enemy_stat_query: &mut Query<&mut EnemyStats, With<Enemy>>,
)
-> u32
{
    let enemy_stats = match enemy_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };
    let player_stats = match player_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };
    match enemy_stat_query.single().etype {
        1 => rand_attack(),
        2 => ai_attack(player_stat_query, enemy_stat_query),
        _ => 0,
    }
}

fn rand_attack()
-> u32
{
    let attack = rand::thread_rng().gen_range(0..100);
    return attack;
        //possibly add if statement for healing
}

fn ai_attack(
    player_stat_query: &mut Query<&mut PlayerStats, With<Player>>,
    enemy_stat_query: &mut Query<&mut EnemyStats, With<Enemy>>,
)
-> u32
{
    let enemy_stats = match enemy_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };
    let player_stats = match player_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };
    return 2;
}