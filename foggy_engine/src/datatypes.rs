use std::rc::Rc;
use std::cell::{RefCell};
// to parse xpath-like expressions
use crate::parseutils::*;


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


type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct TreeElementContent {
    pub tag: String,
    pub children: Vec<Rcc<Element>>,
    pub attributes: Vec<(String, String)>,
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ElementContent {
    Tree(TreeElementContent),  // just tree structure
    Text(String),              // element contains text value
    // binary type ??
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Element {
    pub content: ElementContent,
}


// Used as a return type to xpath queries
#[derive(Debug)]
#[derive(PartialEq)]
pub enum QueryResult {
    Elt(Rcc<Element>), 
    Text(String),     
}

impl Element {
    // Constructor for a tree element
    pub fn new(tag: &str) -> Element{
        Element {
            content: ElementContent::Tree(TreeElementContent{
                tag: String::from(tag),
                children: vec![],
                attributes:vec![],
            }),
            
        }
    }

    // Constructor for a text element
    pub fn str(text: &str) -> Element{
        Element {
            content: ElementContent::Text(String::from(text)),
        }
    }

    // builder to add a child element
    pub fn child(mut self, e:Element) -> Self{
        if let ElementContent::Tree(ref mut content) = self.content{
            content.children.push(rcc(e));
        }
        else{
            panic!("Child can only be added to tree elements");
        }
        return self;
    }

    // builder to add attributes
    pub fn attr(mut self, key:&str, value:&str) -> Self{
        self.push_attribute(key, value);
        return self;
    }

    //Deprecated
    pub fn new_str(tag: &str, text: &str) -> Element{
        Element::new(tag).child(Element::str(text))
    }

    pub fn get_attr(&self, key: &str) -> Option<String>{
        if let ElementContent::Tree(content) = &self.content{
            //smells like I need a hashmap instead
            for (k,v) in &content.attributes{
                if k == key {
                    return Some(v.to_string());
                }
            }
        }
        return None;
    }

    pub fn push(&mut self, ptr:Rc<RefCell<Element>>){
        if let ElementContent::Tree(ref mut content) = self.content{
            content.children.push(ptr);
        }
        else{
            panic!("Element is not a tree");
        }
    }

    pub fn push_attribute(&mut self, key:&str, value:&str){
        if let ElementContent::Tree(ref mut content) = self.content{
            content.attributes.push((key.to_string(), value.to_string()));
        }
        else{
            panic!("Element is not a tree");
        }
    }

    pub fn get_attributes(&self)->&Vec<(String, String)>{
        if let ElementContent::Tree(content) = &self.content{
            return &content.attributes;
        }
        else{
            panic!("Element is not a tree");
        }
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

    pub fn get_tag(&self)->String{
        if let ElementContent::Tree(content) = &self.content{
            return content.tag.to_string();
        }
        else{
            panic!("Element is not a text element");
        }
    }

    pub fn is_text(&self)->bool{
        match &self.content{
            ElementContent::Text(_)=>true,
            _=>false
        }
    }

    pub fn is_tree(&self)->bool{
        match &self.content{
            ElementContent::Tree(_)=>true,
            _=>false
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
            ElementContent::Tree(content) =>{
                if content.children.len()==0{
                    xml.push_str(format!("<{}", content.tag).as_str());
                    for (k, v) in &content.attributes{
                        xml.push_str(format!(" {}=\"{}\"", k, v).as_str());
                    }
                    xml.push_str("/>\n");
                }
                else {
                    xml.push_str(format!("<{}", content.tag).as_str());
                    for (k, v) in &content.attributes{
                        xml.push_str(format!(" {}=\"{}\"", k, v).as_str());
                    }
                    xml.push_str(">\n");
                    for c in &content.children{
                        xml.push_str(c.borrow()._to_xml(level+1).as_str());
                    }
                    for _ in 0..level{
                        xml.push('\t');
                    }
                    xml.push_str(format!("</{}>\n", content.tag).as_str());
                }
            },
            ElementContent::Text(text) =>{
                // No escaping. I expect the text to be escaped by the application
                xml.push_str(&text.as_str());
                xml.push('\n');
            }
        }

        return xml;
    }


    // support a simple subset of xpath
    // relative path to children
    // text()
    pub fn get(&self, path:&str)->Option<QueryResult>{
        println!("{:?} {:?}\n\n", self, path);

        let (remaining, prefix)=consume_until_token_in_list(path, &["/"]).unwrap();


        println!("prefix {:?}  remaining {:?}\n", prefix, remaining);

        match &self.content{
            ElementContent::Tree(content) =>{
                //Special case 
                if remaining == "" && prefix == "text()" {
                    if content.children.len() == 1{
                        return content.children[0].borrow().get(prefix);
                    }
                    else{
                        return None;
                    }
                }

                // find child that match 'prefix'
                let mut child:Option<Rc<RefCell<Element>>> = None;
                for c in &content.children{
                    if c.borrow().get_tag() == prefix{
                        if child != None{
                            return None; // child is invalid if several childs with the same tag exist
                        }
                        else{
                            child = Some(Rc::clone(&c));
                        }
                    }
                }

                if let Some(c)=child {
                    if let Ok((subpath, _))=consume_token_in_list(remaining, &["/"]){
                        if subpath.len() ==0{
                            return Some(QueryResult::Elt(Rc::clone(&c)));
                        }
                        else{
                            return c.borrow().get(subpath);
                        }
                    }
                }
            },
            ElementContent::Text(text) =>{
                if remaining == "" && prefix == "text()" {    
                    return Some(QueryResult::Text(text.to_string()));
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
    if let ElementContent::Tree(content) = &element.content{
        let mut iter=content.children.iter();
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

        let elt2 =Element::str("text");
        assert!(elt2.is_text());
        assert!(!elt2.is_tree());
    }

    #[test]
    fn test_recurse() {
        let tree = rcc(Element::new("sequencediagram:header")
            .child(Element::new("name")
                .child(Element::str("alice"))
            )
            .child(Element::new("name")
                .child(Element::str("bob"))
            )
        );
            
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
        let element=Element::str("lorem ipsum");
        assert_eq!(element.to_xml(), String::from("lorem ipsum\n"));
    }

    #[test]
    fn test_xml3() {
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::str("lorem ipsum"))));
        assert_eq!(element.to_xml(), String::from("<p>\n\tlorem ipsum\n</p>\n"));
    }

    #[test]
    fn test_xml4() {
        let mut element=Element::new("img");
        element.push_attribute("src", "surprised_pikachu.jpg");
        assert_eq!(element.to_xml(), String::from("<img src=\"surprised_pikachu.jpg\"/>\n"));
    }

    #[test]
    fn test_xml5() {
        let mut body=Element::new("body");
        let mut element=Element::new("p");
        element.push(Rc::new(RefCell::new(Element::str("lorem ipsum"))));
        body.push(Rc::new(RefCell::new(element)));

        assert_eq!(body.to_xml(), String::from("<body>\n\t<p>\n\t\tlorem ipsum\n\t</p>\n</body>\n"));
    }

    #[test]
    fn test_xml6() {
        let mut element=Element::new("p")
            .attr("class", "american")
            .child(Element::str("lorem ipsum"));

        assert_eq!(element.to_xml(), String::from("<p class=\"american\">\n\tlorem ipsum\n</p>\n"));
    }



    #[test]
    fn test_xpath1() {
        let mut body=Element::new("body")
            .child(
                Element::new("p")
                    .child(
                        Element::new("span")
                            .child(
                                Element::str("lorem ipsum")
                            )
                    )
            );


        assert_eq!(body.get("p/span/text()"), Some(QueryResult::Text(String::from("lorem ipsum"))));
    }

}
