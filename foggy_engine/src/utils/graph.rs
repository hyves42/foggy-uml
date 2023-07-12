// A graph structure allocated in the same contiguous memory region
// allocated memory can only grow, there is no freeing mechanism
use crate::utils::uid::*;
use core::fmt::Debug;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Default)]
struct DigraphNode<T> {
    pub data: T,
    pub nb_in: u32,
    pub nb_out: u32,
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct DigraphEdge<W, U> {
    pub orig: U,
    pub dest: U,
    pub weight: W,
}

#[derive(Debug, PartialEq)]
pub struct Digraph<T, W, U> {
    // Directed edges ordered by growing origin
    edges: Vec<DigraphEdge<W, U>>,
    backward_edges: Vec<DigraphEdge<W, U>>,
    nodes: UidStore<DigraphNode<T>, U>,
}

impl<T, W, U> Digraph<T, W, U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Eq,
    U: Copy,
    U: std::fmt::Debug,
    W: std::fmt::Debug,
{
    pub fn new() -> Self {
        Digraph {
            edges: Vec::new(),
            backward_edges: Vec::new(),
            nodes: UidStore::new(),
        }
    }

    fn _find_first_edge_inf(&self, id: U) -> usize {
        let mut i = self.edges.len() / 2;
        let mut step = i;
        let orig: u64 = id.into();
        // Find the first occurence of id in edges list by dichotomy
        loop {
            step = (step / 2).max(1);

            if i == self.edges.len() {
                return i;
            }
            if let Some(e) = self.edges.get(i) {
                //println!(" i = {:?}, id = {:?}, id[i] = {:?}, step={:?}", i, orig, e.orig, step);
                if orig > e.orig.into() {
                    if i == 0 {
                        // Special case for array of length 1
                        return 1;
                    }
                    i = i + step;
                } else {
                    // id <= e.orig
                    if i == 0 {
                        return 0;
                    }
                    // make sure that id > e-1.orig
                    if let Some(e2) = self.edges.get(i - 1) {
                        //println!(" i = {:?}, id[i-1] = {:?}, step={:?}", i, e2.orig, step);
                        if orig > e2.orig.into() {
                            // Found it
                            return i;
                        } else {
                            i = i - step;
                        }
                    }
                }
            }
        }
    }

    fn _find_first_edge(&self, id: U) -> Option<usize> {
        let index = self._find_first_edge_inf(id);
        if index == self.edges.len() {
            return None;
        }
        let edge = &self.edges[index];
        if edge.orig != id {
            return None;
        }
        Some(index)
    }

    pub fn add_edge(&mut self, orig: U, dest: U, weight: W) -> Result<(), String> {
        if self.nodes.get(orig).is_none() {
            return Err(format!("origin {:?} doesn't exist.", orig));
        }
        if self.nodes.get(dest).is_none() {
            return Err(format!("destination {:?} doesn't exist.", dest));
        }
        //println!("Edges before : {:?} (self={:p})", self.edges, &self);
        //println!(" Try to insert orig {:?} to {:?}", orig, dest);
        // add edge in list, sorted by origin
        let idx = self._find_first_edge_inf(orig);

        self.edges.insert(idx, DigraphEdge { orig, dest, weight });
        //println!(" Insert at idx {:?}", idx);
        //println!(" Edges now : {:?}", self.edges);

        let orig_node = self.nodes.get_mut(orig).unwrap();
        orig_node.nb_out += 1;
        let dest_node = self.nodes.get_mut(dest).unwrap();
        dest_node.nb_in += 1;

        Ok(())
    }

    pub fn add_node(&mut self, id: U, data: T) -> Result<U, String> {
        return self.nodes.insert(
            id,
            DigraphNode {
                data,
                nb_in: 0,
                nb_out: 0,
            },
        );
    }

    pub fn get_edge(&self, orig: U, dest: U) -> Option<&W> {
        let mut i = self._find_first_edge(orig)?;

        while let Some(edge) = self.edges.get(i) {
            if edge.dest == dest {
                return Some(&edge.weight);
            }
            i += 1;
        }
        return None;
    }

    pub fn get_node(&self, id: U) -> Option<&T> {
        Some(&self.nodes.get(id)?.data)
    }

    pub fn walk_from(&self, id: U) -> DigraphNodeOutWalk<U> {
        return DigraphNodeOutWalk::new(id);
    }
}

// A graph is simply a digraph with double the amout of edges
#[derive(Debug, PartialEq)]
pub struct Graph<T, W, U> {
    digraph: Digraph<T, W, U>,
}

impl<T, W, U> Graph<T, W, U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Eq,
    U: Copy,
    U: std::fmt::Debug,
    W: std::fmt::Debug,
    W: Copy,
{
    pub fn new() -> Self {
        Graph {
            digraph: Digraph::new(),
        }
    }

    pub fn add_edge(&mut self, orig: U, dest: U, weight: W) -> Result<(), String> {
        self.digraph.add_edge(orig, dest, weight)?;
        self.digraph.add_edge(dest, orig, weight)?;
        Ok(())
    }

    pub fn add_node(&mut self, id: U, data: T) -> Result<U, String> {
        return self.digraph.add_node(id, data);
    }

    pub fn get_edge(&self, orig: U, dest: U) -> Option<&W> {
        let edge1 = self.digraph.get_edge(orig, dest)?;
        let _edge2 = self.digraph.get_edge(dest, orig)?;

        return Some(edge1);
    }

    pub fn get_node(&self, id: U) -> Option<&T> {
        self.digraph.get_node(id)
    }

    pub fn walk_from(&self, id: U) -> DigraphNodeOutWalk<U> {
        return self.digraph.walk_from(id);
    }
}

/////////////////////////////////
// Walkers

// Walk through adjacent output nodes
#[derive(Debug, PartialEq)]
pub struct DigraphNodeOutWalk<U> {
    node: U,
    index: Option<usize>,
}

impl<U> DigraphNodeOutWalk<U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
{
    pub fn new(origin: U) -> Self {
        DigraphNodeOutWalk {
            node: origin,
            index: None,
        }
    }

