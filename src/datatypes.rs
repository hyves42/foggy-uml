use std::rc::Rc;
use std::cell::{RefCell};
use parseutils::*;


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
    #[cfg(test)]
    pub fn new_for_tests(text: &'static str)->SliceWithContext{
        SliceWithContext {
            slice: text,
            line: 0,
            pos: 0,
            file_name: Rc::new(String::from("file.txt")),
        }
    }

}





#[derive(Debug)]
#[derive(PartialEq)]
pub enum ElementContent {
    Tree(Vec<Rc<RefCell<Element>>>), // just tree structure
    Text(String),                    // element contains text value
    // binary type ??
}

// Not completely convinced. about this
// Something with an enum and different fields for each value enum could be better
// And I would like this to be expandable without defining all possible fields for every possible use cases/diagram
// todo: think about it after I get some sleep
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Element {
    pub tag: String,
    pub content: ElementContent,
    pub attributes: Vec<(String, String)>,
}

impl Element {
    pub fn new(tag: &str) -> Element{
        Element {
            tag: String::from(tag),
            content: ElementContent::Tree(vec![]),
            attributes:vec![],
        }
    }

    pub fn new_str(tag: &str, text: &str) -> Element{
        Element {
            tag: String::from(tag),
            content: ElementContent::Text(String::from(text)),
            attributes:vec![],
        }
    }
    pub fn new_string(tag: &str, text: String) -> Element{
        Element {
            tag: String::from(tag),
            content: ElementContent::Text(text),
            attributes:vec![],
        }
    } 

    pub fn get_attr(&self, key: &str) -> Option<String>{
        //smells like I need a hashmap instead
        for (k,v) in &self.attributes{
            if k == key {
                return Some(v.to_string());
            }
        }
        return None;
    }

    pub fn push(&mut self, ptr:Rc<RefCell<Element>>){
        if let ElementContent::Tree(ref mut children) = self.content{
            children.push(ptr);
        }
        else{
            panic!("Element is not a tree");
        }
    }

    pub fn push_attribute(&mut self, key:&str, value:&str){
        self.attributes.push((key.to_string(), value.to_string()));
    }

    pub fn get_text(&self)->String{
        if let ElementContent::Text(text) = &self.content{
            //TODO  not optimal
            return text.to_string();
        }
        else{
            panic!("Element is not a text element");
        }
    }

    pub fn is_text(&self)->bool{
        match &self.content{
            ElementContent::Tree(_)=>false,
            ElementContent::Text(_)=>true
        }
    }

    pub fn is_tree(&self)->bool{
        match &self.content{
            ElementContent::Tree(_)=>true,
            ElementContent::Text(_)=>false
        }
    }

    pub fn to_xml(&self)->String{
        self._to_xml(0)
    }

    fn _to_xml(&self, level:usize)->String{
        let mut xml=String::new();
        for _ in 0..level{
            xml.push('\t');
        }

        match &self.content{
            ElementContent::Tree(children) =>{
                if children.len()==0{
                    xml.push_str(format!("<{}", self.tag).as_str());
                    for (k, v) in &self.attributes{
                        xml.push_str(format!(" {}=\"{}\"", k, v).as_str());
                    }
                    xml.push_str("/>\n");
                }
                else {
                    xml.push_str(format!("<{}", self.tag).as_str());
                    for (k, v) in &self.attributes{
                        xml.push_str(format!(" {}=\"{}\"", k, v).as_str());
                    }
                    xml.push_str(">\n");
                    for c in children{
                        xml.push_str(c.borrow()._to_xml(level+1).as_str());
                    }
                    for _ in 0..level{
                        xml.push('\t');
                    }
                    xml.push_str(format!("</{}>\n", self.tag).as_str());
                }
            },
            ElementContent::Text(text) =>{
                if self.tag.len() ==0{
                    // No escaping. I expect the text to be escaped by the application
                    xml.push_str(&text.as_str());
                    xml.push('\n');
                }
                else{
                    xml.push_str(format!("<{}", self.tag).as_str());
                    for (k, v) in &self.attributes{
                        xml.push_str(format!(" {}=\"{}\"", k, v).as_str());
                    }
                    xml.push_str(format!(">{}</{}>\n", text, self.tag).as_str());
                }
            }
        }

        return xml;
    }


    // support a simple subset of xpath
    // relative path to children
    // text()
    pub fn get(&self, path:&str)->Option<ElementContent>{
        let (remaining, prefix)=consume_until_token_in_list(path, &["/"]).unwrap();

        match &self.content{
            ElementContent::Tree(children) =>{

                // find child that match 'prefix'
                let mut child:Option<Rc<RefCell<Element>>> = None;
                for c in children{
                    if c.borrow().tag == prefix{
                        if child != None{
                            return None; // child is invalid if several childs with the same tag exist
                        }
                        else{
                            child = Some(Rc::clone(c));
                        }
                    }
                }

                if let Some(c)=child {
                    if let Ok((subpath, _))=consume_token_in_list(remaining, &["/"]){
                        if subpath.len() ==0{
                            return Some(ElementContent::Tree(vec![Rc::clone(&c)]));
                        }
                        else{
                            return c.borrow().get(subpath);
                        }
                    }
                }
            },
            ElementContent::Text(text) =>{
                if remaining == "" && prefix == "text()" {    
                    return Some(ElementContent::Text(text.to_string()));
                }
            }

        }
        return None;
    } 

}

