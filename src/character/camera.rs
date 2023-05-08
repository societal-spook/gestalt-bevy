use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::KinematicCharacterController;

const CAMERA_OFFSET: Vec3 = Vec3 {
    x: 0.0,
    y: 0.75,
    z: 0.0,
};

pub fn camera_move(
    characters: Query<&Transform, With<KinematicCharacterController>>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<KinematicCharacterController>)>,
) {
    let character = characters.single();
    let mut camera = cameras.single_mut();

    camera.translation = character.translation + CAMERA_OFFSET;
}

pub fn camera_look(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    let mut camera = cameras.single_mut();

    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }
    mouse_delta *= 0.1;

    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    pitch -= (mouse_delta.y).to_radians();
    yaw -= (mouse_delta.x).to_radians();

    pitch = pitch.clamp(-1.2, 1.4);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0)
}
