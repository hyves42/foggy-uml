// A tree structure allocated in the same contiguous memory region
// allocated memory can only grow, there is no freeing mechanism

#[derive(Debug, PartialEq, Default)]
pub struct TreeNode<T> {
    pub parent: Option<usize>,
    pub first_child: Option<usize>,
    pub sibling_left: Option<usize>,
    pub sibling_right: Option<usize>,
    pub data: T,
}

#[derive(Debug, PartialEq, Default)]
pub struct TreeContainer<T> {
    // nodes organized as a tree
    pub root: Option<usize>,
    // Flat list of all the nodes
    // It is allowed to have elements in this flat list that are not present in
    pub flat: Vec<Option<TreeNode<T>>>,
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

    pub fn parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        return self;
    }

    pub fn first_child(mut self, child: usize) -> Self {
        self.first_child = Some(child);
        return self;
    }

    pub fn sibling_left(mut self, left: usize) -> Self {
        self.sibling_left = Some(left);
        return self;
    }

    pub fn sibling_right(mut self, right: usize) -> Self {
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

    pub fn add_root(&mut self, t: T) -> usize {
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
    pub fn add_child(&mut self, parent: usize, pos: usize, t: T) -> Result<usize,&str> {
        if !self._has_child(parent) {
            // case A
            if pos > 0 {
                panic!();
            }
            // Insert the first child
            let node = TreeNode::new(t).parent(parent);
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
                let mut node = TreeNode::new(t).parent(parent).sibling_left(left_id);
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
                let node = TreeNode::new(t).parent(parent).sibling_right(right_id);

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
    pub fn push_child(&mut self, parent: usize, t: T) -> usize {
        return 0;
    }

    // Prepend a sibling node
    // Does not work on root
    pub fn prepend(&mut self, sibling: usize, t: T) -> usize {
        return 0;
    }

    // Append a sibling node
    // Does not work on root
    pub fn append(&mut self, sibling: usize, t: T) -> usize {
        return 0;
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        let c = self.flat.get(id)?;
        match c {
            None => None,
            Some(n) => Some(&n.data),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
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
    pub fn id_by_path(&self, path: &str) -> Option<usize> {
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

    // path is a string of form "0:12:4:1..."
    pub fn by_path(&self, path: &str) -> Option<&T> {
        let id = self.id_by_path(path)?;
        return self.get(id);
    }

    pub fn by_path_mut(&mut self, path: &str) -> Option<&mut T> {
        let id = self.id_by_path(path)?;
        return self.get_mut(id);
    }

    fn _get_node(&self, id: usize) -> Option<&TreeNode<T>> {
        let c = self.flat.get(id)?;
        match c {
            None => None,
            Some(n) => Some(&n),
        }
    }


    // panics if id does not exist
    fn _has_child(&self, id: usize) -> bool {
        if let Some(n) = self.flat.get(id).unwrap() {
            return n.first_child.is_some();
        } else {
            panic!();
        }
    }

    fn _first_child_id(&self, id: usize) -> Option<usize> {
        if let Some(n) = self.flat.get(id)? {
            n.first_child
        } else {
            None
        }
    }

    fn _nth_child_id(&self, id: usize, offset: usize) -> Option<usize> {
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

    fn _last_child_id(&self, id: usize) -> Option<usize> {
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



//     pub fn by_id(&self, id: u64) -> Result<Rcc<TreeLayoutElement>, &str> {
//         return Err("");
//     }

//     // // insert new element as a sibling of path
//     // horizontalInsert(element, path){
//     //   // find parent
//     //   const fragments = path.split(':');
//     //   if (fragments.length<2) return false;
//     //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

//     //   let parent = this.byPath(parentPath);
//     //   if (parent == null) return false;

//     //   //insert at the right place in parent array
//     //   let index = parseInt(fragments[fragments.length-1], 10);
//     //   //index can be equal to length if I want to insert at the end of array
//     //   if (index == NaN || index<0 || index > parent.children.length){
//     //     return false;
//     //   }
//     //   parent.children.splice(index, 0, element);
//     //   return true;
//     // }

//     // // insert new element as a parent of path
//     // verticalInsert(element, path){
//     //   // special case for root which has no parent
//     //   if (path == '0'){
//     //     return this._verticalInsertRoot(element);
//     //   }
//     //   // find parent
//     //   const fragments = path.split(':');
//     //   if (fragments.length<2) return false;
//     //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

//     //   let parent = this.byPath(parentPath);
//     //   let original = this.byPath(path);
//     //   if (parent == null || original == null || element == null) return false;

//     //   //insert at the right place in parent array
//     //   let index = parseInt(fragments[fragments.length-1], 10);
//     //   if (index == NaN || index<0 || index >= parent.children.length){
//     //     return false;
//     //   }
//     //   element.children.push(original);
//     //   parent.children.splice(index, 1, element);
//     //   return true;
//     // }

//     // _verticalInsertRoot(element){
//     //   if (element == null) return false;
//     //   element.children.push(this.root);
//     //   this.root = element;
//     //   return true;
//     // }

//     // remove(path){
//     //   // find parent
//     //   const fragments = path.split(':');
//     //   if (fragments.length<2) return false;
//     //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

//     //   let parent = this.byPath(parentPath);
//     //   let original = this.byPath(path);
//     //   if (parent == null || original == null) return false;

//     //   //delete at the right place in parent array
//     //   let index = parseInt(fragments[fragments.length-1], 10);
//     //   if (index == NaN || index<0 || index >= parent.children.length){
//     //     return false;
//     //   }
//     //   parent.children.splice(index, 1);
//     //   return original;
//     // }

//     // move(origin, target){
//     //   //todo check that destination exists
//     //   original = this.remove(origin);
//     //   if (original == false) return false;

//     //   return this.horizontalInsert(element, path);
//     // }

//     // newElement(){
//     //   return { children: [] };
//     // }

//     // _forEach(element, path, f){
//     //   f(element, path);
//     //   var i=0
//     //   element.children.forEach(e => {this._forEach(e, path+':'+i, f); i++});
//     // }

//     // // call function f(elt, path) on each element of the tree
//     // forEach(f) {
//     //   this._forEach(this.root, '0', f);
//     // }
// }

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

        assert_eq!(Some(child_id), cont._first_child_id(root_id));
        assert_eq!(Some(child_id), cont._last_child_id(root_id));
        assert_eq!(Some(child_id), cont._nth_child_id(root_id, 0));
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
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 1));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_right, None);

        assert_eq!(cont._get_node(child2_id).unwrap().parent, Some(root_id));


        let child3_id = cont.add_child(root_id, 1, 304958).unwrap();
        assert_eq!(*cont.get(child3_id).unwrap(), 304958);
        assert_eq!(Some(child3_id), cont._nth_child_id(root_id, 1));
        assert_eq!(Some(child2_id), cont._nth_child_id(root_id, 2));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_left, Some(child1_id));
        assert_eq!(cont._get_node(child2_id).unwrap().sibling_left, Some(child3_id));
        assert_eq!(cont._get_node(child3_id).unwrap().sibling_right, Some(child2_id));
        assert_eq!(cont._get_node(child3_id).unwrap().parent, Some(root_id));

        let child4_id = cont.add_child(root_id, 0, 452579).unwrap();
        assert_eq!(*cont.get(child4_id).unwrap(), 452579);
        assert_eq!(Some(child1_id), cont._nth_child_id(root_id, 1));
        assert_eq!(Some(child4_id), cont._nth_child_id(root_id, 0));
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_left, None);
        assert_eq!(cont._get_node(child1_id).unwrap().sibling_left, Some(child4_id));
        assert_eq!(cont._get_node(child4_id).unwrap().sibling_right, Some(child1_id));
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
