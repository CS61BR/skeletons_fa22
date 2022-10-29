#![allow(dead_code)] // allow things in this module to go unused

// some convenience colors - feel free to add more
pub const WHITE: Color = rgb(0xffffff);
pub const BLACK: Color = rgb(0x000000);
pub const GRAY: Color = rgb(0x808080);
pub const DARK_GRAY: Color = rgb(0x404040);
pub const DARKER_GRAY: Color = rgb(0x202020);
pub const RED: Color = rgb(0xff0000);
pub const GREEN: Color = rgb(0x00ff00);
pub const DARK_GREEN: Color = rgb(0x023020);
pub const BLUE: Color = rgb(0x0000ff);
pub const MIDNIGHT_BLUE: Color = rgb(0x191970);
pub const ORANGE: Color = rgb(0xffc800);
pub const YELLOW: Color = rgb(0xffff00);
pub const MAGENTA: Color = rgb(0xff00ff);
pub const PINK: Color = rgb(0xffafaf);

pub type Color = u32; // rgba format

pub const fn rgb(rgb: u32) -> Color {
    rgb << 8 | 0xff
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
}
