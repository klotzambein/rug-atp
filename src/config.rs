use serde::{Deserialize, Serialize};

/// This config defines all the parameters of a simulation, the repetitions in
/// batch mode and for how many ticks the simulation should last.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Under what altitude should we choose the ocean biome
    pub ocean_cutoff: isize,
    /// Under what altitude should we choose the beach biome
    pub beach_cutoff: isize,
    /// Time in the the agents will leave their job and the market to go home.
    pub closing_time: u32,
    /// At this energy an agent will stop doing its job and go home to eat.
    pub critical_energy: u32,
    /// Length of one day, with this value numerous other values also have to
    /// change.
    pub day_length: u32,
    /// The time in ticks it takes for an order to expire.
    pub default_exp: u32,
    /// The time in ticks it takes for an order to be re evaluated.
    pub default_rval: u32,
    /// The time in ticks agents will explore their surroundings before deciding
    /// what job to take, when they are an explorer.
    pub exploration_timeout: u16,
    /// Mean of the greed distribution.
    pub greed_mean: f32,
    /// Standard deviation of the greed distribution.
    pub greed_sd: f32,
    /// Initial cash of the agents
    pub initial_cash: u32,
    /// Initial energy of the agents
    pub initial_energy: u32,
    /// Initial resources of the agents, this will be the same for all 4 resources.
    pub initial_inventory: u32,
    /// The initial nutritional value of all resources per agent.
    pub initial_nutrition: u8,
    /// The update rate of the market price moving average.
    pub market_price_update: f32,
    /// Maximum energy of the agents.
    pub max_energy: u32,
    /// Energy cost per step per agent. This will be subtracted every step.
    pub energy_cost: u32,
    /// When an agent eats a resource, this will be added to the nutritional
    /// values of the resources that the agent did not just eat.
    pub nutrition_add: u8,
    /// When an agent eats a resource, this will be subtracted from the
    /// nutritional values of the resources that the agent just ate.
    pub nutrition_sub: u8,
    /// When an order gets reevaluated and the market price is lower decay the
    /// order with this constant
    pub order_price_decay: u32,
    /// Mean amount of resources generated on one resource tile.
    pub resource_amount_mean: f32,
    /// SD of the resource amount.
    pub resource_amount_sd: f32,
    /// When exploring how high should we value nearby boats.
    pub explorer_fish_points: u32,
    /// When exploring, what should we divide the amount of resources by to get the value.
    pub explorer_resource_divisor: u32,
    /// How long until resources respawn (ticks).
    pub resource_timeout: u16,
    /// When agents search for something, how big is the search radius. Note
    /// that we have O(n^2) complexity.
    pub search_radius: usize,
    /// Timeout duration until agents become explorers to reevaluate their job if they are unsuccessful.
    pub timeout_quota: u16,
    /// Chance of an agent not walking in a random direction to not get stuck.
    pub unstuckifier_chance: f64,
    /// Total amount of ticks to run the simulation for in batch mode.
    pub batch_total_step_count: u32,
    /// Total amount of repetitions to repeat the simulation for in batch mode.
    pub repetitions: u32,
}

impl Default for Config {
    fn default() -> Self {
        const DAY_LENGTH: u32 = 200;
        Config {
            ocean_cutoff: -300,
            beach_cutoff: -250,
            closing_time: (DAY_LENGTH * 3) / 4,
            critical_energy: 500,
            day_length: DAY_LENGTH,
            default_exp: DAY_LENGTH * 10,
            default_rval: DAY_LENGTH * 3,
            exploration_timeout: 500,
            greed_mean: 5.,
            greed_sd: 10.,
            initial_cash: 20000,
            initial_energy: 5000,
            initial_inventory: 0,
            initial_nutrition: 100,
            market_price_update: 0.01,
            max_energy: 10000,
            energy_cost: 2,
            nutrition_add: 4,
            nutrition_sub: 9,
            order_price_decay: 75,
            resource_amount_mean: 20.,
            resource_amount_sd: 10.,
            resource_timeout: DAY_LENGTH as u16 * 10,
            explorer_fish_points: 50,
            explorer_resource_divisor: 10,
            search_radius: 15,
            timeout_quota: DAY_LENGTH as u16 * 10,
            unstuckifier_chance: 0.75,
            batch_total_step_count: DAY_LENGTH * 5000,
            repetitions: 1,
        }
    }
}
