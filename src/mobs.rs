use bevy::prelude::*;

use crate::tdcomponents::HP;


#[derive(Bundle, Default, Clone)]
pub struct MobBundle {
    pub kind: MobKind,
    pub stats: MobStats,
    pub rank: MobRank,
    pub path: MobPath,
}

#[derive(Component, Default, Clone)]
pub struct MobStats {
    pub hp: HP,
    pub armor: i32,
    pub speed: f32,
}

#[derive(Component)]
pub struct MobAnimationTimer(pub Timer);

impl Default for MobAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Component, Default, Clone)]
pub enum MobKind {
    Air,
    Ice,
    Fire,
    Wizard,
    Posion,
    #[default]
    Normal,
}

#[derive(Component, Default, Clone)]
pub struct MobPath {
    pub path: Vec<Vec2>,
    pub side: u8,
    pub path_index: usize,
}

/// MobRank: used to define MobStats
#[derive(Component, Default, Clone)]
pub enum MobRank {
    /// Normal  - x1
    #[default]
    Normal,
    /// Captain - x3
    Captain,
    /// General - x6
    General,
}

pub fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MobPath, &MobRank, &MobStats)>,
) {
    for (mut transform, mut path, _rank, stats) in query.iter_mut() {
        if path.path_index >= path.path.len() - 1 {
            continue;
        }

        let next_waypoint = path.path[path.path_index + 1];
        let diff = next_waypoint - transform.translation.truncate();
        let dist = diff.length();

        let step = stats.speed * time.delta_seconds();
        if step < dist {
            transform.translation.x +=
                step / dist * (next_waypoint.x - transform.translation.x);
            transform.translation.y +=
                step / dist * (next_waypoint.y - transform.translation.y);
        } else {
            transform.translation.x = next_waypoint.x;
            transform.translation.y = next_waypoint.y;
            path.path_index += 1;
        }
    }
}

#[derive(Component)]
pub struct FakeMob;

pub fn spawn_mob(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-110.66701, -236.66667, 3.0),
            texture: asset_server.load("sprites/boneco.png"),
            ..default()
        },
        FakeMob {}
    ));
}

