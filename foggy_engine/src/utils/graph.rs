// A graph structure allocated in the same contiguous memory region
// allocated memory can only grow, there is no freeing mechanism
use crate::utils::uid::*;
use std::collections::VecDeque;



#[derive(Debug, PartialEq, Default)]
struct DigraphNode<T> {
    pub data: T,
    pub nb_in:u32,
    pub nb_out:u32
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct DigraphEdge<W,U> {
    pub orig: U,
    pub dest: U,
    pub weight: W
}


#[derive(Debug, PartialEq)]
pub struct Digraph<T,W,U> {
    root: Option<U>,
    // Directed edges ordered by growing origin
    edges: Vec<DigraphEdge<W,U>>,
    backward_edges:Vec<DigraphEdge<W,U>>,
    nodes: UidStore<DigraphNode<T>, U>,
}



impl<T,W,U> Digraph<T,W,U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new() -> Self {
        Digraph {
            root: None,
            edges: Vec::new(),
            backward_edges: Vec::new(),
            nodes: UidStore::new(),
        }
    }

    fn _find_first_edge_inf(&self, id:U) -> usize{
        let mut i = self.edges.len() / 2;
        let mut step = i;
        let orig:u64 = id.into();
        // Find the first occurence of id in edges list by dichotomy
        loop{
            step = (step+1)/2;

            if i == self.edges.len(){
                return i;
            }
            if let Some(e) = self.edges.get(i){
                if orig > e.orig.into(){
                    if i==0{
                        // Special case for array of length 1
                        return 1;
                    }
                    i = i + step;
                }
                else{ // id <= e.orig
                    if i==0{
                        return 0;
                    }
                    // make sure that id > e-1.orig
                    if let Some(e2) = self.edges.get(i-1){
                        if orig > e2.orig.into(){
                            // Found it
                            return i;
                        }
                        else{
                            i = i - step;
                        }
                    }
                }
            }
        }
    }

    pub fn add_edge(&mut self, orig:U, dest:U, weight:W){
        // add edge in list, sorted by origin
        let idx = self._find_first_edge_inf(orig);

        self.edges.insert(idx, DigraphEdge{orig, dest, weight});

        let orig_node = self.nodes.get_mut(orig).unwrap();
        orig_node.nb_out += 1;
        let dest_node = self.nodes.get_mut(dest).unwrap();
        dest_node.nb_in += 1;
    }

    pub fn add_node(&mut self, id:U, data:T)->Result<U,&str>{
        return self.nodes.insert(id, 
            DigraphNode {
                data,
                nb_in:0,
                nb_out:0
            });
    }

    pub fn walk_from(&self, id:U) -> DigraphNodeOutWalk<U>{
        return DigraphNodeOutWalk::new(id);
    }
}





// Walk through adjacent output nodes 
pub struct DigraphNodeOutWalk<U>{
    node: U,
    index: Option<usize>,
}

impl<U> DigraphNodeOutWalk<U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(origin:U) -> Self {
        DigraphNodeOutWalk {
            node:origin,
            index: None,
        }
    }

    pub fn next<T,W> (&mut self, graph: &Digraph<T,W,U>) -> Option<(W,U)>
    where W:Copy {
        // if I'm already iterating through edges
        if let Some(id) = self.index{
            if id + 1 >= graph.edges.len(){
                //Finito
                return None;
            }
            let node:u64 = self.node.into();
            let edge = &graph.edges[id+1];
            if edge.orig.into() != node {
                return None;
            }
            self.index = Some(id+1);
            return Some((edge.weight, edge.dest));
        }
        // First call of iterator, find first edge
        else{
            // This function returns the first edge with an origin >= id
            // we have to check for the equality before accepting the result
            let index = graph._find_first_edge_inf(self.node);
            if index == graph.edges.len(){
                return None;
            }
            let node:u64 = self.node.into();
            let edge = &graph.edges[index];
            if edge.orig.into() != node {
                return None;
            }
            self.index = Some(index);
            return Some((edge.weight, edge.dest));
        }
    }
}

