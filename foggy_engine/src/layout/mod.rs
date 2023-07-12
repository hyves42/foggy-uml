pub mod diagram;
pub mod drawable;
pub mod layout;
use derive_more::{From, Into};


// Type of ID used for components of the logical domain of the diagram
#[derive(Hash, Eq, Debug, PartialEq, From, Into, Copy, Clone)]
pub struct DiagramGuid(u64);
// ids for links
#[derive(Hash, Eq, Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LinkGuid(u64);

// Type od ID used for components of the physical domain of the layout
#[derive(Hash, Eq, Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LayoutGuid(u64);

pub trait LayoutDimensions {
    fn box_dimensions(&self, box_id: LayoutGuid) -> Option<(u32, u32, u32, u32)>;
}