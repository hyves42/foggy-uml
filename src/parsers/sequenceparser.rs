use std::rc::Rc;
use std::cell::{RefCell};
use parsers::datatypes::{ElementType, Element, Document, Parser, ParserResult};
use datatypes::{SliceWithContext};
use parseutils::*;
use parsers::stringparseutils::*;
use std::collections::HashMap;

use maplit::hashmap;


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
    "end box",
    "title",
    "hide",
    "show",
    ];

static reserved_tokens_sequence: [&'static str;18] = [
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
];






#[derive(Copy, Clone, PartialEq)]
enum ArrowDirection{
    Left,
    Right,
    Bidirectional
}

#[derive(Copy, Clone, PartialEq)]
enum ArrowLineType{
    Normal,
    Dotted,
}

#[derive(Copy, Clone, PartialEq)]
enum ArrowType{
    Normal,
    Fine,
    Top,
    Bottom,
    TopFine,
    BottomFine,
    Bidirectional,
    BidirectionalFine
}

#[derive(Copy, Clone, PartialEq)]
enum ArrowDecor{
    Round,
    Cross
}




#[derive(PartialEq)]
enum SequenceDiagramParserState{
    Header,
    Content
}

#[derive(PartialEq)]
enum HDCloseCondition {
    EndBox, // Close on 'end box'
}


struct SequenceDiagramParser {
    // a string buffer to collect text for the element being parsed
    collec: Option<String>,
    // full tree structure of document
    title: Option<Element>,
    header: Vec<Rc<RefCell<Element>>>, //header root element
    sequence: Vec<Rc<RefCell<Element>>>,
    state: SequenceDiagramParserState,
    open_header_tokens: Vec<(Rc<RefCell<Element>>, HDCloseCondition)>,
}


impl SequenceDiagramParser {
    fn new() -> SequenceDiagramParser {
        SequenceDiagramParser {
            collec: Some(String::new()),
            title:None,
            header:vec![],
            sequence: vec![],
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


        { // parse participant name, store it in name_element
            let res=Self::consume_name(slice);
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

        // Build participant element ans push it to header
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

        self.push_to_header(Rc::new(RefCell::new(header_element)));
        return Ok(());
    }


    fn add_box<'a>(&mut self, input: &str)
        -> Result<(), String>{

        let mut slice=input;
        let mut name_element: Option<Rc<RefCell<Element>>>=None;
        let mut alias_name: Option<String>=None;

        //expected : space name [space] ['#' color]]

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            if spaces.len()==0{
                return Err(String::from("Expecting spaces"));
            }
            slice = new_slice;
        }


        { // parse box name, store it in name_element
            let res=Self::consume_name(slice);
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
            slice = new_slice;
        }
        // Handle optional '# color' part
        if slice.len()>0{
            // TODO
        }


        // Build box element ans push it to header
        let mut header_element = Element::new();
        header_element.attributes.push((String::from("type"), String::from("box")));
        header_element.children.push(name_element.take().unwrap());

        let ptr = Rc::new(RefCell::new(header_element));
        self.push_to_header(Rc::clone(&ptr));
        self.open_header_tokens.push((Rc::clone(&ptr), HDCloseCondition::EndBox));


        return Ok(());
    }

    fn end_box<'a>(&mut self)-> Result<(), String>{
        if !self.is_header_close_condition(&HDCloseCondition::EndBox){
            return Err(String::from("'end box' token not expected"));
        }

        while let Some((_, token)) = self.open_header_tokens.pop(){
            if token == HDCloseCondition::EndBox{
                return Ok(());
            }
        }
        // Should not be reached
        return Ok(());
    }

