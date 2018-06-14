extern crate tcod;

use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

fn main() {
    let mut root = Root::initializer()
        .font("rsrc/lucida10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Unnamed Sci-Fi RogueLike")
        .init();

    let mut player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);

    tcod::system::set_fps(60);

    root.set_default_background(colors::BLACK);

    while !root.window_closed() {
        root.set_default_foreground(player.color);
        root.put_char(player.x, player.y, player.char, BackgroundFlag::None);
        root.flush();
        root.put_char(player.x, player.y, ' ', BackgroundFlag::None);
        if handle_input(&mut root, &mut player) == true {
            break;
        }
    }
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }
}

fn handle_input(root: &mut Root, player: &mut Object) -> bool {
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => {
            player.y -= 1;
        }
        Key { code: Down, .. } => {
            player.y += 1;
        }
        Key { code: Left, .. } => {
            player.x -= 1;
        }
        Key { code: Right, .. } => {
            player.x += 1;
        }
        _ => {}
    }

    false
}
