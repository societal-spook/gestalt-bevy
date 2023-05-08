use bevy::prelude::{not, Condition, IntoSystemConfig, Plugin};

mod common;
pub use common::{Interactable, InteractionKind};
mod pick_up;

use crate::character::camera::camera_move;

use self::{
    common::{init_interaction, interact_is_toggled, is_interacting},
    pick_up::{drop, pick_up, throw},
};

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(drop.run_if(is_interacting.and_then(interact_is_toggled)));
        app.add_system(
            pick_up
                .after(camera_move)
                .run_if(is_interacting.and_then(not(interact_is_toggled))),
        );

        app.add_system(throw);

        app.add_system(init_interaction.run_if(not(is_interacting).and_then(interact_is_toggled)));
    }
}
