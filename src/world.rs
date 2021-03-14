use dear_gui::graphics::primitives::{Sprite, Vf2};
use glium::Display;
use rand::Rng;

use crate::{
    entity::{
        agent::{Agent, AgentAction},
        building::Building,
        resources::Resource,
    },
    entity::{Entity, EntityId, EntityType},
    generation::BiomeMap,
    grid::CanvasGrid,
    tile::TileType,
};

pub struct World {
    pub tiles_type: Vec<TileType>,
    pub tiles_entity: Vec<Option<EntityId>>,
    // tiles_resource: Vec<u8>,
    // tiles_action: Vec<TileAction>,
    entities: Vec<Entity>,
    // conflicts: Vec<Vec<TileAction>>,
    pub width: usize,
    pub height: usize,
    pub dirty: bool,
}

impl World {
    pub fn new(width: usize, height: usize, agent_count: usize, rng: &mut impl Rng) -> World {
        let biomes = BiomeMap::new();

        let mut entities: Vec<_> = (0..agent_count)
            .map(|_i| Entity {
                pos: Pos(0, 0),
                ty: EntityType::Agent(Agent::default()),
            })
            .collect();
        let mut tiles_entity = vec![None; width * height];
        let tiles_type = (0..width * height)
            .map(|i| {
                let pos = Pos((i % width) as i16, (i / width) as i16);
                let (tt, e) = biomes.get(pos, rng);
                if let Some(mut e) = e {
                    e.initialize(pos, &mut entities);
                    entities.push(Entity { pos, ty: e });
                    tiles_entity[i] = Some(EntityId::new(entities.len() - 1))
                }
                tt
            })
            .take(width * height)
            .collect::<Vec<_>>();

        World {
            tiles_type,
            tiles_entity,
            // tiles_resource: vec![0; width * height],
            // tiles_action: vec![TileAction::default(); width * height],
            entities,
            // conflicts: Vec::new(),
            width,
            height,
            dirty: true,
        }
    }

    pub fn idx(&self, p: Pos) -> usize {
        let x = p.0 as usize;
        let y = p.1 as usize;
        debug_assert!(x < self.width && y < self.height);
        x + y * self.width
    }

    pub fn iter_neighbours(&self) -> impl Iterator<Item = (Pos, [TileType; 9])> + '_ {
        (0..self.width)
            .flat_map(move |x| (0..self.height).map(move |y| (x, y)))
            .map(move |(x, y)| {
                let left = ((x + self.width - 1) % self.width) as i16;
                let right = ((x + 1) % self.width) as i16;
                let bot = ((y + self.width - 1) % self.width) as i16;
                let top = ((y + 1) % self.width) as i16;
                let x = x as i16;
                let y = y as i16;
                (
                    Pos(x, y),
                    [
                        self.tiles_type[self.idx(Pos(left, top))],
                        self.tiles_type[self.idx(Pos(x, top))],
                        self.tiles_type[self.idx(Pos(right, top))],
                        self.tiles_type[self.idx(Pos(left, y))],
                        self.tiles_type[self.idx(Pos(x, y))],
                        self.tiles_type[self.idx(Pos(right, y))],
                        self.tiles_type[self.idx(Pos(left, bot))],
                        self.tiles_type[self.idx(Pos(x, bot))],
                        self.tiles_type[self.idx(Pos(right, bot))],
                    ],
                )
            })
    }

    pub fn wrap_pos(&self, x: isize, y: isize) -> Pos {
        Pos(
            x.rem_euclid(self.width as isize) as i16,
            y.rem_euclid(self.width as isize) as i16,
        )
    }

    /// This function searches for a tile around the given position, by going
    /// around in a spiral. We will call the closure until it returns true, or we
    /// reach n tiles.
    pub fn find_tile_around(
        &self,
        p: Pos,
        n: usize,
        mut f: impl FnMut(Pos) -> bool,
    ) -> Option<Pos> {
        (2..)
            .map(|i| (i / 2, i % 4))
            .flat_map(|(n, d)| std::iter::repeat(d).take(n))
            .scan((p.0 as isize, p.1 as isize), |p, d| {
                let pos = self.wrap_pos(p.0, p.1);
                match d {
                    0 => *p = (p.0, p.1 + 1),
                    1 => *p = (p.0 + 1, p.1),
                    2 => *p = (p.0, p.1 - 1),
                    3 => *p = (p.0 - 1, p.1),
                    _ => unreachable!(),
                }
                Some(pos)
            })
            .take(n)
            .find(|p| f(*p))
    }

    pub fn entity(&self, id: EntityId) -> &Entity {
        &self.entities[id.as_index()]
    }

    pub fn entity_at(&self, pos: Pos) -> Option<&Entity> {
        let e = self.tiles_entity[self.idx(pos)]?;
        Some(&self.entities[e.as_index()])
    }

    pub fn step(&mut self) {
        for i in 0..self.entities.len() {
            let mut entity = self.entities[i].clone();
            match &mut entity.ty {
                EntityType::Agent(a) => {
                    self.step_agent(a, &mut entity.pos, i);
                }
                EntityType::Resource(r) => {
                    self.step_resource(r, &mut entity.pos, i);
                }
                EntityType::Building(b) => self.step_building(b, &mut entity.pos, i),
            }
            self.entities[i] = entity;
        }
    }

    fn step_agent(&mut self, a: &mut Agent, pos: &mut Pos, i: usize) {
        if *pos == Pos(-1, -1) {
            return;
        }
        let current_tile_idx = self.idx(*pos);
        match a.step(*pos, &self) {
            AgentAction::Move(p) => {
                let idx = self.idx(p);
                if self.tiles_entity[idx].is_none() {
                    self.tiles_entity[current_tile_idx] = None;
                    self.tiles_entity[idx] = Some(EntityId::new(i));
                    *pos = p;
                }
            }
            AgentAction::None => {}
            _ => unimplemented!(),
        }
    }

    fn step_resource(&mut self, _r: &Resource, pos: &mut Pos, _i: usize) {
        let _current_tile_idx = self.idx(*pos);
    }

    fn step_building(&mut self, _b: &Building, pos: &mut Pos, _i: usize) {
        let _current_tile_idx = self.idx(*pos);
    }

    pub fn update_grid(&mut self, display: &Display, grid: &mut CanvasGrid) {
        assert_eq!(grid.width * 32, self.width);
        assert_eq!(grid.height * 32, self.height);
        if self.dirty {
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
            self.dirty = false;
        }

        grid.update_agents(
            display,
            self.entities.iter().map(|a| Sprite {
                vertex: Vf2::new(a.pos.0 as f32 * 10., a.pos.1 as f32 * 10.),
                size: Vf2::new(10., 10.),
                texture_index: a.texture(),
            }),
        )
    }

    pub fn tile_type(&self, p: Pos) -> TileType {
        self.tiles_type[self.idx(p)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i16, pub i16);

impl Pos {
    pub fn wrap(self, world: &World) -> Self {
        Pos(
            self.0.rem_euclid(world.width as i16),
            self.1.rem_euclid(world.height as i16),
        )
    }
}
