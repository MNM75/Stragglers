use bevy::prelude::*;
use rand::prelude::*;

use crate::player::Player;
use crate::enemy::Enemy;

use crate::player::PlayerStats;
use crate::enemy::EnemyStats;

pub fn chooseMove(
    player_stat_query: &mut Query<&mut PlayerStats, With<Player>>,
    enemy_stat_query: &mut Query<&mut EnemyStats, With<Enemy>>,
) -> u32{
    if let Ok(mut player_stats) = player_stat_query.get_single_mut() {
        if let Ok(mut enemy_stats) = enemy_stat_query.get_single_mut() {
            let mut tempEhp = enemy_stats.hp;
            let mut tempPhp = player_stats.hp - physatk(enemy_stats.physatk, player_stats.def);


            let phys = recursive(0, tempPhp, enemy_stats.hp, player_stats.max_hp, enemy_stats.max_hp, player_stats.atk, enemy_stats.physatk, player_stats.def, enemy_stats.physdef, player_stats.matk, enemy_stats.mgkatk, player_stats.mdef, enemy_stats.mgkdef);

            tempPhp = player_stats.hp - mgkatk(enemy_stats.mgkatk, player_stats.mdef);
            let mag = recursive(0, tempPhp, enemy_stats.hp, player_stats.max_hp, enemy_stats.max_hp, player_stats.atk, enemy_stats.physatk, player_stats.def, enemy_stats.physdef, player_stats.matk, enemy_stats.mgkatk, player_stats.mdef, enemy_stats.mgkdef);

            tempEhp = enemy_stats.hp + heal(player_stats.matk);
            let heal = recursive(0, player_stats.hp, tempEhp, player_stats.max_hp, enemy_stats.max_hp, player_stats.atk, enemy_stats.physatk, player_stats.def, enemy_stats.physdef, player_stats.matk, enemy_stats.mgkatk, player_stats.mdef, enemy_stats.mgkdef);

            if (phys>mag && phys>heal){
                return 0;
            } else if (mag>heal) {
                return 1;
            } else {
                return 2;
            }
        }
    }
    return 0;
}

pub fn recursive(mut rounds: u32,playerhp: u32, enemyhp: u32, playermaxhp: u32, enemymaxhp: u32, playeratk:u32, enemyatk: u32, playerdef: u32, enemydef: u32, playermatk:u32, enemymatk: u32,  playermdef: u32, enemymdef: u32) -> f64{
            rounds += 1;
            println!("{}",rounds);

            let playermove = player_predictor();
            let mut tempenemyhp = enemyhp;
            let mut tempplayerhp = playerhp;
            if playermove == 1{
                tempenemyhp -= physatk(playeratk, enemydef);
            } else if playermove == 2{
                tempenemyhp -= mgkatk(playeratk, enemymdef);
            } else{
                tempplayerhp += heal(playermatk);
            }

            if(playerhp <= 0){
                return 1.0;
            } else if(enemyhp <= 0){
                return 0.0;
            } else if(rounds >= 10){
            return (0.5 - 0.5*(playerhp as f64)/(playermaxhp as f64) + 0.5*(enemyhp as f64)/(enemymaxhp as f64));
            }

            //phys atk
            tempplayerhp = playerhp - physatk(enemyatk, playerdef);

            let physatk = recursive(rounds, tempplayerhp, tempenemyhp, playermaxhp, enemymaxhp, playeratk, enemyatk, playerdef, enemydef, playermatk, enemymatk, playermdef, enemymdef);


            //mgk atk
            tempenemyhp = enemyhp;
            tempplayerhp = playerhp - mgkatk(enemymatk, playermdef);

           let mgkatk = recursive(rounds, tempplayerhp, tempenemyhp, playermaxhp, enemymaxhp, playeratk, enemyatk, playerdef, enemydef, playermatk, enemymatk, playermdef, enemymdef);


            //heal
            tempenemyhp = enemyhp + heal(enemymatk);
            tempplayerhp = playerhp;

            let heal = recursive(rounds, tempplayerhp, tempenemyhp, playermaxhp, enemymaxhp, playeratk, enemyatk, playerdef, enemydef, playermatk, enemymatk, playermdef, enemymdef);


            return (physatk + mgkatk + heal)/3.0;
        }

pub fn player_predictor() -> u32{
    return 1;
}

fn physatk(physical_attack: u32, physical_defense: u32) -> u32{
    let base_damage = 5;
    let mut final_dmg: u32 = 0;
    let num = rand::thread_rng().gen_range(75..125);
    //attack
    final_dmg = ((base_damage as f64)*(((num as f64)/100.0)*(1.0+(physical_attack as f64)/10.0))) as u32;
    //defend
    final_dmg =((final_dmg as f64)*(1.0+0.5*((physical_defense as f64)/10.0))) as u32;

    return final_dmg;
}

fn mgkatk(magic_attack: u32, magic_defense: u32) -> u32{
    let base_damage = 5;
    let mut final_dmg: u32 = 0;

    //defend
    let num = rand::thread_rng().gen_range(0..100);
    let magic_contest =((((magic_attack as f64)-(magic_defense as f64)+10.0))*5.0+25.0) as u32; 

    if(num<magic_contest){
        final_dmg = ((base_damage as f64)*(1.0+(magic_attack as f64)/10.0)) as u32;
    }

    return final_dmg;
}

fn heal(magic_attack: u32) -> u32{
    let base_heal = 4;
    let final_heal: u32 = ((base_heal as f64)*(1.0+((magic_attack as f64)/10.0)))as u32;

    return final_heal;
}