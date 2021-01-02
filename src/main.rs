use std::io::BufRead;
use std::rc::Rc;
use std::cell::{RefCell};


extern crate foggy_engine;

// For testing
use std::io::Cursor;
use std::io;

use foggy_engine::datatypes::{ElementContent, Element, SliceWithContext};
use foggy_engine::parsers::datatypes::*;
use foggy_engine::parsers::sequenceparser::SequenceDiagramParser;
use foggy_engine::builders::sequencebuilder::SequenceDiagramBuilder;
use foggy_engine::preprocessor;

// Test
struct TestDataSource {}
impl preprocessor::Datasource for TestDataSource {
    fn get_data(&self, filename: &str) -> Result<Box<dyn BufRead>, &'static str> {
        match filename {
            //"Test" => Ok(Box::new(Cursor::new(self.truc.as_bytes()))),
            "utf8" => Ok(Box::new(Cursor::new("你好".as_bytes()))),
            "file.fgu" => Ok(Box::new(Cursor::new(
                "content\nkw\nrest of content".as_bytes(),
            ))),
            "file2" => Ok(Box::new(Cursor::new("pruuuuuuut".as_bytes()))),
            _ => Err("File does not exist"),
        }
    }
}

fn main() {
    // let mut source = TestDataSource {};
    // let mut pre = preprocessor::Preprocessor::new(&source, &["utf8", "utf8", "file.fgu"]);

    let mut parser = SequenceDiagramParser::new();
    parser.step(& mut SliceWithContext::new_for_tests(&"alice->bob : Hello"));
    parser.step(& mut SliceWithContext::new_for_tests(&"alice<-bob : ..."));
    parser.step(& mut SliceWithContext::new_for_tests(&"bob->eve : !!!"));

    let (elements, documents) = parser.flush();
    let mut builder = SequenceDiagramBuilder::new();
    let xml = builder.generate_svg(&elements);
    println!("{}", xml.unwrap())
}