// Breadth-first buddy walk
// (I'm sure there is a name for that in the litterature)
// Explore the graph with a breadth-first method.
// For nodes with several input edges, don't explore output edges  
// until all input edges have been explored.
// In other words, wait for your buddies at the intersection before continuing to walk

// this works only if:
// - there is only one input node for the graph
// - there is no cyclic dependency


pub struct DigraphBuddyWalk<U>{
    current_node: U,
    current_walk: DigraphNodeOutWalk<U>,
    visit_count: UidStore<u32, U>,
    next_nodes: VecDeque<U>,
}

impl<U> DigraphBuddyWalk<U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(from:U) -> Self {
        DigraphBuddyWalk {
            current_node: from,
            current_walk: DigraphNodeOutWalk::new(from),
            visit_count: UidStore::new(),
            next_nodes: VecDeque::new(),
        }
    }

    // return a tuple (weight, origin, destination)
    pub fn next<T,W> (&mut self, graph: &Digraph<T,W,U>) -> Option<(W,U,U)>
    where W:Copy {
        loop{
            let next = self.current_walk.next(graph);
            match next{
                Some((w,dest)) =>{
                    let count:u32;
                    // Count the number of times we visited the destination
                    if let Some(c) = self.visit_count.get_mut(dest){
                        *c += 1;
                        count = *c;
                    }
                    else{
                        self.visit_count.insert(dest, 1).unwrap();
                        count =1;
                    }
                    // If we visited the destination through all its incoming edge
                    // Put it in the list of nodes to visit
                    if graph.nodes.get(dest).unwrap().nb_in == count{
                        self.next_nodes.push_back(dest);
                    }
                    return Some((w, self.current_node, dest));
                },
                None =>{
                    match self.next_nodes.pop_front() {
                        None => return None,
                        Some(id) => {
                            self.current_walk = DigraphNodeOutWalk::new(id);
                            self.current_node = id;
                            continue;
                        }
                    }
                } 
            }
        }
    }
}



// geometric constraint solver
// edges contain the constraints on minimum dimensions
#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct SolverEdge{
    length: u32,
}

pub struct SolverNode{
    pub min_val: Option<u32>,
    pub max_val: Option<u32>,
    //dist0: Option<u32>, // distance to previous edge X
    //dist1: Option<u32>, // distance to next edge X
}


pub struct SolverGraph<U>{
    graph: Digraph<SolverNode, SolverEdge, U>
}


// Iterate over solver nodes
pub struct SolverNodeIterator<'a, U> {
    graph: &'a SolverGraph<U>,
    iter: UidStoreIterator<'a, DigraphNode<SolverNode>, U>
}

impl<U> SolverGraph<U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new() -> Self {
        SolverGraph{
            graph: Digraph::new()
        }
    }

    pub fn add_node(&mut self, id:U){
        self.graph.add_node(id, SolverNode{ min_val: None, max_val: None});
    }

    pub fn add_edge(&mut self, orig:U, dest:U, length:u32){
        self.graph.add_edge(orig, dest, SolverEdge{length});
    }

    pub fn solve(&mut self, origin:U){
        let mut explored_edges:Vec<(u32,U,U)>=Vec::new();
        let mut walker:DigraphBuddyWalk<U> = DigraphBuddyWalk::new(origin);
        //Initialize origin
        let mut origin_node = self.graph.nodes.get_mut(origin).unwrap();
        origin_node.data.min_val = Some(0);
        origin_node.data.max_val = Some(0);

        while let Some((e, orig, dest)) = walker.next(&self.graph){
            let orig_node = self.graph.nodes.get(orig).unwrap();
            let val:u32 = orig_node.data.min_val.unwrap() + e.length;

            let mut dest_node = self.graph.nodes.get_mut(dest).unwrap();        
            dest_node.data.min_val = match dest_node.data.min_val{
                None => Some(val),
                Some(min) => Some(min.max(val))
            };

            // Keep a stack of explored edges to rewind after
            explored_edges.push((e.length, orig, dest));
        }


        // rewind the graph and fill the max values
        while let Some((length, orig, dest)) = explored_edges.pop(){
            let dest_node = self.graph.nodes.get(dest).unwrap();
            let dest_value = match dest_node.data.max_val {
                //nodes at the end of the graph don't have a max value, use the min value then
                None => dest_node.data.min_val.unwrap(), 
                Some(val) => val
            };

            let val:u32 = dest_value - length;
            let mut orig_node= self.graph.nodes.get_mut(orig).unwrap();
            orig_node.data.max_val = match orig_node.data.max_val{
                None => Some(val),
                Some(max) => Some(max.min(val))
            };
        }
    }

    pub fn get_solution(&self, id:U)->Option<u32>{
        let node = self.graph.nodes.get(id)?;
        return match node.data.max_val{
            None => node.data.min_val,
            Some(max) => Some((node.data.min_val? + max) / 2)
        };
    }

    pub fn nodes_iter(&self) -> SolverNodeIterator<U>{
        SolverNodeIterator::new(self)
    }
}



