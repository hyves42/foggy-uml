// A tree structure allocated in the same contiguous memory region
// allocated memory can only grow, there is no freeing mechanism
use crate::utils::uid::*;




#[derive(Debug, PartialEq, Default)]
struct TreeNode<T,U> {
    pub parent: Option<U>,
    pub first_child: Option<U>,
    pub sibling_left: Option<U>,
    pub sibling_right: Option<U>,
    pub data: T,
}



#[derive(Debug, PartialEq)]
pub struct TreeContainer<T,U> {
    // nodes organized as a tree
    root: Option<U>,
    // Flat list of all the nodes
    // It is allowed to have node in this flat list 
    // that are not present in the tree
    flat: UidStore<TreeNode<T, U>, U>,
}

// Iterate over children elements directly
pub struct TreeChildrenIterator<'a, T, U> {
    tree: &'a TreeContainer<T, U>,
    parent: U,
    cur_id: Option<U>
}


// Iterate depth-first over elements
pub struct TreeDepthIterator<'a, T, U> {
    tree: &'a TreeContainer<T, U>,
    cur_id: Option<U>
}

// Visit children nodes without holding a reference to the tree
pub struct ChildrenIdWalk<U> {
    parent: U,
    cur_id: Option<U>
}

impl<T, U> TreeNode<T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(t: T) -> Self {
        TreeNode {
            parent: None,
            first_child: None,
            sibling_left: None,
            sibling_right: None,
            data: t,
        }
    }

    pub fn with_parent(mut self, parent: U) -> Self {
        self.parent = Some(parent);
        return self;
    }

    pub fn with_first_child(mut self, child: U) -> Self {
        self.first_child = Some(child);
        return self;
    }

    pub fn with_sibling_left(mut self, left: U) -> Self {
        self.sibling_left = Some(left);
        return self;
    }

    pub fn with_sibling_right(mut self, right: U) -> Self {
        self.sibling_right = Some(right);
        return self;
    }
}