    pub fn next<T, W>(&mut self, graph: &Digraph<T, W, U>) -> Option<(W, U)>
    where
        W: Copy,
        U: std::fmt::Debug,
        U: Eq,
        W: std::fmt::Debug,
    {
        // if I'm already iterating through edges
        if let Some(id) = self.index {
            if id + 1 >= graph.edges.len() {
                //Finito
                return None;
            }
            let edge = &graph.edges[id + 1];
            if edge.orig != self.node {
                return None;
            }
            self.index = Some(id + 1);
            return Some((edge.weight, edge.dest));
        }
        // First call of iterator, find first edge
        else {
            match graph._find_first_edge(self.node) {
                None => return None,
                Some(index) => {
                    self.index = Some(index);
                    let edge = &graph.edges[index];
                    return Some((edge.weight, edge.dest));
                }
            }
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
// - there is no cyclic dependency (ie it doesn't work with a Graph)

#[derive(Debug, PartialEq)]
pub struct DigraphBuddyWalk<U> {
    current_node: U,
    current_walk: DigraphNodeOutWalk<U>,
    visit_count: UidStore<u32, U>,
    next_nodes: VecDeque<U>,
}

impl<U> DigraphBuddyWalk<U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
    U: Debug,
{
    pub fn new(from: U) -> Self {
        DigraphBuddyWalk {
            current_node: from,
            current_walk: DigraphNodeOutWalk::new(from),
            visit_count: UidStore::new(),
            next_nodes: VecDeque::new(),
        }
    }

    // return a tuple (weight, origin, destination)
    pub fn next<T, W>(&mut self, graph: &Digraph<T, W, U>) -> Option<(W, U, U)>
    where
        W: Copy,
        U: std::fmt::Debug,
        U: Eq,
        W: std::fmt::Debug,
    {
        loop {
            // explore edges of the current node
            let next = self.current_walk.next(graph);
            match next {
                Some((w, dest)) => {
                    // visit this edge
                    let count: u32;
                    // Count the number of times we visited the destination
                    if let Some(c) = self.visit_count.get_mut(dest) {
                        *c += 1;
                        count = *c;
                    } else {
                        self.visit_count.insert(dest, 1).unwrap();
                        count = 1;
                    }
                    // If we visited the destination through all its incoming edges,
                    // put it in the list of nodes to visit
                    if graph.nodes.get(dest).unwrap().nb_in == count {
                        self.next_nodes.push_back(dest);
                    }
                    return Some((w, self.current_node, dest));
                }
                None => {
                    // No more edge to visit on current node
                    // peek a new node in next list
                    match self.next_nodes.pop_front() {
                        // No mode nodes, walk is over
                        None => return None,
                        // Iterate Again with this node
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

//*************************
// Geometric solver

// edges contain the constraints on minimum dimensions
#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct SolverEdge {
    length: u32,
}

#[derive(Debug, PartialEq)]
pub struct SolverNode {
    pub min_val: Option<u32>,
    pub max_val: Option<u32>,
    //dist0: Option<u32>, // distance to previous edge X
    //dist1: Option<u32>, // distance to next edge X
}

#[derive(Debug, PartialEq)]
pub struct SolverGraph<U> {
    graph: Digraph<SolverNode, SolverEdge, U>,
}

// Iterate over solver nodes
pub struct SolverNodeIterator<'a, U> {
    graph: &'a SolverGraph<U>,
    iter: UidStoreIterator<'a, DigraphNode<SolverNode>, U>,
}

impl<U> SolverGraph<U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
    U: Eq,
    U: std::fmt::Debug,
{
    pub fn new() -> Self {
        SolverGraph {
            graph: Digraph::new(),
        }
    }

    pub fn add_node(&mut self, id: U) -> Result<U, String> {
        self.graph.add_node(
            id,
            SolverNode {
                min_val: None,
                max_val: None,
            },
        )
    }

    pub fn add_edge(&mut self, orig: U, dest: U, length: u32) -> Result<(), String> {
        self.graph.add_edge(orig, dest, SolverEdge { length })
    }

    pub fn solve(&mut self, origin: U) {
        let mut explored_edges: Vec<(u32, U, U)> = Vec::new();
        let mut walker: DigraphBuddyWalk<U> = DigraphBuddyWalk::new(origin);
        //Initialize origin
        let mut origin_node = self.graph.nodes.get_mut(origin).unwrap();
        origin_node.data.min_val = Some(0);
        origin_node.data.max_val = Some(0);

        while let Some((e, orig, dest)) = walker.next(&self.graph) {
            let orig_node = self.graph.nodes.get(orig).unwrap();
            let val: u32 = orig_node.data.min_val.unwrap() + e.length;

            let mut dest_node = self.graph.nodes.get_mut(dest).unwrap();
            dest_node.data.min_val = match dest_node.data.min_val {
                None => Some(val),
                Some(min) => Some(min.max(val)),
            };

            // Keep a stack of explored edges to rewind after
            explored_edges.push((e.length, orig, dest));
        }

        // rewind the graph and fill the max values
        while let Some((length, orig, dest)) = explored_edges.pop() {
            let dest_node = self.graph.nodes.get(dest).unwrap();
            let dest_value = match dest_node.data.max_val {
                //nodes at the end of the graph don't have a max value, use the min value then
                None => dest_node.data.min_val.unwrap(),
                Some(val) => val,
            };

            let val: u32 = dest_value - length;
            let mut orig_node = self.graph.nodes.get_mut(orig).unwrap();
            orig_node.data.max_val = match orig_node.data.max_val {
                None => Some(val),
                Some(max) => Some(max.min(val)),
            };
        }
    }

    pub fn get_solution(&self, id: U) -> Option<u32> {
        let node = self.graph.nodes.get(id)?;
        return match node.data.max_val {
            None => node.data.min_val,
            Some(max) => Some((node.data.min_val? + max) / 2),
        };
    }

    pub fn nodes_iter(&self) -> SolverNodeIterator<U> {
        SolverNodeIterator::new(self)
    }
}

impl<'a, U> SolverNodeIterator<'a, U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
    U: Debug,
{
    pub fn new(graph: &'a SolverGraph<U>) -> Self {
        SolverNodeIterator {
            graph,
            iter: graph.graph.nodes.iter(),
        }
    }
}

impl<'a, U> Iterator for SolverNodeIterator<'a, U>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
{
    type Item = (U, &'a SolverNode);

    fn next(&mut self) -> Option<(U, &'a SolverNode)> {
        let (id, node) = self.iter.next()?;
        Some((id, &node.data))
    }
}

//////////////////////////////////
// Path finder

// edges contain the constraints on minimum dimensions
#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct PathEdge<A> {
    length: u32,
    data: A,
}

#[derive(Debug, PartialEq)]
pub struct PathNode {
    pub walkable: bool, // if false, the node is only an entry or exit point
}

// A struct used to store node context during graph traversal
// Not part of the graph itself
#[derive(Debug, PartialEq)]
struct PathNodeInfo<U> {
    pub dist: u32,   // shortest distance from origin
    pub shortest: U, //shorted incoming path
}

pub struct PathFinderGraph<U, A> {
    graph: Graph<PathNode, PathEdge<A>, U>,
}

impl<U, A> PathFinderGraph<U, A>
where
    U: From<u64>,
    U: Into<u64>,
    U: Copy,
    U: Eq,
    U: std::fmt::Debug,
    A: Copy,
    A: std::fmt::Debug,
{
    pub fn new() -> Self {
        PathFinderGraph {
            graph: Graph::new(),
        }
    }

    pub fn add_node(&mut self, id: U, walkable: bool) -> Result<U, String> {
        self.graph.add_node(id, PathNode { walkable })
    }

    pub fn add_edge(&mut self, orig: U, dest: U, length: u32, data: A) -> Result<(), String> {
        self.graph.add_edge(orig, dest, PathEdge { length, data })?;
        Ok(())
    }

    pub fn get_edge(&self, orig: U, dest: U) -> Option<(u32, A)> {
        let edge = self.graph.get_edge(orig, dest)?;
        Some((edge.length, edge.data))
    }

    pub fn solve(&self, origin: U, destination: U) -> Result<Vec<U>, ()> {
        let mut next_nodes: VecDeque<U> = VecDeque::new();
        let mut visited: UidStore<PathNodeInfo<U>, U> = UidStore::new();
        next_nodes.push_back(origin);
        visited
            .insert(
                origin,
                PathNodeInfo {
                    dist: 0,
                    shortest: origin,
                },
            )
            .unwrap();

        while let Some(id) = next_nodes.pop_front() {
            let orig_dist = visited.get(id).unwrap().dist;
            let mut walker = self.graph.walk_from(id);

            while let Some((edge, neighbour)) = walker.next(&self.graph.digraph) {
                let distance = orig_dist + edge.length;
                let dest = visited.get(neighbour);

                // If we did not visit the destination,
                // or if we just found a shorter path to the destination
                // put the destination node into the next_nodes list
                if dest.is_none() || dest.unwrap().dist > distance {
                    visited
                        .set(
                            neighbour,
                            PathNodeInfo {
                                dist: distance,
                                shortest: id,
                            },
                        )
                        .unwrap();

                    if self
                        .graph
                        .digraph
                        .nodes
                        .get(neighbour)
                        .unwrap()
                        .data
                        .walkable
                    {
                        next_nodes.push_back(neighbour);
                    }
                }
            }
        }

        let mut path: Vec<U> = Vec::new();
        let mut cur = destination;
        //println!("visited: {:?}", visited);
        loop {
            path.insert(0, cur);
            if cur == origin {
                return Ok(path);
            }
            match visited.get(cur) {
                Some(info) => cur = info.shortest,
                None => return Err(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edges_ordering1() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
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

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id2, id3, 1).unwrap();
        graph.add_edge(id1, id2, 1).unwrap();

        // Check that edges are sorted in the right order
        assert_eq!(graph.edges[0].orig, id0);
        assert_eq!(graph.edges[1].orig, id1);
        assert_eq!(graph.edges[2].orig, id2);
    }

    #[test]
    fn test_edges_ordering2() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
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

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id2, id3, 1).unwrap();
        graph.add_edge(id1, id2, 1).unwrap();
        graph.add_edge(id1, id4, 1).unwrap();
        graph.add_edge(id4, id2, 1).unwrap();

        // Check that edges are sorted in the right order
        assert_eq!(graph.edges[0].orig, id0);
        assert_eq!(graph.edges[1].orig, id1);
        assert_eq!(graph.edges[2].orig, id1);
        assert_eq!(graph.edges[3].orig, id2);
        assert_eq!(graph.edges[4].orig, id4);
    }

    #[test]
    fn test_edges_ordering3() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
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

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id3, id2, 1).unwrap();
        graph.add_edge(id0, id3, 1).unwrap();
        graph.add_edge(id1, id2, 1).unwrap();
        graph.add_edge(id4, id1, 1).unwrap();
        graph.add_edge(id0, id4, 1).unwrap();

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
    fn test_getters() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
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
        graph.add_node(id1, 10).unwrap();
        graph.add_node(id2, 20).unwrap();
        graph.add_node(id3, 30).unwrap();
        graph.add_node(id4, 40).unwrap();

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id3, id2, 2).unwrap();
        graph.add_edge(id0, id3, 3).unwrap();
        graph.add_edge(id1, id2, 4).unwrap();
        graph.add_edge(id4, id1, 5).unwrap();
        graph.add_edge(id0, id4, 6).unwrap();

        assert_eq!(graph.get_node(id0), Some(&0));
        assert_eq!(graph.get_node(id1), Some(&10));
        assert_eq!(graph.get_node(id2), Some(&20));
        assert_eq!(graph.get_node(id3), Some(&30));
        assert_eq!(graph.get_node(id4), Some(&40));

        assert_eq!(graph.get_edge(id0, id1), Some(&1));
        assert_eq!(graph.get_edge(id3, id2), Some(&2));
        assert_eq!(graph.get_edge(id0, id3), Some(&3));
        assert_eq!(graph.get_edge(id1, id2), Some(&4));
        assert_eq!(graph.get_edge(id4, id1), Some(&5));
        assert_eq!(graph.get_edge(id0, id4), Some(&6));

        assert_eq!(graph.get_edge(id1, id3), None);
        assert_eq!(graph.get_node(gen.get()), None);
    }

    #[test]
    fn test_adjacent_walk() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
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

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id1, id2, 1).unwrap();
        graph.add_edge(id2, id5, 1).unwrap();
        graph.add_edge(id1, id4, 1).unwrap();
        graph.add_edge(id1, id3, 1).unwrap();
        graph.add_edge(id4, id2, 1).unwrap();
        graph.add_edge(id3, id2, 1).unwrap();

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
        let mut gen: GuidManager<Guid> = GuidManager::new();
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

        graph.add_edge(id0, id1, 1).unwrap();
        graph.add_edge(id1, id2, 1).unwrap();
        graph.add_edge(id2, id5, 1).unwrap();
        graph.add_edge(id5, id6, 1).unwrap();
        graph.add_edge(id1, id3, 1).unwrap();
        graph.add_edge(id3, id5, 1).unwrap();
        graph.add_edge(id0, id4, 1).unwrap();
        graph.add_edge(id4, id5, 1).unwrap();

        let mut walk = DigraphBuddyWalk::new(id0);
        assert_eq!(walk.next(&graph), Some((1, id0, id4)));
        assert_eq!(walk.next(&graph), Some((1, id0, id1)));
        assert_eq!(walk.next(&graph), Some((1, id4, id5)));
        assert_eq!(walk.next(&graph), Some((1, id1, id3)));
        assert_eq!(walk.next(&graph), Some((1, id1, id2)));
        assert_eq!(walk.next(&graph), Some((1, id3, id5)));
        assert_eq!(walk.next(&graph), Some((1, id2, id5)));
        assert_eq!(walk.next(&graph), Some((1, id5, id6)));
    }

    #[test]
    fn test_solver() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
        let mut solver: SolverGraph<Guid> = SolverGraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 -> id1 -> id2 -> id4
        //   \----> id3  ------^

        solver.add_node(id0).unwrap();
        solver.add_node(id1).unwrap();
        solver.add_node(id2).unwrap();
        solver.add_node(id3).unwrap();
        solver.add_node(id4).unwrap();

        solver.add_edge(id0, id1, 2).unwrap();
        solver.add_edge(id1, id2, 2).unwrap();
        solver.add_edge(id2, id4, 2).unwrap();
        solver.add_edge(id0, id3, 12).unwrap();
        solver.add_edge(id3, id4, 4).unwrap();

        solver.solve(id0);
        assert_eq!(solver.get_solution(id0), Some(0));
        assert_eq!(solver.get_solution(id4), Some(16));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.min_val, Some(2));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.max_val, Some(12));
        assert_eq!(solver.graph.nodes.get(id1).unwrap().data.min_val, Some(2));
        assert_eq!(solver.get_solution(id1), Some(7));
        assert_eq!(solver.get_solution(id2), Some(9));
    }