    // TODO doesn't handle markdown formatters in name string yet
    fn  consume_name (input: &str)->Result<(&str, Rc<RefCell<Element>>), String>{
        let string_tokens=["\"", "'"];
        if starts_with_token(input, &string_tokens){
            // Read participant name as a string
            match consume_between_tokens(input, &string_tokens){
                Err(_)=> return Err(String::from("unfinished string")),
                Ok((remaining, str_content, offset)) => {
                    let (_, string) = unescape_to_string(str_content);
                    let element = Element::new_string(string);
                    return Ok((remaining, Rc::new(RefCell::new(element))));
                }
            }
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

    fn add_message(&mut self, input:&str)-> Result<(), String>{
        let mut slice=input;
        let mut name_element: Option<Rc<RefCell<Element>>>=None;
        let mut left_name: Option<String>=None;
        let mut right_name: Option<String>=None;
        let mut arrow_direction:ArrowDirection = ArrowDirection::Right;
        let mut arrow_line_type:ArrowLineType = ArrowLineType::Normal;
        let mut arrow_type:ArrowType = ArrowType::Normal;
        let mut arrow_decor:Option<ArrowDecor> =None;
        let mut arrow_text:Option<String> =None;
        let mut arrow_id:Option<String> =None;

        // expected format :['{'+id+'}'] + [spaces] + left_name + [spaces] + arrow + [spaces] + name +[spaces] +[ ':' +[spaces]+message]
        //                 :['{'+id+'}'] + [spaces] + '['                  + arrow + [spaces] + name +[spaces] +[ ':' +[spaces]+message]
        //                 :['{'+id+'}'] + [spaces] + left_name + spaces   + arrow +            ']'  +[spaces] +[ ':' +[spaces]+message]

        // First look for optional {id}
        if let Ok((_,_)) = consume_token_in_list(slice, &["{"]){
            match consume_between_tokens(slice, &["{"]){
                Err(_)=> return Err(String::from("unfinished {id}")),
                Ok((remaining, content, offset)) => {
                    arrow_id=Some(String::from(content));
                    slice=remaining;
                }
            }
        }

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            slice = new_slice;    
        }


        // Then look for either '[' or left name
        if let Ok((remaining,token)) = consume_token_in_list(slice, &["["]){
            //no left_name to parse
            slice=remaining;
        }
        else if let Ok((_,_)) = consume_token_in_list(slice, &["'", "\""]){
            // parse left_name name from quote-delimited string
            match consume_between_tokens(slice, &["'", "\""]){
                Err(_)=> return Err(String::from("unfinished string")),
                Ok((remaining, str_content, offset)) => {
                    let (_, string) = unescape_to_string(str_content);
                    left_name = Some(string);
                    slice=remaining;
                }
            }
        }
        else{
            // parse name until whitespace or something that looks like an arrow
            match consume_until_token_in_list(slice, &[" ", "\t", "<", "\\", "/", "-"]) {
                Ok((remaining, parsed)) => {
                    slice=remaining;
                    left_name = Some(String::from(parsed));
                },
                Err(_) => {return Err(String::from("Bad format"));},
            }
        }

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            slice = new_slice;    
        }
        if (slice.len() ==0){
            return Err(String::from("expecting arrow"))
        }
        // parse arrow
        {
            let (remaining, direction, l_type, a_type, decor) = Self::consume_arrow(slice)?;
            slice=remaining;
            arrow_direction = direction;
            arrow_line_type = l_type;
            arrow_type = a_type;
            arrow_decor = decor;
        }

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            slice = new_slice;    
        }

        if slice.len()==0{
            return Err(String::from("expecting right name"))
        }


        // Then look for either ']' or right name
        if let Ok((remaining,token)) = consume_token_in_list(slice, &["]"]){
            //no left_name to parse
            slice=remaining;
        }
        else if let Ok((_,_)) = consume_token_in_list(slice, &["'", "\""]){
            // parse left_name name from quote-delimited string
            match consume_between_tokens(slice, &["'", "\""]){
                Err(_)=> return Err(String::from("unfinished string")),
                Ok((remaining, str_content, offset)) => {
                    let (_, string) = unescape_to_string(str_content);
                    right_name = Some(string);
                    slice=remaining;
                }
            }
        }
        else{
            // parse name until whitespace or something that looks a special token or the message start
            match consume_until_token_in_list(slice, &[" ", "\t", "+", "*", "!", ":"]) {
                Ok((remaining, parsed)) => {
                    slice=remaining;
                    right_name = Some(String::from(parsed));
                },
                Err(_) => {return Err(String::from("Bad format"));},
            }
        }

