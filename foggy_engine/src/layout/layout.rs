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



#[derive(Debug, PartialEq, Default)]
pub struct LayoutElement {
    pub constraint: BoxConstraints,
    pub dimensions: BoxContainer,
    pub direction: Direction,
}




#[derive(Debug, PartialEq, Default)]
pub struct LayoutContainer {
    // layoutElements organized as a tree
    pub tree: TreeContainer<LayoutElement>,
    pub root: NodeId
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


impl LayoutContainer {
    pub fn new(root_elt:LayoutElement) -> Self{
        let mut tree: TreeContainer<LayoutElement> = TreeContainer::new();
        let root = tree.add_root(root_elt);
        return LayoutContainer{
            tree: tree,
            root:root
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;


    }
}
