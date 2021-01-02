use std::io::{BufRead, Lines};
use std::iter::Iterator;
use std::rc::Rc;

use crate::datatypes::LineWithContext;
use crate::parseutils::*;

// a data source for the preprocessor
// typically a wrapper around file access
// but we can add : in memory data source, cache, file access security checks, etc.
pub trait Datasource {
    fn get_data(&self, filename: &str) -> Result<Box<dyn BufRead>, &'static str>;
}

struct StackContext {
    lines: Lines<Box<dyn BufRead>>,
    filename: Rc<String>,
    namespace: Rc<String>,
}

pub struct Preprocessor<'a> {
    input: &'a dyn Datasource,
    stack: Vec<StackContext>,
    processed_files: Vec<String>,
    files: Vec<String>,
}

impl<'a> Preprocessor<'a> {
    pub fn new(input: &'a dyn Datasource, files: &[&str]) -> Preprocessor<'a> {
        let mut obj = Preprocessor {
            input: input,
            stack: Vec::new(),
            processed_files: Vec::new(),
            files: Vec::new(),
        };

        //println!("** new preprocessor with list of files:");
        for file in files {
            println!("-{}", file);
            obj.files.push(file.to_string());
        }
        return obj;
    }

    fn file_already_processed(&self, file: &str) -> bool {
        self.processed_files.iter().any(|e| e == file)
    }
}

// Returm None if simply no match
// Return Some(Err()) if bogus match
// Return Some((token, file)) if valid match
fn parse_import_include_statement(input: &str) -> Option<Result<(&str, &str), &'static str>> {
    match consume_token_in_list(&input, &["/import", "/include"]) {
        Err(s) => return None,
        Ok((rem, token)) => {
            let (rem, unused) = consume_whitespaces(rem);
            if let Ok((rem, file)) = consume_until_whitespace(rem) {
                let (rem, unused) = consume_whitespaces(rem);
                if rem.len() == 0 {
                    return Some(Ok((token, file)));
                }
            }
        }
    }
    return Some(Err("Invalid statement"));
}

impl<'a> Iterator for Preprocessor<'a> {
    type Item = Result<LineWithContext, &'static str>;

    fn next(&mut self) -> Option<Result<LineWithContext, &'static str>> {
        let mut result: Option<Result<LineWithContext, &'static str>> = None;

        // no file currently processed, look for work to do in list of files
        if self.stack.len() == 0 {
            match self.files.pop() {
                // All done
                None => return None,
                Some(file) => {
                    self.processed_files.push(file.clone());

                    // if file exists/no error, put line iterator into the stack
                    match self.input.get_data(&file) {
                        Ok(d) => self.stack.push(StackContext {
                            lines: d.lines(),
                            filename: Rc::new(file.clone()),
                            namespace: Rc::new(file.clone()),
                        }),
                        Err(s) => return Some(Err(s)),
                    }
                }
            }
        }

