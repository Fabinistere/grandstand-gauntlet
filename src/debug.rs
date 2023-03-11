use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::characters::{
    aggression::Hp,
    animations::CharacterState,
    movement::Speed,
    npcs::boss::behaviors::{BossAction, BossBehavior},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<CharacterState>()
                .register_inspectable::<BossBehavior>()
                // .register_inspectable::<BossActions>()
                .register_inspectable::<BossAction>()
                .register_inspectable::<Hp>()
                .register_inspectable::<Speed>()
                // UI
                ;
        }
    }
}
