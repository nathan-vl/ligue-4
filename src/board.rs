use crate::tile::Tile;

#[derive(Debug)]
pub struct Board {
    tiles: [[Option<Tile>; 7]; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[None; 7]; 6],
        }
    }

    pub fn place_tile(&mut self, column: usize, tile: &Tile) -> Option<(usize, usize)> {
        for (y, row) in self.tiles.iter_mut().enumerate().rev() {
            if row[column].is_none() {
                row[column] = Some(*tile);
                return Some((column, y));
            }
        }
        None
    }

    pub fn check_row(&self, tile: &Tile, row: usize) -> bool {
        const SLICE_SIZE: usize = 4;
        for x in 0..self.tiles[0].len() - SLICE_SIZE {
            let slice = &self.tiles[row][x..x + SLICE_SIZE];

            if !(slice.contains(&None) || slice.contains(&Some(tile.opposite()))) {
                return true;
            }
        }

        false
    }

    pub fn check_column(&self, tile: &Tile, column: usize) -> bool {
        const SLICE_SIZE: usize = 4;
        for y in 0..self.tiles[0].len() - SLICE_SIZE {
            let slice = [
                self.tiles[y][column],
                self.tiles[y + 1][column],
                self.tiles[y + 2][column],
                self.tiles[y + 3][column],
            ];

            if !(slice.contains(&None) || slice.contains(&Some(tile.opposite()))) {
                return true;
            }
        }

        false
    }

    pub fn check_direct_diagonal(&self, tile: &Tile) -> bool {
        const SLICE_SIZE: usize = 4;
        for y in 0..self.tiles.len() - SLICE_SIZE {
            for x in 0..self.tiles[0].len() - SLICE_SIZE {
                let slice = [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)];
                let slice = slice.map(|item| self.tiles[item.1][item.0]);
                if !(slice.contains(&None) || slice.contains(&Some(tile.opposite()))) {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_inverse_diagonal(&self, tile: &Tile) -> bool {
        const SLICE_SIZE: usize = 4;
        for y in 0..self.tiles.len() - SLICE_SIZE {
            for x in (SLICE_SIZE - 1)..self.tiles[0].len() {
                let slice = [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)];
                let slice = slice.map(|item| self.tiles[item.1][item.0]);
                if !(slice.contains(&None) || slice.contains(&Some(tile.opposite()))) {
                    return true;
                }
            }
        }

        false
    }

    pub fn print(&self) {
        print!("-");
        for i in 0..self.tiles[0].len() {
            print!("{}", i + 1);
        }
        println!("-");

        print!("╔");
        for _ in 0..self.tiles[0].len() {
            print!("═");
        }
        println!("╗");

        for row in self.tiles {
            print!("║");
            for tile in row {
                if let Some(tile) = tile {
                    print!("{}", tile);
                } else {
                    print!(" ");
                }
            }
            println!("║");
        }

        print!("╚");
        for _ in 0..self.tiles[0].len() {
            print!("═");
        }
        println!("╝");
    }
}