impl<T, U> TreeContainer<T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new() -> Self {
        TreeContainer {
            root: None,
            flat: UidStore::new(),
        }
    }

    pub fn with_root(mut self, t: T, uid:U) -> Self {
        let n = TreeNode::new(t);
        self.flat.insert(uid,n);
        self.root = Some(uid);
        return self;
    }

    pub fn root_id(&self) -> Option<U> {
        return self.root;
    }




    pub fn add_root(&mut self, t: T, uid:U) -> U {
        let n = TreeNode::new(t);
        self.flat.insert(uid,n);
        self.root = Some(uid);
        return uid;
    }

    //    x      x         x           x
    //       A-> |  B->   /      C->  /
    //           y       y--z        a--y--z
    // Add a child a the given index.
    // Panics if pos is > the number of children or if parent does not exist
    pub fn add_child(&mut self, parent: U, pos: usize, t: T, uid: U) -> Result<U,&str> {
        if !self._has_child(parent) {
            // case A
            if pos > 0 {
                panic!();
            }
            // Insert the first child
            let node = TreeNode::new(t).with_parent(parent);
            if let Some(p) = self.flat.get_mut(parent) {
                p.first_child = Some(uid);
                self.flat.insert(uid, node);
            } else {
                panic!();
            }
            return Ok(uid);
        } else {
            if pos > 0{
                // case B
                let left_id = self._nth_child_id(parent, pos-1).unwrap();
                let mut node = TreeNode::new(t).with_parent(parent).with_sibling_left(left_id);
                let mut right_id: Option<U> = None;

                if let Some(l) = self.flat.get_mut(left_id) {
                    right_id = l.sibling_right;
                    l.sibling_right = Some(uid);
                } else {
                    panic!();
                }

                if let Some(id) = right_id{
                    if let Some(r) = self.flat.get_mut(id) {
                        r.sibling_left = Some(uid);
                        node.sibling_right = Some(id);
                    }
                }
                self.flat.insert(uid, node); 
                return Ok(uid);
            }
            else{
                // case C
                // Insert the child at 1st position
                let right_id = self._first_child_id(parent).unwrap();
                let node = TreeNode::new(t).with_parent(parent).with_sibling_right(right_id);

                if let Some(p) = self.flat.get_mut(parent) {
                    p.first_child = Some(uid);
                    self.flat.insert(uid, node);
                } else {
                    panic!();
                }

                if let Some(r) = self.flat.get_mut(right_id) {
                    r.sibling_left = Some(uid);
                } else {
                    panic!();
                }

                return Ok(uid);
            }
        }
    }


    // Push a child at the last available position
    pub fn push_child(&mut self, parent: U, t: T, uid:U) -> Result<U,&str>  {
        if !self._has_child(parent) {
            // Insert the first child
            let node = TreeNode::new(t).with_parent(parent);

            if let Some(p) = self.flat.get_mut(parent) {
                p.first_child = Some(uid);
                self.flat.insert(uid, node);
            } else {
                panic!();
            }
            return Ok(uid);
        } else {
            // Push child at the end of list
            let left_id = self._last_child_id(parent).unwrap();
            let mut node = TreeNode::new(t).with_parent(parent).with_sibling_left(left_id);
            
            if let Some(l) = self.flat.get_mut(left_id) {
                l.sibling_right = Some(uid);
            } else {
                panic!();
            }

            self.flat.insert(uid, node); 
            return Ok(uid); 
        }
    }


    // unplug a node from the tree structure but keep the node and its children in storage
    pub fn unplug(&mut self, node:U) -> Result<U, &str>{
        return Err("Not implemented");
    }

    // plug a node that already exists in the tree structure at a new place
    // The node must not be part of the tree struct, i.e.it must not have a parent
    pub fn replug(&mut self, node:U, parent:U, position:usize) -> Result<U, &str>{
        return Err("Not implemented");
    }

    // // Prepend a sibling node
    // // Does not work on root
    // pub fn prepend(&mut self, sibling: U, t: T) -> U {
    //     return U::from(0);
    // }

    // // Append a sibling node
    // // Does not work on root
    // pub fn append(&mut self, sibling: U, t: T) -> U {
    //     return U::from(0);
    // }

    pub fn get(&self, id: U) -> Option<&T> {
        let c = self.flat.get(id)?;
        Some(&c.data)
    }

    pub fn get_mut(&mut self, id: U) -> Option<&mut T> {
        let c = self.flat.get_mut(id)?;
        Some(&mut c.data)
    }

    // Return the number of nodes below node id
    // Panic if id is invalid
    pub fn len(&self, id:U) -> usize {
        let node = self._get_node(id).unwrap(); 
        let mut count = 0;
        if let Some(mut cur) = node.first_child {
            count+=1;

            loop {
                let node = self._get_node(cur).unwrap();
                if let Some(sibling) = node.sibling_right {
                    cur = sibling;
                    count+=1;
                } else {
                    return count;
                }
            }
        }
        else{
            return 0;
        }
    }

    // path is a string of form "0:12:4:1..."
    // "0" is the root
    // "0:1" is the first child of the root
    // etc.
    pub fn id_by_path(&self, path: &str) -> Option<U> {
        let mut iter = path.split(':').map(|i| usize::from_str_radix(i, 10));

        // First fragment must be '0'
        match iter.next(){
            Some(Ok(i)) => if i!=0 {return None},
            _ => return None
        }

        let mut cursor: U = self.root?;

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

    pub fn for_each_child<F>(&self, parent:U, mut f: F)
    where
        F: FnMut(U, &T),
    {
        if let Some(node) = self._get_node(parent){
            let mut next: Option<U> = node.first_child;
            while let Some(cur_id) = next{
                f(cur_id, self.get(cur_id).unwrap());
                next = self._get_node(cur_id).unwrap().sibling_right;
            }
        }
    }

    pub fn for_each_child_mut<F>(&mut self, parent:U, mut f: F)
    where
        F: FnMut(U, &mut T),
    {
        if let Some(node) = self._get_node(parent){
            let mut next: Option<U> = node.first_child;
            while let Some(cur_id) = next{
                f(cur_id, self.get_mut(cur_id).unwrap());
                next = self._get_node(cur_id).unwrap().sibling_right;
            }
        }
    }


    pub fn children_iter(&self, parent:U)
    -> TreeChildrenIterator<T,U>
    {
        TreeChildrenIterator::new(&self, parent)
    }

    pub fn depth_iter(&self)
    -> TreeDepthIterator<T,U>
    {
        TreeDepthIterator::new(&self)
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
    fn _new_id(&mut self, t:T, uid: U) -> U {
        let node = TreeNode::new(t);
        self.flat.insert(uid,node);
        return uid;      
    }

    fn _get_node(&self, id: U) -> Option<&TreeNode<T,U>> {
        self.flat.get(id)
    }


    // panics if id does not exist
    fn _has_child(&self, id: U) -> bool {
        self.flat.get(id).unwrap().first_child.is_some()
    }

    fn _first_child_id(&self, id: U) -> Option<U> {
        if let Some(n) = self.flat.get(id) {
            n.first_child
        } else {
            None
        }
    }

    fn _nth_child_id(&self, id: U, offset: usize) -> Option<U> {
        let n = self.flat.get(id)? ;
        let mut cur = n.first_child?;
        let mut cnt = 0;

        while cnt < offset {
            let node = self.flat.get(cur)?;
            cur = node.sibling_right?;
            cnt += 1;
        }

        return Some(cur);
    }

    fn _last_child_id(&self, id: U) -> Option<U> {
        let n = self.flat.get(id)?;
        let mut cur = n.first_child?;

        loop {
            let node = self.flat.get(cur)? ;
            if let Some(sibling) = node.sibling_right {
                cur = sibling;
            } else {
                return Some(cur);
            }
        }
 
    }

    fn _sibling_id(&self, id: U) -> Option<U> {
        self._get_node(id)?.sibling_right
    }

    fn _parent_id(&self, id: U) -> Option<U> {
        self._get_node(id)?.parent
    }
}

impl <'a, T, U> TreeChildrenIterator<'a, T, U> {
    pub fn new(tree :&'a TreeContainer<T, U>, parent:U ) -> Self {
        TreeChildrenIterator {
            tree,
            parent,
            cur_id: None
        }
    }
}

impl <'a, T, U> Iterator for TreeChildrenIterator<'a, T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    type Item = (U, &'a T);

    fn next(&mut self) -> Option<(U, &'a T)> {
        // if I'm already iterating through children nodes
        if let Some(id) = self.cur_id{
            self.cur_id = Some(self.tree._get_node(id)?.sibling_right?);
        }
        // First call of iterator, return first child
        else{
            self.cur_id = Some(self.tree._get_node(self.parent)?.first_child?);
        }
        return Some((self.cur_id?, self.tree.get(self.cur_id?)?));
    }    
}

impl <'a, T, U> TreeDepthIterator<'a, T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(tree :&'a TreeContainer<T, U>) -> Self {
        TreeDepthIterator {
            tree,
            cur_id: None
        }
    }
}

