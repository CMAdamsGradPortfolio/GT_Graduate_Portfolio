use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::{
    prelude::*,
    window::close_on_esc,
};
use project_bones::{
    GameState,
    components_events::*,
    systems::InteractionsPlugin,
    movement::MovementPlugin, 
    setup::*,
};

fn main() {
    App::new()
        // Basic Plugins, Setup/Loading Logic
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_state(GameState::Loading)
        .add_plugin(SetupPlugin)
        
        // ldtk configuring
        .register_ldtk_entity::<PlayerBodyPartBundle>("Player_start")

        .register_ldtk_entity::<InteractableBundle>("NPC_spawn")
        .register_ldtk_entity::<InteractableBundle>("Vendor")
        .register_ldtk_entity::<InteractableBundle>("Gum_Machine")

        .register_ldtk_entity::<BackgroundObject>("Banner")
        .register_ldtk_entity::<BackgroundObject>("Stool")
        .register_ldtk_entity::<BackgroundObject>("Coffee_Table")

        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: true, },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        
        .add_system(close_on_esc)
        
        // Runtime Plugins
        .add_plugin(MovementPlugin)
        .add_plugin(InteractionsPlugin)

        .run();


}
