use bevy::prelude::*;

use crate::{
    mobs::{MobBundle, MobKind, MobPath, MobRank, MobStats},
    tdcomponents::HP,
    TDHashMap, TDState,
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Waves::default())
            .insert_resource(WaveStats::default());

        app.add_system(spawn_mobs.in_set(OnUpdate(TDState::Playing)));
    }
}

#[derive(Component)]
pub struct Wave {
    pub path: Vec<Vec2>,
    pub enemy: MobBundle,
    pub interval: f32,
    pub delay: f32,
}

impl Wave {
    pub fn new(paths: &TDHashMap<i32, Vec<Vec2>>) -> Wave {
        let side = paths.len() - 1;
        let path = paths.get(&(side as i32)).unwrap().to_vec();

        let enemy_bundle = MobBundle {
            kind: MobKind::Ice,
            stats: MobStats {
                hp: HP {
                    current: 10,
                    max: 10,
                },
                armor: 1,
                speed: 50.,
            },
            rank: MobRank::Normal,
            path: MobPath {
                path: path.clone(),
                side: side as u8,
                path_index: 0,
            },
            ..Default::default()
        };

        Wave {
            path: path.clone(),
            enemy: enemy_bundle,
            interval: 1.5,
            delay: 1.,
        }
    }
}

#[derive(Resource)]
pub struct WaveStats {
    pub delay_timer: Timer,
    pub spawn_timer: Timer,
    pub remaining: usize,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            delay_timer: Timer::from_seconds(1., TimerMode::Once),
            spawn_timer: Timer::from_seconds(1., TimerMode::Repeating),
            remaining: 0,
        }
    }
}

impl From<&Wave> for WaveStats {
    fn from(value: &Wave) -> Self {
        Self {
            delay_timer: Timer::from_seconds(value.delay, TimerMode::Once),
            spawn_timer: Timer::from_seconds(value.interval, TimerMode::Repeating),
            remaining: 10,
        }
    }
}

#[derive(Resource, Default)]
pub struct Waves {
    pub list: Vec<Wave>,
    current: usize,
}

impl Waves {
    pub fn current(&self) -> Option<&Wave> {
        self.list.get(self.current)
    }

    pub fn advance(&mut self) -> Option<&Wave> {
        self.current += 1;
        self.current()
    }
}

pub fn spawn_mobs(
    mut commands: Commands,
    mut waves: ResMut<Waves>,
    mut wave_stat: ResMut<WaveStats>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let Some(current_wave) = waves.current() else {
        return;
    };

    wave_stat.delay_timer.tick(time.delta());
    if !wave_stat.delay_timer.finished() {
        return;
    };

    wave_stat.spawn_timer.tick(time.delta());
    if !wave_stat.spawn_timer.just_finished() {
        return;
    };

    let path = current_wave.path.clone();
    let point = path[0];

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(point.x, point.y, 5.0),
            texture: asset_server.load("sprites/boneco.png"),
            ..default()
        },
        waves.current().unwrap().enemy.clone(),
    ));

    wave_stat.remaining -= 1;

    if wave_stat.remaining == 0 {
        if let Some(next) = waves.advance() {
            commands.insert_resource(WaveStats::from(next))
        }
    }
}
