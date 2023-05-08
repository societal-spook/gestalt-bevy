use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

pub fn character_move(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut characters: Query<&mut KinematicCharacterController>,
    cameras: Query<&Transform, With<Camera3d>>,
) {
    let mut character = characters.single_mut();

    let mut local_z = Vec3::ZERO;
    for camera in cameras.iter() {
        local_z = camera.local_z()
    }

    let mut velocity = Vec3::ZERO;
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::W => {
                velocity += forward;
            }
            KeyCode::S => {
                velocity -= forward;
            }
            KeyCode::D => {
                velocity += right;
            }
            KeyCode::A => {
                velocity -= right;
            }
            _ => {}
        }
    }
    velocity -= Vec3::new(0.0, 1.0, 0.0);
    velocity = velocity.normalize_or_zero();
    character.translation = Some(velocity * time.delta_seconds() * 5.0);
}
