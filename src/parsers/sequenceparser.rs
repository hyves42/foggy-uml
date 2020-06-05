use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use parsers::datatypes::{ElementType, Element, Document, Parser, ParserResult};
use parsers::stringparser::StringParser;
use datatypes::{SliceWithContext};
use parseutils::*;



//elements: 
// 1 array of actors, in their display order, eventually inside a box element.
// Even actors that are created during the sequence are in this array, in that case they haave an attribute
//
// 1 array of 'things that happen' during the sequence:
// - message passing/sending
// - activation/deactivation
// - alt
// - object creation/deletion
// - notes
// - state changes
// - vertical sepration

static reserved_tokens_header: [&'static str;12] = [
    //actors definitions in header
    "participant",
    "actor", 
    "boundary", 
    "control", 
    "entity", 
    "database", 
    "collections",
    //other stuff
    "box",
    "title",
    "end",
    "hide",
    "show",
    ];

static reserved_tokens_sequence: [&'static str;20] = [
    "alt",
    "else",
    "group",
    "loop",
    "end",
    "ref",
    "note",
    "rnote", 
    "hnote",
    "activate",
    "destroy",
    "deactivate",
    "deactivate",
    "return",
    "...",
    "|||",
    "||",
    "==",
    "[",
    "{"
];

#[derive(PartialEq)]
enum SequenceDiagramParserState{
    Header,
    Content
}

struct SequenceDiagramParser {
    // a string buffer to collect text for the element being parsed
    collec: Option<String>,
    // full tree structure of document
    title: Option<Element>,
    header: Vec<Rc<RefCell<Element>>>, //header root element
    sequence: Option<Element>,
    state: SequenceDiagramParserState,
    open_header_tokens: Vec<Rc<RefCell<Element>>>,
}


impl SequenceDiagramParser {
    fn new() -> SequenceDiagramParser {
        SequenceDiagramParser {
            collec: Some(String::new()),
            title:None,
            header:vec![],
            sequence: None,
            state: SequenceDiagramParserState::Header,
            open_header_tokens: vec![],
        }
    }

    fn add_participant<'a>(&mut self, input: &str, def_token: &str)
        -> Result<(), String>{

        let mut slice=input;
        let mut name_element: Option<Rc<RefCell<Element>>>=None;
        let mut alias_name: Option<String>=None;        

        //expected : space name [space ['as' alias]]
        // ! statement has to be 1-line, no support for multi-line strings (yet)
        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            if spaces.len()==0{
                return Err(String::from("Expecting spaces"));
            }
            slice = new_slice;
        }



        {
            let res=Self::consume_participant_name(slice);
            match res{
                Ok((new_slice, ptr)) => {
                    name_element = Some(Rc::clone(&ptr));
                    slice = new_slice;
                }
                Err(s) => return Err(s)
            }
        }

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            if spaces.len()==0 && new_slice.len() > 0{
                return Err(String::from("Expecting spaces before 'as' token"));
            }
            slice = new_slice;
        }

        // Handle 'as alias' part
        if slice.len()>0{
            let res = consume_token_in_list(slice, &["as"]);
            if res.is_err() {
                return Err(String::from("Unexpected token"));
            }
            else{
                let (remaining, _)=res.unwrap(); 
                slice = remaining;
            }

            {
                let (new_slice, spaces) = consume_whitespaces(slice);
                if spaces.len()==0 && new_slice.len() > 0{
                    return Err(String::from("Expecting spaces after 'as' token"));
                }
                slice = new_slice;
            } 

            {
                let (remaining, alias) =consume_until_whitespace(slice).unwrap();
                slice = remaining;
                if alias.len()==0{
                    return Err(String::from("Expecting value for alias name"));
                }
                alias_name=Some(String::from(alias));
            } 
        }

        let mut header_element = Element::new();
        header_element.attributes.push((String::from("type"), String::from(def_token)));
        header_element.children.push(name_element.take().unwrap());

        match alias_name{
            Some(n) => header_element.attributes.push((String::from("alias"), String::from(n))),
            None => {
                if let Some(name) = header_element.children.first(){
                    header_element.attributes.push((String::from("alias"), 
                        name.borrow().value.clone()));
                }   
            } 

        }

        self.header.push(Rc::new(RefCell::new(header_element)));
        return Ok(());
    }

    // TODO doesn't handle markdown formatters in name string
    fn  consume_participant_name (input: &str)->Result<(&str, Rc<RefCell<Element>>), String>{
        if StringParser::is_start_word(&input){
            // Read name as a string

            // TODO Not very happy about his kludge. It works but ... meh
            // For some reason the borrow checker is particularly unhappy if I feed input 
            // directly to the stringparser, I can't get it back after that
            // workaround: extract just the good slice and fake the slicewithcontext information...
            let (remaining, start_token) = consume_token_in_list(input, StringParser::get_start_words()).unwrap();
            let escaped_token=format!("\\{}", start_token);
            let stop_tokens=[escaped_token.as_str(), start_token]; //order is important
            let mut collector = String::from(start_token);
            let mut new_slice = remaining;
            let mut slice_to_return=input;
            loop{
                {
                    let (rem, consumed) = consume_until_token_in_list(new_slice, &stop_tokens).unwrap();
                    new_slice=rem;
                    collector.push_str(consumed);
                }
                {
                    let (rem, consumed) = consume_token_in_list(new_slice, &stop_tokens).unwrap();
                    new_slice=rem;
                    collector.push_str(consumed);
                    if (consumed==start_token){
                        slice_to_return = rem;
                        break;
                    }
                }
            }

            // Now 'collector' is a string that contains exactly the string to parse, with start and stop tokens
            let mut parser = StringParser::new();
            let mut slice = SliceWithContext{
                slice: collector.as_str(),
                line: 0,
                pos: 0,
                file_name: Rc::new(String::new()),
            };
            let mut new_slice:&str=&"";

            let res = parser.step(&mut slice);
            match res{
                Err((_, s))=> return Err(s),
                Ok(r) => match r {
                    ParserResult::Done(slice_ctx) => {
                        new_slice = slice_ctx.slice;
                    },
                    _ => return Err(String::from("unfinished string")),
                }
            }
            let (elements, documents) = parser.flush();
            return Ok((slice_to_return, Rc::clone(elements.first().unwrap())));

        }
        else{
            // read participant name as a simple slice
            if let Ok((remaining, consumed))=consume_until_whitespace(input){
                let element = Element::new_str(consumed);
                return Ok((remaining, Rc::new(RefCell::new(element))));
            }
        }
        // should not be reached
        return Err(String::from("unhandled"))
    }

    fn push_to_header(&mut self, element:Rc<RefCell<Element>>){
        match self.open_header_tokens.last() {
            Some(ptr) => ptr.borrow_mut().children.push(element),
            None => self.header.push(element),
        }
    }
}

