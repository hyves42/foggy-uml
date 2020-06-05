use std::rc::Rc;
use std::cell::{RefCell};
use datatypes::{SliceWithContext};



#[derive(Debug)]
#[derive(PartialEq)]
pub enum ElementType {
    StringType,    // element contains text
    //TokenType,     // for code struct ??
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
}

#[derive(Debug)]
pub struct Document {
    pub children: Vec<Element>,
}




pub enum ParserResult<'a> {
    Busy,                              // then feed with new line
    Partial(&'a SliceWithContext<'a>), // reached valid end, but no explicit ending. compositor will try to feed next line
    // return valid elements becsuse we don't know, maybe the combinator will reach an end token and wi8ll kill us
    Done(&'a SliceWithContext<'a>), // reached stop token. compositor will not feed next line
                                    //Error((&'a SliceWithContext<'a>, String)), // data doesn't match expected structure. If returned after a Partial, it just mneans that the previous partiel was the end of block to parse. usefule for blocks with no explicit end

                                    //Yield(&'a SliceWithContext<'a>, &'b dyn Parser) // during parsing, encountered something that should be handled by an other parser enrirely. not sure if it's a good idea. I guess the combinator shall instanciate the parser
}

pub trait Parser {
	// feed the parser line by line
    fn step<'a>(
        &mut self,
        input: &'a mut SliceWithContext<'a>,
    ) -> Result<ParserResult<'a>, (&'a SliceWithContext<'a>, String)>;
    // flush must be called only after all the data has been fed to the parser
    // -> parser returned Done
    // -> combinator found that the context of this parser is finished (end token or start token of an other parser)
    // After flush is called, I consider that the parser cannot be used anymore
    fn flush(&mut self) -> (Vec<Rc<RefCell<Element>>>, Vec<Rc<RefCell<Document>>>);
}
