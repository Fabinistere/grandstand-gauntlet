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
        use super::CHAR_Z;
        use crate::characters::animations::CharacterState;

        pub const PLAYER_HP: i32 = 40;
        // TODO: optional - to Test and perfectly replicate the past speed, just spawn a dummy entity with a movement controller (and old formula).
        pub const PLAYER_SPEED: f32 = 15.;
        pub const PLAYER_POSITION: (f32, f32, f32) = (-1080., -60., CHAR_Z);

        pub const BOTTOM_WHIP_POS: (f32, f32, f32) = (10., -5.5, 0.);
        pub const FRONT_WHIP_POS: (f32, f32, f32) = (30., -3., 0.);
        pub const CHARGED_ATTACK_HOLD: f32 = 0.5;

        pub const PLAYER_HITBOX_SIZE: f32 = 12.;
        pub const PLAYER_HITBOX_OFFSET_Y: (f32, f32, f32) = (0., 2., 0.);
        pub const PLAYER_ATTACK_HITBOX_BOTTOM: (f32, f32) = (21., 1.5);
        pub const PLAYER_ATTACK_HITBOX_FRONT: (f32, f32) = (20., 7.);

        // -- Animation --
        pub const PLAYER_IDLE_FRAMES: (usize, usize, CharacterState) = (0, 4, CharacterState::Idle);
        pub const PLAYER_RUN_FRAMES: (usize, usize, CharacterState) = (5, 12, CharacterState::Idle);
        pub const PLAYER_TRANSITION_TO_CHARGE_FRAMES: (usize, usize, CharacterState) =
            (13, 14, CharacterState::Charge);
        pub const PLAYER_CHARGE_FRAMES: (usize, usize, CharacterState) =
            (15, 18, CharacterState::Charge);
        pub const PLAYER_CHARGED_ATTACK_FRAMES: (usize, usize, CharacterState) =
            (19, 26, CharacterState::Idle);
        // First Swing
        pub const PLAYER_FIRST_ATTACK_FRAMES: (usize, usize, CharacterState) =
            (19, 22, CharacterState::Idle);
        pub const PLAYER_SECOND_ATTACK_FRAMES: (usize, usize, CharacterState) =
            (23, 26, CharacterState::Idle);
        pub const PLAYER_HIT_FRAMES: (usize, usize, CharacterState) =
            (27, 28, CharacterState::Idle);
        // The dead phase is a loop one
        pub const PLAYER_DEAD_FRAMES: (usize, usize, CharacterState) =
            (29, 34, CharacterState::Dead);
    }

    pub mod boss {
        use crate::characters::animations::CharacterState;

        pub const BOSS_HP: i32 = 1000;
        pub const BOSS_SPEED: f32 = 10.;

        pub const BOSS_SMASH_COOLDOWN: f32 = 5.;

        pub const FRONT_SMASH_POS_TOP: (f32, f32, f32) = (42., 11., 0.);
        pub const BOSS_ATTACK_HITBOX_SMASH_TOP: (f32, f32) = (10., 5.);
        pub const FRONT_SMASH_POS_BOTTOM: (f32, f32, f32) = (20., -4., 0.);
        pub const BOSS_ATTACK_HITBOX_SMASH_BOTTOM: (f32, f32) = (40., 10.);

        pub const BOSS_HITBOX_SIZE: f32 = 12.;

        pub mod attack_hitbox {

            pub const FALLEN_ANGEL_POS: (f32, f32, f32) = (0., -5.5, 0.);
            pub const BOSS_ATTACK_HITBOX_FALLEN_ANGEL: (f32, f32) = (45., 7.);

            pub const BOSS_RANGE_HITBOX_SIZE: f32 = 40.;
            pub const BOSS_HITBOX_OFFSET_Y: (f32, f32, f32) = (0., 5., 0.);
        }

        pub mod behaviors_sensors {
            pub const BACKSTROKE_DASH_POS: (f32, f32, f32) = (90., -20., 0.);
            pub const BACKSTROKE_DASH_SENSOR: (f32, f32) = (15., 60.);
        }

        // -- Animation --
        pub const BOSS_IDLE_FRAMES: (usize, usize, CharacterState) = (0, 4, CharacterState::Idle);
        pub const BOSS_RUN_FRAMES: (usize, usize, CharacterState) = (5, 10, CharacterState::Idle);
        pub const BOSS_TRANSITION_TO_DASH_FRAMES: (usize, usize, CharacterState) =
            (56, 59, CharacterState::Dash);
        pub const BOSS_DASH_FRAMES: (usize, usize, CharacterState) = (60, 65, CharacterState::Idle);
        pub const BOSS_TP_OUT_FRAMES: (usize, usize, CharacterState) =
            (29, 34, CharacterState::TpIn);
        pub const BOSS_TP_IN_FRAMES: (usize, usize, CharacterState) = (5, 10, CharacterState::Idle);
        // pub const BOSS_CHARGE_FRAMES: (usize, usize, CharacterState) =
        //     (11, 14, CharacterState::Attack);
        pub const BOSS_FEINT_FRAMES: (usize, usize, CharacterState) =
            (41, 45, CharacterState::Idle);
        pub const BOSS_TRANSITION_TO_SMASH_FRAMES: (usize, usize, CharacterState) =
            (11, 14, CharacterState::Attack);
        pub const BOSS_SMASH_FRAMES: (usize, usize, CharacterState) =
            (15, 18, CharacterState::Idle);
        // TODO: Fix last frame FX -----vvv
        pub const BOSS_FALLEN_ANGEL_FRAMES: (usize, usize, CharacterState) =
            (19, 26, CharacterState::Idle);
        pub const BOSS_LASER_RAIN_FRAMES: (usize, usize, CharacterState) =
            (47, 57, CharacterState::Idle);
        pub const BOSS_HIT_FRAMES: (usize, usize, CharacterState) = (27, 28, CharacterState::Idle);
        pub const BOSS_DEAD_FRAMES: (usize, usize, CharacterState) = (29, 34, CharacterState::Dead);
    }
}

pub mod crowd {
    pub const CROWD_SIZE: usize = 50;
    pub const CROWD_SPAN: f32 = 1000.0;
    pub const CROWD_Y: f32 = -55.0;
    pub const CROWD_Z: f32 = 2.5;
}

pub mod ui {
    use bevy::prelude::Color;

    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
}
