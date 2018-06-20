pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocks_move: bool,
    pub blocks_sight: bool,
}

impl Tile {
    pub fn floor() -> Self {
        Tile {
            blocks_move: false,
            blocks_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocks_move: true,
            blocks_sight: true,
        }
    }
}
