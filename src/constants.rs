//! Constants
//!
//! 1 =?= one pixel
//! magical number = ratio

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const TILE_SIZE: f32 = 1.;

pub mod character {

    pub const FRAME_TIME: f32 = 0.1;

    pub const CHAR_Z: f32 = 10.;
    pub const CHAR_POSITION: (f32, f32, f32) = (0., -60., CHAR_Z);

    pub mod player {
        pub const BOTTOM_WHIP_POS: (f32, f32, f32) = (10., -5.5, 0.);
        pub const FRONT_WHIP_POS: (f32, f32, f32) = (30., -3., 0.);
        pub const CHARGED_ATTACK_HOLD: f32 = 0.5;

        pub const PLAYER_HITBOX_SIZE: f32 = 12.;
        pub const PLAYER_HITBOX_OFFSET_Y: (f32, f32, f32) = (0., 2., 0.);
        pub const PLAYER_ATTACK_HITBOX_BOTTOM: (f32, f32) = (21., 1.5);
        pub const PLAYER_ATTACK_HITBOX_FRONT: (f32, f32) = (20., 7.);

        // -- Animation --
        pub const PLAYER_IDLE_FRAMES: (usize, usize) = (0, 4);
        pub const PLAYER_RUN_FRAMES: (usize, usize) = (5, 12);
        pub const PLAYER_TRANSITION_TO_CHARGE_FRAMES: (usize, usize) = (13, 14);
        pub const PLAYER_CHARGE_FRAMES: (usize, usize) = (15, 18);
        // TODO: Seperate Charge and Attack to perfectly activate the hitbox
        // First Swing: (19, 23)
        pub const PLAYER_FULL_ATTACK_FRAMES: (usize, usize) = (19, 26);
        pub const PLAYER_SECOND_ATTACK_FRAMES: (usize, usize) = (24, 26);
        pub const PLAYER_HIT_FRAMES: (usize, usize) = (27, 28);
        pub const PLAYER_DEAD_FRAMES: (usize, usize) = (29, 34);
    }

    pub mod boss {
        pub const BOSS_HP: i32 = 1000;

        pub const BOSS_SMASH_COOLDOWN: f32 = 5.;

        pub const FRONT_SMASH_POS: (f32, f32, f32) = (10., -5.5, 0.);
        pub const BOSS_ATTACK_HITBOX_FRONT: (f32, f32) = (20., 7.);

        pub const BOSS_HITBOX_SIZE: f32 = 12.;
        pub const BOSS_RANGE_HITBOX_SIZE: f32 = 40.;
        pub const BOSS_HITBOX_OFFSET_Y: (f32, f32, f32) = (0., 5., 0.);

        // -- Animation --
        pub const BOSS_IDLE_FRAMES: (usize, usize) = (0, 4);
        pub const BOSS_RUN_FRAMES: (usize, usize) = (5, 10);
        pub const BOSS_TRANSITION_TO_CHARGE_FRAMES: (usize, usize) = (11, 18);
        pub const BOSS_CHARGE_FRAMES: (usize, usize) = (15, 18);
        // First Swing: (19, 23)
        pub const BOSS_FULL_ATTACK_FRAMES: (usize, usize) = (19, 26);
        pub const BOSS_SECOND_ATTACK_FRAMES: (usize, usize) = (24, 26);
        pub const BOSS_HIT_FRAMES: (usize, usize) = (27, 28);
        pub const BOSS_DEAD_FRAMES: (usize, usize) = (29, 34);
    }
}

pub mod crowd {
    pub const CROWD_SIZE: usize = 50;
    pub const CROWD_SPAN: f32 = 1000.0;
    pub const CROWD_Y: f32 = -55.0;
    pub const CROWD_Z: f32 = 2.5;
}
