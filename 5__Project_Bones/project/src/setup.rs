use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashSet;

use crate::components_events::*;
use crate::GameState;
use crate::settings::load_settings;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(ldtk_setup)
            
            .add_event::<Hotkeys>()
            .add_event::<CameraSetupEvent>()

            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(add_player_parent)
            )

            .add_system_set(
                SystemSet::on_enter(GameState::Setup)
                    .with_system(load_settings)
                    .with_system(camera_setup)
            )
    
            .add_system_set(
                SystemSet::on_update(GameState::Setup)
                    .with_system(hotkey_setup)
            );

            
    }
}

const LEVEL_IIDS: [&str; 4] = [
    "af5e2950-5110-11ed-befe-d36fb482da1b",
    "7bbce970-5110-11ed-befe-35cfe01a3e95",
    "514859e0-5110-11ed-aff0-7b38360eac55",
    "a93622c0-5110-11ed-8a11-77f4d92bb0a8"
];


fn ldtk_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let iids: HashSet<String> = LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect();

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("data\\levels\\floor_1.ldtk"),
        level_set: LevelSet { iids: iids },
        ..Default::default()
    });
    info!("Began loading asset");
}

fn add_player_parent(
    query: Query<Added<BodyPart>>,
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
) {
    for _query in query.iter() {
        commands.spawn_bundle(PlayerParentBundle::default());
        app_state.overwrite_set(GameState::Setup).unwrap();
        info!("Setup Player");
    }
}

fn camera_setup(
    mut commands: Commands,
    body_part: Query<&Transform, With<BodyPart>>,
) {
    let body_part = body_part.single();
        
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.35,
            ..default()
        },
        transform: *body_part,
        ..default()
    });
}

fn hotkey_setup(
    mut hotkey_event: EventReader<Hotkeys>,
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
) {
    for input_map in hotkey_event.iter() {
        let query = query.single();
        
        commands.entity(query).insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: input_map.0.clone(),
        });
        info!("Setup Hotkeys");
        app_state.overwrite_set(GameState::Running).unwrap();
    }  
}