    #[test]
    fn test_graph() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
        let mut graph: Graph<u32, u32, Guid> = Graph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();

        // Build this graph :
        // id0 <-> id1
        //  | ^----v |
        // id3 <-> id2

        graph.add_node(id0, 0).unwrap();
        graph.add_node(id1, 0).unwrap();
        graph.add_node(id2, 0).unwrap();
        graph.add_node(id3, 0).unwrap();

        graph.add_edge(id0, id1, 0).unwrap();
        graph.add_edge(id1, id2, 0).unwrap();
        graph.add_edge(id2, id3, 0).unwrap();
        graph.add_edge(id3, id0, 0).unwrap();
        graph.add_edge(id2, id0, 0).unwrap();

        {
            let mut walker = graph.walk_from(id0);
            let mut neighbours: Vec<u64> = Vec::new();
            while let Some((_, id)) = walker.next(&graph.digraph) {
                neighbours.push(id.into());
            }
            neighbours.sort();
            assert_eq!(neighbours, [id1.into(), id2.into(), id3.into()]);
        }
        {
            let mut walker = graph.walk_from(id1);
            let mut neighbours: Vec<u64> = Vec::new();
            while let Some((_, id)) = walker.next(&graph.digraph) {
                neighbours.push(id.into());
            }
            neighbours.sort();
            assert_eq!(neighbours, [id0.into(), id2.into()]);
        }
        {
            let mut walker = graph.walk_from(id2);
            let mut neighbours: Vec<u64> = Vec::new();
            while let Some((_, id)) = walker.next(&graph.digraph) {
                neighbours.push(id.into());
            }
            neighbours.sort();
            assert_eq!(neighbours, [id0.into(), id1.into(), id3.into()]);
        }
        {
            let mut walker = graph.walk_from(id3);
            let mut neighbours: Vec<u64> = Vec::new();
            while let Some((_, id)) = walker.next(&graph.digraph) {
                neighbours.push(id.into());
            }
            neighbours.sort();
            assert_eq!(neighbours, [id0.into(), id2.into()]);
        }
    }

    #[test]
    fn test_pathfinder1() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
        let mut graph: PathFinderGraph<Guid, ()> = PathFinderGraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 <-> id1 <-> id2 <-> id3
        //     \--   id4  --/

        graph.add_node(id0, true).unwrap();
        graph.add_node(id1, true).unwrap();
        graph.add_node(id2, true).unwrap();
        graph.add_node(id3, true).unwrap();
        graph.add_node(id4, true).unwrap();

        graph.add_edge(id0, id1, 1, ()).unwrap();
        graph.add_edge(id1, id2, 1, ()).unwrap();
        graph.add_edge(id2, id3, 1, ()).unwrap();
        graph.add_edge(id0, id4, 4, ()).unwrap();
        graph.add_edge(id4, id3, 4, ()).unwrap();

        let result = graph.solve(id0, id3);
        assert_eq!(result, Ok(vec![id0, id1, id2, id3]));
    }

    #[test]
    fn test_pathfinder2() {
        let mut gen: GuidManager<Guid> = GuidManager::new();
        let mut graph: PathFinderGraph<Guid, ()> = PathFinderGraph::new();
        let id0 = gen.get();
        let id1 = gen.get();
        let id2 = gen.get();
        let id3 = gen.get();
        let id4 = gen.get();

        // Build this graph :
        // id0 <-> id1 <-> id2 <-> id3
        //     \-----   id4  -----/

        graph.add_node(id0, true).unwrap();
        graph.add_node(id1, false).unwrap();
        graph.add_node(id2, true).unwrap();
        graph.add_node(id3, true).unwrap();
        graph.add_node(id4, true).unwrap();

        graph.add_edge(id0, id1, 1, ()).unwrap();
        graph.add_edge(id1, id2, 1, ()).unwrap();
        graph.add_edge(id2, id3, 1, ()).unwrap();
        graph.add_edge(id0, id4, 4, ()).unwrap();
        graph.add_edge(id4, id3, 4, ()).unwrap();

        let result = graph.solve(id0, id3);
        assert_eq!(result, Ok(vec![id0, id4, id3]));
    }
}
