use bevy::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::player::PlayerStats;
use crate::enemy::EnemyStats;

#[derive(Resource)]
pub struct TurnOrder {
    action_queue: BinaryHeap<Reverse<(u32, Entity)>>, // Priority queue of (next_action_tick, entity)
    current_tick: u32, // Tracks the current turn count
}

impl TurnOrder {
    pub fn new() -> Self {
        TurnOrder {
            action_queue: BinaryHeap::new(),
            current_tick: 0,
        }
    }    

    // Adds a character (Player or Enemy) to the action queue
    pub fn add_character(&mut self, entity: Entity, speed: u32) {
        let delay = 100 - speed; // You can adjust this formula to tweak the speed calculation
        let next_action_tick = self.current_tick + delay;
        self.action_queue.push(Reverse((next_action_tick, entity)));
    }

    // Update turn order by processing the next character's turn
    pub fn update_turn_order(
        &mut self,
        mut player_query: Query<(Entity, &mut PlayerStats)>,
        mut enemy_query: Query<(Entity, &mut EnemyStats)>,
    ) {
        if let Some(Reverse((tick, entity))) = self.action_queue.pop() {
            self.current_tick = tick; // Update the current tick to the character's action time

            // If it's the player's turn
            if let Ok((player_entity, mut player_stats)) = player_query.get_mut(entity) {
                // Do player's turn action
                self.update_next_action_tick_for_player(&mut player_stats);

                // Update the action queue with the new tick
                self.action_queue.push(Reverse((
                    player_stats.next_action_tick,
                    player_entity,
                )));
            }

            // If it's the enemy's turn
            if let Ok((enemy_entity, mut enemy_stats)) = enemy_query.get_mut(entity) {
                // Do enemy's turn action
                self.update_next_action_tick_for_enemy(&mut enemy_stats);

                // Update the action queue with the new tick
                self.action_queue.push(Reverse((
                    enemy_stats.next_action_tick,
                    enemy_entity,
                )));
            }
        }
    }

    // Updates the player's next action tick based on their speed
    fn update_next_action_tick_for_player(&mut self, player_stats: &mut PlayerStats) {
        let delay = 100 - player_stats.spd; // Example calculation for player speed
        let next_action_tick = self.current_tick + delay;

        player_stats.next_action_tick = next_action_tick;
    }

    // Updates the enemy's next action tick based on their speed
    fn update_next_action_tick_for_enemy(&mut self, enemy_stats: &mut EnemyStats) {
        let delay = 100 - enemy_stats.speed; // Example calculation for enemy speed
        let next_action_tick = self.current_tick + delay;

        enemy_stats.next_action_tick = next_action_tick;
    }

    // Forces a character to act immediately (priority move)
    pub fn priority_move(&mut self, entity: Entity) {
        self.action_queue.push(Reverse((self.current_tick, entity)));
    }
}
