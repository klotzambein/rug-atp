//! Resources are the source of basically all value im our simulation. Resources
//! spawn on rocks on the world. And in some other spaces. They can be gathered
//! by the agents. After a resource has been depleted it will respawn somewhere
//! else on a new rock or other spot.

// TODO IVO: here is where resources are defined
#[derive(Debug, Clone, Hash)]
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

#[derive(Debug, Clone, Hash, Default)]
pub struct PerResource<T> {
    pub wheat: T,
    pub berry: T,
    pub fish: T,
    pub meat: T,
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
