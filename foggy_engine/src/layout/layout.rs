use crate::utils::uid::*;
use crate::utils::tree::*;
use crate::utils::graph::*;
use crate::layout::*;
use std::collections::HashMap;



// Tables are part of the Layout tree woth special types for layout.type:
//     TableRoot, TableLine, and TableLane,
// structure where cells are aligned like in a table
// One dimension is fixed in the ctx associated to the root : the number of columns
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

// A special constraint is added during the layout construction 
// To make sure all cells are aligned in both directions





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


impl BoxConstraints {
    pub fn new() -> Self {
        Default::default()
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


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TableRootCtx{
    // Number of lanes in the table
    // nb: the number of lines is simply the number of children
    pub lanes: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TableLaneCtx{
    // A reference to the table root box
    pub root_id: LayoutGuid,
}

// The specific types of layout box
// For some layout construction we may need to store some additional context
// Keep in mind additional context will impact the seize of all objects
// So keep the context small
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LayoutBoxType {
    Basic,
    TableRoot(TableRootCtx),
    TableLine,
    TableLane(TableLaneCtx),
}
 

// A layoutbox dimensions are determined by 2 points:
//  - topright
//  - bottomleft
// These 2 points are referenced by their ids
// and their actual value is stored in the layout/in the solver
//
//    topleft
//       X------------+
//       |            |
//       |            |
//       |            |
//       |            |
//       +------------X
//                  bottomright




#[derive(Debug, PartialEq)]
pub struct LayoutBox {
    pub constraint: BoxConstraints,
    pub top_left: DimensionGuid,
    pub bottom_right: DimensionGuid,
    pub direction: Direction,
    pub t: LayoutBoxType,
}



impl LayoutBox {
    pub fn new(direction: Direction, top_left_id:DimensionGuid, bottom_right_id:DimensionGuid) -> Self {
        LayoutBox {
            constraint: BoxConstraints::new(),
            top_left:top_left_id,
            bottom_right:bottom_right_id,
            direction,
            t: LayoutBoxType::Basic,
        }
    }

    pub fn with_type(mut self, the_type: LayoutBoxType) -> Self{
        self.t = the_type;
        self
    }

}



// The root container of all the diagram physical layout.
// Owns all the layout boxes and the id generator
struct Layout {
    pub tree: TreeContainer<LayoutBox, LayoutGuid>,
    // ID generator for all the layout boxes
    pub layout_id: GuidManager<LayoutGuid>,
    // ID generator for the computable dimensions
    pub dimension_id: GuidManager<DimensionGuid>,
    box_x : UidStore<u32,DimensionGuid>,
    box_y : UidStore<u32,DimensionGuid>,
    pub direction: Direction,
}


impl Layout {
    pub fn new(direction: Direction) -> Self {
        let mut layout = Layout {
            tree: TreeContainer::new(),
            layout_id: GuidManager::new(),
            dimension_id: GuidManager::new(),
            box_x : UidStore::new(),
            box_y : UidStore::new(),
            direction,
        };
        layout.tree.add_root(
            LayoutBox::new(direction, 
                layout.dimension_id.get(),
                layout.dimension_id.get()),
                layout.layout_id.get()).unwrap();
        return layout;
    }

    pub fn get_root(&self)->LayoutGuid{
        return self.tree.root_id().unwrap();
    }

    // Table stuff
    pub fn create_table(&mut self, root:LayoutGuid, direction: Direction, lanes: usize){
        if lanes ==0{
            panic!();
        }
        if let Some(l) = self.tree.get_mut(root){
            if l.t != LayoutBoxType::Basic{
                panic!();
            }
            l.t = LayoutBoxType::TableRoot(TableRootCtx {lanes});
        }
    }

    pub fn add_line(&mut self, root:LayoutGuid) -> Vec<LayoutGuid>{
        // Get the table ctx
        let root_box = self.tree.get(root).unwrap();

        let lanes;
        if let LayoutBoxType::TableRoot(ctx) = &root_box.t {
            lanes = ctx.lanes;    
        }
        else{
            panic!();
        }

        let mut cells :Vec<LayoutGuid> = vec![];

        let line = self.tree.push_child(
            root, 
            LayoutBox::new(self.direction.orthogonal(),
                    self.dimension_id.get(),
                    self.dimension_id.get()
                ).with_type(
                    LayoutBoxType::TableLine),
                    self.layout_id.get()
                ).unwrap();
        for _ in 0..lanes{
            let cell = self.tree.push_child(
                    line, 
                    LayoutBox::new(self.direction, self.dimension_id.get(),self.dimension_id.get())
                        .with_type(LayoutBoxType::TableLane(TableLaneCtx{root_id:root})),
                    self.layout_id.get()
                ).unwrap();
            cells.push(cell);
        }
        return cells;
    }

    pub fn add_lane(&mut self, root:LayoutGuid) -> Vec<LayoutGuid>{
        // Get the table ctx
        let root_box = self.tree.get(root).unwrap();

        let lanes;
        if let LayoutBoxType::TableRoot(ctx) = &root_box.t {
            lanes = ctx.lanes;
        }
        else{
            panic!();
        }
        
        let mut cells :Vec<LayoutGuid> = vec![];

        let mut line_iter = ChildrenIdWalk::new(root);
        while let Some(line_uid) = line_iter.next(&self.tree){
            let cell = self.tree.push_child(
                    line_uid, 
                    LayoutBox::new(self.direction, self.dimension_id.get(),self.dimension_id.get())
                        .with_type(LayoutBoxType::TableLane(TableLaneCtx{root_id:root})),
                    self.layout_id.get()
                ).unwrap();
            cells.push(cell);
        }

        let root_mut = self.tree.get_mut(root).unwrap();
        root_mut.t = LayoutBoxType::TableRoot(TableRootCtx {lanes:lanes+1});
        return cells;
    }

    pub fn box_dimensions(&self, box_id:LayoutGuid) -> Result<(u32, u32, u32, u32), &str>{
        if let Some(b) = self.tree.get(box_id){
            Ok((*self.box_x.get(b.top_left).unwrap(),
                *self.box_y.get(b.top_left).unwrap(),
                *self.box_x.get(b.bottom_right).unwrap(),
                *self.box_y.get(b.bottom_right).unwrap()))
        }
        else{
            Err("Invalid box id")
        }
    }

    pub fn solve(&mut self) -> Result<u32, &str>{
        self.box_x.clear();
        self.box_y.clear();

        let mut solver_x : SolverGraph<DimensionGuid> = SolverGraph::new();
        let mut solver_y : SolverGraph<DimensionGuid> = SolverGraph::new();

        // a list of virtual box dimensions to align 
        let mut hash:HashMap<LayoutGuid, Vec<DimensionGuid>> = HashMap::new();

        //First add all nodes to solver graph
        for (id, b) in self.tree.depth_iter(){
            // Add top_left and bottom_right positions to the solver
            solver_x.add_node(b.top_left).unwrap();
            solver_x.add_node(b.bottom_right).unwrap();
            solver_y.add_node(b.top_left).unwrap();
            solver_y.add_node(b.bottom_right).unwrap();

            solver_x.add_edge(b.top_left, b.bottom_right, 
                b.constraint.pref_w.unwrap_or(0)).unwrap();
            
            solver_y.add_edge(b.top_left, b.bottom_right, 
                b.constraint.pref_h.unwrap_or(0)).unwrap();

            // For tables, add lane constraint nodes to the solver
            if let LayoutBoxType::TableRoot(ctx) = b.t{
                let nb_nodes = ctx.lanes-1;
                let mut points:Vec<DimensionGuid> = Vec::new();
                for _ in 0..nb_nodes{
                    let dim_id = self.dimension_id.get();
                    points.push(dim_id);
                    match b.direction {
                        Direction::Vertical => solver_x.add_node(dim_id).unwrap(),
                        Direction::Horizontal => solver_y.add_node(dim_id).unwrap()
                    };
                }
                hash.insert(id, points);
            }
        }

        // Then constraint edges
        let mut walk: TreeDepthIdWalk<LayoutGuid> = TreeDepthIdWalk::new();
        while let Some(id) = walk.next(&self.tree) {
            let b = self.tree.get(id).unwrap();
            let direction: Direction;
            let p_topleft:DimensionGuid;
            let p_bottomright:DimensionGuid;


            if let Some((_, p)) = self.tree.parent(id) {
                direction = p.direction;
                p_topleft = p.top_left;
                p_bottomright = p.bottom_right;
            }
            else{
                //root
                continue;
            }

            match direction {
                Direction::Vertical =>{
                    solver_x.add_edge(p_topleft, b.top_left, 0).unwrap();
                    solver_x.add_edge(b.bottom_right, p_bottomright, 0).unwrap();

                    // Table layout constraints are specific
                    if let LayoutBoxType::TableLane(ctx) = b.t {
                        let lane_idx = walk.path().last().unwrap();
                        let limits = hash.get(&ctx.root_id).unwrap();

                        if *lane_idx > 0 {
                            //left constraint to table lane
                            let left_id =  limits[*lane_idx-1];
                            solver_y.add_edge(left_id, b.top_left, 0).unwrap();
                        }
                        else {
                            //left constraint to parent
                            solver_y.add_edge(p_topleft, b.top_left, 0).unwrap();
                        }
                        if *lane_idx < limits.len() {
                            //right constraint to table line
                            let right_id =  limits[*lane_idx];
                            solver_y.add_edge(b.bottom_right, right_id, 0).unwrap();
                        }                            
                        else {
                            //last  lane, add right constraint to parent
                            solver_y.add_edge(b.bottom_right, p_bottomright, 0).unwrap();
                        }                       
                    }
                    else { // generic box constraints
                        if let Some((_, left)) = self.tree.left_sibling(id){
                            // Add  constraint to previous sibling
                            solver_y.add_edge(left.bottom_right, b.top_left, 0).unwrap();
                        }
                        else{
                            // add constraint to parent
                            solver_y.add_edge(p_topleft, b.top_left, 0).unwrap();
                        }

                        if let Some((_, right)) = self.tree.right_sibling(id){
                            // Add  constraint to next sibling
                            solver_y.add_edge(b.bottom_right, right.top_left, 0).unwrap();
                        }
                        else{
                            // add constraint to parent
                            solver_y.add_edge(b.bottom_right, p_bottomright, 0).unwrap();
                        }  
                    }              
                },
                Direction::Horizontal =>{
                    solver_y.add_edge(p_topleft, b.top_left, 0).unwrap();
                    solver_y.add_edge(b.bottom_right, p_bottomright, 0).unwrap();

                    // Table layout constraints are specific
                    if let LayoutBoxType::TableLane(ctx) = b.t {
                        let lane_idx = walk.path().last().unwrap();
                        let limits = hash.get(&ctx.root_id).unwrap();

                        if *lane_idx > 0 {
                            //left constraint to table lane
                            let left_id =  limits[*lane_idx-1];
                            solver_x.add_edge(left_id, b.top_left, 0).unwrap();
                        }
                        else {
                            //left constraint to parent
                            solver_x.add_edge(p_topleft, b.top_left, 0).unwrap();
                        }
                        if *lane_idx < limits.len() {
                            //right constraint to table line
                            let right_id =  limits[*lane_idx];
                            solver_x.add_edge(b.bottom_right, right_id, 0).unwrap();
                        }                            
                        else {
                            //last  lane, add right constraint to parent
                            solver_x.add_edge(b.bottom_right, p_bottomright, 0).unwrap();
                        }
                    }
                    else{ // generic box constraints
                        if let Some((_, left)) = self.tree.left_sibling(id){
                            // Add  constraint to previous sibling
                            solver_x.add_edge(left.bottom_right, b.top_left, 0).unwrap();
                        }
                        else{
                            // add constraint to parent
                            solver_x.add_edge(p_topleft, b.top_left, 0).unwrap();
                        }

                        if let Some((_, right)) = self.tree.right_sibling(id){
                            // Add  constraint to next sibling
                            solver_x.add_edge(b.bottom_right, right.top_left, 0).unwrap();
                        }
                        else{
                            // add constraint to parent
                            solver_x.add_edge(b.bottom_right, p_bottomright, 0).unwrap();
                        }
                    }
                }
            }
        }

        //println!(" Boxes x : {:?}", self.tree);
        //println!(" Solver x : {:?}", solver_x);
        //println!(" Solver y : {:?}", solver_y);

        let topleft = self.tree.get(self.get_root()).unwrap().top_left;
        solver_x.solve(topleft);
        solver_y.solve(topleft);



        for (id, n) in solver_x.nodes_iter(){
            if let Some(solution) = n.min_val{
                self.box_x.insert(id, solution).unwrap();
            }
            else{ 
                return Err("Some x dimensions were not solved");
            }
        }
        for (id, n) in solver_y.nodes_iter(){
            if let Some(solution) = n.min_val{
                self.box_y.insert(id, solution).unwrap();
            }
            else{ 
                return Err("Some y dimensions were not solved");
            }
        }


        //Release the table dimsnsions constraints
        for (_, table) in hash.iter() {
            for id in table.iter(){
                self.dimension_id.drop(*id);
            }
        }
        Ok(42)
    }
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn layout_solve1() {
        let mut layout = Layout::new(Direction::Vertical);

        let cell1 = layout.tree.push_child(
                    layout.get_root(), 
                    LayoutBox::new(Direction::Horizontal, layout.dimension_id.get(),layout.dimension_id.get()),
                    layout.layout_id.get()
                ).unwrap();

        let c1 = layout.tree.get_mut(cell1).unwrap();
        c1.constraint.pref_w = Some(10);
        c1.constraint.pref_h = Some(10);
        layout.solve().unwrap();

        assert_eq!(layout.box_dimensions(layout.get_root()), Ok((0,0,10,10)));
    }


    #[test]
    fn layout_solve2() {
        let mut layout = Layout::new(Direction::Vertical);

        let cell1 = layout.tree.push_child(
                    layout.get_root(), 
                    LayoutBox::new(Direction::Horizontal, layout.dimension_id.get(),layout.dimension_id.get()),
                    layout.layout_id.get()
                ).unwrap();
        let cell2 = layout.tree.push_child(
                    layout.get_root(), 
                    LayoutBox::new(Direction::Horizontal, layout.dimension_id.get(),layout.dimension_id.get()),
                    layout.layout_id.get()
                ).unwrap();
        let cell21 = layout.tree.push_child(
                    cell2, 
                    LayoutBox::new(Direction::Vertical, layout.dimension_id.get(),layout.dimension_id.get()),
                    layout.layout_id.get()
                ).unwrap();
        let cell22 = layout.tree.push_child(
                    cell2, 
                    LayoutBox::new(Direction::Vertical, layout.dimension_id.get(),layout.dimension_id.get()),
                    layout.layout_id.get()
                ).unwrap();
        {
                let c1 = layout.tree.get_mut(cell1).unwrap();
                c1.constraint.pref_w = Some(10);
                c1.constraint.pref_h = Some(10);
                let c21 = layout.tree.get_mut(cell21).unwrap();
                c21.constraint.pref_w = Some(5);
                c21.constraint.pref_h = Some(5);
                let c22 = layout.tree.get_mut(cell22).unwrap();
                c22.constraint.pref_w = Some(7);
                c22.constraint.pref_h = Some(7);
        }
        layout.solve().unwrap();

        assert_eq!(layout.box_dimensions(layout.get_root()), Ok((0,0,12,17)));
    }

    #[test]
    fn table_build() {
        let mut layout = Layout::new(Direction::Vertical);
        layout.create_table(layout.get_root(), Direction::Vertical, 3);

        let root = layout.tree.get(layout.get_root()).unwrap();
        assert_eq!(root.t, LayoutBoxType::TableRoot(TableRootCtx{lanes:3}));
    }    

    #[test]
    fn table_add_lines() {
        let mut layout = Layout::new(Direction::Vertical);
        layout.create_table(layout.get_root(), Direction::Vertical, 4);
        let line1_ids = layout.add_line(layout.get_root());
        assert_eq!(line1_ids.len(), 4);
        assert_eq!(layout.tree.id_by_path_str("0:0:0"),Some(line1_ids[0]));
        assert_eq!(layout.tree.id_by_path_str("0:0:1"),Some(line1_ids[1]));
        assert_eq!(layout.tree.id_by_path_str("0:0:2"),Some(line1_ids[2]));
        assert_eq!(layout.tree.id_by_path_str("0:0:3"),Some(line1_ids[3]));

        let line2_ids = layout.add_line(layout.get_root());
        assert_eq!(line2_ids.len(), 4);
        assert_eq!(layout.tree.id_by_path_str("0:1:0"),Some(line2_ids[0]));
        assert_eq!(layout.tree.id_by_path_str("0:1:1"),Some(line2_ids[1]));
        assert_eq!(layout.tree.id_by_path_str("0:1:2"),Some(line2_ids[2]));
        assert_eq!(layout.tree.id_by_path_str("0:1:3"),Some(line2_ids[3]));
    }

    #[test]
    fn table_add_lanes() {
        let mut layout = Layout::new(Direction::Vertical);
        layout.create_table(layout.get_root(), Direction::Vertical, 3);

        layout.add_line(layout.get_root());
        layout.add_line(layout.get_root());
        layout.add_line(layout.get_root());

        let new_lane = layout.add_lane(layout.get_root());

        let root = layout.tree.get(layout.get_root()).unwrap();
        assert_eq!(root.t, LayoutBoxType::TableRoot(TableRootCtx{lanes:4}));
        assert_eq!(layout.tree.id_by_path_str("0:0:3"),Some(new_lane[0]));
        assert_eq!(layout.tree.id_by_path_str("0:1:3"),Some(new_lane[1]));
        assert_eq!(layout.tree.id_by_path_str("0:2:3"),Some(new_lane[2]));
    }

    #[test]
    fn table_add_lanes_lines() {
        let mut layout = Layout::new(Direction::Vertical);
        layout.create_table(layout.get_root(), Direction::Vertical, 3);

        layout.add_line(layout.get_root());
        layout.add_line(layout.get_root());
        layout.add_line(layout.get_root());

        layout.add_lane(layout.get_root());
        let line4_ids = layout.add_line(layout.get_root());
        let root = layout.tree.get(layout.get_root()).unwrap();
        assert_eq!(root.t, LayoutBoxType::TableRoot(TableRootCtx{lanes:4}));
        assert_eq!(layout.tree.id_by_path_str("0:3:0"),Some(line4_ids[0]));
        assert_eq!(layout.tree.id_by_path_str("0:3:1"),Some(line4_ids[1]));
        assert_eq!(layout.tree.id_by_path_str("0:3:2"),Some(line4_ids[2]));
        assert_eq!(layout.tree.id_by_path_str("0:3:3"),Some(line4_ids[3]));
    }

    #[test]
    fn table_constraints_vertical1() {
        let mut layout = Layout::new(Direction::Vertical);
        layout.create_table(layout.get_root(), Direction::Vertical, 2);

        let line0_ids = layout.add_line(layout.get_root());
        let line1_ids = layout.add_line(layout.get_root());

        {
            // top left cell
            let cell = layout.tree.get_mut(line0_ids[0]).unwrap();
            cell.constraint.pref_w = Some(10);
            cell.constraint.pref_h = Some(10);
        }
        {
            // bottom right cell
            let cell = layout.tree.get_mut(line1_ids[1]).unwrap();
            cell.constraint.pref_w = Some(20);
            cell.constraint.pref_h = Some(20);
        }

        layout.solve().unwrap();

        // Verify the layout of individual cells
        {
            assert_eq!(layout.box_dimensions(line0_ids[0]), Ok((0, 0, 10, 10)));
        }
        {
            //Only assumptions about the position, not the size because we didn't put any constraint on size
            let dimensions= layout.box_dimensions(line0_ids[1]).unwrap();
            assert_eq!(dimensions.0, 10);
            assert_eq!(dimensions.1, 0);   
        }
        {
            //Only assumptions about the position, not the size because we didn't put any constraint on size
            let dimensions= layout.box_dimensions(line1_ids[0]).unwrap();
            assert_eq!(dimensions.0, 0);
            assert_eq!(dimensions.1, 10);
        }
        {
            assert_eq!(layout.box_dimensions(line1_ids[1]), Ok((10, 10, 30, 30)));
        }

        // Verify the root dimensions
        assert_eq!(layout.box_dimensions(layout.tree.root_id().unwrap()), Ok((0, 0, 30, 30)));

    }

}
