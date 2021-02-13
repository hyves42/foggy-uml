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
// +--------+---------+----------+   i   +----------+
// | xxx    | xxx     | xxx      |   n   | xxx      |
// +--------+---------+----------+   s   +----------+
// | xxx    | xxx     | xxx      |   e   | xxx      |
// +--------+---------+----------+   r   +----------+ ---
// | xxx    | xxx     | xxx      |   t   | xxx      | lane
// +--------+---------+----------+ line  +----------+ ---


//                root    direction:vertical
//               /  |  \ 
//          line1 line2 line3 ... direction : horizontal
//         /  |  \
//    lane1 lane2 lane3 ...


// Typically, number of lanes is fixed beforehand at builder creation
// And lines are appended during table construction
// So lines number is intended to be far superior to lanes number


#[derive(Debug, PartialEq, Default)]
pub struct TableBuilder {
    pub layout: TreeContainer<LayoutElement>,
    pub direction: Direction,
    pub lanes: usize,
}

impl TableBuilder {
    pub fn new(direction: Direction, lanes: usize) -> Self {
        let elt = LayoutElement::new(direction);

        return TableBuilder {
            layout: TreeContainer::new().with_root(LayoutElement::new(direction)),
            lanes: lanes,
            direction: direction,
        };
    }

    // Re
    pub fn add_line(&mut self) -> Vec<NodeId>{
        let mut cells :Vec<NodeId> = vec![];

        let line = self.layout.push_child(
                self.layout.root_id().unwrap(), 
                LayoutElement::new(self.direction.orthogonal())
            ).unwrap();
        for _ in 0..self.lanes{
            let cell = self.layout.push_child(
                    line, 
                    LayoutElement::new(self.direction)
                ).unwrap();
            cells.push(cell);
        }
        return cells;
    } 

    pub fn add_lane(&mut self) -> Vec<NodeId>{
        let root = self.layout.root_id().unwrap();
        let mut cells :Vec<NodeId> = vec![];

        let mut line_iter = ChildrenIdWalk::new(root);
        while let Some(line_uid) = line_iter.next(&self.layout){
            let cell = self.layout.push_child(
                    line_uid, 
                    LayoutElement::new(self.direction)
                ).unwrap();
            cells.push(cell);
        }
        self.lanes += 1;
        return cells;
    }

