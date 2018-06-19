extern crate tcod;

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

    pub fn move_to(self: &mut Self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_by(self: &mut Self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}