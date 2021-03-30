//! Resources are the source of basically all value im our simulation. Resources
//! spawn in the world when it is generated. After a resource has been depleted
//! it will respawn at the same space after some time

use crate::entity::agent::Agent;
use crate::market::Market;
use std::slice::Iter;

// This is a resource as it is in the world.
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Resource {
    pub amount: u16,
    pub timeout: u16,
    pub resource: ResourceItem,
}

impl Resource {
    pub fn new(resource: ResourceItem, amount: u16) -> Resource {
        Resource {
            amount,
            resource,
            timeout: 0,
        }
    }

    /// Gather one item from this resource.
    pub fn farm(&mut self) -> Option<ResourceItem> {
        if self.amount > 0 {
            self.amount = self.amount.saturating_sub(1);
            Some(self.resource)
        } else {
            None
        }
    }

    pub fn product(&self) -> ResourceItem {
        self.resource
    }

    pub fn produces_item(&self, item: ResourceItem) -> bool {
        self.product() == item
    }

    pub fn available(&self) -> u16 {
        self.amount
    }
}

/// These are the four different kind of resources in the world
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

    pub fn sorted(agent: &Agent, market: &Market) -> [ResourceItem; 4] {
        let mut resource_item: [ResourceItem; 4] = [
            ResourceItem::Wheat,
            ResourceItem::Berry,
            ResourceItem::Fish,
            ResourceItem::Meat,
        ];

        resource_item.sort_by_key(|r| {
            let projected_price = market.market_price(*r);
            let projected_energy = agent.nutrition[*r] as u32 * 1_000_000;

            std::cmp::Reverse(projected_energy / (projected_price + 1))
        });

        resource_item
    }

    pub fn from_index(index: u8) -> ResourceItem {
        match index {
            0 => ResourceItem::Wheat,
            1 => ResourceItem::Berry,
            2 => ResourceItem::Fish,
            3 => ResourceItem::Meat,
            _ => panic!(
                "Index {} out of bounds when trying to access a ResourceItem",
                index
            ),
        }
    }
}

/// Throughout our codebase we often need to store some information for every
/// resource, this struct helps a lot with that.
#[derive(Debug, Clone, Hash, Default, PartialEq, Eq)]
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

    /// Transform all values in this struct.
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> PerResource<U> {
        PerResource {
            wheat: f(&self.wheat),
            berry: f(&self.berry),
            fish: f(&self.fish),
            meat: f(&self.meat),
        }
    }

    /// Like map but for two PerResources.
    pub fn combine<V, U>(
        &self,
        other: &PerResource<V>,
        mut f: impl FnMut(&T, &V) -> U,
    ) -> PerResource<U> {
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
            _ => panic!(
                "Index {} out of bounds when trying to access a ResourceItem",
                index
            ),
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
            _ => panic!(
                "Index {} out of bounds when trying to access a ResourceItem",
                index
            ),
        }
    }
}
