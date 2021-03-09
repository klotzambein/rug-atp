//! Collection of utilities for world generation.

use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;

use crate::{entity::EntityType, tile::TileTexture};

pub struct TileDistribution {
    tiles: Vec<(TileTexture, Option<EntityType>)>,
    weights: WeightedIndex<u16>,
}