        {
            let (new_slice, spaces) = consume_whitespaces(slice);
            slice = new_slice;    
        }

        // Then handle optional magic sortcuts
        if let Ok((remaining, token)) = consume_token_in_list(slice, &["++", "**", "!!"]){
            return Err(String::from("Not implemented yet"));
        }

        // Look for ':' separator, followed by arrow text
        if let Ok((remaining, token)) = consume_token_in_list(slice, &[":"]){
            slice=remaining;
            {
                let (new_slice, spaces) = consume_whitespaces(slice);
                slice = new_slice;    
            }  

            // All the remaining slice is the arrow text
            if let Ok((_, _)) = consume_token_in_list(slice, &["'", "\""]){
                // Read message text as a string
                match consume_between_tokens(slice, &["'", "\""]){
                    Err(_)=> return Err(String::from("unfinished string")),
                    Ok((remaining, str_content, offset)) => {
                        let (_, string) = unescape_to_string(str_content);
                        arrow_text=Some(string);
                    }
                }
            }
            else{
                // Even 
                let (_, string) = unescape_to_string(slice);
                arrow_text=Some(string);
            }
        }


        //Now create the connector
        let mut element:Element =Element::new();
        if let Some(text) = arrow_text{
            element.children.push(Rc::new(RefCell::new(Element::new_string(text))));
        }

        if let Some(name)=left_name{
            self.create_participant_if_needed(name.as_str());
            match arrow_direction{
                ArrowDirection::Right | ArrowDirection::Bidirectional =>{
                    element.attributes.push((String::from("origin"), name));
                },
                ArrowDirection::Left => element.attributes.push((String::from("target"), name))
            }
        }

        if let Some(name)=right_name{
            self.create_participant_if_needed(name.as_str());
            match arrow_direction{
                ArrowDirection::Right | ArrowDirection::Bidirectional =>{
                    element.attributes.push((String::from("target"), name));
                },
                ArrowDirection::Left => element.attributes.push((String::from("origin"), name))
            }
        }
        element.attributes.push((String::from("line-style"), 
            match arrow_line_type{
                ArrowLineType::Normal=>String::from("normal"),
                ArrowLineType::Dotted=>String::from("dotted"),
            }
        ));

        element.attributes.push((String::from("arrow-style"), 
            match arrow_type{
                ArrowType::Normal=>String::from("normal"),
                ArrowType::Fine=>String::from("fine"),
                ArrowType::Top=>String::from("Top"),
                ArrowType::Bottom=>String::from("bottom"),
                ArrowType::TopFine=>String::from("top-fine"),
                ArrowType::BottomFine=>String::from("bottom-fine"),
                ArrowType::Bidirectional=>String::from("bidirectional"),
                ArrowType::BidirectionalFine=>String::from("bidirectional-fine"),
            }
        ));

        self.sequence.push(Rc::new(RefCell::new(element)));

