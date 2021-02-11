// A tree structure allocated in the same contiguous memory region
// allocated memory can only grow, there is no freeing mechanism


pub type NodeId = usize;

#[derive(Debug, PartialEq, Default)]
struct TreeNode<T> {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub sibling_left: Option<NodeId>,
    pub sibling_right: Option<NodeId>,
    pub data: T,
}



#[derive(Debug, PartialEq, Default)]
pub struct TreeContainer<T> {
    // nodes organized as a tree
    root: Option<NodeId>,
    // Flat list of all the nodes
    // It is allowed to have elements in this flat list that are not present in
    flat: Vec<Option<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(t: T) -> Self {
        TreeNode {
            parent: None,
            first_child: None,
            sibling_left: None,
            sibling_right: None,
            data: t,
        }
    }

    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        return self;
    }

    pub fn with_first_child(mut self, child: usize) -> Self {
        self.first_child = Some(child);
        return self;
    }

    pub fn with_sibling_left(mut self, left: usize) -> Self {
        self.sibling_left = Some(left);
        return self;
    }

    pub fn with_sibling_right(mut self, right: usize) -> Self {
        self.sibling_right = Some(right);
        return self;
    }
}

impl<T> TreeContainer<T> {
    pub fn new() -> Self {
        TreeContainer {
            root: None,
            flat: vec![],
        }
    }

    pub fn add_root(&mut self, t: T) -> NodeId {
        let n = TreeNode::new(t);
        self.flat.push(Some(n));
        let idx = self.flat.len() - 1;
        self.root = Some(idx);
        return idx;
    }

    //    x      x         x           x
    //       A-> |  B->   /      C->  /
    //           y       y--z        a--y--z
    // Add a child a the given index.
    // Panics if pos is > the number of children or if parent does not exist
    pub fn add_child(&mut self, parent: NodeId, pos: usize, t: T) -> Result<NodeId,&str> {
        if !self._has_child(parent) {
            // case A
            if pos > 0 {
                panic!();
            }
            // Insert the first child
            let node = TreeNode::new(t).with_parent(parent);
            let new_id = self.flat.len();
            if let Some(p) = self.flat.get_mut(parent).ok_or("").unwrap() {
                p.first_child = Some(new_id);
                self.flat.push(Some(node));
            } else {
                panic!();
            }
            return Ok(new_id);
        } else {
            if pos > 0{
                // case B
                let left_id = self._nth_child_id(parent, pos-1).ok_or("").unwrap();
                let new_id = self.flat.len();
                let mut node = TreeNode::new(t).with_parent(parent).with_sibling_left(left_id);
                let mut right_id: Option<usize> = None;
                

                if let Some(l) = self.flat.get_mut(left_id).unwrap() {
                    right_id = l.sibling_right;
                    l.sibling_right = Some(new_id);
                } else {
                    panic!();
                }

                if let Some(id) = right_id{
                    if let Some(r) = self.flat.get_mut(id).unwrap() {
                        r.sibling_left = Some(new_id);
                        node.sibling_right = Some(id);
                    }
                }
                self.flat.push(Some(node)); 
                return Ok(new_id);
            }
            else{
                // case C
                // Insert the child at 1st position
                let new_id = self.flat.len();
                let right_id = self._first_child_id(parent).ok_or("").unwrap();
                let node = TreeNode::new(t).with_parent(parent).with_sibling_right(right_id);

                if let Some(p) = self.flat.get_mut(parent).ok_or("").unwrap() {
                    p.first_child = Some(new_id);
                    self.flat.push(Some(node));
                } else {
                    panic!();
                }

                if let Some(r) = self.flat.get_mut(right_id).ok_or("").unwrap() {
                    r.sibling_left = Some(new_id);
                } else {
                    panic!();
                }

                return Ok(new_id);
            }
        }
    }


    // Push a child at the last available position
    pub fn push_child(&mut self, parent: NodeId, t: T) -> Result<NodeId,&str>  {
        if !self._has_child(parent) {
            // Insert the first child
            let node = TreeNode::new(t).with_parent(parent);
            let new_id = self.flat.len();
            if let Some(p) = self.flat.get_mut(parent).ok_or("").unwrap() {
                p.first_child = Some(new_id);
                self.flat.push(Some(node));
            } else {
                panic!();
            }
            return Ok(new_id);
        } else {
            // Push child at the end of list
            let left_id = self._last_child_id(parent).ok_or("").unwrap();
            let new_id = self.flat.len();
            let mut node = TreeNode::new(t).with_parent(parent).with_sibling_left(left_id);
            

            if let Some(l) = self.flat.get_mut(left_id).unwrap() {
                l.sibling_right = Some(new_id);
            } else {
                panic!();
            }

            self.flat.push(Some(node)); 
            return Ok(new_id); 
        }
    }


    // unplug a node from the tree structure but keep the node and its children in storage
    pub fn unplug(&mut self, node:NodeId) -> Result<NodeId, &str>{
        return Err("Not implemented");
    }