impl Parser for SequenceDiagramParser{
    fn step<'b>(
        &mut self,
        input: &'b mut SliceWithContext<'b>,
    ) -> Result<ParserResult<'b>, (&'b SliceWithContext<'b>, String)> {
        let mut slice: &str = input.slice;

        // Shall not happen
        if self.collec == None {
            return Err((input, String::from("runtime error, collector shall not be none")));
        }

        // header line starts with keyword
        if self.state == SequenceDiagramParserState::Header{
            // Check for tokens that are always at the start of line in the header
            if  let Ok((new_slice, token)) = consume_token_in_list(slice, &reserved_tokens_header) {
            	input.slice = new_slice;
                match token {
                    "participant"|"actor"|"boundary"|"control"|"entity"|"database"|"collections" 
                        => {self.add_participant(input.slice, token);},
                    "box" => return Err((input, String::from("runtime error, invalid condition"))),
                    "title" => return Err((input, String::from("runtime error, invalid condition"))),
                    "end" => return Err((input, String::from("runtime error, invalid condition"))),
                    "hide" => return Err((input, String::from("runtime error, invalid condition"))),
                    "show" => return Err((input, String::from("runtime error, invalid condition"))),
                    _ => return Err((input, String::from("runtime error, invalid condition"))),
                }

            }
            else {
                self.state = SequenceDiagramParserState::Content;
            }
        }
        // content line starts with keyword
        if  let Ok((slice, token)) = consume_token_in_list(slice, &reserved_tokens_sequence) {
            match token {

                _ => return Err((input, String::from("runtime error, invalid condition"))),
            }
            let (slice, unused) = consume_whitespaces(slice);
        }

        //while slice.len() > 0 {
            // let stop_tokens = ["**", "*", "~~"];

            // let (mut new_slice, mut consumed) =
            //     consume_until_token_in_list(slice, &stop_tokens).unwrap();


            // // If no existing container to store this text create a paragraph
            // if self.open_tokens.len()==0{
            //     self.push_paragraph();
            // }

            // self.collec.as_mut().unwrap().push_str(consumed);
            // slice = new_slice;

            // if new_slice.len() == 0 {
            //     break;
            // }

            // // consume the token that stopped us
            // let (new_slice, consumed) = consume_token_in_list(new_slice, &stop_tokens).unwrap();
            // // are we closing something this this token ?
            // let condition=MDCloseCondition::Token(consumed.to_string());
            // if self.is_close_condition(&condition){
            //     self.collect_all_open_tokens_until_cond(&condition);
            // }
            // else{
            //     match consumed{
            //         "**" => self.push_formatter("**", "bold"),
            //         "*" => self.push_formatter("*", "italic"),
            //         "~~" => self.push_formatter("~~", "strikethrough"),
            //         _ => return Err((
            //             input,
            //             String::from("runtime error, met unexpected token"),
            //         )) // no other token apart from those in stop_tokens array shall be consumed
            //     }
            // }
            // slice=new_slice;
        //}


        // Full line was consumed

        // check for this close condition
 
        return Ok(ParserResult::Partial(input));
    }

    fn flush(&mut self) -> (Vec<Rc<RefCell<Element>>>, Vec<Rc<RefCell<Document>>>){
        return (vec![], vec![]);
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use datatypes::{LineWithContext, SliceWithContext};
    use std::rc::Rc;

    #[test]
    fn test_sequenceparser_header1() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext {
            slice: &"participant bob",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
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
        
        ];
        assert_eq!(parser.header, expected);
    }


    #[test]
    fn test_sequenceparser_header2() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext {
            slice: &"participant bobby as bob",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("bobby"),
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
        ];
        assert_eq!(parser.header, expected);
    }

    #[test]
    fn test_sequenceparser_header3() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext {
            slice: &"participant \"bobby the 🐶\" as bob",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("bobby the 🐶"),
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
        ];
        assert_eq!(parser.header, expected);
    }

        #[test]
    fn test_sequenceparser_header4() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext {
            slice: &"actor \"bobby the 🐶\" as bob",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("bobby the 🐶"),
                        etype: ElementType::StringType,
                        children: vec![],
                        attributes: vec![],
                    }))
                ],
                attributes: vec![
                    (String::from("type"), String::from("actor")),
                    (String::from("alias"), String::from("bob")),
                ]
            }))        
        ];
        assert_eq!(parser.header, expected);
    }
}