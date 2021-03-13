//! Collection of utilities for world generation.

use noise::{NoiseFn, ScalePoint, Seedable, SuperSimplex};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{
    entity::{building::Building, EntityType},
    tile::TileType,
    world::Pos,
};

const OCEAN_CUTOFF: isize = -300;
const BEACH_CUTOFF: isize = -250;

pub struct Biome {
    tiles: TileDistribution,
    /// To select what biome goes where we use two noise maps, one containing
    /// elevation, and one containing climate, from cold to warm. This function
    /// takes these two map scores and combines them into a final score for this
    /// biome. The highest scoring biome will be selected.
    score_fn: Box<dyn Fn(isize, isize) -> isize>,
}

impl Biome {
    pub fn ocean() -> Biome {
        Biome {
            tiles: TileDistribution::ocean(),
            score_fn: Box::new(|elevation, _climate| (elevation < OCEAN_CUTOFF) as isize * 10000),
        }
    }
    pub fn beach() -> Biome {
        Biome {
            tiles: TileDistribution::beach(),
            score_fn: Box::new(|elevation, _climate| (elevation < BEACH_CUTOFF) as isize * 9000),
        }
    }
    pub fn grass() -> Biome {
        Biome {
            tiles: TileDistribution::grass(),
            score_fn: Box::new(|_elevation, climate| 1000 - climate.abs()),
        }
    }
    pub fn desert() -> Biome {
        Biome {
            tiles: TileDistribution::desert(),
            score_fn: Box::new(|_elevation, climate| climate * 2 + 500),
        }
    }
    pub fn high_lands() -> Biome {
        Biome {
            tiles: TileDistribution::high_lands(),
            score_fn: Box::new(|elevation, climate| 500 - climate.abs() + elevation),
        }
    }
}

pub struct BiomeMap {
    biomes: Vec<Biome>,
    elevation: Vec<ScalePoint<SuperSimplex>>,
    climate: Vec<ScalePoint<SuperSimplex>>,
}

impl BiomeMap {
    pub fn new() -> BiomeMap {
        let elevation = [0.02, 0.04]
            .iter()
            .map(|s| {
                ScalePoint::new(SuperSimplex::new().set_seed(rand::random()))
                    .set_all_scales(*s, *s, *s, *s)
            })
            .collect();
        let climate = [0.01, 0.01]
            .iter()
            .map(|s| {
                ScalePoint::new(SuperSimplex::new().set_seed(rand::random()))
                    .set_all_scales(*s, *s, *s, *s)
            })
            .collect();
        BiomeMap {
            biomes: vec![
                Biome::ocean(),
                Biome::grass(),
                Biome::beach(),
                Biome::desert(),
                Biome::high_lands(),
            ],
            elevation,
            climate,
        }
    }

    pub fn get(&self, p: Pos, rng: &mut impl Rng) -> (TileType, Option<EntityType>) {
        let pos = [p.0 as f64 /  5., p.1 as f64 / 5.];
        let elevation = (self.elevation.iter().map(|e| e.get(pos)).sum::<f64>() * 500.) as isize;
        let climate = (self.climate.iter().map(|e| e.get(pos)).sum::<f64>() * 500.) as isize;
        let b = self
            .biomes
            .iter()
            .max_by_key(|b| (b.score_fn)(elevation, climate))
            .unwrap();

        b.tiles.sample(rng)
    }
}

pub struct TileDistribution {
    tiles: Vec<(TileType, Option<EntityType>)>,
    weights: WeightedIndex<u16>,
}

impl TileDistribution {
    pub fn grass() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileType::Grass, None),
                (TileType::GrassRock, None),
                (
                    TileType::Grass,
                    Some(EntityType::Building(Building::Hut)),
                ),
                (
                    TileType::Grass,
                    Some(EntityType::Building(Building::Market)),
                ),
            ],
            weights: WeightedIndex::new(&[5000, 200, 30, 10]).unwrap(),
        }
    }

    pub fn high_lands() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileType::Dirt, None),
                (TileType::DirtRock, None),
                (TileType::DirtTreeDead, None),
                (TileType::Dirt, Some(EntityType::Building(Building::Hut))),
            ],
            weights: WeightedIndex::new(&[2000, 20, 10, 1]).unwrap(),
        }
    }

    pub fn ocean() -> TileDistribution {
        TileDistribution {
            tiles: vec![(TileType::Water, None), (TileType::WaterRock, None)],
            weights: WeightedIndex::new(&[1000, 1]).unwrap(),
        }
    }
    pub fn desert() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileType::Sand, None),
                (TileType::SandRock, None),
                (TileType::SandTreeDead, None),
                (
                    TileType::Sand,
                    Some(EntityType::Building(Building::Market)),
                ),
            ],
            weights: WeightedIndex::new(&[1000, 10, 15, 2]).unwrap(),
        }
    }
    pub fn beach() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileType::Sand, None),
                (TileType::SandPalm, None),
                (TileType::SandTreeDead, None),
            ],
            weights: WeightedIndex::new(&[1000, 20, 5]).unwrap(),
        }
    }
}

impl Distribution<(TileType, Option<EntityType>)> for TileDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (TileType, Option<EntityType>) {
        self.tiles[self.weights.sample(rng)].clone()
    }
}
