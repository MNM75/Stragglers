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
    let attack = rand::thread_rng().gen_range(0..100)%3;
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

    let mut physAttackOp = 0;
    let mut magAttackOp = 0;
    let mut healOp = 0;

    let mut temp: i32 = (enemy_stats.physatk as i32) - (enemy_stats.mgkatk as i32);
    if (temp > 0) {
        physAttackOp += (temp/2);
    } else if (temp<0) {
        magAttackOp += (temp*-1)/2;
    }

    temp = (player_stats.mdef as i32) - (player_stats.def as i32);
    if (temp > 0) {
        physAttackOp += (temp/2);
    } else if (temp < 0) {
        magAttackOp += (temp*-1)/2;
    }

    let mut temp_hp = (enemy_stats.hp as f32)/(enemy_stats.max_hp as f32);
    if (temp_hp >= 0.9){
        healOp = -999;
    } else if (temp_hp> 0.6) {
        healOp = healOp+1;
    } else if (temp_hp> 0.3) {
        healOp = healOp+2;
    } else {
        healOp = healOp+6;
    }

    temp_hp = (player_stats.hp as f32)/(player_stats.max_hp as f32);
    if (temp_hp < 0.2){
        physAttackOp += 10;
        magAttackOp += 10;
    }

    if (physAttackOp >= magAttackOp && physAttackOp >= healOp) {
        return 0;
    } else if (magAttackOp >= healOp) {
        return 1;
    } else {
        return 2;
    }
}