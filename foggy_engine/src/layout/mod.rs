pub mod layout;
pub mod drawable;
pub mod diagram;
use derive_more::{From, Into};

// Type of ID used for components of the logical domain of the diagram
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct DiagramGuid(u64);
// ids for links
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LinkGuid(u64);
// ids for x/y dimensions, shared between the physical layout and the constraint solvers
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct DimensionGuid(u64);
// Type od ID used for components of the physical domain of the layout
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LayoutGuid(u64);