    // Set dimensions of each LayoutElement based on the BoxConstraints
    pub fn construct(&mut self) {
        let root = self.layout.root_id().unwrap();
        let lines_count = self.layout.len(root);

        // Max dimensions for each line/lane read from constraints
        let mut max_lane = vec![0.0; self.lanes];

        let mut max_line = vec![0.0; lines_count];

        let mut line_index = 0;
        // 1st pass :
        // Read constraints to get max dimension of each lane and line
        // let mut line_iter = ChildrenIdWalk::new(self.layout.root_id());
        // while let Some(line_uid) = line_iter.next(&self.layout){
        for (line_uid, _) in self.layout.children_iter(root){
            let mut lane_index = 0;
            for (_, element) in self.layout.children_iter(line_uid){
                let constraint = &element.constraint;

                let width:f32 = constraint.pref_w.unwrap_or(0.0) 
                    + constraint.padding_x.unwrap_or(0.0);
                let height:f32 = constraint.pref_h.unwrap_or(0.0) 
                    + constraint.padding_y.unwrap_or(0.0);

                match self.direction{
                    Direction::Vertical =>{
                        if width > max_lane[lane_index]{
                            max_lane[lane_index] = width;
                        }
                        if height > max_line[line_index]{
                            max_line[line_index] = height;
                        }
                    }
                    Direction::Horizontal =>{
                        if height > max_lane[lane_index]{
                            max_lane[lane_index] = height;
                        }
                        if width > max_line[line_index]{
                            max_line[line_index] = width;
                        }
                    }
                }
                lane_index+=1;
            }
            line_index+=1;
        }


        
        // 2nd pass :
        // set dimension of each cell
        line_index = 0;
        let mut line_offset:f32 = 0.0;
        let mut lane_offset:f32 = 0.0;

        let mut line_iter = ChildrenIdWalk::new(root);
        while let Some(line_uid) = line_iter.next(&self.layout){
            lane_offset = 0.0;
            let mut lane_index = 0;

            let mut lane_iter = ChildrenIdWalk::new(line_uid);
            while let Some(elt_uid) = lane_iter.next(&self.layout){
                let element = self.layout.get_mut(elt_uid).unwrap();

                element.dimensions = match self.direction{
                    Direction::Vertical => BoxContainer{
                            x: lane_offset,
                            y: line_offset,
                            w: max_lane[lane_index],
                            h: max_line[line_index]
                        },
                    Direction::Horizontal => BoxContainer{
                            x: line_offset,
                            y: lane_offset,
                            w: max_line[line_index],
                            h: max_lane[lane_index]
                        }
                };

                lane_offset+=max_lane[lane_index];
                lane_index+=1;
            }

            // set the dimensions of the line container
            let line_elt = self.layout.get_mut(line_uid).unwrap();
            line_elt.dimensions = match self.direction{
                Direction::Vertical => BoxContainer{
                        x: 0.0,
                        y: line_offset,
                        w: lane_offset,
                        h: max_line[line_index]
                    },
                Direction::Horizontal => BoxContainer{
                        x: line_offset,
                        y: 0.0,
                        w: max_line[line_index],
                        h: lane_offset
                    }
            };

            line_offset+=max_line[line_index];         
            line_index+=1;
        }

        // set the dimensions of the root container
        let root_elt = self.layout.get_mut(root).unwrap();
        root_elt.dimensions = match self.direction{
            Direction::Vertical => BoxContainer{
                    x: 0.0,
                    y: 0.0,
                    w: lane_offset,
                    h: line_offset
                },
            Direction::Horizontal => BoxContainer{
                    x: 0.0,
                    y: 0.0,
                    w: line_offset,
                    h: lane_offset
                }
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;


    #[test]
    fn add_lines() {
        let mut builder = TableBuilder::new(Direction::Vertical,4);
        let line1_ids = builder.add_line();
        assert_eq!(line1_ids.len(), 4);
        assert_eq!(builder.layout.id_by_path("0:0:0"),Some(line1_ids[0]));
        assert_eq!(builder.layout.id_by_path("0:0:1"),Some(line1_ids[1]));
        assert_eq!(builder.layout.id_by_path("0:0:2"),Some(line1_ids[2]));
        assert_eq!(builder.layout.id_by_path("0:0:3"),Some(line1_ids[3]));

        let line2_ids = builder.add_line();
        assert_eq!(line2_ids.len(), 4);
        assert_eq!(builder.layout.id_by_path("0:1:0"),Some(line2_ids[0]));
        assert_eq!(builder.layout.id_by_path("0:1:1"),Some(line2_ids[1]));
        assert_eq!(builder.layout.id_by_path("0:1:2"),Some(line2_ids[2]));
        assert_eq!(builder.layout.id_by_path("0:1:3"),Some(line2_ids[3]));
    }

    #[test]
    fn add_lanes() {
        let mut builder = TableBuilder::new(Direction::Vertical,3);
        let line1_ids = builder.add_line();
        let line2_ids = builder.add_line();
        let line3_ids = builder.add_line();

        let new_lane = builder.add_lane();

        assert_eq!(builder.lanes, 4);
        assert_eq!(builder.layout.id_by_path("0:0:3"),Some(new_lane[0]));
        assert_eq!(builder.layout.id_by_path("0:1:3"),Some(new_lane[1]));
        assert_eq!(builder.layout.id_by_path("0:2:3"),Some(new_lane[2]));
    }

    #[test]
    fn add_lanes_lines() {
        let mut builder = TableBuilder::new(Direction::Vertical,3);
        let line1_ids = builder.add_line();
        let line2_ids = builder.add_line();
        let line3_ids = builder.add_line();

        let new_lane = builder.add_lane();
        let line4_ids = builder.add_line();
        assert_eq!(line4_ids.len(), 4);
        assert_eq!(builder.layout.id_by_path("0:3:0"),Some(line4_ids[0]));
        assert_eq!(builder.layout.id_by_path("0:3:1"),Some(line4_ids[1]));
        assert_eq!(builder.layout.id_by_path("0:3:2"),Some(line4_ids[2]));
        assert_eq!(builder.layout.id_by_path("0:3:3"),Some(line4_ids[3]));
    }

    #[test]
    fn constraints_vertical1() {
        let mut builder = TableBuilder::new(Direction::Vertical,2);
        let line0_ids = builder.add_line();
        let line1_ids = builder.add_line();

        {
            // top left cell
            let cell = builder.layout.get_mut(line0_ids[0]).unwrap();
            cell.constraint.pref_w = Some(10.0);
            cell.constraint.pref_h = Some(10.0);
        }
        {
            // bottom right cell
            let cell = builder.layout.get_mut(line1_ids[1]).unwrap();
            cell.constraint.pref_w = Some(20.0);
            cell.constraint.pref_h = Some(20.0);
        }

        builder.construct();

        // Verify the layout of individual cells
        {
            let cell = builder.layout.get(line0_ids[0]).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 10.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 10.0));
        }
        {
            let cell = builder.layout.get(line0_ids[1]).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 10.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 20.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 10.0));
        }
        {
            let cell = builder.layout.get(line1_ids[0]).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 10.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 10.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 20.0));
        }
        {
            let cell = builder.layout.get(line1_ids[1]).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 10.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 10.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 20.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 20.0));
        }



        // verify the lines layout
        {
            let cell = builder.layout.get(builder.layout.id_by_path("0:0").unwrap()).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 30.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 10.0));
        }
        {
            let cell = builder.layout.get(builder.layout.id_by_path("0:1").unwrap()).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 10.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 30.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 20.0));
        }

        // Verify the root layout
        {
            let cell = builder.layout.get(builder.layout.root_id().unwrap()).unwrap();
            assert!( approx_eq!(f32, cell.dimensions.x, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.y, 0.0) );
            assert!( approx_eq!(f32, cell.dimensions.w, 30.0));
            assert!( approx_eq!(f32, cell.dimensions.h, 30.0));
        }

    }

}