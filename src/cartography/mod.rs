pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocks_move: bool,
    pub blocks_sight: bool,
    pub char: char,
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

pub fn make_map(width: i32, height: i32) -> Map {
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
