use crate::datatypes::{LineWithContext, Document};
//use parsers::stringparser::StringParser;


struct ParserCombinator {}

impl ParserCombinator {
    fn interpret(
        input: &mut impl Iterator<Item = Result<LineWithContext, &'static str>>,
    ) -> Result<Vec<Document>, &'static str> {
        // let mut parser = StringParser::new();

        // while let Some(line) = input.next() {
        //     let line_content = line.unwrap();

        //     let mut slice = SliceWithContext {
        //         slice: &line_content.text,
        //         line: line_content.line,
        //         pos: 0,
        //         file_name: Rc::clone(&line_content.file_name),
        //     };
        //     parser.step(&mut slice);
        // }

        return Err("tu vas compiler oui");
    }
}