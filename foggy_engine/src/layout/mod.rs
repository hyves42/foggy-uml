pub mod tablebuilder;


// Basic struct to represent how drawable elements will be layout
// Think of it like the CSS box
#[derive(Debug, PartialEq, Default)]
pub struct BoxConstraints {
    pub pref_w: Option<f32>,
    pub pref_h: Option<f32>,
    pub margin_x: Option<f32>,
    pub margin_y: Option<f32>,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
}

// Actual concrete container for drawable elements
// This is the result of the layout algorithm on the boxConstraints
#[derive(Debug, PartialEq, Default)]
pub struct BoxContainer {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
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
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
    #[inline(always)]
    pub fn origin(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    #[inline(always)]
    pub fn end(&self) -> (f32, f32) {
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
