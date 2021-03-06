use std::num::NonZeroU16;

use dear_gui::graphics::primitives::{Sprite, Vf2};
use glium::Display;

use crate::{
    agent::{Agent, AgentAction, AgentId},
    grid::CanvasGrid,
    tile::{TileAction, TileTexture},
};

pub struct World {
    tiles_type: Vec<TileTexture>,
    tiles_agent: Vec<Option<AgentId>>,
    // tiles_resource: Vec<u8>,
    // tiles_action: Vec<TileAction>,
    agents: Vec<Agent>,
    // conflicts: Vec<Vec<TileAction>>,
    width: usize, // Q from Andrei: Should this remain usize or u16?
    height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, agent_count: usize) -> World {
        let tiles_type = std::iter::repeat_with(|| {
            if rand::random::<f32>() < 0.6 {
                TileTexture::Grass
            } else {
                TileTexture::GrassRock
            }
        })
        .take(width * height)
        .collect::<Vec<_>>();

        World {
            tiles_type,
            tiles_agent: vec![None; width * height],
            // tiles_resource: vec![0; width * height],
            // tiles_action: vec![TileAction::default(); width * height],
            agents: vec![Agent::default(); agent_count],
            // conflicts: Vec::new(),
            width,
            height,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width && y < self.height);
        x + y * self.width
    }

    pub fn iter_neighbours(&self) -> impl Iterator<Item = ((usize, usize), [TileTexture; 9])> + '_ {
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
                        self.tiles_type[self.idx(left, top)],
                        self.tiles_type[self.idx(x, top)],
                        self.tiles_type[self.idx(right, top)],
                        self.tiles_type[self.idx(left, y)],
                        self.tiles_type[self.idx(x, y)],
                        self.tiles_type[self.idx(right, y)],
                        self.tiles_type[self.idx(left, bot)],
                        self.tiles_type[self.idx(x, bot)],
                        self.tiles_type[self.idx(right, bot)],
                    ],
                )
            })
    }

    // pub fn step_game_of_life(&mut self) {
    //     std::mem::swap(&mut self.old_tiles, &mut self.tiles);
    //     let mut tiles = std::mem::take(&mut self.tiles);
    //     for ((x, y), ts) in self.iter_neighbours() {
    //         let n_count: usize = ts
    //             .iter()
    //             .map(|x| (x.texture != TileTexture::Grass) as usize)
    //             .sum();
    //         let live = match (ts[4].texture != TileTexture::Grass, n_count) {
    //             (false, 3) => true,
    //             (true, 3) | (true, 4) => true,
    //             _ => false,
    //         };
    //         tiles[x + y * self.width].texture = if live {
    //             TileTexture::SandPalm
    //         } else {
    //             TileTexture::Grass
    //         };
    //     }
    //     self.tiles = tiles;
    // }
    // fn create_conflict(&mut self, a: TileAction, b: TileAction) -> u16 {
    //     self.conflicts.push(vec![a, b]);
    //     (self.conflicts.len() - 1) as u16
    // }

    // fn add_to_conflict(&mut self, conflict: u16, a: TileAction) {
    //     self.conflicts[conflict as usize].push(a);
    // }

    pub fn step(&mut self) {
        let mut agents = std::mem::take(&mut self.agents);
        for (i, agent) in agents.iter_mut().enumerate() {
            match agent.preferred_action(&self) {
                AgentAction::Move(x, y) => {
                    let idx = self.idx(x.into(), y.into());
                    if self.tiles_agent[idx].is_none() {
                        self.tiles_agent[idx] = Some(AgentId::new(i));
                        agent.pos_x = x;
                        agent.pos_y = y;
                    }
                }
                AgentAction::None => {}
                _ => unimplemented!(),
            }
        }
        self.agents = agents;
        // self.tiles_action.iter_mut().for_each(|a|)
    }

    pub fn update_grid(&self, display: &Display, grid: &mut CanvasGrid) {
        assert_eq!(grid.width * 32, self.width);
        assert_eq!(grid.height * 32, self.height);
        for cx in 0..grid.width {
            for cy in 0..grid.height {
                let start = cy * 32 * self.width + cx * 32;
                grid.update_chunk(
                    (cx, cy),
                    (0..32).flat_map(|y| {
                        self.tiles_type[start + y * self.width..start + y * self.width + 32]
                            .iter()
                            .copied()
                    }),
                )
            }
        }

        grid.update_agents(display, self.agents.iter().map(|a| Sprite {
            vertex: Vf2::new(a.pos_x as f32 * 10., a.pos_y as f32 * 10.),
            size: Vf2::new(10., 10.),
            texture_index: 1,
        }))
    }

    pub fn tile_type(&self, x: u16, y: u16) -> TileTexture {
        self.tiles_type[self.idx(x as usize, y as usize)]
    }
}
