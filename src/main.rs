extern crate tcod;

mod cartography;
mod entity;

use entity::Object;

use tcod::colors::{self, Color};
use tcod::console::*;
// use tcod::input::{self, Event, Key, Mouse};
// use tcod::map::{FovAlgorithm, Map};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 60;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

fn main() {
    let mut root = Root::initializer()
        .font("rsrc/lucida10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Unnamed Sci-Fi RogueLike")
        .init();

    let mut player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);

    let mut map = cartography::Map::new();

    tcod::system::set_fps(60);

    root.set_default_background(colors::BLACK);

    while !root.window_closed() {
        render(&mut player, &mut map, &mut root);
        root.put_char(player.x, player.y, ' ', BackgroundFlag::None);
        if handle_input(&mut root, &mut player, &map) == true {
            break;
        }
    }
}

fn handle_input(root: &mut Root, player: &mut Object, map: &cartography::Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } | Key { code: NumPad8, .. } => {
            player.move_by(&map, 0, -1);
        }
        Key { code: Down, .. } | Key { code: NumPad2, .. } => {
            player.move_by(&map, 0, 1);
        }
        Key { code: Left, .. } | Key { code: NumPad4, .. } => {
            player.move_by(&map, -1, 0);
        }
        Key { code: Right, .. } | Key { code: NumPad6, .. } => {
            player.move_by(&map, 1, 0);
        }
        Key { code: NumPad7, .. } => {
            player.move_by(&map, -1, -1);
        }
        Key { code: NumPad9, .. } => {
            player.move_by(&map, 1, -1);
        }
        Key { code: NumPad1, .. } => {
            player.move_by(&map, -1, 1);
        }
        Key { code: NumPad3, .. } => {
            player.move_by(&map, 1, 1);
        }
        _ => {}
    }

    false
}

fn render(player: &mut Object, map: &mut cartography::Map, root: &mut Root) {
    blit(
        &mut map.render(player),
        (0, 0),
        (50, 50),
        root,
        (0, 0),
        1.0,
        1.0,
    );

    root.flush();
}