    // plug a node that already exists in the tree structure at a new place
    // The node must not be part of the tree struct, i.e.it must not have a parent
    pub fn replug(&mut self, node:NodeId, parent:NodeId, position:usize) -> Result<NodeId, &str>{
        return Err("Not implemented");
    }

    // Prepend a sibling node
    // Does not work on root
    pub fn prepend(&mut self, sibling: NodeId, t: T) -> NodeId {
        return 0;
    }

    // Append a sibling node
    // Does not work on root
    pub fn append(&mut self, sibling: NodeId, t: T) -> NodeId {
        return 0;
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        let c = self.flat.get(id)?;
        match c {
            None => None,
            Some(n) => Some(&n.data),
        }
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        let c = self.flat.get_mut(id)?;
        match c {
            None => None,
            Some(n) => Some(&mut n.data)
        }
    }


    // path is a string of form "0:12:4:1..."
    // "0" is the root
    // "0:1" is the first child of the root
    // etc.
    pub fn id_by_path(&self, path: &str) -> Option<NodeId> {
        let mut iter = path.split(':').map(|i| usize::from_str_radix(i, 10));

        // First fragment must be '0'
        match iter.next(){
            Some(Ok(i)) => if i!=0 {return None},
            _ => return None
        }

        let mut cursor:usize = self.root?;

        while let Some(fragment) = iter.next() {
            if let Ok(i) = fragment {
                if let Some(c) = self._nth_child_id(cursor, i) {
                    cursor = c;
                }
                else {
                    return None;
                }
            }
            else {
                return None;
            }
        }
        return Some(cursor);
    }

    pub fn for_each_child<F>(&self, parent:NodeId, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(node) = self._get_node(parent){
            let mut next: Option<NodeId> = node.first_child;
            while let Some(cur_id) = next{
                f(self.get(cur_id).unwrap());
                next = self._get_node(cur_id).unwrap().sibling_right;
            }
        }
    }

    pub fn for_each_child_mut<F>(&mut self, parent:NodeId, mut f: F)
    where
        F: FnMut(&mut T),
    {
        if let Some(node) = self._get_node(parent){
            let mut next: Option<NodeId> = node.first_child;
            while let Some(cur_id) = next{
                f(self.get_mut(cur_id).unwrap());
                next = self._get_node(cur_id).unwrap().sibling_right;
            }
        }
    }

    // path is a string of form "0:12:4:1..."
    pub fn by_path(&self, path: &str) -> Option<&T> {
        let id = self.id_by_path(path)?;
        return self.get(id);
    }

    pub fn by_path_mut(&mut self, path: &str) -> Option<&mut T> {
        let id = self.id_by_path(path)?;
        return self.get_mut(id);
    }

    // Store a new node and return its id. The node will be floating
    fn _new_id(&mut self, t:T) -> NodeId {
        let node = TreeNode::new(t);
        let new_id = self.flat.len();
        self.flat.push(Some(node));
        return new_id;      
    }

    fn _get_node(&self, id: NodeId) -> Option<&TreeNode<T>> {
        let c = self.flat.get(id)?;
        match c {
            None => None,
            Some(n) => Some(&n),
        }
    }


    // panics if id does not exist
    fn _has_child(&self, id: NodeId) -> bool {
        if let Some(n) = self.flat.get(id).unwrap() {
            return n.first_child.is_some();
        } else {
            panic!();
        }
    }

    fn _first_child_id(&self, id: NodeId) -> Option<NodeId> {
        if let Some(n) = self.flat.get(id)? {
            n.first_child
        } else {
            None
        }
    }

    fn _nth_child_id(&self, id: NodeId, offset: usize) -> Option<NodeId> {
        if let Some(n) = self.flat.get(id)? {
            let mut cur = n.first_child?;
            let mut cnt = 0;

            while cnt < offset {
                if let Some(node) = self.flat.get(cur)? {
                    cur = node.sibling_right?;
                } else {
                    return None;
                }
                cnt += 1;
            }

            return Some(cur);
        } else {
            return None;
        }
    }

