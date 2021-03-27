use crate::entity::{
    resources::{PerResource, ResourceItem},
    EntityId,
};

pub const DEFAULT_EXP: u32 = 200;

#[derive(Debug, Clone, Default)]
pub struct Market {
    pub market_price: PerResource<u32>,
    pub market_demand: PerResource<u32>,
    // TODO Ivo - make sure the orders are sorted
    orders: PerResource<Vec<Order>>,
}

impl Market {
    pub fn cache_prices(&mut self, tick: u32) {
        if tick % 200 == 0 {
            self.market_demand = Default::default();
        }

        for (r, orders) in self.orders.iter_mut() {
            let market_price = self.market_price[r];
            orders.iter_mut().for_each(|o| o.cache_price(market_price));
            orders.sort_by_key(|o| o.cached_price)
        }
    }

    pub fn prices(&self) -> PerResource<Option<u32>> {
        self.orders.map(|os| Some(os.last()?.cached_price))
    }

    pub fn order(&mut self, agent: EntityId, item: ResourceItem, price: u32, amount: u32) {
        let orders = &mut self.orders[item];
        let pos = orders
            .binary_search_by_key(&price, |o| o.cached_price)
            .unwrap_or_else(|e| e);
        orders.insert(
            pos,
            Order {
                cached_price: price,
                value: price,
                amount,
                agent,
                expiration: DEFAULT_EXP,
            },
        );
    }

    pub fn volume(&self) -> PerResource<u32> {
        self.orders.map(|o| o.iter().map(|a| a.amount).sum())
    }

    // pub fn purchase(&mut self, item: ResourceItem) -> (EntityId, u32) {
    //     let o = self.orders[item].last_mut().unwrap();
    //     let price = o.cached_price;
    //     let agent = o.agent;
    //     o.amount -= 1;
    //     if o.amount == 0 {
    //         self.orders[item].pop();
    //     }

    //     (agent, price)
    // }

    /// Private internal method that executes the purchase of the specific resource    
    fn buy_resource(
        &mut self,
        resource: ResourceItem,
        amount: u32,
        cash_available: u32,
    ) -> (Vec<(EntityId, u32)>, u32) {
        let mut sellers: Vec<(EntityId, u32)> = Vec::new();

        let orders: &mut Vec<Order> = &mut self.orders[resource];
        // Variables keeping track of the amount left to fulfill the buy order and accumulated price
        let mut am_left = amount;
        let mut acc_price: u32 = 0;

        // Iterating through the orders from the cheapest to the most expensive and fulfilling them
        // until the requested amount is filled
        for order in orders.iter_mut().rev() {
            if am_left == 0 {
                break;
            }

            // If the agent wants to partially fulfill the order, it still stays
            // but its amount is decremented
            if order.amount > am_left.into() {
                if am_left * order.cached_price + acc_price > cash_available {
                    break;
                }

                // Fulfill the order
                let (m_p, demand) = order.fulfill(am_left.into());
                sellers.push((order.agent, am_left * order.cached_price));

                // Update the demand and market price of the resource
                self.market_price[resource] = m_p;
                self.market_demand[resource] = self.market_demand[resource].saturating_add(demand);

                acc_price = acc_price.saturating_add(order.cached_price * am_left);
                am_left = 0;
            } else {
                if order.amount * order.cached_price + acc_price > cash_available {
                    break;
                }

                am_left = am_left.saturating_sub(order.amount.into());
                acc_price = acc_price.saturating_add(order.cached_price * order.amount);

                let (m_p, demand) = order.fulfill(order.amount);
                sellers.push((order.agent, order.amount * order.cached_price));

                // Update the demand and market price of the resource
                self.market_price[resource] = m_p;
                self.market_demand[resource] = self.market_demand[resource].saturating_add(demand);
            }
        }

        // If the order is fully fulfilled only the accumulated price is returned
        // Otherwise, the fulfilled amount is returned as well
        return (sellers, amount - am_left);
    }

    pub fn buy(
        &mut self,
        resource: ResourceItem,
        amount: u32,
        cash_available: u32,
    ) -> (Vec<(EntityId, u32)>, u32) {
        let result = self.buy_resource(resource, amount, cash_available);

        // Remove all orders where the amount is 0

        self.orders[resource].retain(|ord| ord.amount > 0);

        result
    }

    pub fn market_price(&self, resource_item: ResourceItem) -> (u32, u32) {
        (
            self.market_price[resource_item],
            self.market_demand[resource_item],
        )
    }

    pub fn total_price(&self, meals: &PerResource<u32>) -> u32 {
        let mut sum: u32 = 0;
        for r_item in ResourceItem::iterator() {
            sum += self.market_price[*r_item] * meals[*r_item];
        }

        sum
    }

    fn total_amount(&self, orders: &Vec<Order>) -> u32 {
        let mut sum: u32 = 0;
        for order in orders.iter() {
            sum += order.amount;
        }

        sum
    }

    pub fn availability(&self, resource_item: ResourceItem) -> u32 {
        self.total_amount(&self.orders[resource_item])
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    cached_price: u32,
    value: u32,
    amount: u32,
    agent: EntityId,
    expiration: u32,
}

impl Order {
    pub fn cache_price(&mut self, market_price: u32) {
        // If the order has expired, its price needs to update to be better
        // suited to the market and its expiration is set back to the default
        if self.expiration == 0 {
            // The price of the order is updated so it is twice as close
            // to the market price - that is if the order is more expensive
            // than the market price
            if self.cached_price > market_price {
                let diff = self.cached_price - market_price;
                self.cached_price = self.cached_price.saturating_sub(diff / 2)
            }
            // If the order price is actually below the market price, then the
            // market has shrunk and the market price needs to fall
            // In that case the order price will be reduced by 25%
            else {
                self.cached_price = self.cached_price.saturating_mul(3) / 4;
            }
            // In DEFAULT_EXP steps, if the order is still not fulfilled,
            // its price will be updated again
            self.expiration = DEFAULT_EXP;
        }
        // If the order has not expired, it simply lowers the expiration
        else {
            self.expiration -= 1;
        }
    }

    pub fn fulfill(&mut self, _amount: u32) -> (u32, u32) {
        if _amount >= self.amount {
            self.amount = 0;
        } else {
            self.amount = self.amount.saturating_sub(_amount);
        }
        (self.cached_price, self.amount)
    }
}
