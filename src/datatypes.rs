use std::rc::Rc;

#[derive(Debug)]
pub struct LineWithContext {
    pub text: String,
    pub line: u32,
    pub file_name: Rc<String>,
    pub namespace: Rc<String>,
}

#[derive(Debug)]
pub struct SliceWithContext<'a> {
    pub slice: &'a str,
    pub line: u32,
    pub pos: u32,
    pub file_name: Rc<String>,
}



impl<'a> SliceWithContext<'a>{
    pub fn new_for_tests(text: &'static str)->SliceWithContext{
        SliceWithContext {
            slice: text,
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        }
    }

}