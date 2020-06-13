use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;
use datatypes::*;
use std::cmp;
use parseutils::*;
use simple_xml_builder::XMLElement;
use builders::svgbuilder::*;

static FONT_SIZE:f32=4.0; // in SVG user coordinates. This is the size from baseline to baseline
static FONT_INTERLINE:f32=6.0; // in SVG user coordinates.

static LIFELINE_WIDTH:f32=0.1;
static ACTIVATION_BOX_WIDTH:f32=4.0;

static PARTICIPANT_BOX_WIDTH:f32=28.0;
static PARTICIPANT_BOX_HEIGHT:f32=18.0;

static CHAR_AVERAGE_RATIO:f32=0.6; // for test block size estimations
static CHAR_AVERAGE_RATIO_BOLD:f32=0.67; // for test block size estimations


static PARTICIPANTS_TYPES: [&'static str;7] = [
    "participant",
    "actor", 
    "boundary", 
    "control", 
    "entity", 
    "database", 
    "collections"
];

pub struct SequenceDiagramBuilder{

}

struct ParticipantExtraInfo{
    index:isize,
    depth:usize,
    right_gap:f32,
    x:f32
}
impl ParticipantExtraInfo{
    pub fn new(index:isize, depth:usize)->ParticipantExtraInfo{
        ParticipantExtraInfo{
            index:index,
            depth:depth,
            right_gap:0.0,
            x:0.0
        }
    }
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

    pub fn estimate_arrow_dimensions(element:Rc<RefCell<Element>>)->(f32, f32){
        let elt = element.borrow();

        if elt.get_attr("type").unwrap_or_default() != "arrow"{
            return (0.0,0.0);
        }
        match elt.children.first(){ // text is not mandatory on arrows
            None => return (0.0,0.0),
            Some(e) =>{
                let (w, h) = Self::estimate_text_size(Rc::clone(e));
                return (w+4.0, h+4.0);
            }
        }
    }

    pub fn generate_svg (&mut self, description: &[Rc<RefCell<Element>>])->Result<String, String>{
        if description.len()!=2{
            return Err(String::from("Bad format for input data"));
        }
        //First element shall be the header
        let header:Rc<RefCell<Element>>=Rc::clone(&description[0]);
        //Second element shall be the header
        let content:Rc<RefCell<Element>>=Rc::clone(&description[1]);

        // Build a list of all participants
        // I need a list ordered by alias for fast lookup: participant + y
        let mut participants_map:HashMap<String, (Rc<RefCell<Element>>, ParticipantExtraInfo)>=HashMap::new(); 
        // and also a list ordered by declaration order: participant + depth
        let mut participants_list:Vec<Rc<RefCell<Element>>>=vec![];
        let mut index:isize=0;

        //1st pass on sequence : check
        // - list of participants
        // - text length between participants
        // - notes, their relative position and the size of text inside
        recurse_element_tree(Rc::clone(&header), 
            |e, d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.etype == ElementType::StructureType 
                    && PARTICIPANTS_TYPES.contains(&elt.get_attr("type").unwrap_or_default().as_str())
                    && elt.get_attr("alias") != None{

                    participants_list.push(Rc::clone(&e));
                    participants_map.insert(elt.get_attr("alias").unwrap(), 
                        (Rc::clone(&e), ParticipantExtraInfo::new(index, d)));
                    index +=1;
                }
            }
        );

        let mut document_height:f32=0.0;

        recurse_element_tree(Rc::clone(&content), 
            |e, d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.etype == ElementType::StructureType 
                    && elt.get_attr("type").unwrap_or_default() == "arrow"{
                    let origin=elt.get_attr("origin");
                    let target=elt.get_attr("target");
                    if origin==None || target==None{
                        return;
                    }
                    let origin_str = origin.unwrap();
                    let target_str = target.unwrap();

                    let target_index:isize={
                        if let Some ((_, target_info)) = participants_map.get(target_str.as_str()){
                            target_info.index
                        }
                        else{
                            -1
                        }
                    };

                    if let Some ((_, origin_info)) = participants_map.get_mut(origin_str.as_str()){
                        // arrow to self : same index
                        // arrow between neighbour participants: index differ by +-1
                        // other arrows are longer and don't dictate the space between neighbours
                        if origin_info.index == target_index
                            || origin_info.index == target_index+1
                            || origin_info.index == target_index-1{
                            let (w,h) = Self::estimate_arrow_dimensions(Rc::clone(&e));
                            origin_info.right_gap =f32::max(origin_info.right_gap, w);
                            document_height+=h;

                        }
                    }

                }
            }
        );

        let mut xml_stack:Vec<&XMLElement>=vec![];
        let mut depth=0;


        //2nd pass
        // iteratively build svg content
        // Build header
        let mut document_root=create_svg(1000, 1000);
        xml_stack.push(&document_root);
        {
            let header=create_group(Some("header"));
            xml_stack.push(&header);
            document_root.add_child(header);
        }
        let mut x = 20.0;
        recurse_element_tree(Rc::clone(&header), 
            |e, d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.etype == ElementType::StructureType 
                    && PARTICIPANTS_TYPES.contains(&elt.get_attr("type").unwrap_or_default().as_str())
                    && elt.get_attr("alias") != None{

                    if let Some ((_, info)) = participants_map.get_mut(&elt.get_attr("alias").unwrap()){
                        info.x = x;
                        // set a minimum gap if none was set by text length
                        info.right_gap = f32::max(info.right_gap, 30.0);
                        x+=info.right_gap;

                        let rect=create_rect(info.x-PARTICIPANT_BOX_WIDTH/2.0, 0.0,
                            PARTICIPANT_BOX_WIDTH,
                            PARTICIPANT_BOX_HEIGHT, 
                            "fill:#ede7d9;fill-opacity:1;stroke:#2e282a;stroke-width:0.26458332;stroke-opacity:1",
                            Some(0.8), None);

                        // this one won't hav3e any children so it doesn't need to be pushed to stack
                        xml_stack.last_mut().unwrap().add_child(rect);

                    }

                }
            }
        );

        //TODO no support for boxes yet


        // then enclose in an svg document


        return Ok(String::new());
    }
}


