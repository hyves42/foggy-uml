use crate::utils::tree::*;
use crate::datatypes::Rcc;
use crate::layout::datatypes::*;
use crate::layout::layout::*;
use std::cell::RefCell;
use std::rc::Rc;

// Tablebuilder builds a TreeLayoutElements structure where cells are aligned like in a table
// One dimension is fixed : the number of columns
// For a vertical table grid
// | lane   |
// +--------+---------+----------+
// | xxx    | xxx     | xxx      |
// +--------+---------+----------+
// | xxx    | xxx     | xxx      |
// +--------+---------+----------+
// insert line
// +--------+---------+----------+ ---
// | xxx    | xxx     | xxx      | line
// +--------+---------+----------+ ---

// For a horizontal table grid
// | line   |
// +--------+---------+----------+  +----------+
// | xxx    | xxx     | xxx      |  | xxx      |
// +--------+---------+----------+  +----------+
// | xxx    | xxx     | xxx      |  | xxx      |
// +--------+---------+----------+  +----------+ ---
// | xxx    | xxx     | xxx      |  | xxx      | lane
// +--------+---------+----------+  +----------+ ---


//                root    direction:vertical
//               /  |  \ 
//          line1 line2 line3 ... direction : horizontal
//         /  |  \
//    lane1 lane2 lane3 ...

#[derive(Debug, PartialEq, Default)]
pub struct TableBuilder {
    pub layout: LayoutContainer,
    pub direction: Direction,
    pub lanes: usize,
}

impl TableBuilder {
    pub fn new(direction: Direction, lanes: usize) -> Self {
        let elt = LayoutElement::new(direction);
        let mut layout = LayoutContainer::new(elt);


        return TableBuilder {
            layout: layout,
            lanes: lanes,
            direction: direction,
        };
    }

    // Re
    pub fn add_line(&mut self) -> Vec<NodeId>{
        let mut cells :Vec<NodeId> = vec![];


        let line = self.layout.tree.push_child(
                self.layout.root, 
                LayoutElement::new(self.direction.orthogonal())
            ).unwrap();
        for i in 0..self.lanes{
            let cell = self.layout.tree.push_child(
                    line, 
                    LayoutElement::new(self.direction)
                ).unwrap();
            cells.push(cell);
        }
        return cells;
    } // row or column, depends on table orientation

    pub fn add_lane(&mut self) -> Vec<NodeId>{
        //Not implemented
        return vec![];
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn add_lines() {
        let mut builder = Tablebuilder::new(Direction::vertical,4);
        let line1 = builder.add_line();
        assert_eq!(line1.len(), 4);
        assert_eq!(builder.layout.tree.id_by_path("0:0:0"),line1[0]);
        assert_eq!(builder.layout.tree.id_by_path("0:0:1"),line1[1]);
        assert_eq!(builder.layout.tree.id_by_path("0:0:2"),line1[2]);
        assert_eq!(builder.layout.tree.id_by_path("0:0:3"),line1[3]);

        let line2 = builder.add_line();
        assert_eq!(line2.len(), 4);
        assert_eq!(builder.layout.tree.id_by_path("0:1:0"),line2[0]);
        assert_eq!(builder.layout.tree.id_by_path("0:1:1"),line2[1]);
        assert_eq!(builder.layout.tree.id_by_path("0:1:2"),line2[2]);
        assert_eq!(builder.layout.tree.id_by_path("0:1:3"),line2[3]);
    }

}