        return Ok(());
    }



    fn consume_arrow(input:&str)->Result<(&str, ArrowDirection, ArrowLineType, ArrowType, Option<ArrowDecor>), String>{
        let  map: std::collections::HashMap<&str, (ArrowDirection, ArrowLineType, ArrowType)>= 
            hashmap![
                "->"     => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::Normal),
                "<-"     => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::Normal),
                "-->"    => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::Normal),
                "<--"    => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::Normal),
                "->>"    => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::Fine),
                "<<-"    => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::Fine),
                "-->>"   => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::Fine),
                "<<--"   => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::Normal),
                "-/"     => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::Bottom),
                "\\-"    => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::Bottom),
                "--/"    => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::Bottom),
                "\\--"   => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::Bottom),
                "-\\"    => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::Top),
                "/-"     => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::Top),
                "--\\"   => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::Top),
                "/--"    => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::Top),
                "-//"    => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::BottomFine),
                "\\\\-"  => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::BottomFine),
                "--//"   => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::BottomFine),
                "\\\\--" => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::BottomFine),
                "-\\\\"  => (ArrowDirection::Right, ArrowLineType::Normal, ArrowType::TopFine),
                "//-"    => (ArrowDirection::Left, ArrowLineType::Normal, ArrowType::TopFine),
                "--\\\\" => (ArrowDirection::Right, ArrowLineType::Dotted, ArrowType::TopFine),
                "//--"   => (ArrowDirection::Left, ArrowLineType::Dotted, ArrowType::TopFine),
                "<->"    => (ArrowDirection::Bidirectional, ArrowLineType::Normal, ArrowType::Bidirectional),
                "<<->>"  => (ArrowDirection::Bidirectional, ArrowLineType::Normal, ArrowType::BidirectionalFine),
                "<-->"   => (ArrowDirection::Bidirectional, ArrowLineType::Dotted, ArrowType::Bidirectional), 
                "<<-->>" => (ArrowDirection::Bidirectional, ArrowLineType::Dotted, ArrowType::BidirectionalFine),
            ];


        let valid_tokens=[
            "->","<-","-->","<--",
            "->>","<<-","-->>","<<--",
            "-/","\\-","--/","\\--",
            "-\\","/-","--\\","/--",
            "-//", "\\\\-", "--//", "\\\\--",
            "-\\\\", "//-", "--\\\\", "//--",
            "<->","<<->>","<-->","<<-->>",
        ];

        let mut slice = input;

        let mut arrow_direction:ArrowDirection = ArrowDirection::Right;
        let mut arrow_line_type:ArrowLineType = ArrowLineType::Normal;
        let mut arrow_type:ArrowType = ArrowType::Normal;
        let mut arrow_decor:Option<ArrowDecor> =None;

        // First check for optional o or x decorators
        if let Ok((remaining, token)) = consume_token_in_list(slice, &["x", "o"]){
            match token{
                "x" => arrow_decor=Some(ArrowDecor::Cross),
                "o" => arrow_decor=Some(ArrowDecor::Round),
                _ => panic!()
            }
            slice=remaining;
        }
        
        if let Ok((remaining, token)) = consume_token_in_list(slice, &valid_tokens){
            match map.get(token){
                Some((direction, line_type, a_type)) => {
                    arrow_direction = *direction;
                    arrow_line_type = *line_type;
                    arrow_type= *a_type;
                    if arrow_direction == ArrowDirection::Bidirectional && arrow_decor != None {
                        return Err(String::from("Invalid arrow : bidirectional arrows can't have any decoration"));
                    }
                    // Previously set decoration is only valid if the arrow goes left
                    if arrow_direction == ArrowDirection::Right && arrow_decor != None {
                        return Err(String::from("Invalid arrow : decoration before right arrow must be on right ride"));
                    }
                    slice = remaining;
                }
                _ => panic!(),
            }
        }
        else{
            return Err(String::from("Invalid arrow"));
        }

        // If arrow goes right, look for optional decoration
        if let Ok((remaining, token)) = consume_token_in_list(slice, &["x", "o"]){
            match token{
                "x" => arrow_decor=Some(ArrowDecor::Cross),
                "o" => arrow_decor=Some(ArrowDecor::Round),
                _ => panic!()
            }
            slice=remaining;
        }
        return Ok((slice, arrow_direction, arrow_line_type, arrow_type, arrow_decor));
    }


    fn create_participant_if_needed(&mut self, name: &str){
        //TODO



    }

    fn push_to_header(&mut self, element:Rc<RefCell<Element>>){
        match self.open_header_tokens.last() {
            Some((ptr,_)) => ptr.borrow_mut().children.push(element),
            None => self.header.push(element),
        }
    }

    fn is_header_close_condition(&mut self, cond: &HDCloseCondition) -> bool{

        let iter = self.open_header_tokens.iter();

        for (_, token) in iter {
            if token == cond{
                return true;
            }
        }
        return false;
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

        {
            // first remove indentation from the line
           let (trimmed_slice, _) = consume_whitespaces(slice);
           slice = trimmed_slice;
       }

        // header line starts with keyword
        if self.state == SequenceDiagramParserState::Header{
            // Check for tokens that are always at the start of line in the header
            if  let Ok((new_slice, token)) = consume_token_in_list(slice, &reserved_tokens_header) {
            	input.slice = new_slice;
                match token {
                    "participant"|"actor"|"boundary"|"control"|"entity"|"database"|"collections" 
                        => {self.add_participant(input.slice, token);},
                    "box" => {self.add_box(input.slice);},
                    "end box" => {self.end_box();},
                    "title" => return Err((input, String::from("runtime error, invalid condition"))),
                    "hide" => return Err((input, String::from("runtime error, invalid condition"))),
                    "show" => return Err((input, String::from("runtime error, invalid condition"))),
                    _ => return Err((input, String::from("runtime error, invalid condition"))),
                }

            }
            else {
                // close any open box in the header
                //TODO report warning if array is not empty ?
                self.open_header_tokens.clear();
                self.state = SequenceDiagramParserState::Content;
            }
        }
        // content line starts with keyword
        if  let Ok((slice, token)) = consume_token_in_list(slice, &reserved_tokens_sequence) {
            match token {
                _ => return Err((input, String::from("not implemented"))),
            }
            let (slice, unused) = consume_whitespaces(slice);
        }
        else{
            self.add_message(slice);
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



    // Internal unit tests
    // These tests verify the internal state of the parser
    // As it is based on a trees/stacks build, 
    // I want to verify my assumptions about the instance internal state
    #[test]
    fn test_int_sequenceparser_header1() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext::new_for_tests(&"participant bob");
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
    fn test_int_sequenceparser_header2() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext::new_for_tests(&"participant bobby as bob");

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
    fn test_int_sequenceparser_header3() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext::new_for_tests(&"participant \"bobby the 🐶\" as bob");

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
    fn test_int_sequenceparser_header4() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext::new_for_tests(&"actor \"bobby the 🐶\" as bob");

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


    #[test]
    fn test_int_sequenceparser_box1() {
        let mut parser = SequenceDiagramParser::new();

        let mut slice = SliceWithContext::new_for_tests(&"box \"boxxy\"");

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("boxxy"),
                        etype: ElementType::StringType,
                        children: vec![],
                        attributes: vec![],
                    }))
                ],
                attributes: vec![
                    (String::from("type"), String::from("box")),
                ]
            }))        
        ];
        assert_eq!(parser.header, expected);
    }

    #[test]
    fn test_int_sequenceparser_box2() {
        let mut parser = SequenceDiagramParser::new();

        {
            let mut slice = SliceWithContext::new_for_tests(&"box \"boxxy\"");
            let returned = parser.step(&mut slice);
        }
        {
            let mut slice = SliceWithContext::new_for_tests(&"participant bob");
            let returned = parser.step(&mut slice);
        }
        assert_eq!(parser.header.len(), 1);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("boxxy"),
                        etype: ElementType::StringType,
                        children: vec![],
                        attributes: vec![],
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
                attributes: vec![
                    (String::from("type"), String::from("box")),
                ]
            }))        
        ];
        assert_eq!(parser.header, expected);
    }

    #[test]
    fn test_int_sequenceparser_box3() {
        let mut parser = SequenceDiagramParser::new();

        {
            let mut slice = SliceWithContext::new_for_tests(&"box \"boxxy\"");
            let returned = parser.step(&mut slice);
        }
        {
            let mut slice = SliceWithContext::new_for_tests(&"end box");
            let returned = parser.step(&mut slice);
        }
        {
            let mut slice = SliceWithContext::new_for_tests(&"participant bob");
            let returned = parser.step(&mut slice);
        }
        assert_eq!(parser.header.len(), 2);

        let expected:Vec<Rc<RefCell<Element>>>=vec![
            Rc::new(RefCell::new(Element{
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![
                    Rc::new(RefCell::new(Element{
                        value: String::from("boxxy"),
                        etype: ElementType::StringType,
                        children: vec![],
                        attributes: vec![],
                    })),
                ],
                attributes: vec![
                    (String::from("type"), String::from("box")),
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
        ];
        assert_eq!(parser.header, expected);
    }

    // And now for some external tests
}