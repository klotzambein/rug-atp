use crate::{
    grid::CanvasGrid,
    tile::{Tile, TileTexture},
};

pub struct World {
    tiles: Vec<Tile>,
    old_tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let tiles = std::iter::repeat_with(|| {
            Tile::new(if rand::random() {
                TileTexture::Grass
            } else {
                TileTexture::GrassRock
            })
        })
        .take(width * height)
        .collect::<Vec<_>>();

        World {
            old_tiles: tiles.clone(),
            tiles,
            width: width,
            height: height,
        }
    }

    pub fn old_tile(&self, x: usize, y: usize) -> Tile {
        self.old_tiles[y * self.width + x].clone()
    }

    pub fn iter_neighbours(&self) -> impl Iterator<Item = ((usize, usize), [Tile; 9])> + '_ {
        (0..self.width)
            .flat_map(move |x| (0..self.height).map(move |y| (x, y)))
            .map(move |(x, y)| {
                let left = (x + self.width - 1) % self.width;
                let right = (x + 1) % self.width;
                let bot = (y + self.width - 1) % self.width;
                let top = (y + 1) % self.width;
                (
                    (x, y),
                    [
                        self.old_tile(left, top),
                        self.old_tile(x, top),
                        self.old_tile(right, top),
                        self.old_tile(left, y),
                        self.old_tile(x, y),
                        self.old_tile(right, y),
                        self.old_tile(left, bot),
                        self.old_tile(x, bot),
                        self.old_tile(right, bot),
                    ],
                )
            })
    }

    pub fn step_game_of_life(&mut self) {
        std::mem::swap(&mut self.old_tiles, &mut self.tiles);
        let mut tiles = std::mem::take(&mut self.tiles);
        for ((x, y), ts) in self.iter_neighbours() {
            let n_count: usize = ts
                .iter()
                .map(|x| (x.texture != TileTexture::Grass) as usize)
                .sum();
            let live = match (ts[4].texture != TileTexture::Grass, n_count) {
                (false, 3) => true,
                (true, 3) | (true, 4) => true,
                _ => false,
            };
            tiles[x + y * self.width].texture = if live {
                TileTexture::SandPalm
            } else {
                TileTexture::Grass
            };
        }
        self.tiles = tiles;
    }

    pub fn update_grid(&self, grid: &CanvasGrid) {
        assert_eq!(grid.width * 32, self.width);
        assert_eq!(grid.height * 32, self.height);
        for cx in 0..grid.width {
            for cy in 0..grid.height {
                let start = cy * 32 * self.width + cx * 32;
                grid.update_chunk(
                    (cx, cy),
                    (0..32).flat_map(|y| {
                        self.tiles[start + y * self.width..start + y * self.width + 32]
                            .iter()
                            .map(|t| t.texture)
                    }),
                )
            }
        }
    }
}
