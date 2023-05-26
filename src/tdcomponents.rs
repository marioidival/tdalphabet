#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Score(usize);

#[derive(Component)]
pub struct Gold(usize);

/// Goal: the mob's goal or something that the Player should protect
#[derive(Component)]
pub struct Goal;

/// HP: could be used by: Goal, Mob
#[derive(Component, Default, Clone)]
pub struct HP {
    pub current: usize,
    pub max: usize,
}

#[derive(Component)]
pub struct Bullet {
    pub target: Entity,
}

#[derive(Bundle)]
pub struct TowerBundle {
    pub stats: TowerStats,
    pub kind: TowerKind,
}

/// TowerStats: Some ideas:
#[derive(Component)]
pub struct TowerStats {
    pub range: f32,
    pub speed: f32,
    pub damage: usize,
    /// - Level: 1-8, update with gold
    pub level: u8,
}

#[derive(Component)]
pub enum TowerKind {
    Air,
    Aura,
    Eletric,
    Fire,
    Frost,
    Multishot,
    Posion,
}
