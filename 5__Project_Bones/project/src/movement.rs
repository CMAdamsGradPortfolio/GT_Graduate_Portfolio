use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{components_events::*, GameState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(player_movement)
                    .with_system(cycle_part)
                    .with_system(
                        camera_follow
                        .after(player_movement)
                    )
                    .with_system(detach_part)
            );
    }
}

const MOVE_SPEED: f32 = 2.;
const ARM_SPEED: f32 = 1.;

pub fn player_movement(
    action_state: Query<&ActionState<Action>, With<Player>>,
    query: Query<(&Player, &BodyParts)>,
    mut body_parts: Query<(&mut Transform, &BodyPart,)>,
    mut arm: Query<&mut ArmVec>,
    mouse_buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let player = query.single();
    let action_state = action_state.single();

    // Body Movement 
    if *player.0 == Player::Moving && player.1.current_part == BodyPart::Body {
        
        if action_state.pressed(Action::Up) {
            for mut body_part in body_parts.iter_mut() {
                if *body_part.1 == BodyPart::Body {
                    body_part.0.translation.y += MOVE_SPEED;
                }    
            } 
        }
        
        if action_state.pressed(Action::Left) {
            
            for mut body_part in body_parts.iter_mut() {
                if *body_part.1 == BodyPart::Body {
                    body_part.0.translation.x -= MOVE_SPEED; 
                } 
            }
        }
        
        if action_state.pressed(Action::Down) {
            
            for mut body_part in body_parts.iter_mut() {
                if *body_part.1 == BodyPart::Body {
                    body_part.0.translation.y -= MOVE_SPEED; 
                }  
            }         
        }
        
        if action_state.pressed(Action::Right) {
            
            for mut body_part in body_parts.iter_mut() {
                if *body_part.1 == BodyPart::Body {
                    body_part.0.translation.x += MOVE_SPEED; 
                }  
            }      
        }
    }

    // Arm Movement
    if *player.0 == Player::Moving && (player.1.current_part == BodyPart::LeftArm || player.1.current_part == BodyPart::RightArm) {

        let mut arm_vec = arm.single_mut();
        let window = windows.get_primary().unwrap();
        if mouse_buttons.just_pressed(MouseButton::Left) {
            arm_vec.0 = window.cursor_position().unwrap();
        }

        if mouse_buttons.just_released(MouseButton::Left) {
            let new_position =  window.cursor_position().unwrap();
            
            for mut body_part in body_parts.iter_mut() {
                if *body_part.1 == BodyPart::LeftArm || *body_part.1 == BodyPart::RightArm   {
                    body_part.0.translation.x -= ARM_SPEED * (new_position.x - arm_vec.0.x);
                    body_part.0.translation.y -= ARM_SPEED * (new_position.y - arm_vec.0.y);
                } 
            }
            
        }
    }

}

fn camera_follow(
    player: Query<&BodyParts, With<Player>>,
    body_parts: Query<(&GlobalTransform, &BodyPart), Without<Camera>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();

    for body_part in body_parts.iter() {
        if player.current_part == *body_part.1 {
            camera.translation.x = body_part.0.compute_transform().translation.x;
            camera.translation.y = body_part.0.compute_transform().translation.y;
        }
    }
  
}

pub fn cycle_part(
    action_state: Query<&ActionState<Action>, With<Player>>,
    mut query: Query<&mut BodyParts, With<Player>>,
    parts: Query<(&Transform, &BodyPart), Without<Camera>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut player = query.single_mut();
    let action_state = action_state.single();
    let mut camera = camera.single_mut();
    let max_size = player.body_parts.len() - 1;

    if action_state.just_pressed(Action::CycleForward) {
        
        if player.index + 1 > max_size {
            player.index = 0;
            player.current_part = player.body_parts[player.index].clone();
        } else {
            player.index += 1;
            player.current_part = player.body_parts[player.index].clone();
        }

        for part in parts.iter() {
            if player.current_part == *part.1 {
                camera.translation.x = part.0.translation.x;
                camera.translation.y = part.0.translation.y + 680.;
            }
        }
    }

    if action_state.just_pressed(Action::CycleBackward) {
        
        if player.index as i32 - 1 < 0 {
            player.index = max_size;
            player.current_part = player.body_parts[player.index].clone();
        } else {
            player.index -= 1;
            player.current_part = player.body_parts[player.index].clone();
        }

        for part in parts.iter() {
            if player.current_part == *part.1 {
                camera.translation.x = part.0.translation.x;
                camera.translation.y = part.0.translation.y + 680.;
            }
        }
    }
}

fn detach_part(
    mut commands: Commands,
    action_state: Query<&ActionState<Action>, With<Player>>,
    mut parent: Query<&mut BodyParts, With<Player>>,
    body_parts: Query<(&BodyPart, &Transform), Without<Camera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    let action_state = action_state.single();
    let mut parent = parent.single_mut();
    let mut camera = camera.single_mut();

    for body_part in body_parts.iter() {
        if *body_part.0 == BodyPart::Body && parent.current_part == BodyPart::Body {
            if action_state.just_pressed(Action::Split) {
        
                let x = body_part.1.translation.x - 20.0;
                let y = body_part.1.translation.y - 20.0;

                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(25.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
                        transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                        ..default()
                    })
                    .insert(BodyPart::LeftArm)
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::ball(25.0));
        
                parent.body_parts.push(BodyPart::LeftArm);
                parent.current_part = BodyPart::LeftArm;
                parent.index = parent.body_parts.len();

                camera.translation.x = x;
                camera.translation.y = y;
            }
        }
    } 
}
