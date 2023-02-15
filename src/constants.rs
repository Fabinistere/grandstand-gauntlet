//! Constants
//!
//! 1 =?= one pixel
//! magical number = ratio

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const TILE_SIZE: f32 = 1.;

pub mod character {
    pub const CHAR_Z: f32 = 10.;
    pub const CHAR_POSITION: (f32, f32, f32) = (0., -60., CHAR_Z);
    pub const CROWD_CHARACTER_Z: f32 = 2.5;
}

pub mod crowd {
    pub const CROWD_SIZE: usize = 50;
    pub const CROWD_SPAN: f32 = 1000.0;
}
