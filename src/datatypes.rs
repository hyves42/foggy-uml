use std::rc::Rc;
use std::cell::{RefCell};


#[derive(Debug)]
pub struct LineWithContext {
    pub text: String,
    pub line: u32,
    pub file_name: Rc<String>,
    pub namespace: Rc<String>,
}

#[derive(Debug)]
pub struct SliceWithContext<'a> {
    pub slice: &'a str,
    pub line: u32,
    pub pos: u32,
    pub file_name: Rc<String>,
}



impl<'a> SliceWithContext<'a>{
    #[cfg(test)]
    pub fn new_for_tests(text: &'static str)->SliceWithContext{
        SliceWithContext {
            slice: text,
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        }
    }

}



#[derive(Debug)]
#[derive(PartialEq)]
pub enum ElementType {
    StringType,    // element contains text
    StructureType, // just tree structure
}

// Not completely convinced. about this
// Something with an enum and different fields for each value enum could be better
// And I would like this to be expandable without defining all possible fields for every possible use cases/diagram
// todo: think about it after I get some sleep
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Element {
    pub value: String, // in case of string type, the text content. otherwise, an ID? an element name ?
    pub etype: ElementType,
    pub children: Vec<Rc<RefCell<Element>>>,
    pub attributes: Vec<(String, String)>,
}

impl Element {
    pub fn new() -> Element{
        Element {
            value: String::new(),
            etype: ElementType::StructureType,
            children:vec![],
            attributes:vec![],
        }
    }

    pub fn new_str(text: &str) -> Element{
        Element {
            value: String::from(text),
            etype: ElementType::StringType,
            children:vec![],
            attributes:vec![],
        }
    }
    pub fn new_string(text: String) -> Element{
        Element {
            value: text,
            etype: ElementType::StringType,
            children:vec![],
            attributes:vec![],
        }
    } 
}

#[derive(Debug)]
pub struct Document {
    pub children: Vec<Element>,
}

// We use this structure so that we don't have to pass the closure
// as a parameter of the recursive function.
// This seems less confusing for the borrow checker
struct KludgeFn<'s>{
    f: &'s mut FnMut(&Element, usize)
}

// Depth-first tree traversal
pub fn recurse_element_tree<F>(elt: Rc<RefCell<Element>>, mut f: F)
where 
    F: FnMut(&Element, usize) 
{
    let mut k = KludgeFn{f:&mut f};
    (k.f)(&elt.borrow(), 0);
    return _recurse_tree(elt, &mut k, 1);
}



fn _recurse_tree(elt: Rc<RefCell<Element>>, k: & mut KludgeFn, depth:usize)
{
    // Hardcoded max depth. Ought to be enough for anyone.
    if depth > 50{
        return;
    }
    let element=elt.borrow();
    let mut iter=element.children.iter();
    while let Some(e) = iter.next(){
        (k.f)(&e.borrow(), depth);
        _recurse_tree(Rc::clone(e), k, depth+1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        let tree= Rc::new(RefCell::new(Element{
            value: String::new(),
            etype: ElementType::StructureType,
            children: vec![
                Rc::new(RefCell::new(Element{
                    value: String::new(),
                    etype: ElementType::StructureType,
                    children: vec![
                        Rc::new(RefCell::new(Element{
                            value: String::from("alice"),
                            etype: ElementType::StringType,
                            children: vec![],
                            attributes: vec![],
                        }))
                    ],
                    attributes: vec![
                        (String::from("type"), String::from("participant")),
                        (String::from("alias"), String::from("alice")),
                    ]
                })),
                Rc::new(RefCell::new(Element{
                    value: String::new(),
                    etype: ElementType::StructureType,
                    children: vec![
                        Rc::new(RefCell::new(Element{
                            value: String::from("bob"),
                            etype: ElementType::StringType,
                            children: vec![],
                            attributes: vec![],
                        }))
                    ],
                    attributes: vec![
                        (String::from("type"), String::from("participant")),
                        (String::from("alias"), String::from("bob")),
                    ]
                }))
            ],
            attributes: vec![(String::from("type"), String::from("sequencediagram:header"))]
        }));
            
        recurse_element_tree(Rc::clone(&tree), |e, d|{println!("{}:{:?}", d, e)});
        let mut str_count=0;
        let mut struct_count=0;
        recurse_element_tree(Rc::clone(&tree), 
            |e, d|{
                match e.etype{
                    ElementType::StringType => str_count+=1,
                    ElementType::StructureType => struct_count+=1,
                }
        });
        assert_eq!(str_count, 2);
        assert_eq!(struct_count, 3);
        println!("string {:?}", str_count);
        println!("struct {:?}", struct_count);
    }
}