    fn _last_child_id(&self, id: NodeId) -> Option<NodeId> {
        if let Some(n) = self.flat.get(id)? {
            let mut cur = n.first_child?;

            loop {
                if let Some(node) = self.flat.get(cur)? {
                    if let Some(sibling) = node.sibling_right {
                        cur = sibling;
                    } else {
                        return Some(cur);
                    }
                } else {
                    return None;
                }
            }

            return None;
        } else {
            return None;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_builder() {
        let node: TreeNode<u32> = TreeNode::new(42)
            .parent(43)
            .first_child(44)
            .sibling_left(45)
            .sibling_right(46);

        assert_eq!(node.data, 42);
        assert_eq!(node.parent, Some(43));
        assert_eq!(node.first_child, Some(44));
        assert_eq!(node.sibling_left, Some(45));
        assert_eq!(node.sibling_right, Some(46));
    }

    #[test]
    fn test_basic() {
        let mut cont: TreeContainer<i64> = TreeContainer::new();
        let id = cont.add_root(32412345);
        assert_eq!(*cont.get(id).unwrap(), 32412345);

        if let Some(mut n) = cont.get_mut(id) {
            *n = 123456;
        }
        assert_eq!(*cont.get(id).unwrap(), 123456);
    }

    #[test]
    fn add_first_child() {
        let mut cont: TreeContainer<i64> = TreeContainer::new();
        let root_id = cont.add_root(32412345);

        assert_eq!(*cont.get(root_id).unwrap(), 32412345);

        let child_id = cont.add_child(root_id, 0, 834576098).unwrap();
        assert_eq!(*cont.get(child_id).unwrap(), 834576098);

        // assumptions about the internal API
        assert_eq!(Some(child_id), cont._first_child_id(root_id));
        assert_eq!(Some(child_id), cont._last_child_id(root_id));
        assert_eq!(Some(child_id), cont._nth_child_id(root_id, 0));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child_id).unwrap().sibling_left, None);
        assert_eq!(cont._get_node(child_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child_id).unwrap().parent, Some(root_id));
    }

    #[test]
    fn add_children() {
        let mut cont: TreeContainer<i64> = TreeContainer::new();
        let root_id = cont.add_root(32412345);
        let child1_id = cont.add_child(root_id, 0, 834576098).unwrap();


        let child2_id = cont.add_child(root_id, 1, 908720349875).unwrap();
        assert_eq!(*cont.get(child2_id).unwrap(), 908720349875);
        // assumptions about the internal API
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 1));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, None);

        assert_eq!(cont._get_node(child2_id).unwrap().parent, Some(root_id));


        let child3_id = cont.add_child(root_id, 1, 304958).unwrap();
        assert_eq!(*cont.get(child3_id).unwrap(), 304958);
        // assumptions about the internal API
        assert_eq!(Some(child3_id), cont._nth_child_id(root_id, 1));
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 2));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, Some(child2_id));
        assert_eq!(cont._get_node(child3_id).unwrap().parent, Some(root_id));

        let child4_id = cont.add_child(root_id, 0, 452579).unwrap();
        assert_eq!(*cont.get(child4_id).unwrap(), 452579);
        // assumptions about the internal API
        assert_eq!(Some(child1_id), cont._nth_child_id(root_id, 1));
        assert_eq!(Some(child4_id), cont._nth_child_id(root_id, 0));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_left, None);
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_left, Some(child4_id));
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_right, Some(child1_id));
        assert_eq!(cont._get_node(child4_id).unwrap().parent, Some(root_id));
    }


    #[test]
    fn push_children() {
        let mut cont: TreeContainer<i64> = TreeContainer::new();
        let root_id = cont.add_root(32412345);
        let child1_id = cont.push_child(root_id, 834576098).unwrap();
        assert_eq!(*cont.get(child_id).unwrap(), 834576098);

        // assumptions about the internal API
        assert_eq!(Some(child1_id), cont._first_child_id(root_id));
        assert_eq!(Some(child1_id), cont._last_child_id(root_id));
        assert_eq!(Some(child1_id), cont._nth_child_id(root_id, 0));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_left, None);
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child1_id).unwrap().parent, Some(root_id));


        let child2_id = cont.push_child(root_id, 908720349875).unwrap();
        assert_eq!(*cont.get(child2_id).unwrap(), 908720349875);
        // assumptions about the internal API
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 1));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child2_id).unwrap().parent, Some(root_id));
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_right, Some(child2_id));


        let child3_id = cont.push_child(root_id, 304958).unwrap();
        assert_eq!(*cont.get(child3_id).unwrap(), 304958);
        // assumptions about the internal API
        assert_eq!(Some(child3_id), cont._nth_child_id(root_id, 2));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_left, Some(child2_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child3_id).unwrap().parent, Some(root_id));

        let child4_id = cont.push_child(root_id, 0, 452579).unwrap();
        assert_eq!(*cont.get(child4_id).unwrap(), 452579);
        // assumptions about the internal API
        assert_eq!(Some(child4_id), cont._nth_child_id(root_id, 3));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_left, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, Some(child4_id));
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child4_id).unwrap().parent, Some(root_id));
    }



    #[test]
    fn test_path() {
        let mut cont: TreeContainer<i64> = TreeContainer::new();
        let root_id = cont.add_root(32412345);
        let id1 = cont.add_child(root_id, 0, 65874567).unwrap();
        let id2 = cont.add_child(root_id, 1, 267869890).unwrap();
        let id3 = cont.add_child(id2, 0, 3454).unwrap();

        assert_eq!(cont.id_by_path("0"), Some(root_id));
        assert_eq!(cont.id_by_path("0:0"), Some(id1));
        assert_eq!(cont.id_by_path("0:1"), Some(id2));
        assert_eq!(cont.id_by_path("0:1:0"), Some(id3));

        assert_eq!(cont.id_by_path("1"), None);
        assert_eq!(cont.id_by_path("0:1:1"), None);
        assert_eq!(cont.id_by_path("0:2"), None);
    }
}