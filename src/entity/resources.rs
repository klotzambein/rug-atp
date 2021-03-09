use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ResourceId(NonZeroU16);

impl ResourceId {
    pub fn new(idx: usize) -> ResourceId {
        ResourceId(NonZeroU16::new((idx + 1) as u16).expect("Resource ID overflow"))
    }
}

#[derive(Debug, Clone, Default, Hash)]
pub struct Resource {
    pub amount: u8,
    pub refresh: u8, // Time to refresh a resource (0 for non-regenerable).
}