        // Process stack content
        while self.stack.len() > 0 {
            // The context we want to process/resume
            let mut ctx = self.stack.pop().unwrap();

            //Read line by line
            while let Some(line) = ctx.lines.next() {
                let line_contents = line.unwrap();
                println!("{} : {}", ctx.filename, line_contents);

                // TODO: define, if, endif
                if line_contents.starts_with("//") {
                    //comment, ignore line
                    continue;
                } else if let Some(result) = parse_import_include_statement(&line_contents) {
                    // import content of file if not imported yet
                    // eg: /import file.fgy
                    // or, include content of file regardless of if it was included before
                    // eg: /include file.fgy

                    match result {
                        Err(s) => return Some(Err(s)),
                        Ok((token, file)) => {
                            if token == "/import" && self.file_already_processed(file) {
                                //println!("ignore import of {}", file);
                                //File already imported, nothing to do, ignore line
                                continue;
                            }

                            self.processed_files.push(String::from(file));
                            match self.input.get_data(file) {
                                Ok(d) => {
                                    // File exists, push new file to stack and iter one more tine
                                    //println!("importing {} ({})", file, token);
                                    self.stack.push(ctx);
                                    self.stack.push(StackContext {
                                        lines: d.lines(),
                                        filename: Rc::new(String::from(file)),
                                        namespace: Rc::new(String::from(file)),
                                    });
                                    break;
                                }
                                // File does not exist, return am error
                                Err(s) => return Some(Err(s)),
                            }
                        }
                    }
                } else if line_contents.starts_with("/includecode") {
                    // include content of file without preprocessing, and with provided header and footer tag
                    // eg: /includecode example.py tag:pythoncode
                } else {
                    // Normal line without specific treatment
                    result = Some(Ok(LineWithContext {
                        text: line_contents.clone(),
                        line: 0,
                        file_name: Rc::clone(&ctx.filename),
                        namespace: Rc::clone(&ctx.namespace),
                    }));

                    self.stack.push(ctx);
                    return result;
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Datasource;
    use super::LineWithContext;
    use super::Preprocessor;
    use std::io::BufRead;
    use std::io::Cursor;

    use super::parse_import_include_statement;

    // Stub datasource that provides file contents for tests
    struct SimpleTestDataSource {}
    impl Datasource for SimpleTestDataSource {
        fn get_data(&self, filename: &str) -> Result<Box<dyn BufRead>, &'static str> {
            match filename {
                "utf8" => Ok(Box::new(Cursor::new("你好".as_bytes()))),
                "test_1line" => Ok(Box::new(Cursor::new("content".as_bytes()))),
                "test_3lines" => Ok(Box::new(Cursor::new(
                    "content1\ncontent2\ncontent3".as_bytes(),
                ))),
                "file_with_import" => Ok(Box::new(Cursor::new(
                    "/import imported_file\n/import imported_file\ncontent1\ncontent2\ncontent3"
                        .as_bytes(),
                ))),
                "file_with_include" => Ok(Box::new(Cursor::new(
                    "/include imported_file\n/include imported_file\ncontent1\ncontent2\ncontent3"
                        .as_bytes(),
                ))),
                "imported_file" => Ok(Box::new(Cursor::new(
                    "imported line 1\nimported line 2".as_bytes(),
                ))),
                "file_with_bad_include" => Ok(Box::new(Cursor::new(
                    "/include doesnotcompute.txt".as_bytes(),
                ))),
                _ => Err("File does not exist"),
            }
        }
    }

    #[test]
    fn test_preproc_1line() {
        let mut source = SimpleTestDataSource {};
        let mut pre = Preprocessor::new(&source, &["test_1line"]);

        let output: Vec<Result<LineWithContext, &'static str>> = pre.collect();
        assert_eq!(output.len(), 1);
        assert!(!output[0].is_err());
        let t = &output[0];
        let line = t.as_ref().unwrap();
        assert_eq!(line.text, String::from("content"));
    }

    #[test]
    fn test_preproc_3line() {
        let mut source = SimpleTestDataSource {};
        let mut pre = Preprocessor::new(&source, &["test_3lines"]);

        let output: Vec<Result<LineWithContext, &'static str>> = pre.collect();
        assert_eq!(output.len(), 3);
        assert!(!output[0].is_err());
        assert!(!output[1].is_err());
        assert!(!output[2].is_err());

        let t = &output[0];
        let line = t.as_ref().unwrap();
        assert_eq!(line.text, String::from("content1"));
    }

    #[test]
    fn test_preproc_import() {
        let mut source = SimpleTestDataSource {};
        let mut pre = Preprocessor::new(&source, &["file_with_import"]);

        let output: Vec<Result<LineWithContext, &'static str>> = pre.collect();
        assert_eq!(output.len(), 5);

        for out in output {
            assert!(!out.is_err());
        }
    }

    #[test]
    fn test_preproc_include() {
        let mut source = SimpleTestDataSource {};
        let mut pre = Preprocessor::new(&source, &["file_with_include"]);

        let output: Vec<Result<LineWithContext, &'static str>> = pre.collect();
        assert_eq!(output.len(), 7);
    }

    #[test]
    fn test_file_with_bad_include() {
        let mut source = SimpleTestDataSource {};
        let mut pre = Preprocessor::new(&source, &["file_with_bad_include"]);

        let output: Vec<Result<LineWithContext, &'static str>> = pre.collect();
        assert_eq!(output.len(), 1);
        assert!(output[0].is_err());
    }

    #[test]
    fn test_parse_import_include_statement() {
        assert_eq!(
            parse_import_include_statement("/import toto.txt"),
            Some(Ok(("/import", "toto.txt")))
        );
        assert_eq!(
            parse_import_include_statement("/include toto.txt"),
            Some(Ok(("/include", "toto.txt")))
        );
        assert_eq!(parse_import_include_statement("abracadabra"), None);
        assert_eq!(parse_import_include_statement("/ignore toto.txt"), None);
        assert_eq!(
            parse_import_include_statement("/import toto.txt tata.txt"),
            Some(Err("Invalid statement"))
        );
        assert_eq!(
            parse_import_include_statement("/import файл.txt"),
            Some(Ok(("/import", "файл.txt")))
        );
        assert_eq!(
            parse_import_include_statement("/import 파일.txt"),
            Some(Ok(("/import", "파일.txt")))
        );
    }
}
