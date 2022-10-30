use crate::{
    bindings::Storage,
    drawing::{
        board::Board,
        tiles::{FLOWER, GRASS, NOTHING, WALL},
    },
    random::Random,
};

pub struct Game {}

impl Game {
    pub fn new(_storage: Storage) -> Self {
        Self {}
    }

    pub fn respond_to_keyboard_input(&mut self, _c: char) {}

    pub fn render(&mut self) -> Board {
        // uncomment these lines to try out different worlds!
        boring_world()
        // random_world()
        // plus_world()
    }
}

fn boring_world() -> Board {
    let mut b = Board::new(60, 30);
    for x in 20..35 {
        for y in 5..10 {
            b.set_tile(x, y, WALL);
        }
    }
    b
}

fn random_world() -> Board {
    let tile_choices = [WALL, GRASS, FLOWER, NOTHING];
    let mut random = Random::new("I like cheese");
    let mut b = Board::new(60, 30);
    for x in 0..b.width() {
        for y in 0..b.height() {
            let tile = &tile_choices[random.next_below(tile_choices.len())];
            b.set_tile(x, y, tile.clone());
        }
    }
    b
}

fn plus_world() -> Board {
    todo!()
}
