// pub const closing_time: u16 = (DAY_LENGTH * 3) / 4;
// pub const critical_energy: u32 = 1000;
// pub const day_length: u16 = 200;
// pub const day_length: u32 = 200;
// pub const default_exp: u32 = DAY_LENGTH * 10;
// pub const default_rval: u32 = DAY_LENGTH * 3;
// pub const exploration_timeout: u16 = 500;
// pub const greed_mean: f32 = 5.;
// pub const greed_sd: f32 = 10.;
// pub const initial_cash: u32 = 5000;
// pub const initial_energy: u32 = 5000;
// pub const initial_inventory: u32 = 0;
// pub const initial_nutrition: u8 = 100;
// pub const max_energy: u32 = 10000;
// pub const nutrition_add: u8 = 4;
// pub const nutrition_sub: u8 = 9;
// pub const order_price_decay: u32 = 75;
// pub const resource_amount_mean: f32 = 20.;
// pub const resource_amount_sd: f32 = 10.;
// pub const resource_timeout: u16 = DAY_LENGTH as u16 * 10;
// pub const search_radius: usize = 15;
// pub const timeout_quota: u16 = (DAY_LENGTH as u16) * 10;
// pub const unstuckifier_chance: f64 = 0.75;

#[derive(Debug, Clone)]
pub struct Config {
    pub closing_time: u32,
    pub critical_energy: u32,
    pub day_length: u32,
    pub default_exp: u32,
    pub default_rval: u32,
    pub exploration_timeout: u16,
    pub greed_mean: f32,
    pub greed_sd: f32,
    pub initial_cash: u32,
    pub initial_energy: u32,
    pub initial_inventory: u32,
    pub initial_nutrition: u8,
    pub market_price_update: f32,
    pub max_energy: u32,
    pub nutrition_add: u8,
    pub nutrition_sub: u8,
    pub order_price_decay: u32,
    pub resource_amount_mean: f32,
    pub resource_amount_sd: f32,
    pub resource_timeout: u16,
    pub search_radius: usize,
    pub timeout_quota: u16,
    pub unstuckifier_chance: f64,
}

impl Default for Config {
    fn default() -> Self {
        const DAY_LENGTH: u32 = 200;
        Config {
            closing_time: (DAY_LENGTH * 3) / 4,
            critical_energy: 1000,
            day_length: DAY_LENGTH,
            default_exp: DAY_LENGTH * 10,
            default_rval: DAY_LENGTH * 3,
            exploration_timeout: 500,
            greed_mean: 5.,
            greed_sd: 10.,
            initial_cash: 5000,
            initial_energy: 5000,
            initial_inventory: 0,
            initial_nutrition: 100,
            market_price_update: 0.01,
            max_energy: 10000,
            nutrition_add: 4,
            nutrition_sub: 9,
            order_price_decay: 75,
            resource_amount_mean: 20.,
            resource_amount_sd: 10.,
            resource_timeout: DAY_LENGTH as u16 * 10,
            search_radius: 15,
            timeout_quota: (DAY_LENGTH as u16) * 10,
            unstuckifier_chance: 0.75,
        }
    }
}
