use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;
use crate::datatypes::*;
use std::cmp;
use crate::parseutils::*;
use crate::builders::svgbuilder::*;

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


//handles left and right directions
fn create_arrow(x_origin:f32, x_target:f32, y:f32, id:Option<&str>)->Element{
    // <g
    //    id="g1017">
    //   <path
    //      style="fill:none;stroke:#2e282a;stroke-width:0.40000001;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
    //      d="m 76,143 h 36"
    //      id="path4562" />
    //   <path
    //      style="fill:#857970;fill-opacity:1;stroke:#2e282a;stroke-width:0.40000001;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
    //      d="m 108,142 4,1 -4,1 z"
    //      id="path4564" />
    // </g>

    let mut group=create_group(id);
    //horizontal line
    let path1=create_path(format!("m {},{} h {}", x_origin, y, x_target-x_origin).as_str(), 
        "fill:none;stroke:#2e282a;stroke-width:0.40000001;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1", 
        None);
    let path2 = create_path(match x_origin < x_target{
            true => format!("m {},{} 4,1 -4,1 z", x_target-4.0, y-1.0), 
            false => format!("m {},{} -4,1 4,1 z", x_target+4.0, y-1.0)
        }.as_str(),
        "fill:#857970;fill-opacity:1;stroke:#2e282a;stroke-width:0.40000001;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1", 
        None);



    group.push(Rc::new(RefCell::new(path1)));
    group.push(Rc::new(RefCell::new(path2)));
    return group;
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
        // other elements are not supported yet. 
        // TODO add support for more complicated text trees with text format
        if !elt.is_text(){
            return (0.0, 0.0);
        }

        let mut max_line_width:usize = 0;
        let mut lines_counter:usize=1;

        let text=elt.get_text();
        let mut slice:&str = &text.as_str();

        while slice.len()>0{
            {
                let (remaining, consumed) = consume_until_token_in_list(slice, &["\n"]).unwrap();
                max_line_width = cmp::max(max_line_width, consumed.len());
                slice=remaining;
            }
            if let Ok((remaining, _)) = consume_token_in_list(slice, &["\n"]){
                lines_counter += 1;
                slice=remaining;
            }

        }
        return (max_line_width as f32*CHAR_AVERAGE_RATIO*FONT_SIZE, lines_counter as f32*FONT_INTERLINE);
    }


    pub fn estimate_arrow_dimensions(element:Rc<RefCell<Element>>)->(f32, f32){
        let elt = element.borrow();

        if elt.get_tag() != "arrow"{
            return (0.0,0.0);
            //TODO panic ?
        }
        if let ElementContent::Tree(content) = &elt.content{
            match content.children.first(){ // text is not mandatory on arrows
                None => return (6.0,6.0),
                Some(e) =>{
                    let (w, h) = Self::estimate_text_size(Rc::clone(e));
                    return (w+4.0, h+2.0);
                }
            }
        }
        return (0.0,0.0);
        //TODO panic ?
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

        //1st pass on sequence : check list of participants in header
        recurse_element_tree(Rc::clone(&header), 
            |e, d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.is_tree()
                    && PARTICIPANTS_TYPES.contains(&elt.get_tag().as_str())
                    && elt.get_attr("alias") != None{

                    participants_list.push(Rc::clone(&e));
                    participants_map.insert(elt.get_attr("alias").unwrap(), 
                        (Rc::clone(&e), ParticipantExtraInfo::new(index, d)));
                    index +=1;
                }
            }
        );

        let mut document_height:f32=PARTICIPANT_BOX_HEIGHT; // start value
        // 1st pass on content, check
        // - text length between participants
        // - height of document
        recurse_element_tree(Rc::clone(&content), 
            |e, _d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.is_tree()
                    && elt.get_tag() == "arrow"{
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
                        else{
                            let (_,h) = Self::estimate_arrow_dimensions(Rc::clone(&e));
                            document_height+=h;
                        }
                    }

                }
            }
        );
        document_height+=4.0; //for good measure

        // compute document width
        let mut x = 40.0;
        recurse_element_tree(Rc::clone(&header), 
            |e, _d|{
                let elt = e.borrow();
                if elt.is_tree()
                    && PARTICIPANTS_TYPES.contains(&elt.get_tag().as_str())
                    && elt.get_attr("alias") != None{

                    if let Some ((_, info)) = participants_map.get_mut(&elt.get_attr("alias").unwrap()){
                        info.x = x;
                        // set a minimum gap if none was set by text length
                        info.right_gap = f32::max(info.right_gap, 40.0);
                        x+=info.right_gap;
                    }
                }
            }
        );
        let document_width:f32=x+40.0;

        let mut xml_stack:Vec<Rc<RefCell<Element>>>=vec![];


        //2nd pass
        // iteratively build svg content
        // Build header
        let document_root=Rc::new(RefCell::new(create_svg(document_width, document_height)));
        xml_stack.push(Rc::clone(&document_root));
        {
            let header_g=Rc::new(RefCell::new(create_group(Some("header"))));
            xml_stack.push(Rc::clone(&header_g));
            document_root.borrow_mut().push(Rc::clone(&header_g));
        }
        // Add participant and boxes
        recurse_element_tree(Rc::clone(&header), 
            |e, _d|{
                let elt = e.borrow();
                // if element is a participant definition
                if !elt.is_tree(){
                    return;
                }// Text element are handled from their parent element

                if PARTICIPANTS_TYPES.contains(&elt.get_tag().as_str())
                    && elt.get_attr("alias") != None{

                    if let Some ((_, info)) = participants_map.get_mut(&elt.get_attr("alias").unwrap()){
                        let rect=Rc::new(RefCell::new(create_rect(info.x-PARTICIPANT_BOX_WIDTH/2.0, 0.0,
                            PARTICIPANT_BOX_WIDTH,
                            PARTICIPANT_BOX_HEIGHT, 
                            "fill:#ede7d9;fill-opacity:1;stroke:#2e282a;stroke-width:0.26458332;stroke-opacity:1",
                            Some(0.8), None)));

                        xml_stack.last().unwrap().borrow_mut().push(rect);

                        if let Some(QueryResult::Text(text)) = elt.get("name/text()"){
                            let mut text_elt=create_text(info.x, PARTICIPANT_BOX_HEIGHT/2.0-4.0, 
                                "font-style:normal;font-weight:normal;font-size:3.8px;line-height:1.25;font-family:sans-serif;letter-spacing:0px;word-spacing:0px;fill:#000000;fill-opacity:1;", 
                                None);
                            text_elt.push_attribute("text-anchor","middle");
                            text_elt.push(Rc::new(RefCell::new(Element::new_str("tspan", &text))));
                            xml_stack.last().unwrap().borrow_mut().push(Rc::new(RefCell::new(text_elt)));
                        }

                        let path= create_path(format!("m {},{} v {}", info.x, PARTICIPANT_BOX_HEIGHT, document_height-PARTICIPANT_BOX_HEIGHT).as_str(),
                            "stroke:#568259;stroke-width:0.5;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1",
                            None);
                        xml_stack.last().unwrap().borrow_mut().push(Rc::new(RefCell::new(path)));
                    }
                }
                else if elt.get_tag() == "box"{
                    panic!("not implemented");
                    //TODO no support for boxes yet

                }
            }
        );
        // remove header group from stack
        xml_stack.pop();

        {
            let content_g=Rc::new(RefCell::new(create_group(Some("content"))));
            xml_stack.push(Rc::clone(&content_g));
            document_root.borrow_mut().push(Rc::clone(&content_g));
        }
        let mut y= PARTICIPANT_BOX_HEIGHT;
        recurse_element_tree(Rc::clone(&content), 
            |e, _d|{
                let elt = e.borrow();
                // if element is a participant definition
                if elt.is_tree()
                    && elt.get_tag() == "arrow"{
                    let origin=elt.get_attr("origin");
                    let target=elt.get_attr("target");
                    if origin==None || target==None{
                        return;
                    }
                    let origin_str = origin.unwrap();
                    let target_str = target.unwrap();

                    let (_w,h) = Self::estimate_arrow_dimensions(Rc::clone(&e));

                    let target_x = 
                        if let Some ((_, target_info)) = participants_map.get(target_str.as_str()){
                            target_info.x
                        }
                        else{ 0.0 };

                    let origin_x = 
                        if let Some ((_, origin_info)) = participants_map.get(origin_str.as_str()){
                            origin_info.x 
                        }
                        else {0.0};

                    let arrow= create_arrow(origin_x, target_x, y+h, None);
                    xml_stack.last().unwrap().borrow_mut().push(Rc::new(RefCell::new(arrow)));

                    if let Some(QueryResult::Text(text)) = elt.get("text/text()"){
                        let mut text_elt=create_text(f32::min(origin_x, target_x)+4.0, y+h-1.0, 
                            "font-style:normal;font-weight:normal;font-size:3.8px;line-height:1.25;font-family:sans-serif;letter-spacing:0px;word-spacing:0px;fill:#000000;fill-opacity:1;", 
                            None);
                        text_elt.push(Rc::new(RefCell::new(Element::new_str("tspan", &text))));
                        xml_stack.last().unwrap().borrow_mut().push(Rc::new(RefCell::new(text_elt)));
                    }
                    y+=h;
                }
            }
        );
        return Ok(document_root.borrow().to_xml());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatypes::*;
    use std::rc::Rc;

    #[test]
    fn test_sequencebuilder1() {


        let elements:Vec<Rcc<Element>>=vec![
            rcc(Element::new("sequencediagram:header")
                .child(Element::new("participant")
                    .attr("alias", "alice")
                    .child(
                        Element::new("name")
                            .child(Element::str("alice"))
                    )
                )
                .child(Element::new("participant")
                    .attr("alias", "bob")
                    .child(
                        Element::new("name")
                            .child(Element::str("bob"))
                    )
                )
            ),
            rcc(Element::new("sequencediagram:content")
                .child(Element::new("arrow")
                    .attr("origin", "alice")
                    .attr("target", "bob")
                    .attr("line-style", "normal")
                    .attr("arrow-style", "normal")                
                )
                .child(Element::new("arrow")
                    .attr("target", "alice")                    
                    .attr("origin", "bob")
                    .attr("line-style", "normal")
                    .attr("arrow-style", "normal")
                )
                .child(Element::new("arrow")
                    .attr("target", "alice")                    
                    .attr("origin", "bob")
                    .attr("line-style", "normal")
                    .attr("arrow-style", "normal")
                )
            )
        ];


        
        let mut builder = SequenceDiagramBuilder::new();

        let xml = builder.generate_svg(&elements);
        assert!(!xml.is_err());
        if let Ok(s) = xml{
            println!("{}", s);
        }
    }

}