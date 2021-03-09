use dear_gui::graphics::primitives::{Sprite, Vf2};
use glium::Display;

use crate::{
    agent::{Agent, AgentAction},
    building::Building,
    entity::{Entity, EntityId, EntityType},
    grid::CanvasGrid,
    resources::Resource,
    tile::TileTexture,
};

pub struct World {
    tiles_type: Vec<TileTexture>,
    tiles_agent: Vec<Option<EntityId>>,
    // tiles_resource: Vec<u8>,
    // tiles_action: Vec<TileAction>,
    entities: Vec<Entity>,
    // conflicts: Vec<Vec<TileAction>>,
    pub width: usize, // Q from Andrei: Should this remain usize or u16?
    pub height: usize,
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

        let entities = (0..agent_count)
            .map(|i| Entity {
                pos: (0, 0),
                ty: EntityType::Agent(Agent {
                    job_id: (i % 64) as u8,
                    health: 255,
                    cash: 0,
                }),
            })
            .collect();

        World {
            tiles_type,
            tiles_agent: vec![None; width * height],
            // tiles_resource: vec![0; width * height],
            // tiles_action: vec![TileAction::default(); width * height],
            entities,
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

    pub fn entity(&self, id: EntityId) -> &Entity {
        &self.entities[id.as_index()]
    }

    pub fn step(&mut self) {
        let mut entities = std::mem::take(&mut self.entities);
        for (i, entity) in entities.iter_mut().enumerate() {
            match &entity.ty {
                EntityType::Agent(a) => {
                    self.step_agent(a, &mut entity.pos, i);
                }
                EntityType::Resource(r) => {
                    self.step_resource(r, &mut entity.pos, i);
                }
                EntityType::Building(b) => self.step_building(b, &mut entity.pos, i),
            }
        }
        self.entities = entities;
    }

    fn step_agent(&mut self, a: &Agent, pos: &mut (u16, u16), i: usize) {
        let current_tile_idx = self.idx(pos.0.into(), pos.1.into());
        match a.preferred_action(*pos, &self) {
            AgentAction::Move(x, y) => {
                let idx = self.idx(x.into(), y.into());
                if self.tiles_agent[idx].is_none() {
                    self.tiles_agent[current_tile_idx] = None;
                    self.tiles_agent[idx] = Some(EntityId::new(i));
                    pos.0 = x;
                    pos.1 = y;
                }
            }
            AgentAction::None => {}
            _ => unimplemented!(),
        }
    }

    fn step_resource(&mut self, _r: &Resource, pos: &mut (u16, u16), _i: usize) {
        let _current_tile_idx = self.idx(pos.0.into(), pos.1.into());
    }

    fn step_building(&mut self, _b: &Building, pos: &mut (u16, u16), _i: usize) {
        let _current_tile_idx = self.idx(pos.0.into(), pos.1.into());
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

        grid.update_agents(
            display,
            self.entities.iter().map(|a| Sprite {
                vertex: Vf2::new(a.pos.0 as f32 * 10., a.pos.1 as f32 * 10.),
                size: Vf2::new(10., 10.),
                texture_index: a.agent().unwrap().job_id as i32,
            }),
        )
    }

    pub fn tile_type(&self, x: u16, y: u16) -> TileTexture {
        self.tiles_type[self.idx(x as usize, y as usize)]
    }
}
