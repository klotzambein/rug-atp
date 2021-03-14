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
