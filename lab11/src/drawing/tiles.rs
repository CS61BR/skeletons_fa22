#![allow(dead_code)] // allow things in this module to go unused

use super::colors::{
    rgb, Color, BLUE, DARK_GRAY, DARK_GREEN, GRAY, GREEN, MAGENTA, ORANGE, PINK, WHITE, YELLOW,
};

// some convenience tiles - feel free to add more
pub const NOTHING: Tile = new_solid_tile(None, "nothing");
pub const AVATAR: Tile = new_text_tile("@", WHITE, None, "you");
pub const WALL: Tile = new_text_tile("#", rgb(0xd88080), Some(DARK_GRAY), "wall");
pub const FLOOR: Tile = new_text_tile("·", DARK_GRAY, None, "floor");
pub const GRASS: Tile = new_text_tile("\"", GREEN, Some(DARK_GREEN), "grass");
pub const WATER: Tile = new_text_tile("=", BLUE, None, "water");
pub const FLOWER: Tile = new_text_tile("❀", MAGENTA, Some(PINK), "flower");
pub const LOCKED_DOOR: Tile = new_text_tile("█", ORANGE, None, "locked door");
pub const UNLOCKED_DOOR: Tile = new_text_tile("▢", ORANGE, None, "unlocked door");
pub const SAND: Tile = new_text_tile("▒", YELLOW, None, "sand");
pub const MOUNTAIN: Tile = new_text_tile("▲", GRAY, None, "mountain");
pub const TREE: Tile = new_text_tile("♠", GREEN, None, "tree");
pub const CHEESE: Tile = new_image_tile(12345, None, "cheese");

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tile {
    pub contents: TileContents,
    pub background_color: Option<Color>, // will default to background color of canvas
    pub description: &'static str,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TileContents {
    Text {
        text: &'static str,
        text_color: Color,
    },
    Image {
        image_id: usize,
    },
    None,
}

const fn new_text_tile(
    text: &'static str,
    text_color: Color,
    background_color: Option<Color>,
    description: &'static str,
) -> Tile {
    Tile {
        contents: TileContents::Text { text, text_color },
        background_color,
        description,
    }
}

const fn new_image_tile(
    image_id: usize,
    background_color: Option<Color>,
    description: &'static str,
) -> Tile {
    Tile {
        contents: TileContents::Image { image_id },
        background_color,
        description,
    }
}

const fn new_solid_tile(background_color: Option<Color>, description: &'static str) -> Tile {
    Tile {
        contents: TileContents::None,
        background_color,
        description,
    }
}
