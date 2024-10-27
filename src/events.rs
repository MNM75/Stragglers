use bevy::prelude::*;

// collision event
#[derive(Event)]
pub struct EnemyCollisionEvent;

// end game event
#[derive(Event)]
pub struct EndGameEvent;