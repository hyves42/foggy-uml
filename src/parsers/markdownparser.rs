use std::rc::Rc;
use std::cell::{RefCell};
use parsers::datatypes::{ElementType, Element, Document, Parser, ParserResult};
use datatypes::{SliceWithContext};
use parseutils::*;

#[derive(PartialEq)]
enum MDCloseCondition {
//    NoCondition,   // Never closed (for root element)
    Token(String), // Close on textual token
    Eol,           // close at end of line
    NoRBracket,    // close when (one) '>' disappears from line start
    EmptyLine,     // close on empty line
    NoMoreIndent,  // close when no "  " prefix starts the line
}


struct MarkdownParser {
    // a string buffer to collect text for the element being parsed
    collec: Option<String>,
    // full tree structure of document
    root: Element,
    // indicates the current depth in the element branches, with associated closing conditions
    open_tokens: Vec<(Rc<RefCell<Element>>,MDCloseCondition)>,
}
// Principle of operation is simple:
// we are always writing (adding children) at the end of the element tree designated by self.root.
// The current writing depth corresponds to the number of currently open tokens.
// For exemple at the start of the document or after a paragraph is finished, 
// there is no currently open token (self.open_tokens array is empty). 
// The next element will be added just below self.root

// push = we 'open' a new parsing depth : 
// - add a container element in the right place
// - record the condition to stop the current parsing operation in (self.open_tokens)

// collect = we 'close' a parsing depth :
// - take the currently collected text (self.collec) and put it inside its container 
// - remove the stop condition from (self.open_tokens)

impl MarkdownParser {
    fn new() -> MarkdownParser {
        MarkdownParser {
            collec: Some(String::new()),
            root: Element {
                value: String::new(),
                etype: ElementType::StructureType,
                children: vec![],
                attributes: vec![],
            },
            open_tokens: Vec::new(),
        }
    }

    fn push_title(&mut self, level: u32) {
        // Convention : all titles are children of the document root
        // So we haver to close any open token that we may have
        self.collect_all_open_tokens();

        let element = Element {
            value: String::new(),
            etype: ElementType::StructureType,
            children: vec![],
            attributes: vec![
                (String::from("format"), String::from("title")),
                (String::from("level"), level.to_string()),
            ],
        };
        let ptr=Rc::new(RefCell::new(element));
        self.root.children.push(Rc::clone(&ptr));
        self.open_tokens.push((Rc::clone(&ptr), MDCloseCondition::Eol));
    }

    fn push_formatter(&mut self, token: &str, format: &str) {
        let element = Element {
            value: String::new(),
            etype: ElementType::StructureType,
            children: vec![],
            attributes: vec![
                (String::from("format"), String::from(format)),
            ],
        };
        let ptr=Rc::new(RefCell::new(element));

        self.collect_to_last_leaf();
        self.push_to_last_leaf(Rc::clone(&ptr));
        self.open_tokens.push((Rc::clone(&ptr), MDCloseCondition::Token(String::from(token))));
    }

    fn push_paragraph(&mut self) {
        let element = Element {
            value: String::new(),
            etype: ElementType::StructureType,
            children: vec![],
            attributes: vec![
                (String::from("format"), String::from("paragraph")),
            ],
        };
        let ptr=Rc::new(RefCell::new(element));
        self.collect_to_last_leaf();
        self.push_to_last_leaf(Rc::clone(&ptr));
        self.open_tokens.push((Rc::clone(&ptr),MDCloseCondition::EmptyLine));
    }


    fn push_to_last_leaf(&mut self, element:Rc<RefCell<Element>>){
        match self.open_tokens.last() {
            Some((ptr, _)) => ptr.borrow_mut().children.push(element),
            None => self.root.children.push(element),
        }
    }

