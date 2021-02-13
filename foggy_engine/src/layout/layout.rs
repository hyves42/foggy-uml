use crate::datatypes::*;
use crate::layout::datatypes::*;
use crate::utils::tree::*;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

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


// todo
// modifier/compléter le champ direction pour décrire :
// - que les enfants sont placés à des positions fixes qu'il ne faut pas modifier 
// - qu'il faut placer les éléments enfants sur un cercle
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
