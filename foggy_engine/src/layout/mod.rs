pub mod tablebuilder;
pub mod drawable;
pub mod placeable;
use derive_more::{From, Into};

// Type od ID used for components of the logical domain of the diagram
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct DiagramGuid(u64);
// Type od ID used for components of the physical domain of the layout
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LayoutGuid(u64);


// Basic struct to represent how drawable elements will be layout
// Think of it like the CSS box
#[derive(Debug, PartialEq, Default)]
pub struct BoxConstraints {
    pub pref_w: Option<u32>,
    pub pref_h: Option<u32>,
    pub margin_x: Option<u32>,
    pub margin_y: Option<u32>,
    pub padding_x: Option<u32>,
    pub padding_y: Option<u32>,
}

// Actual concrete container for drawable elements
// This is the result of the layout algorithm on the boxConstraints
#[derive(Debug, PartialEq, Default)]
pub struct BoxContainer {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl BoxConstraints {
    pub fn new() -> Self {
        Default::default()
    }
}

impl BoxContainer {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    pub fn center(&self) -> (u32, u32) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
    #[inline(always)]
    pub fn origin(&self) -> (u32, u32) {
        (self.x, self.y)
    }
    #[inline(always)]
    pub fn end(&self) -> (u32, u32) {
        (self.x + self.w, self.y + self.h)
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Vertical
    }
}

impl Direction {
    pub fn orthogonal(&self) -> Direction {
        match self {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        }
    }
}


#[derive(Debug, PartialEq, Default)]
pub struct LayoutElement {
    pub constraint: BoxConstraints,
    pub dimensions: BoxContainer,
    pub direction: Direction,
}



impl LayoutElement {
    pub fn new(direction: Direction) -> Self {
        LayoutElement {
            constraint: BoxConstraints::new(),
            dimensions: BoxContainer::new(),
            direction: direction,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    
}