    fn collect_to_last_leaf(&mut self){
        // Add currently collected text to the last leaf of the tree
        if self.collec.as_ref().unwrap().len() == 0{
            return;
        }

        let element= Element {
            value: self.collec.take().unwrap(),
            etype: ElementType::StringType,
            children: vec![],
            attributes: vec![],
        };
        let ptr=Rc::new(RefCell::new(element));
        self.push_to_last_leaf(Rc::clone(&ptr));

        // we have taken self.collec, create it again
        self.collec=Some(String::new());
    }

    // collect current text and start with an empty list of open tokens
    fn collect_all_open_tokens(&mut self){
        self.collect_to_last_leaf();
        self.open_tokens.clear();
    }

    // collect current text and start from the parent of the token that was just closed
    fn collect_all_open_tokens_until_cond(&mut self, cond: &MDCloseCondition){
        self.collect_to_last_leaf();

        while let Some((_, token)) = self.open_tokens.pop(){
            if token == *cond{
                return;
            }
        }
    }

    fn is_close_condition(&mut self, cond: &MDCloseCondition) -> bool{

        let iter = self.open_tokens.iter();

        for (_, token) in iter {
            if token == cond{
                return true;
            }
        }
        return false;
    }


}

impl Parser for MarkdownParser{
    fn step<'b>(
        &mut self,
        input: &'b mut SliceWithContext<'b>,
    ) -> Result<ParserResult<'b>, (&'b SliceWithContext<'b>, String)> {
        let mut slice: &str = input.slice;

        // Shall not happen
        if self.collec == None {
            return Err((input, String::from("runtime error, collector shall not be none")));
        }

        // Check for tokens that are always at the start of line
        let line_start_tokens = ["###", "##", "#", ">"];
        if let Ok((new_slice, token)) = consume_token_in_list(slice, &line_start_tokens) {
            match token {
                "###" => self.push_title(3),
                "##" => self.push_title(2),
                "#" => self.push_title(1),
                _ => return Err((input, String::from("runtime error, invalid condition"))),
            }
            let (new_slice, _) = consume_whitespaces(new_slice);
            slice = new_slice;
        }
        else if slice.trim().len() ==0{
            //empty line, if a paragraph is open, close it
            let cond = MDCloseCondition::EmptyLine;
            if self.is_close_condition(&cond){
                self.collect_all_open_tokens_until_cond(&cond);
            }            
            slice=&slice[..0];
        }


        while slice.len() > 0 {
            let stop_tokens = ["**", "*", "~~"];

            let (new_slice, consumed) =
                consume_until_token_in_list(slice, &stop_tokens).unwrap();


            // If no existing container to store this text create a paragraph
            if self.open_tokens.len()==0{
                self.push_paragraph();
            }

            self.collec.as_mut().unwrap().push_str(consumed);
            slice = new_slice;

            if new_slice.len() == 0 {
                break;
            }

            // consume the token that stopped us
            let (new_slice, consumed) = consume_token_in_list(new_slice, &stop_tokens).unwrap();
            // are we closing something this this token ?
            let condition=MDCloseCondition::Token(consumed.to_string());
            if self.is_close_condition(&condition){
                self.collect_all_open_tokens_until_cond(&condition);
            }
            else{
                match consumed{
                    "**" => self.push_formatter("**", "bold"),
                    "*" => self.push_formatter("*", "italic"),
                    "~~" => self.push_formatter("~~", "strikethrough"),
                    _ => return Err((
                        input,
                        String::from("runtime error, met unexpected token"),
                    )) // no other token apart from those in stop_tokens array shall be consumed
                }
            }
            slice=new_slice;
        }


        // Full line was consumed

        // check for this close condition
        let cond = MDCloseCondition::Eol;
        if self.is_close_condition(&cond){
            self.collect_all_open_tokens_until_cond(&cond);
        }
        input.slice = slice;
        return Ok(ParserResult::Partial(input));
    }

    fn flush(&mut self) -> (Vec<Rc<RefCell<Element>>>, Vec<Rc<RefCell<Document>>>) {
        self.collect_all_open_tokens();
        return (self.root.children.clone(), vec![]);
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use datatypes::{LineWithContext, SliceWithContext};
    use std::rc::Rc;


    #[test]
    fn test_markdownparser_title() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"## Title",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("title")),
                    (String::from("level"), "2".to_string()),            
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "Title".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    }))
                ],
            }))
        ];


        assert_eq!(elements, expected );
    }

    #[test]
    fn test_markdownparser_empty() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![];
        assert_eq!(elements, expected);
    }

    #[test]
    fn test_markdownparser_normal_text() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"line 1",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("paragraph")),
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "line 1".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    }))
                ]
            }))
        ];
        assert_eq!(elements, expected);
    }


    #[test]
    fn test_markdownparser_normal_text2() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"line 1",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };
        match parser.step(&mut slice).unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let mut slice2 = SliceWithContext {
            slice: &"line 2",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };
        match parser.step(&mut slice2).unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("paragraph")),
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "line 1line 2".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    }))
                ]
            }))
        ];
        assert_eq!(elements, expected);
    }



    #[test]
    fn test_markdownparser_bold() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"**bold**",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("paragraph")),
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "".to_string(),
                        etype: ElementType::StructureType,
                        attributes: vec![
                            (String::from("format"), String::from("bold")),           
                        ],
                        children: vec![Rc::new(RefCell::new(Element {
                            value: "bold".to_string(),
                            etype: ElementType::StringType,
                            attributes: vec![],
                            children: vec![],
                        }))],
                    }))
                ],
            }))
        ];

        assert_eq!(elements, expected );
    }


    #[test]
    fn test_markdownparser_bold2() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"text1**bald**text2",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("paragraph")),
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "text1".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Element {
                        value: "".to_string(),
                        etype: ElementType::StructureType,
                        attributes: vec![
                            (String::from("format"), String::from("bold")),           
                        ],
                        children: vec![Rc::new(RefCell::new(Element {
                            value: "bald".to_string(),
                            etype: ElementType::StringType,
                            attributes: vec![],
                            children: vec![],
                        }))],
                    })),
                    Rc::new(RefCell::new(Element {
                        value: "text2".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    }))
                ]
            }))
        ];

        assert_eq!(elements, expected);
    }


    #[test]
    fn test_markdownparser_2lines_title_text() {
        let mut parser = MarkdownParser::new();

        let mut slice = SliceWithContext {
            slice: &"# title **but bold this time",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };
        match parser.step(&mut slice).unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let mut slice2 = SliceWithContext {
            slice: &"normal text",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };
        match parser.step(&mut slice2).unwrap() {
            // Check that everything was consumed
            ParserResult::Partial(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }


        let (elements, documents) = parser.flush();

        let expected = vec![
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("title")),
                    (String::from("level"), "1".to_string()),            
                ],
                children: vec![
                    Rc::new(RefCell::new(Element {
                        value: "title ".to_string(),
                        etype: ElementType::StringType,
                        attributes: vec![],
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Element {
                        value: "".to_string(),
                        etype: ElementType::StructureType,
                        attributes: vec![
                            (String::from("format"), String::from("bold")),           
                        ],
                        children: vec![Rc::new(RefCell::new(Element {
                            value: "but bold this time".to_string(),
                            etype: ElementType::StringType,
                            attributes: vec![],
                            children: vec![],
                        }))],
                    })),
                ],
            })),
            Rc::new(RefCell::new(Element {
                value: "".to_string(),
                etype: ElementType::StructureType,
                attributes: vec![
                    (String::from("format"), String::from("paragraph")),
                ],
                children: vec![Rc::new(RefCell::new(Element {
                    value: "normal text".to_string(),
                    etype: ElementType::StringType,
                    attributes: vec![],
                    children: vec![],
                }))],
            })),
        ];

        assert_eq!(elements, expected);
    }

}