#[derive(Debug)]
pub struct Document {
    pub children: Vec<Element>,
}

// We use this structure so that we don't have to pass the closure
// as a parameter of the recursive function.
// This seems less confusing for the borrow checker
struct KludgeFn<'s>{
    f: &'s mut FnMut(Rc<RefCell<Element>>, usize)
}

// Depth-first tree traversal
// Not an Element method because I want the Rc as a closure parameter, 
// not the element direcly
pub fn recurse_element_tree<F>(elt: Rc<RefCell<Element>>, mut f: F)
where 
    F: FnMut(Rc<RefCell<Element>>, usize) 
{
    let mut k = KludgeFn{f:&mut f};
    (k.f)(Rc::clone(&elt), 0);
    return _recurse_tree(elt, &mut k, 1);
}



fn _recurse_tree(elt: Rc<RefCell<Element>>, k: & mut KludgeFn, depth:usize)
{
    // Hardcoded max depth. Ought to be enough for anyone.
    if depth > 50{
        return;
    }
    let element=elt.borrow();
    if let ElementContent::Tree(children) = &element.content{
        let mut iter=children.iter();
        while let Some(e) = iter.next(){

            (k.f)(Rc::clone(&e), depth);
            _recurse_tree(Rc::clone(e), k, depth+1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types(){
        let elt1 =Element::new("tag");
        assert!(elt1.is_tree());
        assert!(!elt1.is_text());

        let elt2 =Element::new_str("tag", "text");
        assert!(elt2.is_text());
        assert!(!elt2.is_tree());
    }

    #[test]
    fn test_recurse() {
        let tree= Rc::new(RefCell::new(Element{
            tag: String::from("sequencediagram:header"),
            content: ElementContent::Tree(vec![
                Rc::new(RefCell::new(Element{
                    tag: String::new(),
                    content: ElementContent::Tree(vec![
                        Rc::new(RefCell::new(Element{
                            tag: String::new(),
                            content: ElementContent::Text(String::from("alice")),
                            attributes: vec![],
                        }))
                    ]),
                    attributes: vec![]
                })),
                Rc::new(RefCell::new(Element{
                    tag: String::new(),
                    content: ElementContent::Tree(vec![
                        Rc::new(RefCell::new(Element{
                            tag: String::from("name"),
                            content: ElementContent::Text(String::from("bob")),
                            attributes: vec![],
                        }))
                    ]),
                    attributes: vec![]
                }))
            ]),
            attributes: vec![]
        }));
            
        recurse_element_tree(Rc::clone(&tree), |e, d|{println!("{}:{:?}", d, e.borrow())});
        let mut str_count=0;
        let mut struct_count=0;
        recurse_element_tree(Rc::clone(&tree), 
            |e, d|{
                if e.borrow().is_text(){
                    str_count+=1;
                }
                else{
                    struct_count+=1;
                }
        });
        assert_eq!(str_count, 2);
        assert_eq!(struct_count, 3);
    }


    #[test]
    fn test_xml1() {
        let element=Element::new_str("", "lorem ipsum");
        assert_eq!(element.to_xml(), String::from("lorem ipsum\n"));
    }

    #[test]
    fn test_xml2() {
        let element=Element::new_str("p", "lorem ipsum");
        assert_eq!(element.to_xml(), String::from("<p>lorem ipsum</p>\n"));
    }

    #[test]
    fn test_xml3() {
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::new_str("", "lorem ipsum"))));
        assert_eq!(element.to_xml(), String::from("<p>\n\tlorem ipsum\n</p>\n"));
    }

    #[test]
    fn test_xml4() {
        let mut element=Element::new("img");
        element.attributes.push(("src".to_string(), "surprised_pikachu.jpg".to_string()));
        assert_eq!(element.to_xml(), String::from("<img src=\"surprised_pikachu.jpg\"/>\n"));
    }

    #[test]
    fn test_xml5() {
        let mut body=Element::new("body");
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::new_str("", "lorem ipsum"))));
        body.push(Rc::new(RefCell::new(element)));

        assert_eq!(body.to_xml(), String::from("<body>\n\t<p>\n\t\tlorem ipsum\n\t</p>\n</body>\n"));
    }

    #[test]
    fn test_xml6() {
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::new_str("", "lorem ipsum"))));
        element.attributes.push(("class".to_string(), "american".to_string()));

        assert_eq!(element.to_xml(), String::from("<p class=\"american\">\n\tlorem ipsum\n</p>\n"));
    }



    #[test]
    fn test_xpath1() {
        let mut body=Element::new("body");
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::new_str("span", "lorem ipsum"))));
        body.push(Rc::new(RefCell::new(element)));

        assert_eq!(body.get("p/span/text()"), Some(ElementContent::Text(String::from("lorem ipsum"))));
    }

}
