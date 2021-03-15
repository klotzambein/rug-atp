use crate::entity::EntityId;

#[derive(Debug, Clone, Default)]
pub struct Market {
    orders_berry: Vec<Order>,
    orders_wheat: Vec<Order>,
    orders_fish: Vec<Order>,
    orders_meat: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct Order {
    value: u16,
    amount: u16,
    agent: EntityId,
}
