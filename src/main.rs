use std::io::BufRead;
use std::rc::Rc;
use std::cell::{RefCell};

mod datatypes;
mod parsers;
mod builders;
mod parseutils;
mod preprocessor;
extern crate maplit;

// For testing
use std::io::Cursor;
use std::io;

use datatypes::{ElementContent, Element};
use builders::svgbuilder::*;

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
    let mut source = TestDataSource {};
    let mut pre = preprocessor::Preprocessor::new(&source, &["utf8", "utf8", "file.fgu"]);

    let mut svg = create_svg(200.0,200.0);
    svg.push(Rc::new(RefCell::new(create_rect(10.0, 10.0, 10.0, 10.0, "", Some(1.0), None))));
    println!("svg :\n{:?}", svg.to_xml());

}
