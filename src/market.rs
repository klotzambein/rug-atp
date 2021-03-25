use crate::entity::EntityId;
use crate::entity::resources::*;

//use crate::sorted_vec;

use std::convert::TryInto;

#[derive(Debug, Clone, Default)]
pub struct Market {
    // TODO Ivo - make sure the orders are sorted
    orders_berry: Vec<Order>,
    orders_wheat: Vec<Order>,
    orders_fish: Vec<Order>,
    orders_meat: Vec<Order>,
    market_price_berry: (u16, u16),
    market_price_wheat: (u16, u16),
    market_price_fish: (u16, u16),
    market_price_meat: (u16, u16),
}

#[derive(Debug, Clone)]
pub struct Order {
    value: u16,
    amount: u16,
    agent: EntityId,
    expiration: u16
}

impl Market
{
    /*
        Private internal method that executes the purchase of the specific resource
    */
    fn buy_resource(&mut self, resource: ResourceItem, amount: u16) 
        -> Result<u16, (u16, u16)>
    {
        let mut orders: &Vec<Order> = self.get_orders(resource);
        // Variables keeping track of the amount left to fulfill the buy order and accumulated price
        let mut am_left = amount;
        let acc_price: u16 = 0;

        // Variables keeping track of the price of the last fulfilled order and the accumulated amount
        // of the resource being sold
        let mut last_fulfilled: u16;
        let mut demand_ord: u16 = 0;

        // Iterating through the orders from the cheapest to the most expensive and fulfilling them
        // until the requested amount is filled
        for mut order in orders.iter()
        {
            if am_left == 0 { break; }

            if order.amount > am_left.into() {
                order.amount.saturating_sub(am_left.into());
                acc_price.saturating_add(order.value * am_left);
                last_fulfilled = order.value;
                demand_ord += am_left;
                am_left = 0;
            }

            else {
                am_left.saturating_sub(order.amount.try_into().unwrap());
                acc_price.saturating_add(order.value * order.amount);

                last_fulfilled = order.value;
                demand_ord += order.amount;

                order.amount.saturating_sub(order.amount);
            }
        }

        // If the order is fully fulfilled only the accumulated price is returned
        // Otherwise, the fulfilled amount is returned as well
        if am_left == 0 { 
            return Ok(acc_price);
        }
        
        return Err((acc_price, am_left));
    }

    pub fn buy(&mut self, resource: ResourceItem, amount: u16) -> Result<u16, (u16, u16)>
    {
        let result = self.buy_resource(resource, amount);
        
        // Remove all orders where the amount is 0
        
        match resource {
            ResourceItem::Berry => {
                self.orders_berry.retain(|ord| ord.amount > 0);
            }
            ResourceItem::Wheat => {
                self.orders_wheat.retain(|ord| ord.amount > 0);
            }
            ResourceItem::Meat => {
                self.orders_meat.retain(|ord| ord.amount > 0);
            }
            ResourceItem::Fish => {
                self.orders_fish.retain(|ord| ord.amount > 0);
            }
        };
        result
    }

    pub fn market_price(&self, resource_item: ResourceItem) -> (u16, u16)
    {
        match resource_item {
            ResourceItem::Wheat => self.market_price_wheat,
            ResourceItem::Berry => self.market_price_berry,
            ResourceItem::Fish => self.market_price_fish,
            ResourceItem::Meat=> self.market_price_meat,
        }
    }

    pub fn get_orders(&mut self, resource_item: ResourceItem) -> &mut Vec<Order> {
        match resource_item {
            ResourceItem::Wheat => &mut self.orders_wheat,
            ResourceItem::Berry => &mut self.orders_berry,
            ResourceItem::Fish => &mut self.orders_fish,
            ResourceItem::Meat=> &mut self.orders_meat,
        }
    }

    fn total_amount(&self, orders: &Vec<Order>) -> u16 {
        let mut sum: u16 = 0;
        for order in orders.iter() {
            sum += order.amount;
        }

        sum
    }

    pub fn availability(&self, resource_item: ResourceItem) -> u16 
    {
        match resource_item {
            ResourceItem::Wheat => self.total_amount(&self.orders_wheat),
            ResourceItem::Berry => self.total_amount(&self.orders_wheat),
            ResourceItem::Fish => self.total_amount(&self.orders_wheat),
            ResourceItem::Meat=> self.total_amount(&self.orders_wheat),
        }
    }
}