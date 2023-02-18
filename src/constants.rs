//! Constants
//!
//! 1 =?= one pixel
//! magical number = ratio

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const TILE_SIZE: f32 = 1.;

pub mod character {
    pub const CHAR_Z: f32 = 10.;
    pub const CHAR_POSITION: (f32, f32, f32) = (0., -60., CHAR_Z);

    pub mod player {
        pub const BOTTOM_WHIP_POS: (f32, f32, f32) = (10., -5.5, 0.);
        pub const FRONT_WHIP_POS: (f32, f32, f32) = (30., -3., 0.);
    }
}

pub mod crowd {
    pub const CROWD_SIZE: usize = 50;
    pub const CROWD_SPAN: f32 = 1000.0;
    pub const CROWD_Y: f32 = -55.0;
    pub const CROWD_Z: f32 = 2.5;
}
