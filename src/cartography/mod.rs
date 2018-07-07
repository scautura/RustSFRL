extern crate tcod;

use super::entity;
use tcod::colors;
use tcod::console::*;

const MAP_WIDTH: i32 = 256;
const MAP_HEIGHT: i32 = 256;

const MAP_SCREEN_WIDTH: i32 = 50;
const MAP_SCREEN_HEIGHT: i32 = 50;

pub type Maptype = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocks_move: bool,
    pub blocks_sight: bool,
    pub char: char,
}

pub struct Map {
    pub map: Maptype,
}

impl Tile {
    pub fn floor() -> Self {
        Tile {
            blocks_move: false,
            blocks_sight: false,
            char: '.',
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocks_move: true,
            blocks_sight: true,
            char: '#',
        }
    }
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: Map::make_map(MAP_WIDTH, MAP_HEIGHT),
        }
    }

    pub fn render(self: &Self, player: &mut entity::Object) -> Offscreen {
        let mut screen = Offscreen::new(MAP_SCREEN_WIDTH, MAP_SCREEN_HEIGHT);

        for y in 0..MAP_SCREEN_HEIGHT {
            for x in 0..MAP_SCREEN_WIDTH {
                let wall = self.map[x as usize][y as usize];
                if wall.blocks_sight {
                    screen.put_char_ex(x, y, wall.char, colors::WHITE, super::COLOR_DARK_WALL);
                } else {
                    screen.put_char_ex(x, y, wall.char, colors::WHITE, super::COLOR_DARK_GROUND);
                }
            }
        }
        screen.set_default_foreground(player.color);
        screen.put_char(player.x, player.y, player.char, BackgroundFlag::None);
        //screen.flush();

        screen
    }

    pub fn make_map(width: i32, height: i32) -> Maptype {
        let mut map = vec![vec![Tile::floor(); height as usize]; width as usize];
        for x in 0..width {
            map[x as usize][0] = Tile::wall();
            map[x as usize][height as usize - 1] = Tile::wall();
        }

        for y in 0..height {
            map[0][y as usize] = Tile::wall();
            map[width as usize - 1][y as usize] = Tile::wall();
        }

        map[30][22] = Tile::wall();
        map[50][22] = Tile::wall();

        map
    }
}