impl <'a, T, U> Iterator for TreeDepthIterator<'a, T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    type Item = (U, &'a T);

    fn next(&mut self) -> Option<(U, &'a T)> {
        // First call of iterator, return root if any
        if self.cur_id.is_none(){
            self.cur_id = self.tree.root;
        }
        else if let Some(id) = self.tree._first_child_id(self.cur_id?){
            //return first child
            self.cur_id = Some(id);    
        }
        else if let Some(id) = self.tree._sibling_id(self.cur_id?){
            //return next sibling
            self.cur_id = Some(id); 
        }
        else{
            // parent sibling
            let mut cur = self.cur_id?;

            loop {
                if let Some(parent) = self.tree._parent_id(cur) {
                    if let Some(sibling) = self.tree._sibling_id(parent) {
                        self.cur_id = Some(sibling);
                        break;
                    }
                    cur=parent;
                } else {
                    self.cur_id = None;
                    break;
                }
            }
        }
        return Some((self.cur_id?, self.tree.get(self.cur_id?)?));
    }    
}


impl<U> ChildrenIdWalk<U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new(parent:U) -> Self {
        ChildrenIdWalk {
            parent,
            cur_id: None
        }
    }

    pub fn next<T> (&mut self, tree: & TreeContainer<T, U>) -> Option<U> {
        // if I'm already iterating through children nodes
        if let Some(id) = self.cur_id{
            self.cur_id = Some(tree._get_node(id)?.sibling_right?);
            return self.cur_id;
        }
        // First call of iterator, return first child
        else{
            self.cur_id = Some(tree._get_node(self.parent)?.first_child?);
            return self.cur_id;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_builder() {
        let node: TreeNode<u32, Guid> = TreeNode::new(42)
            .with_parent(Guid::from(43))
            .with_first_child(Guid::from(44))
            .with_sibling_left(Guid::from(45))
            .with_sibling_right(Guid::from(46));

        assert_eq!(node.data, 42);
        assert_eq!(node.parent, Some(Guid::from(43)));
        assert_eq!(node.first_child, Some(Guid::from(44)));
        assert_eq!(node.sibling_left, Some(Guid::from(45)));
        assert_eq!(node.sibling_right, Some(Guid::from(46)));
    }

    #[test]
    fn test_basic() {
        let mut gen = GuidManager::new();
        let mut cont: TreeContainer<i64, Guid> = TreeContainer::new();
        let id = gen.get();
        cont.add_root(32412345, id);
        assert_eq!(*cont.get(id).unwrap(), 32412345);

        if let Some(mut n) = cont.get_mut(id) {
            *n = 123456;
        }
        assert_eq!(*cont.get(id).unwrap(), 123456);
    }

    #[test]
    fn add_first_child() {
        let mut gen = GuidManager::new();
        let mut cont: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = cont.add_root(32412345, gen.get());

        assert_eq!(*cont.get(root_id).unwrap(), 32412345);

        let child_id = cont.add_child(root_id, 0, 834576098, gen.get()).unwrap();
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
        let mut gen = GuidManager::new();
        let mut cont: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = cont.add_root(32412345, gen.get());
        let child1_id = cont.add_child(root_id, 0, 834576098, gen.get()).unwrap();


        let child2_id = cont.add_child(root_id, 1, 908720349875, gen.get()).unwrap();
        assert_eq!(*cont.get(child2_id).unwrap(), 908720349875);
        // assumptions about the internal API
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 1));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, None);

        assert_eq!(cont._get_node(child2_id).unwrap().parent, Some(root_id));


        let child3_id = cont.add_child(root_id, 1, 304958, gen.get()).unwrap();
        assert_eq!(*cont.get(child3_id).unwrap(), 304958);
        // assumptions about the internal API
        assert_eq!(Some(child3_id), cont._nth_child_id(root_id, 1));
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 2));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, Some(child2_id));
        assert_eq!(cont._get_node(child3_id).unwrap().parent, Some(root_id));

        let child4_id = cont.add_child(root_id, 0, 452579, gen.get()).unwrap();
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
        let mut gen = GuidManager::new();
        let mut cont: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = cont.add_root(32412345, gen.get());
        let child1_id = cont.push_child(root_id, 834576098, gen.get()).unwrap();
        assert_eq!(*cont.get(child1_id).unwrap(), 834576098);

        // assumptions about the internal API
        assert_eq!(Some(child1_id), cont._first_child_id(root_id));
        assert_eq!(Some(child1_id), cont._last_child_id(root_id));
        assert_eq!(Some(child1_id), cont._nth_child_id(root_id, 0));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_left, None);
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child1_id).unwrap().parent, Some(root_id));


        let child2_id = cont.push_child(root_id, 908720349875, gen.get()).unwrap();
        assert_eq!(*cont.get(child2_id).unwrap(), 908720349875);
        // assumptions about the internal API
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 1));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child2_id).unwrap().parent, Some(root_id));
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_right, Some(child2_id));


        let child3_id = cont.push_child(root_id, 304958, gen.get()).unwrap();
        assert_eq!(*cont.get(child3_id).unwrap(), 304958);
        // assumptions about the internal API
        assert_eq!(Some(child3_id), cont._nth_child_id(root_id, 2));
        // assumptions about the data structure
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_left, Some(child2_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, None);
        assert_eq!(cont._get_node(child3_id).unwrap().parent, Some(root_id));

        let child4_id = cont.push_child(root_id, 452579, gen.get()).unwrap();
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
        let mut gen = GuidManager::new();
        let mut cont: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = cont.add_root(32412345, gen.get());
        let id1 = cont.add_child(root_id, 0, 65874567, gen.get()).unwrap();
        let id2 = cont.add_child(root_id, 1, 267869890, gen.get()).unwrap();
        let id3 = cont.add_child(id2, 0, 3454, gen.get()).unwrap();

        assert_eq!(cont.id_by_path("0"), Some(root_id));
        assert_eq!(cont.id_by_path("0:0"), Some(id1));
        assert_eq!(cont.id_by_path("0:1"), Some(id2));
        assert_eq!(cont.id_by_path("0:1:0"), Some(id3));

        assert_eq!(cont.id_by_path("1"), None);
        assert_eq!(cont.id_by_path("0:1:1"), None);
        assert_eq!(cont.id_by_path("0:2"), None);
    }



    #[test]
    fn children_iterator() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        let id1 = tree.push_child(root_id, 1, gen.get()).unwrap();
        let id2 = tree.push_child(root_id, 2, gen.get()).unwrap();
        let id3 = tree.push_child(root_id, 3, gen.get()).unwrap();
        let id4 = tree.push_child(root_id, 4, gen.get()).unwrap();
        
        let mut iterator =  tree.children_iter(root_id);

        assert_eq!(Some((id1, &1)), iterator.next());
        assert_eq!(Some((id2, &2)), iterator.next());
        assert_eq!(Some((id3, &3)), iterator.next());
        assert_eq!(Some((id4, &4)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn children_iterator2() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        
        let mut iterator =  tree.children_iter(root_id);
        assert_eq!(None, iterator.next());
    
    }

    #[test]
    fn children_iterator3() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        
        let mut sum = 0;
        for (_,val) in tree.children_iter(root_id){
            sum +=val;
        }

        assert_eq!(5, sum);
    }


    #[test]
    fn children_walk() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        let id1 = tree.push_child(root_id, 1, gen.get()).unwrap();
        let id2 = tree.push_child(root_id, 2, gen.get()).unwrap();
        let id3 = tree.push_child(root_id, 3, gen.get()).unwrap();
        let id4 = tree.push_child(root_id, 4, gen.get()).unwrap();
        
        let mut walk =  ChildrenIdWalk::new(root_id);

        assert_eq!(Some(id1), walk.next(&tree));
        assert_eq!(Some(id2), walk.next(&tree));
        assert_eq!(Some(id3), walk.next(&tree));
        assert_eq!(Some(id4), walk.next(&tree));
        assert_eq!(None, walk.next(&tree));
    }

    #[test]
    fn children_walk_ownership() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        let id1 = tree.push_child(root_id, 1, gen.get()).unwrap();
        let id2 = tree.push_child(root_id, 2, gen.get()).unwrap();
        let id3 = tree.push_child(root_id, 3, gen.get()).unwrap();
        let id4 = tree.push_child(root_id, 4, gen.get()).unwrap();
        
        let mut walk =  ChildrenIdWalk::new(root_id);

        while let Some(id) = walk.next(&tree){
            let e = tree.get_mut(id).unwrap();
            *e=51;
        }

        assert_eq!(*tree.get(id1).unwrap(), 51);
        assert_eq!(*tree.get(id2).unwrap(), 51);
        assert_eq!(*tree.get(id3).unwrap(), 51);
        assert_eq!(*tree.get(id4).unwrap(), 51);

    }
 

    #[test]
    fn depth_iterator() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(1, gen.get());
        let id1 = tree.push_child(root_id, 2, gen.get()).unwrap();
        let id11 = tree.push_child(id1,    3, gen.get()).unwrap();
        let id12 = tree.push_child(id1,    4, gen.get()).unwrap();
        let id2 = tree.push_child(root_id, 5, gen.get()).unwrap();
        let id21 = tree.push_child(id2,    6, gen.get()).unwrap();
        let id211 = tree.push_child(id21,  7, gen.get()).unwrap();
        let id22 = tree.push_child(id2,    8, gen.get()).unwrap();
        let id221 = tree.push_child(id22,  9, gen.get()).unwrap();
        let id3 = tree.push_child(root_id, 10, gen.get()).unwrap();
        let id4 = tree.push_child(root_id, 11, gen.get()).unwrap();
        
        let mut iterator =  tree.depth_iter();

        assert_eq!(Some((root_id,&1)), iterator.next());
        assert_eq!(Some((id1,    &2)), iterator.next());
        assert_eq!(Some((id11,   &3)), iterator.next());
        assert_eq!(Some((id12,   &4)), iterator.next());
        assert_eq!(Some((id2,    &5)), iterator.next());
        assert_eq!(Some((id21,   &6)), iterator.next());
        assert_eq!(Some((id211,  &7)), iterator.next());
        assert_eq!(Some((id22,   &8)), iterator.next());
        assert_eq!(Some((id221,  &9)), iterator.next());
        assert_eq!(Some((id3,   &10)), iterator.next());
        assert_eq!(Some((id4,   &11)), iterator.next());
        assert_eq!(None, iterator.next());
    }


    #[test]
    fn children_count() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());
        tree.push_child(root_id, 1, gen.get());

        assert_eq!(5, tree.len(root_id));
    }

    #[test]
    fn children_count2() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());

        assert_eq!(0, tree.len(root_id));
    }


    #[test]
    #[should_panic]
    fn double_insert1() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        let root_id = tree.add_root(42, root_id);
    }


    #[test]
    #[should_panic]
    fn double_insert2() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let root_id = tree.add_root(42, gen.get());
        tree.push_child(root_id, 1, root_id);
    }

    #[test]
    fn get_nonexisting() {
        let mut gen = GuidManager::new();
        let mut tree: TreeContainer<i64, Guid> = TreeContainer::new();
        let res_id = gen.get();
        let root_id = tree.add_root(42, gen.get());
        tree.push_child(root_id, 1, gen.get());
        assert_eq!(tree.get(res_id), None);
    }

}
