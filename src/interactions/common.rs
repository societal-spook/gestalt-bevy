use bevy::prelude::{
    Camera3d, Commands, Component, Entity, Input, KeyCode, Query, Res, Transform, With,
};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::{character::Protagonist, common::Active};

use super::pick_up::PickUp;

pub enum InteractionKind {
    PickUp,
}

#[derive(Component)]
pub struct Interactable {
    kind: InteractionKind,
}

impl Interactable {
    pub fn new(kind: InteractionKind) -> Self {
        Self { kind }
    }
}

pub fn interact_is_toggled(inputs: Res<Input<KeyCode>>) -> bool {
    inputs.just_pressed(KeyCode::E)
}

pub fn is_interacting(active_interactables: Query<&Interactable, With<Active>>) -> bool {
    !active_interactables.is_empty()
}

pub fn init_interaction(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    interactables: Query<&Interactable>,
    cameras: Query<&Transform, With<Camera3d>>,
    protagonist: Query<Entity, With<Protagonist>>,
) {
    let camera = cameras.single();
    let filter = QueryFilter::default().exclude_collider(protagonist.single());

    if let Some((entity, _)) =
        rapier_context.cast_ray(camera.translation, -camera.local_z(), 4.0, true, filter)
    {
        if let Ok(interactable) = interactables.get(entity) {
            match interactable.kind {
                InteractionKind::PickUp => {
                    commands.entity(entity).insert((Active, PickUp));
                }
            }
        }
    }
}
