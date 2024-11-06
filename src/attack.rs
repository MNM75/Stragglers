use bevy::prelude::*;
use rand::prelude::*;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::player::PlayerStats;
use crate::enemy::EnemyStats;

// BattleStats Component is no longer needed, commented out parts
#[derive(Component)]
pub struct BattleStats {
    pub phys_def: u32,
    pub mag_def: u32,
}

impl BattleStats {
    pub fn new() -> Self {
        Self {
            phys_def: 0,
            mag_def: 0,
        }
    }

    pub fn set_phys_def(&mut self, value: u32) {
        self.phys_def = value;
    }

    pub fn set_mag_def(&mut self, value: u32) {
        self.mag_def = value;
    }
}

// The function signature no longer needs BattleStats
pub fn choose_attack(
    mut player_stat_query: Query<&mut PlayerStats, With<Player>>, 
    mut enemy_stat_query: Query<&mut EnemyStats, With<Enemy>>,
) -> u32 {
    let mut enemy_stats = match enemy_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };
    let mut player_stats = match player_stat_query.get_single_mut() {
        Ok(stats) => stats,
        Err(_) => return 0,
    };

    match enemy_stats.etype {
        1 => rand_attack(),
        2 => ai_attack(&mut player_stats, &mut enemy_stats),
        _ => 0,
    }
}

fn rand_attack() -> u32 {
    rand::thread_rng().gen_range(0..100)
}

// ai_attack doesn't require BattleStats anymore
pub fn ai_attack(
    player_stats: &mut PlayerStats,
    enemy_stats: &mut EnemyStats,
) -> u32 {
    let heal_prior = 1.0 - (enemy_stats.hp as f32 / enemy_stats.max_hp as f32);
    let atk_prior = 1.0 - (player_stats.hp as f32 / player_stats.max_hp as f32);

    if heal_prior > atk_prior {
        return 2; // Healing action
    } else {
        let phys_prior = enemy_stats.physatk - player_stats.physDefense;
        let mag_prior = enemy_stats.mgkatk - player_stats.mgkDefense;

        if phys_prior >= mag_prior {
            return 0; // Physical attack
        } else {
            return 1; // Magical attack
        }
    }
}
