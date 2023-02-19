pub mod boss;

use bevy::prelude::*;

use self::boss::BossPlugin;

pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BossPlugin)
            ;
    }
}
