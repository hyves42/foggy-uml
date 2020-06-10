use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;
use datatypes::{ElementType, Element, Document};
use std::cmp;
use parseutils::*;

static FONT_SIZE:f32=4.0; // in SVG user coordinates. This is the size from baseline to baseline
static FONT_INTERLINE:f32=6.0; // in SVG user coordinates.

static LIFELINE_WIDTH:f32=0.1;
static ACTIVATIN_BOX_WIDTH:f32=4.0;

static CHAR_AVERAGE_RATIO:f32=0.6; // for test block size estimations
static CHAR_AVERAGE_RATIO_BOLD:f32=0.67; // for test block size estimations


pub struct SequenceDiagramBuilder{

}


impl SequenceDiagramBuilder{
    pub fn new()->SequenceDiagramBuilder{
        SequenceDiagramBuilder{}
    }

    // rule of thumb text size estimation
    pub fn estimate_text_size(element:Rc<RefCell<Element>>)->(f32, f32){


        let elt = element.borrow();
        if elt.etype == ElementType::StringType{
            let mut max_line_width:usize = 0;
            let mut lines_counter:usize=1;

            let mut slice:&str = &elt.value.as_str();

            while slice.len()>0{
                {
                    let (remaining, consumed) = consume_until_token_in_list(slice, &["\n"]).unwrap();
                    max_line_width = cmp::max(max_line_width, consumed.len());
                    slice=remaining;
                }
                if let Ok((remaining, consumed)) = consume_token_in_list(slice, &["\n"]){
                    lines_counter += 1;
                    slice=remaining;
                }

            }
            return (max_line_width as f32*CHAR_AVERAGE_RATIO*FONT_SIZE, lines_counter as f32*FONT_INTERLINE);
        }
        else{
            return ((0.0, 0.0));
        }

        // other elements are not supported yet. 
        // TODO add support for more complicated text trees with text format
    }




    pub fn generate_svg (&mut self, description: &[Rc<RefCell<Element>>])->Result<String, String>{
        if description.len()!=2{
            return Err(String::from("Bad format for input data"));
        }
        //First element shall be the header
        let header:Rc<RefCell<Element>>=Rc::clone(&description[0]);
        //Second element shall be the header
        let content:Rc<RefCell<Element>>=Rc::clone(&description[1]);

        // Build a map of all participants, ordered by alias
        let participants_map:HashMap<String, Rc<RefCell<Element>>>=HashMap::new(); 
        let participants_stack:Vec<Rc<RefCell<Element>>>=vec![Rc::clone(header)]


        //1st pass : check
        // - text length between participants
        // - notes, their relative position and the size of text inside


        //2nd pass
        // iteratively build svg content
        // then enclose in an svg document
        return Ok(String::new());
    }
}