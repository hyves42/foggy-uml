use crate::datatypes::*;
use crate::layout::datatypes::*;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

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

impl Direction{
	pub fn orthogonal(&self) -> Direction{
		match self{
		    Direction::Vertical => Direction::Horizontal,
		    Direction::Horizontal => Direction::Vertical
		}
	}
}

#[derive(Debug, PartialEq, Default)]
pub struct TreeLayoutElement {
    pub children: Vec<Rcc<TreeLayoutElement>>,
    pub constraint: BoxConstraints,
    pub dimensions: BoxContainer,
    pub direction: Direction,
    pub id: i64,
}

#[derive(Debug, PartialEq, Default)]
pub struct TreeContainer {
    pub root: Rcc<TreeLayoutElement>,
}

impl TreeLayoutElement {
    pub fn new(direction: Direction) -> TreeLayoutElement {
        TreeLayoutElement {
            children: vec![],
            constraint: BoxConstraints::new(),
            dimensions: BoxContainer::new(),
            direction: direction,
            id: 0,
        }
    }

    pub fn push(&mut self, child: Rcc<TreeLayoutElement>) -> Rcc<TreeLayoutElement>{
        self.children.push(Rc::clone(&child));
        child.borrow_mut().set_direction(self.direction.orthogonal());
        child
    }

    pub fn set_direction(&mut self, direction: Direction){
    	self.direction = direction;

    	// Apply orthogonal direction to children recursively
    	let child_direction = direction.orthogonal();
    	for child in self.children.iter(){
    		child.borrow_mut().set_direction(child_direction);
    	}
    }

}

impl TreeContainer {
    pub fn new(direction: Direction) -> Self {
        TreeContainer {
            root: rcc(TreeLayoutElement::new(direction)),
        }
    }

    // path is a string of form "0:12:4:1..."
    pub fn by_path(&self, path: &str) -> Result<Rcc<TreeLayoutElement>, &str> {
        let mut iter = path.split(':').map(|i| usize::from_str_radix(i, 10));

        let mut cursor = Rc::clone(&self.root);

        while let Some(fragment) = iter.next() {
            match fragment {
                Err(_) => return Err("Invalid path format"),
                Ok(i) => {
                    let tmp = if let Some(c) = cursor.borrow().children.get(i) {
                        Rc::clone(&c)
                    } else {
                        return Err("Elt doesn't exist");
                    };
                    cursor = tmp;
                }
            }
        }
        return Ok(cursor);
    }

    // // insert new element as a sibling of path
    // horizontalInsert(element, path){
    //   // find parent
    //   const fragments = path.split(':');
    //   if (fragments.length<2) return false;
    //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

    //   let parent = this.byPath(parentPath);
    //   if (parent == null) return false;

    //   //insert at the right place in parent array
    //   let index = parseInt(fragments[fragments.length-1], 10);
    //   //index can be equal to length if I want to insert at the end of array
    //   if (index == NaN || index<0 || index > parent.children.length){
    //     return false;
    //   }
    //   parent.children.splice(index, 0, element);
    //   return true;
    // }

    // // insert new element as a parent of path
    // verticalInsert(element, path){
    //   // special case for root which has no parent
    //   if (path == '0'){
    //     return this._verticalInsertRoot(element);
    //   }
    //   // find parent
    //   const fragments = path.split(':');
    //   if (fragments.length<2) return false;
    //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

    //   let parent = this.byPath(parentPath);
    //   let original = this.byPath(path);
    //   if (parent == null || original == null || element == null) return false;

    //   //insert at the right place in parent array
    //   let index = parseInt(fragments[fragments.length-1], 10);
    //   if (index == NaN || index<0 || index >= parent.children.length){
    //     return false;
    //   }
    //   element.children.push(original);
    //   parent.children.splice(index, 1, element);
    //   return true;
    // }

    // _verticalInsertRoot(element){
    //   if (element == null) return false;
    //   element.children.push(this.root);
    //   this.root = element;
    //   return true;
    // }

    // remove(path){
    //   // find parent
    //   const fragments = path.split(':');
    //   if (fragments.length<2) return false;
    //   let parentPath = fragments.slice(0,fragments.length-1).join(':');

    //   let parent = this.byPath(parentPath);
    //   let original = this.byPath(path);
    //   if (parent == null || original == null) return false;

    //   //delete at the right place in parent array
    //   let index = parseInt(fragments[fragments.length-1], 10);
    //   if (index == NaN || index<0 || index >= parent.children.length){
    //     return false;
    //   }
    //   parent.children.splice(index, 1);
    //   return original;
    // }

    // move(origin, target){
    //   //todo check that destination exists
    //   original = this.remove(origin);
    //   if (original == false) return false;

    //   return this.horizontalInsert(element, path);
    // }

    // newElement(){
    //   return { children: [] };
    // }

    // _forEach(element, path, f){
    //   f(element, path);
    //   var i=0
    //   element.children.forEach(e => {this._forEach(e, path+':'+i, f); i++});
    // }

    // // call function f(elt, path) on each element of the tree
    // forEach(f) {
    //   this._forEach(this.root, '0', f);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let cont = TreeContainer::new(Direction::Horizontal);
        let mut elt1 = TreeLayoutElement::new(Direction::Horizontal);
        let mut elt2 = TreeLayoutElement::new(Direction::Horizontal);
        let mut elt3 = TreeLayoutElement::new(Direction::Horizontal);

        elt1.id = 42;
        elt2.id = 44;
        elt3.id = 444;
        elt2.children.push(rcc(elt3));
        cont.root.borrow_mut().children.push(rcc(elt1));
        cont.root.borrow_mut().children.push(rcc(elt2));
        assert!(cont.by_path("0").is_ok());
        assert_eq!(cont.by_path("0").unwrap().borrow().id, 42);
        assert!(cont.by_path("1").is_ok());
        assert_eq!(cont.by_path("1").unwrap().borrow().id, 44);
        assert!(cont.by_path("1:0").is_ok());
        assert_eq!(cont.by_path("1:0").unwrap().borrow().id, 444);


        assert!(cont.by_path("0:0").is_err());
        assert!(cont.by_path("1:1").is_err());

    }
}
