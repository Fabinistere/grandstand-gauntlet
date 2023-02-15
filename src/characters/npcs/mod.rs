pub mod boss;

use bevy::prelude::*;

use self::boss::BossPlugin;

pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // .add_system_set(
            //     SystemSet::on_enter(Location::Desert)
            //         .with_system(setup_map)
            // )
            .add_plugin(BossPlugin)
            .add_system_set(
                SystemSet::new()
                    // .with_run_criteria(run_if_in_level_one)
                    // .with_system(move_camera_system)
            )
            ;
    }
}
