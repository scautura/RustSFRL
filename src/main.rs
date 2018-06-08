extern crate tcod;

use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map};

fn main() {
    let mut root = Root::initializer()
        .font("rsrc/lucida10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(80, 50)
        .title("Unnamed Sci-Fi RogueLike")
        .init();

    tcod::system::set_fps(60);
    
    root.set_default_foreground(colors::WHITE);
    root.set_default_background(colors::BLACK);

    while !root.window_closed() {
        root.print_ex(40,25,BackgroundFlag::None,TextAlignment::Center,"Hello World!");
        root.flush();
        root.wait_for_keypress(true);
    }
}
