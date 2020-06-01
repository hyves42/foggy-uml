use std::io::BufRead;

mod datatypes;
mod parsers;
mod parseutils;
mod preprocessor;


// For testing
use std::io::Cursor;

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
}
