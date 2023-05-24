use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
pub mod tiled;

fn startup_tiled(mut commands: Commands, _window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    // let window = window_query.get_single().unwrap();
    //
    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());

    let map_hundle: Handle<tiled::TiledMap> = asset_server.load("map_h.tmx");

    commands.spawn(tiled::TiledMapBundle {
        tiled_map: map_hundle,
        ..Default::default()
    });
}

/// ECS: Entity / Component / System
/// Entity: Uma coleção de componentes, uma Pessoa tem position e life
/// Component: alguma caracterista da Entity: Position, Velocity
/// System: Logicas que executam em um conjunto especifico de componentes; Tambem podem ler e
/// modificar `Resources`
/// Resource: pode ser um estado do jogo, regras do jogo

// Bundle sao como templates, que podem facilitar a criacao de entidades com um conjunto em comun
// de components
// #[derive(Bundle)]
// struct PlayerBundle {
//     xp: PlayerXP,
//     name: PlayerName,
//     health: Health,
//     _p: Player,
//
//     // add a nest bundle
//     #[bundle]
//     sprite: SpriteSheetBundle,
// }
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TD Alphabet".to_string(),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(tiled::TiledMapPlugin)
        .add_startup_system(startup_tiled)
        .add_system(bevy::window::close_on_esc)
        .run();
}
