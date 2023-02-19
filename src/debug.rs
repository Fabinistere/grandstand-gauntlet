use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::characters::animations::CharacterState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<CharacterState>()
                // UI
                ;
        }
    }
}
