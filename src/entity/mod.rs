extern crate tcod;

use super::cartography;

use tcod::colors::Color;

pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

/*     pub fn move_to(self: &mut Self, map: &cartography::Map, x: i32, y: i32) {
        if !map[x as usize][y as usize].blocks_move {
            self.x = x;
            self.y = y;
        }
    } */

    pub fn move_by(self: &mut Self, map: &cartography::Map, dx: i32, dy: i32) {
        if !map[(self.x+dx) as usize][(self.y+dy) as usize].blocks_move {
            self.x += dx;
            self.y += dy;
        }
    }
}
