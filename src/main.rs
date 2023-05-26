use std::collections::hash_map::RandomState;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use loading::MapHandles;

use crate::waves::{Wave, WaveStats, Waves};

pub mod loading;
pub mod mobs;
pub mod tdcomponents;
pub mod tdconstants;
pub mod tdtiled;
pub mod waves;

type TDHashMap<K, V> = HashMap<K, V, RandomState>;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TDState {
    Load,
    #[default]
    Spawn,
    Playing,
    GameOver,
}

fn startup_tiled(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_hundle: Handle<tdtiled::TiledMap> = asset_server.load("map_h.tmx");

    commands.spawn(tdtiled::TiledMapBundle {
        tiled_map: map_hundle,
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct Player {}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 3.0),
            texture: asset_server.load("sprites/boneco.png"),
            ..default()
        },
        Player {},
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * tdconstants::PLAYER_SPEED * time.delta_seconds();
    }
}

fn spawn_check(mut next_state: ResMut<NextState<TDState>>, waves: Res<Waves>) {
    if waves.list.is_empty() {
        return;
    }

    println!("change to playing");
    next_state.set(TDState::Playing);
}

fn spawn_map_objects(
    mut commands: Commands,
    mut waves: ResMut<Waves>,
    map_handles: Res<MapHandles>,
    maps: Res<Assets<tdtiled::TiledMap>>,
) {
    let tiled_map = match maps.get(&map_handles.map_h) {
        Some(map) => map,
        None => panic!("I can't load the MAP"),
    };

    println!("map loaded");

    let paths: TDHashMap<i32, Vec<Vec2>> = tiled_map
        .map
        .layers()
        .filter_map(|layer| match layer.layer_type() {
            tiled::LayerType::Objects(layer) => Some(layer),
            _ => None,
        })
        .flat_map(|layer| layer.objects())
        .filter(|object| object.user_type == "enemy_path")
        .filter_map(|object| {
            let (points, index) = match (&object.shape, object.properties.get(&"index".to_string()))
            {
                (
                    tiled::ObjectShape::Polyline { points },
                    Some(tiled::PropertyValue::IntValue(index)),
                ) => (points, index),
                (
                    tiled::ObjectShape::Polygon { points },
                    Some(tiled::PropertyValue::IntValue(index)),
                ) => (points, index),
                _ => return None,
            };

            let transformed: Vec<Vec2> = points
                .iter()
                .map(|(x, y)| {
                    let transform = map_to_world(
                        tiled_map,
                        Vec2::new(*x, *y) + Vec2::new(object.x, object.y),
                        Vec2::ZERO,
                        0.,
                    );
                    transform.translation.truncate()
                })
                .collect();

            Some((*index, transformed))
        })
        .collect();

    let w = Wave::new(&paths);
    println!("pushed Wave");
    waves.list.push(w);
    commands.insert_resource(WaveStats::from(waves.current().unwrap()));
}

fn map_to_world(map: &tdtiled::TiledMap, pos: Vec2, size: Vec2, z: f32) -> Transform {
    let mut transform = Transform::default();

    let map_height = map.map.height * map.map.tile_height;
    let map_width = map.map.width * map.map.tile_width;

    transform.translation.x -= map_width as f32 / 2. - pos.x - size.x / 2.;
    transform.translation.y -= map_height as f32 / 2. - pos.y - size.x / 2.;
    transform.translation.z = z;

    transform
}

fn main() {
    let mut app = App::new();

    app.add_state::<TDState>();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TD Alphabet".to_string(),
                    resolution: [480., 480.].into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            }),
    );

    app.init_collection::<loading::MapHandles>();
    app.insert_resource(Waves::default())
        .insert_resource(WaveStats::default());

    app.add_plugin(TilemapPlugin)
        .add_plugin(tdtiled::TiledMapPlugin)
        .add_plugin(waves::WavePlugin);

    app.add_startup_system(startup_tiled)
        .add_startup_system(mobs::spawn_mob)
        .add_startup_system(spawn_player);

    app.add_system(spawn_map_objects.run_if(in_state(TDState::Spawn)))
        .add_system(player_movement)
        .add_system(mobs::movement)
        .add_system(spawn_check.in_set(OnUpdate(TDState::Spawn)))
        .add_system(bevy::window::close_on_esc);

    app.run();
}
