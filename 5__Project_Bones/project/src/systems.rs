use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::components_events::*;
use crate::GameState;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InteractionWrapper>()
            .add_event::<PuzzleInteraction>()
            .add_event::<ItemInteraction>()
            .add_event::<PersonInteraction>()
            .add_event::<DoorInteraction>()

            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(detect_interactables)
                    .with_system(broadcast_interactions)
                    .with_system(puzzle_interaction)
                    .with_system(item_interaction)
                    .with_system(dialogue_receive)
                    .with_system(dialogue_send)
                    .with_system(action_debug)
                    .with_system(interact)   
                    //.with_system(update_level_selection)
            );
    }
}

pub fn action_debug(
    action_state: Query<&ActionState<Action>, With<Player>>,
) {
    let action_state = action_state.single();
    if action_state.just_pressed(Action::Up) {
        info!("Pressed Up!");
    }

    if action_state.just_pressed(Action::Down) {
        info!("Pressed Down!");
    }
    
    if action_state.just_pressed(Action::Left) {
        info!("Pressed Left!");
    }

    if action_state.just_pressed(Action::Right) {
        info!("Pressed Right!");
    }

    if action_state.just_pressed(Action::Interact) {
        info!("Pressed Interact!");
    }

    if action_state.just_pressed(Action::CycleForward) {
        info!("Pressed CycleForward!");
    }

    if action_state.just_pressed(Action::CycleBackward) {
        info!("Pressed CycleBackward!");
    }
}

pub fn detect_interactables(
    mut query: Query<(&mut Interactable, &Transform)>,
    body_parts: Query<(&Transform, &BodyPart)>,
    parent: Query<&BodyParts>,
) {
    let parent = parent.single();
    
    for body_part in body_parts.iter() {
        for mut interactable in query.iter_mut() {
            if *body_part.1 == parent.current_part {
                let proximity = (
                    (interactable.1.translation.x - body_part.0.translation.x).powi(2) + 
                    (interactable.1.translation.y - body_part.0.translation.y).powi(2)
                ).sqrt();
                if proximity <= 20.0 {
                    interactable.0.can_interact = true;
                } else {
                    interactable.0.can_interact = false;
                }
            }
        }
    }
}

pub fn interact(
    query: Query<(Entity, &Interactable)>,
    action_state: Query<&ActionState<Action>, With<Player>>,
    mut interact_event: EventWriter<InteractionWrapper>,
) {
    let action_state = action_state.single();
    
    if action_state.just_pressed(Action::Interact) {
        for query in query.iter() {
            if query.1.can_interact == true {
                interact_event.send(InteractionWrapper(query.0, query.1.interaction_type.clone()));
            }
        }
    }
}

pub fn broadcast_interactions(
    mut interact_event: EventReader<InteractionWrapper>,
    mut item_interaction: EventWriter<ItemInteraction>,
    mut puzzle_interaction: EventWriter<PuzzleInteraction>,
    mut person_interaction: EventWriter<PersonInteraction>,
    mut door_interaction: EventWriter<DoorInteraction>
) {
    for event in interact_event.iter() {
        match event.1 {
            InteractionType::Item => {
                info!("Interacted with a item!");
                item_interaction.send(ItemInteraction(Item::default()));
            },
            InteractionType::Puzzle => {
                info!("Interacted with a puzzle!");
                puzzle_interaction.send(PuzzleInteraction(Puzzle));
            },
            InteractionType::Person => {
                info!("Interacted with a person!");
                person_interaction.send(PersonInteraction(Person::default(), (0, 0)));
            },
            InteractionType::Door => {
                info!("Interacted with a door!");
                door_interaction.send(DoorInteraction(Door::default(), event.0))
            }
        }
    }
}

pub fn item_interaction(
    mut item_interactions: EventReader<ItemInteraction>,
    mut query: Query<&mut Inventory>
) {
    for item_interaction in item_interactions.iter() {
        let mut inventory = query.single_mut();
        for a in 0..inventory.0.len() {
            if inventory.0[a].is_none() {
                inventory.0[a] = Some(item_interaction.0.clone());
                break;
            }
        }
    }
}

pub fn puzzle_interaction(
   // mut puzzle_interaction: EventReader<PuzzleInteraction>
) {
    
}

pub fn dialogue_receive(
    mut person_receive: EventReader<PersonInteraction>,
) {
    for _statement in person_receive.iter() {
        
    }
}

// called on click of choice match what choise write new event
pub fn dialogue_send(
    //mut person_send: EventWriter<PersonInteraction>,
    //mut commands: Commands,
    //mut query: Query<Option<(Entity, With<DialogueChoice>)>>,
) {
    
    
    
    // Person (usize, usize)
    // (current conversation number, choice)

    //person_send.send(PersonInteraction(statement.0.clone(), (0, 0)));
}

pub fn door_interaction(
    mut door_interaction: EventReader<DoorInteraction>,
    mut query: Query<&mut Inventory>,
    mut commands: Commands,
    mut interactable: Query<&mut Interactable>
) {
    for door in door_interaction.iter() {
        let inventory = query.single_mut();
        if door.0.requirements.is_none() {
           info!("You opened the door!");
        } else {
            for requirement in door.0.requirements.as_ref().unwrap().iter() {
                for slot in inventory.0.iter() {
                    if slot.is_some() && *requirement == slot.as_ref().unwrap().id {
                        // door animation, audio
                        commands.entity(door.1);
                        if let Ok(mut interactable) = interactable.get_mut(door.1) {
                            interactable.door = Some(Door {
                                requirements: None,
                                reusable: door.0.reusable,
                                closed: false,
                            });
                        }
                        info!("You opened the door with the key(s)!");
                    }
                }
            }
        }
    }
}
