pub mod movement;
pub mod components_events;
pub mod systems;
pub mod settings;
pub mod setup;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Setup,
    CameraSetup,
    MainMenu,
    Running,
    Pause,
}