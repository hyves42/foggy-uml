use std::rc::Rc;
use std::cell::{RefCell};
use datatypes::{SliceWithContext, ElementType, Element, Document};
use parsers::datatypes::{Parser, ParserResult};
use parseutils::*;

pub struct StringParser {
    collec: Option<String>,
    start_token: String,
}
impl StringParser {
    pub fn new() -> StringParser {
        StringParser {
            collec: Some(String::new()),
            start_token: String::new(),
        }
    }

    pub fn is_start_word(input: &str) -> bool {
        match consume_token_in_list(input, Self::get_start_words()){
            Ok(_)=> return true,
            Err(_) => return false
        }
    }
    pub fn get_start_words() -> &'static [&'static str] {
        return &["\"\"\"", "\"", "\'"];
    }
}

impl Parser for StringParser {
    fn step<'a>(
        &mut self,
        input: &'a mut SliceWithContext<'a>,
    ) -> Result<ParserResult<'a>, (&'a SliceWithContext<'a>, String)> {
        let mut slice: &str = input.slice;

        if self.collec == None {
            // That means this stringparser instance has already been used
            return Err((
                input,
                String::from("runtime error, collector shall not be none"),
            ));
        }

        if self.start_token.len() == 0 {
            // Brand new string parser. consume opening token
            match consume_token_in_list(slice, StringParser::get_start_words()) {
                Ok((new_slice, token)) => {
                    self.start_token.push_str(token);
                    slice = new_slice;
                }
                Err(_) => return Err((input, String::from("Invalid string start token"))),
            }
        }

        while slice.len() > 0 {
            let stop_tokens = ["\\", &self.start_token.as_str()];

            let (new_slice, consumed) =
                consume_until_token_in_list(slice, &stop_tokens).unwrap();
            self.collec.as_mut().unwrap().push_str(consumed);
            slice = new_slice;

            if new_slice.len() > 0 {
                //remaining characters in current line
                let (new_slice, consumed) = consume_token_in_list(new_slice, &stop_tokens).unwrap();
                if consumed == "\\" {
                    // next char has top be escaped
                    let mut iter = new_slice.chars();

                    if let Some(c) = iter.next() {
                        match c {
                            'r' => self.collec.as_mut().unwrap().push('\r'),
                            'n' => self.collec.as_mut().unwrap().push('\n'),
                            't' => self.collec.as_mut().unwrap().push('\t'),
                            _ => self.collec.as_mut().unwrap().push(c),
                            // todo : handle \u, \U
                        }
                        slice = &new_slice[c.len_utf8()..];
                    } else {
                        // multiline string!
                        return Err((input, String::from("not implemented yet")));
                    }
                } else if consumed == self.start_token {
                    // We finished parsing the string, return its content
                    input.slice = new_slice;
                    return Ok(ParserResult::Done(input));
                }
            }
        }
        // Full line was consumed
        return Err((input, String::from("invalid token")));
    }

    fn flush(&mut self) -> (Vec<Rc<RefCell<Element>>>, Vec<Rc<RefCell<Document>>>) {
        if self.collec == None {
            return (vec![], vec![]);
        }

        let mut elements = Vec::new();
        elements.push(Rc::new(RefCell::new(Element {
            value: self.collec.take().unwrap(),
            etype: ElementType::StringType,
            children: vec![],
            attributes: vec![],
        })));

        return (elements, vec![]);
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use datatypes::{LineWithContext, SliceWithContext};
    use std::rc::Rc;

    #[test]
    fn test_stringparser_nominal_doublequote() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\"coucou\"",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Done(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].borrow().value, "coucou");
    }

    #[test]
    fn test_stringparser_nominal_simplequote() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\'coucou\'",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(!returned.is_err());
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Done(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].borrow().value, "coucou");
    }

    #[test]
    fn test_stringparser_unterminated() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\"coucou",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        assert!(returned.is_err());
    }

    #[test]
    fn test_stringparser_escaped() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\"cou\\ncou\"",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Done(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].borrow().value, "cou\ncou");
    }

    #[test]
    fn test_stringparser_escaped_with_utf8() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\"cou\\☃cou\"",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);
        match returned.unwrap() {
            // Check that everything was consumed
            ParserResult::Done(s) => assert_eq!(s.slice.len(), 0),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].borrow().value, "cou☃cou");
    }

    #[test]
    fn test_stringparser_remaining_slice() {
        let mut parser = StringParser::new();

        let mut slice = SliceWithContext {
            slice: &"\"\"  ",
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        };

        let returned = parser.step(&mut slice);

        match returned.unwrap() {
            // Check that 2 chars remain
            ParserResult::Done(s) => assert_eq!(s.slice.len(), 2),
            _ => assert!(false),
        }

        let (elements, documents) = parser.flush();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].borrow().value, "");
    }


}
