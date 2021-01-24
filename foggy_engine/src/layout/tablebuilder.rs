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

#[derive(Debug, PartialEq, Default)]
pub struct TableBuilder {
    pub tree: TreeContainer,
    pub direction: Direction,
    pub lanes: usize,
}

impl TableBuilder {
    pub fn new(direction: Direction, lanes: usize) -> Self {
        let mut tree = TreeContainer::new(direction);

        return TableBuilder {
            tree: tree,
            lanes: lanes,
            direction: direction,
        };
    }

    // Re
    pub fn add_line() {} // row or column, depends on table orientation

    pub fn add_lane() {}
}
