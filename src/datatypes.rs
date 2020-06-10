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

