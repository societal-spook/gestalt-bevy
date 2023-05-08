use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController};

pub mod camera;
mod movement;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_character)
            .add_system(movement::character_move)
            .add_system(camera::camera_move.after(movement::character_move))
            .add_system(camera::camera_look.before(movement::character_move));

        // app.add_system(start_interaction.run_if(wants_to_interact.and_then(not(is_interacting))));
        // app.add_system(interact.after(start_interaction).run_if(is_interacting));
        // app.add_system(stop_interaction.run_if(wants_to_interact.and_then(is_interacting)));
    }
}

#[derive(Component)]
pub struct Protagonist;

fn spawn_character(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());

    commands
        .spawn(KinematicCharacterController::default())
        .insert(Protagonist)
        .insert(Collider::capsule_y(0.85, 0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
