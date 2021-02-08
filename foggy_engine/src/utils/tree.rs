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
        let idx = self.flat.len() -1;
        self.root = Some(idx);
        return idx;
    }


    //    x      x         x           x
    //       A-> |  B->   /      C->  /
    //           y       y--z        y--z--a
    // Add a child a the given index.
    // Panics if pos is > the number of children or if parent does not exist
    pub fn add_child(&mut self, parent:usize, pos:usize, t: T) -> usize {
        if !self._has_child(parent){ //A
            if pos>0{
                panic!();
            }
            // Insert the first child
            let node = TreeNode::new(t).parent(parent);
            let new_id = self.flat.len();
            if let Some(p) = self.flat.get_mut(parent).unwrap(){
                p.first_child = Some(new_id);
                self.flat.push(Some(node));
            }
            else{
                panic!();
            }
            return new_id;
        }
        else{
            let first = self._get_first_child(parent);

        }
        return 0;
    }

    // Push a child at the last available position
    pub fn push_child(&mut self, parent:usize, t: T) -> usize {
        return 0;
    }

    // Prepend a sibling node
    // Does not work on root
    pub fn prepend(&mut self, sibling:usize, t: T) -> usize {
        return 0;
    }

    // Append a sibling node
    // Does not work on root
    pub fn append(&mut self, sibling:usize, t: T) -> usize {
        return 0;
    }


    pub fn get(&self, id:usize) -> Option<&T> {
        let c = self.flat.get(id)?;
        match c{
            None => None,
            Some(n) => Some(&n.data)
        }
    }

    pub fn get_mut(&mut self, id:usize) -> Option<&mut T> {
        let c = self.flat.get_mut(id)?;
        match c{
            None => None,
            Some(n) => Some(&mut n.data)
        }
    }

    // panics if id does not exist
    fn _has_child(&mut self, id:usize) -> bool{
        if let Some(n) = self.flat.get_mut(id).unwrap(){
            return n.first_child.is_some();
        }
        else{
            panic!();
        }
    }

    // panics if id does not exist
    fn _get_first_child(&mut self, id:usize) -> usize{
        if let Some(n) = self.flat.get_mut(id).unwrap(){
            return n.first_child.unwrap();
        }
        else{
            panic!();
        }
    }



}

//     // path is a string of form "0:12:4:1..."
//     pub fn by_path(&self, path: &str) -> Result<Rcc<TreeLayoutElement>, &str> {
//         let mut iter = path.split(':').map(|i| usize::from_str_radix(i, 10));

//         let mut cursor = Rc::clone(&self.root);

//         while let Some(fragment) = iter.next() {
//             match fragment {
//                 Err(_) => return Err("Invalid path format"),
//                 Ok(i) => {
//                     let tmp = if let Some(c) = cursor.borrow().children.get(i) {
//                         Rc::clone(&c)
//                     } else {
//                         return Err("Elt doesn't exist");
//                     };
//                     cursor = tmp;
//                 }
//             }
//         }
//         return Ok(cursor);
//     }

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
        let node :TreeNode<u32> = TreeNode::new(42)
            .parent(43)
            .first_child(44)
            .sibling_left(45)
            .sibling_right(46);

        assert_eq!(node.data,          42); 
        assert_eq!(node.parent,        Some(43)); 
        assert_eq!(node.first_child,   Some(44)); 
        assert_eq!(node.sibling_left,  Some(45)); 
        assert_eq!(node.sibling_right, Some(46)); 


    }

    #[test]
    fn test_basic() {
        let mut cont : TreeContainer<i64> = TreeContainer::new();
        let id = cont.add_root(32412345);
        assert_eq!(*cont.get(id).unwrap(), 32412345); 

        if let Some(mut n) = cont.get_mut(id){
            *n = 123456;
        }
        assert_eq!(*cont.get(id).unwrap(), 123456); 
    }


    #[test]
    fn add_children() {
        let mut cont : TreeContainer<i64> = TreeContainer::new();
        let id = cont.add_root(32412345);

        assert_eq!(*cont.get(id).unwrap(), 32412345); 

        let child_id = cont.add_child(id, 0, 834576098);
        assert_eq!(*cont.get(child_id).unwrap(), 834576098); 
    }




    // #[test]
    // fn test_path() {
    //     let cont = TreeContainer::new(Direction::Horizontal);
    //     let mut elt1 = TreeLayoutElement::new(Direction::Horizontal);
    //     let mut elt2 = TreeLayoutElement::new(Direction::Horizontal);
    //     let mut elt3 = TreeLayoutElement::new(Direction::Horizontal);

    //     elt1.id = 42;
    //     elt2.id = 44;
    //     elt3.id = 444;
    //     elt2.children.push(rcc(elt3));
    //     cont.root.borrow_mut().children.push(rcc(elt1));
    //     cont.root.borrow_mut().children.push(rcc(elt2));
    //     assert!(cont.by_path("0").is_ok());
    //     assert_eq!(cont.by_path("0").unwrap().borrow().id, 42);
    //     assert!(cont.by_path("1").is_ok());
    //     assert_eq!(cont.by_path("1").unwrap().borrow().id, 44);
    //     assert!(cont.by_path("1:0").is_ok());
    //     assert_eq!(cont.by_path("1:0").unwrap().borrow().id, 444);

    //     assert!(cont.by_path("0:0").is_err());
    //     assert!(cont.by_path("1:1").is_err());

    // }
}
