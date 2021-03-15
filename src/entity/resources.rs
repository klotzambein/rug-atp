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

    pub fn produces_item(&self, item: ResourceItem) -> bool {
        match self {
            Resource::Wheat(_) => item == ResourceItem::Wheat,
            Resource::Berry(_) => item == ResourceItem::Berry,
            Resource::Fish(_) => item == ResourceItem::Fish,
            Resource::Meat(_) => item == ResourceItem::Meat,
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

impl<T: Clone> PerResource<T> {
    pub fn iter(&self) -> impl Iterator<Item = (ResourceItem, T)> {
        let s = self.clone();
        (0..4).map(move |i| match i {
            0 => (ResourceItem::Wheat, s.wheat.clone()),
            1 => (ResourceItem::Berry, s.berry.clone()),
            2 => (ResourceItem::Fish, s.fish.clone()),
            3 => (ResourceItem::Meat, s.meat.clone()),
            _ => unreachable!(),
        })
    }
}
