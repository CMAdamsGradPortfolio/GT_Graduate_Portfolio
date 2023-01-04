use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::{Actionlike, prelude::*};

// PLAYER BUNDLES

#[derive(Default, Bundle)]
pub struct PlayerParentBundle {
    pub player: Player,
    pub inventory: Inventory,
    pub body_parts: BodyParts,
    pub arm_vec: ArmVec,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBodyPartBundle {
    pub body_part: BodyPart,
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
}

// PLAYER COMPONENTS

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub enum Player {
    Moving,
    Interacting
}

impl Default for Player {
    fn default() -> Self {
        let new = Player::Moving;
        new
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub enum BodyPart {
    Body,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg, 
    Head
}

impl Default for BodyPart {
    fn default() -> Self {
        BodyPart::Body
    }
}


#[derive(Clone, Eq, PartialEq, Debug, Component)]
pub struct BodyParts {
    pub body_parts: Vec<BodyPart>,
    pub current_part: BodyPart,
    pub index: usize,
}

impl Default for BodyParts {
    fn default() -> Self {
        let mut new = BodyParts {
            body_parts: Vec::new(),
            current_part: BodyPart::Body,
            index: 0,
        };
        new.body_parts.push(new.current_part.clone());
        new
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct ArmVec(pub Vec2);

#[derive(Component)]
pub struct Inventory(pub [Option<Item>; 8]);

impl Default for Inventory {
    fn default() -> Self {
        let new = Inventory([
            None, None,
            None, None,
            None, None,
            None, None
            ]);
        new
    }
}

// INTERACTABLE BUNDLES

#[derive(Bundle, Clone, LdtkEntity)]
pub struct InteractableBundle {
    #[from_entity_instance]
    pub interactable: Interactable,
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
}

#[derive(Bundle, Clone, LdtkEntity)]
pub struct NpcBundle {
    #[from_entity_instance]
    pub interactable: Interactable,
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
}

impl Default for NpcBundle {
    fn default() -> Self {
        info!("Made NPC");
        NpcBundle {
            interactable: Interactable {
                can_interact: false,
                interaction_type: InteractionType::Person,
                person: Some(default()),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Bundle, Clone, LdtkEntity)]
pub struct InteractableObject {
    pub interactable: Interactable,
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
}

impl Default for InteractableObject {
    fn default() -> Self {
        InteractableObject {    
            interactable: Interactable { 
                can_interact: false,
                interaction_type: InteractionType::Item,  
                item: Some(default()), 
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Bundle, Clone, LdtkEntity)]
pub struct DoorObject {
    #[from_entity_instance]
    pub interactable: Interactable,
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,
}

impl Default for DoorObject {
    fn default() -> Self {
        DoorObject { 
            interactable: Interactable {  
                can_interact: false,
                interaction_type: InteractionType::Door,  
                door: Some(default()),
                ..default()
            },
            ..default()
        }
    }
}

// COMPONENTS

#[derive(Clone, Default, Component)]
pub struct Interactable {
    // sprite
    pub can_interact: bool,
    pub interaction_type: InteractionType,
    pub item: Option<Item>,
    pub puzzle: Option<Puzzle>,
    pub person: Option<Person>,
    pub door: Option<Door>,
}

impl From<EntityInstance> for Interactable {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "NPC_spawn" => Interactable {
                can_interact: false,
                interaction_type: InteractionType::Person,
                person: Some(default()),
                ..default()
            },
        
            "Gum_Machine" => Interactable {
                can_interact: false,
                interaction_type: InteractionType::Item,  
                item: Some(Item {
                    id: "Gumball".to_string(),
                }), 
                ..default()
            },
            
            "Vendor" => Interactable {
                can_interact: false,
                interaction_type: InteractionType::Item,  
                item: Some(default()), 
                ..default()
            },

            _ => Interactable::default(),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
}

impl Default for Item {
    fn default() -> Self {
        let new = Item{ id: String::from("") };
        new
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct DialogueChoice;

#[derive(Copy, Clone, Default, Component)]
pub struct Puzzle;

#[derive(Clone)]
pub struct Dialogue {
    pub prompt: String,
    pub finished: bool,

    pub choice1: Option<(String, usize)>,
    pub choice2: Option<(String, usize)>,
    pub choice3: Option<(String, usize)>,
}

#[derive(Clone)]
pub struct Person(pub Vec<Dialogue>);

impl Default for Person {
    fn default() -> Self {
        let new = Person(Vec::new());
        new
    }
}

#[derive(Clone)]
pub struct Door {
    pub requirements: Option<Vec<String>>,
    pub reusable: bool,
    pub closed: bool,
}

impl Default for Door {
    fn default() -> Self {
        Door {
            requirements: None,
            reusable: true,
            closed: true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum InteractionType {
    Item,
    Puzzle,
    Person,
    Door
}

impl Default for InteractionType {
    fn default() -> Self {
        InteractionType::Door
    }
}

#[derive(Bundle, Clone, Default, LdtkEntity)]
pub struct BackgroundObject {
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
}

// PLAYER ACTIONS

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Up,
    Down, 
    Left,
    Right,

    Interact,
    CycleForward,
    CycleBackward,

    Unused,
    Split,
}

// EVENTS

#[derive(Clone)]
pub struct Hotkeys(pub InputMap<Action>);
pub struct ItemInteraction(pub Item);
pub struct PuzzleInteraction(pub Puzzle);
pub struct PersonInteraction(pub Person, pub (usize, usize));

pub struct DoorInteraction(pub Door, pub Entity);
pub struct InteractionWrapper(pub Entity, pub InteractionType);

pub struct RoomChange(pub usize);
pub struct CameraSetupEvent;