//! Resources are the source of basically all value im our simulation. Resources
//! spawn on rocks on the world. And in some other spaces. They can be gathered
//! by the agents. After a resource has been depleted it will respawn somewhere
//! else on a new rock or other spot.
use std::slice::Iter;
use crate::entity::agent::Agent;
use crate::market::Market;

// TODO IVO: here is where resources are defined
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Resource {
    Wheat(u8),
    Berry(u8),
    Fish(u8),
    Meat(u8),
}

impl Resource {
    pub fn farm(&mut self) -> ResourceItem {
        match self {
            Resource::Wheat(amount) => {
                *amount = amount.saturating_sub(1);
                ResourceItem::Wheat
            }
            Resource::Berry(amount) => {
                *amount = amount.saturating_sub(1);
                ResourceItem::Berry
            }
            Resource::Fish(amount) => {
                *amount = amount.saturating_sub(1);
                ResourceItem::Fish
            }
            Resource::Meat(amount) => {
                *amount = amount.saturating_sub(1);
                ResourceItem::Meat
            }
        }
    }

    pub fn product(&self) -> ResourceItem {
        match self {
            Resource::Wheat(_) => ResourceItem::Wheat,
            Resource::Berry(_) => ResourceItem::Berry,
            Resource::Fish(_) => ResourceItem::Fish,
            Resource::Meat(_) => ResourceItem::Meat,
        }
    }

    pub fn produces_item(&self, item: ResourceItem) -> bool {
        self.product() == item
    }

    pub fn available(&self) -> u8 {
        match self {
            Resource::Wheat(n) => *n,
            Resource::Berry(n) => *n,
            Resource::Fish(n) => *n,
            Resource::Meat(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ResourceItem {
    Wheat,
    Berry,
    Fish,
    Meat,
}

impl ResourceItem {
    pub fn iterator() -> Iter<'static, ResourceItem> {
        static RESOURCE_ITEMS: [ResourceItem; 4] = [
            ResourceItem::Wheat,
            ResourceItem::Berry,
            ResourceItem::Fish,
            ResourceItem::Meat,
        ];
        RESOURCE_ITEMS.iter()
    }

    pub fn sort_by_benefit(
        items: &mut [ResourceItem; 4], 
        agent: &Agent,
        market: &Market
    )
    {

        // Sort (direct selection) the array by the benefit 
        // (given by the accumulated energy over the cost)
        for i in 1..4 {
            
            let mut max_ind: u8 = 0;
            let mut max_energy: u16 = 0;
            let mut max: f32 = 0.0;

            for j in i..4 {
                
                let (projected_price, _) = market.market_price(
                    ResourceItem::from_ind(i)
                );
                let projected_energy = (agent.nutrition[i]) as f32;

                let benefit: f32 = projected_energy / (projected_price as f32);

                if benefit > max {
                    max = benefit;
                    max_ind = j;
                }
            }
            
            let swap: ResourceItem = items[i as usize];
            items[i as usize] = items[max_ind as usize];
            items[max_ind as usize] = swap;
        }
    }

    pub fn sorted(
        agent: &Agent,
        market: &Market
    ) -> [ResourceItem; 4] {
        let mut RESOURCE_ITEMS: [ResourceItem; 4] = [
            ResourceItem::Wheat,
            ResourceItem::Berry,
            ResourceItem::Fish,
            ResourceItem::Meat,
        ];
        ResourceItem::sort_by_benefit(&mut RESOURCE_ITEMS, agent, market);
        RESOURCE_ITEMS
    }

    pub fn from_ind(index: u8) -> ResourceItem {
        match index {
            0 => ResourceItem::Wheat,
            1 => ResourceItem::Berry,
            2 => ResourceItem::Fish,
            3 => ResourceItem::Meat,
            _ => panic!("Index {} out of bounds when trying to access a ResourceItem")
        }
    }
}


#[derive(Debug, Clone, Hash, Default)]
pub struct PerResource<T> {
    pub wheat: T,
    pub berry: T,
    pub fish: T,
    pub meat: T,
}

impl<T: Clone> PerResource<T> {
    pub fn new(val: T) -> PerResource<T> {
        PerResource {
            wheat: val.clone(),
            berry: val.clone(),
            fish: val.clone(),
            meat: val,
        }       
    }
}

impl<T> PerResource<T> {
    pub fn iter(&self) -> impl Iterator<Item = (ResourceItem, &T)> {
        use std::iter::once;
        once((ResourceItem::Wheat, &self.wheat))
            .chain(once((ResourceItem::Berry, &self.berry)))
            .chain(once((ResourceItem::Fish, &self.fish)))
            .chain(once((ResourceItem::Meat, &self.meat)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ResourceItem, &mut T)> + '_ {
        use std::iter::once;
        once((ResourceItem::Wheat, &mut self.wheat))
            .chain(once((ResourceItem::Berry, &mut self.berry)))
            .chain(once((ResourceItem::Fish, &mut self.fish)))
            .chain(once((ResourceItem::Meat, &mut self.meat)))
    }

    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> PerResource<U> {
        PerResource {
            wheat: f(&self.wheat),
            berry: f(&self.berry),
            fish: f(&self.fish),
            meat: f(&self.meat),
        }
    }

    pub fn combine<V, U>(&self, other: &PerResource<V>, mut f: impl FnMut(&T, &V) -> U) -> PerResource<U> {
        PerResource {
            wheat: f(&self.wheat, &other.wheat),
            berry: f(&self.berry, &other.berry),
            fish: f(&self.fish, &other.fish),
            meat: f(&self.meat, &other.meat),
        }
    }
}

impl<T> std::ops::Index<ResourceItem> for PerResource<T> {
    type Output = T;

    fn index(&self, index: ResourceItem) -> &Self::Output {
        match index {
            ResourceItem::Wheat => &self.wheat,
            ResourceItem::Berry => &self.berry,
            ResourceItem::Fish => &self.fish,
            ResourceItem::Meat => &self.meat,
        }
    }
}

impl<T> std::ops::IndexMut<ResourceItem> for PerResource<T> {
    fn index_mut(&mut self, index: ResourceItem) -> &mut Self::Output {
        match index {
            ResourceItem::Wheat => &mut self.wheat,
            ResourceItem::Berry => &mut self.berry,
            ResourceItem::Fish => &mut self.fish,
            ResourceItem::Meat => &mut self.meat,
        }
    }
}

impl<T> std::ops::Index<u8> for PerResource<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.wheat,
            1 => &self.berry,
            2 => &self.fish,
            3 => &self.meat,
            _ => panic!("Index {} out of bounds when trying to access a ResourceItem")
        }
    }
}


impl<T> std::ops::IndexMut<u8> for PerResource<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.wheat,
            1 => &mut self.berry,
            2 => &mut self.fish,
            3 => &mut self.meat,
            _ => panic!("Index {} out of bounds when trying to access a ResourceItem")
        }
    }
}
