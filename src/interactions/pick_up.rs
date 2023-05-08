use bevy::prelude::{Camera3d, Commands, Component, Entity, Query, Transform, Vec3, With, Without};
use bevy_rapier3d::prelude::{Collider, Damping, ExternalImpulse};

use crate::common::Active;

#[derive(Component)]
pub struct PickUp;

#[derive(Component)]
pub struct PickedUp(Collider);

#[derive(Component)]
pub struct Throw(Vec3);

pub fn pick_up(
    mut commands: Commands,
    to_be_picked_ups: Query<(Entity, &Collider), With<PickUp>>,
    mut picked_ups: Query<&mut Transform, With<PickedUp>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<PickedUp>)>,
) {
    if let Ok((pick_up, collider)) = to_be_picked_ups.get_single() {
        commands
            .entity(pick_up)
            .insert(PickedUp(collider.clone()))
            .remove::<(Collider, PickUp)>();
        println!("pick_up")
    }

    if let Ok(mut picked_up) = picked_ups.get_single_mut() {
        let camera = cameras.single();

        picked_up.translation = camera.translation + (-camera.local_z() / 2.0);
        picked_up.scale = Vec3::ONE / 3.0;
        picked_up.rotation = camera.rotation;
    }
}

pub fn drop(
    mut commands: Commands,
    mut picked_ups: Query<(Entity, &PickedUp, &mut Transform), With<Active>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<PickedUp>)>,
) {
    let (entity, picked_up, mut transform) = picked_ups.single_mut();
    let camera = cameras.single();

    transform.scale = Vec3::ONE;
    transform.translation = camera.translation + (-camera.local_z() * 1.5);
    commands
        .entity(entity)
        .insert((picked_up.0.clone(), Throw(-camera.local_z())))
        .remove::<(PickedUp, Active)>();
}

pub fn throw(mut commands: Commands, mut throwns: Query<(Entity, &Throw)>) {
    for (entity, thrown) in throwns.iter_mut() {
        commands
            .entity(entity)
            .insert(ExternalImpulse {
                impulse: thrown.0 * 1.7,
                torque_impulse: Vec3::ZERO,
            })
            .insert(Damping {
                linear_damping: 1.0,
                angular_damping: 4.0,
            })
            .remove::<Throw>();
    }
}