impl <'a, U> SolverNodeIterator<'a, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(graph :&'a SolverGraph<U>) -> Self {
        SolverNodeIterator{
            graph,
            iter: graph.graph.nodes.iter()
        }
    }
}

impl <'a, U> Iterator for SolverNodeIterator<'a, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    type Item = (U, &'a SolverNode);

    fn next(&mut self) -> Option<(U, &'a SolverNode)> {
        let (id, node) = self.iter.next()?;
        Some((id, &node.data))
    }    
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_basic() {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id = gen.get();

    }


    #[test]
    fn test_edges_ordering1() {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();

        // Build this graph :
        // id0 --> id1 --> id2 --> id3 
        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();

        graph.add_edge(id0, id1, 1);
        graph.add_edge(id2, id3, 1);
        graph.add_edge(id1, id2, 1);

        // Check that edges are sorted in the right order
        assert_eq!(graph.edges[0].orig, id0);
        assert_eq!(graph.edges[1].orig, id1);
        assert_eq!(graph.edges[2].orig, id2);
    }




    #[test]
    fn test_edges_ordering2 () {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 --> id1 --> id2 --> id3 
        //          \->id4-^
        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();
        graph.add_node(id4, 0).unwrap();


        graph.add_edge(id0, id1, 1);
        graph.add_edge(id2, id3, 1);
        graph.add_edge(id1, id2, 1);
        graph.add_edge(id1, id4, 1);
        graph.add_edge(id4, id2, 1);

        // Check that edges are sorted in the right order
        assert_eq!(graph.edges[0].orig, id0);
        assert_eq!(graph.edges[1].orig, id1);
        assert_eq!(graph.edges[2].orig, id1);
        assert_eq!(graph.edges[3].orig, id2);
        assert_eq!(graph.edges[4].orig, id4);
    }


    #[test]
    fn test_edges_ordering3 () {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 --> id1 --> id2 
        //   \->id4-^       ^
        //    \---> id3 ---/

        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();
        graph.add_node(id4, 0).unwrap();


        graph.add_edge(id0, id1, 1);
        graph.add_edge(id3, id2, 1);
        graph.add_edge(id0, id3, 1);
        graph.add_edge(id1, id2, 1);
        graph.add_edge(id4, id1, 1);
        graph.add_edge(id0, id4, 1);

        // Check that edges are sorted in the right order
        assert_eq!(graph.edges[0].orig, id0);
        assert_eq!(graph.edges[1].orig, id0);
        assert_eq!(graph.edges[2].orig, id0);
        assert_eq!(graph.edges[3].orig, id1);
        assert_eq!(graph.edges[4].orig, id3);
        assert_eq!(graph.edges[5].orig, id4);

        assert_eq!(graph.nodes.get(id0).unwrap().nb_in, 0);
        assert_eq!(graph.nodes.get(id0).unwrap().nb_out, 3);
        assert_eq!(graph.nodes.get(id1).unwrap().nb_in, 2);
        assert_eq!(graph.nodes.get(id2).unwrap().nb_in, 2);
    }

    #[test]
    fn test_adjacent_walk() {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();
        let id5 = gen.get();

        // Build this graph :
        // id0 --> id1 ----> id2 --> id5
        //           \->id4  -^
        //            \> id3-/

        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();
        graph.add_node(id4, 0).unwrap();
        graph.add_node(id5, 0).unwrap();


        graph.add_edge(id0, id1, 1);
        graph.add_edge(id1, id2, 1);
        graph.add_edge(id2, id5, 1);
        graph.add_edge(id1, id4, 1);
        graph.add_edge(id1, id3, 1);
        graph.add_edge(id4, id2, 1);
        graph.add_edge(id3, id2, 1);

        let mut walk1 = graph.walk_from(id0);
        assert_eq!(walk1.next(&graph), Some((1, id1)));
        assert_eq!(walk1.next(&graph), None);

        let mut walk2 = graph.walk_from(id5);
        assert_eq!(walk2.next(&graph), None);

        let mut walk3 = graph.walk_from(id1);
        assert_eq!(walk3.next(&graph), Some((1, id3)));
        assert_eq!(walk3.next(&graph), Some((1, id4)));
        assert_eq!(walk3.next(&graph), Some((1, id2)));
        assert_eq!(walk3.next(&graph), None);
    }


    #[test]
    fn test_buddy_walk() {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut graph: Digraph<u32, u32, Guid> = Digraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();
        let id5 = gen.get();
        let id6 = gen.get();

        // Build this graph :
        // id0 --> id1 ---> id2 --> id5 -->id6
        //  \        \---> id3  ----^
        //   \-------------id4-----/

        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();
        graph.add_node(id4, 0).unwrap();
        graph.add_node(id5, 0).unwrap();
        graph.add_node(id6, 0).unwrap();


        graph.add_edge(id0, id1, 1);
        graph.add_edge(id1, id2, 1);
        graph.add_edge(id2, id5, 1);
        graph.add_edge(id5, id6, 1);
        graph.add_edge(id1, id3, 1);
        graph.add_edge(id3, id5, 1);
        graph.add_edge(id0, id4, 1);
        graph.add_edge(id4, id5, 1);

        let mut walk = DigraphBuddyWalk::new(id0);
        assert_eq!(walk.next(&graph), Some((1,id0, id4)));
        assert_eq!(walk.next(&graph), Some((1,id0, id1)));
        assert_eq!(walk.next(&graph), Some((1,id4, id5)));
        assert_eq!(walk.next(&graph), Some((1,id1, id3)));
        assert_eq!(walk.next(&graph), Some((1,id1, id2)));
        assert_eq!(walk.next(&graph), Some((1,id3, id5)));
        assert_eq!(walk.next(&graph), Some((1,id2, id5)));
        assert_eq!(walk.next(&graph), Some((1,id5, id6)));

    }


    #[test]
    fn test_solver() {
        let mut gen : GuidManager<Guid> = GuidManager::new();
        let mut solver: SolverGraph<Guid> = SolverGraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 -> id1 -> id2 -> id4
        //   \----> id3  ------^
        
        solver.add_node(id0);
        solver.add_node(id1);
        solver.add_node(id2);
        solver.add_node(id3);
        solver.add_node(id4);

        solver.add_edge(id0, id1, 2);
        solver.add_edge(id1, id2, 2);
        solver.add_edge(id2, id4, 2);
        solver.add_edge(id0, id3, 12);
        solver.add_edge(id3, id4, 4);

        solver.solve(id0);
        assert_eq!(solver.get_solution(id0), Some(0));
        assert_eq!(solver.get_solution(id4), Some(16));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.min_val, Some(2));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.max_val, Some(12));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.min_val, Some(2));
        assert_eq!(solver.get_solution(id1), Some(7));
        assert_eq!(solver.get_solution(id2), Some(9));
    }

}
