//! Saves statistics of a simulation, this is used to display graphs in the
//! interactive mode and export data for science

use std::{
    io::{Result, Write},
    path::Path,
};

use crate::{
    entity::{
        agent::{Agent, Job},
        resources::PerResource,
        Entity,
    },
    world::World,
};

/// Saves statistics of a simulation, this is used to display graphs in the
/// interactive mode and export data for science
#[derive(Debug, Clone)]
pub struct Statistics {
    /// Prices per tick
    pub prices: PerResource<Vec<f32>>,
    /// Market volume per tick. Volume is the total amount of resources.
    pub volume: PerResource<Vec<f32>>,
    /// Count of alive agents per tick
    pub agent_count: Vec<f32>,
    /// Average greed of the alive agents per tick
    pub agent_greed: Vec<f32>,
    /// Distribution of jobs of alive agents
    pub job_counts: [Vec<f32>; 5],
    /// For every agent save the greed value and the time of death.
    pub agents: Vec<Option<(u32, u32)>>,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            prices: Default::default(),
            volume: Default::default(),
            agent_count: Default::default(),
            // agent greed and job counts should have one zero element
            agent_greed: vec![0.0],
            job_counts: [vec![0.0], vec![0.0], vec![0.0], vec![0.0], vec![0.0]],
            agents: Vec::new(),
        }
    }

    /// Call this to initialize the greed of the agents.
    pub fn init_agents(&mut self, entities: &[Entity]) {
        self.agents = entities
            .iter()
            .map(|a| match &a.ty {
                crate::entity::EntityType::Agent(a) => Some((a.greed, 0)),
                crate::entity::EntityType::Resource(_) => None,
                crate::entity::EntityType::Building(_) => None,
            })
            .collect();
    }

    /// This should be called once per step, to record the statistics
    pub fn step(&mut self, world: &World) {
        let prices = &world.market.market_price;
        for (r, p) in self.prices.iter_mut() {
            p.push(prices[r]);
        }
        let volumes = world.market.volume();
        for (r, v) in self.volume.iter_mut() {
            v.push(volumes[r] as f32);
        }

        // At the end/beginning of every step divide the sum by the total and
        // add a new element to the vector which is zero.
        self.agent_count.push(world.alive_count as f32);
        *self.agent_greed.last_mut().unwrap() /= world.alive_count as f32;
        self.agent_greed.push(0.0);
        for jc in &mut self.job_counts {
            *jc.last_mut().unwrap() /= world.alive_count as f32;
            jc.push(0.);
        }
    }

    /// This should be called once per agent per tick, to record death and job
    /// distribution
    pub fn step_agent(&mut self, a: &Agent, idx: usize) {
        if a.dead && self.agents[idx].unwrap().1 == 0 {
            // the length of agent_count is equal to the current step
            self.agents[idx].as_mut().unwrap().1 = self.agent_count.len() as u32;
        }
        if !a.dead {
            *self.agent_greed.last_mut().unwrap() += a.greed as f32;
            match a.job {
                Job::Explorer { .. } => *self.job_counts[0].last_mut().unwrap() += 1.,
                Job::Farmer => *self.job_counts[1].last_mut().unwrap() += 1.,
                Job::Lumberer => *self.job_counts[2].last_mut().unwrap() += 1.,
                Job::Fisher { .. } => *self.job_counts[3].last_mut().unwrap() += 1.,
                Job::Butcher => *self.job_counts[4].last_mut().unwrap() += 1.,
            }
        }
    }

    /// Export all the statistics to csv files
    pub fn export(&self, path: &Path) -> Result<()> {
        let mut steps_path = path.to_path_buf();
        steps_path.set_extension("steps.csv");

        let mut agents_path = path.to_path_buf();
        agents_path.set_extension("agents.csv");

        let mut dir_path = path.to_path_buf();
        dir_path.pop();
        std::fs::create_dir_all(dir_path)?;

        let mut file = std::fs::File::create(steps_path)?;
        writeln!(
            &mut file,
            "tick,agent_count,job_counts_explorer,job_counts_farmer,\
            job_counts_lumberer,job_counts_fisher,job_counts_butcher,\
            prices_wheat,prices_berry,prices_fish,prices_meat,volume_wheat,\
            volume_berry,volume_fish,volume_meat,agent_greed"
        )?;
        for i in 0..self.agent_count.len() {
            writeln!(
                &mut file,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                i,
                self.agent_count[i],
                self.job_counts[0][i],
                self.job_counts[1][i],
                self.job_counts[2][i],
                self.job_counts[3][i],
                self.job_counts[4][i],
                self.prices.wheat[i],
                self.prices.berry[i],
                self.prices.fish[i],
                self.prices.meat[i],
                self.volume.wheat[i],
                self.volume.berry[i],
                self.volume.fish[i],
                self.volume.meat[i],
                self.agent_greed[i],
            )?;
        }

        let mut file = std::fs::File::create(agents_path)?;
        writeln!(&mut file, "greed,lifetime")?;
        for a in &self.agents {
            if let Some(a) = a {
                writeln!(&mut file, "{},{}", a.0, a.1)?;
            }
        }

        Ok(())
    }
}
