use simple_xml_builder::XMLElement;


pub fn create_svg(width:usize, height:usize)->XMLElement{
    // <svg version="1.1" xmlns="http://www.w3.org/2000/svg" 
    //  xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 462.219 381.915" >
    let mut elt: XMLElement = XMLElement::new("svg");
    elt.add_attribute("version", "1.1");
    elt.add_attribute("xmlns", "http://www.w3.org/2000/svg");
    elt.add_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
    elt.add_attribute("viewBox", &format!("0 0 {} {}", width, height));
    return elt;
}



pub fn create_style()->String{
    return String::from("fill:#ede7d9;fill-opacity:1;stroke:#2e282a;stroke-width:0.26458332;stroke-opacity:1")
}


pub fn create_rect(x:f32, y:f32, width:f32, height:f32, style:&str, r:Option<f32>, id:Option<&str>)->XMLElement{
      // <rect
      //    style="fill:#ede7d9;fill-opacity:1;stroke:#2e282a;stroke-width:0.26458332;stroke-opacity:1"
      //    id="rect4566"
      //    width="27.999998"
      //    height="18"
      //    x="60"
      //    y="111"
      //    ry="0.80000001" />
    let mut elt: XMLElement = XMLElement::new("rect");
    elt.add_attribute("x", &format!("{:.4}", x));
    elt.add_attribute("y", &format!("{:.4}", y));
    elt.add_attribute("width", &format!("{:.4}", width));
    elt.add_attribute("height", &format!("{:.4}", height));
    elt.add_attribute("style", style);
    if let Some(ry)=r{
        elt.add_attribute("ry", &format!("{:.4}", ry));
    }
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }

    return elt;
}


pub fn create_translate_group(x:f32, y:f32, id:Option<&str>) ->XMLElement{
    //<g transform="translate(0,-97)" id="layer1">
    let mut elt: XMLElement = XMLElement::new("g");
    elt.add_attribute("transform", &format!("translate({:.4}, {:.4})", x, y));
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }
    return elt;
}

pub fn create_group(id:Option<&str>) ->XMLElement{
    //<g transform="translate(0,-97)" id="layer1">
    let mut elt: XMLElement = XMLElement::new("g");
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }
    return elt;
}


pub fn create_path(d:&str, style:&str, id:Option<&str>) ->XMLElement{
    //  <path
    // style="fill:#ede7d9;fill-opacity:1;stroke:#2e282a;stroke-width:0.26458332;stroke-opacity:1"
    // d="m 29.999912,101.00004 v 1.99988 1.99987 2.0004 h 38.000263 c 1.108001,0 1.999877,-0.8924 1.999877,-2.0004 v -1.99987 -1.99988 h -1.999877 z"
    // id="rect120-6" />
    let mut elt: XMLElement = XMLElement::new("path");
    elt.add_attribute("d", d);
    elt.add_attribute("style", style);
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }
    return elt;
}

pub fn create_text(x:f32, y:f32, style:&str, id:Option<&str>) ->XMLElement{
    //      <text
    // xml:space="preserve"
    // style="font-style:normal;font-weight:normal;font-size:3.88055556px;line-height:1.25;font-family:sans-serif;letter-spacing:0px;word-spacing:0px;fill:#000000;fill-opacity:1;stroke:none;stroke-width:0.26458332;"
    // x="31.714447"
    // y="105.522"
    // id="text4572-2-7-3-7">
    let mut elt: XMLElement = XMLElement::new("text");
    elt.add_attribute("xml:space", "preserve");
    elt.add_attribute("style", style);
    elt.add_attribute("x", &format!("{:.4}", x));
    elt.add_attribute("y", &format!("{:.4}", y));
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }
    return elt;
}
pub fn create_tspan(x:Option<f32>, y:Option<f32>, style:Option<&str>, id:Option<&str>) ->XMLElement{
    // <tspan
    // id="tspan4570-9-0-6-5"
    // x="31.714447"
    // y="105.522"
    // style="font-size:3.88055556px;stroke-width:0.26458332;">
    let mut elt: XMLElement = XMLElement::new("text");
    if let Some(style) = style{
        elt.add_attribute("style", style);
    }
    if let Some(x) = x{
        elt.add_attribute("x", &format!("{:.4}", x));
    }
    if let Some(y) = y{
        elt.add_attribute("y", &format!("{:.4}", y));
    }
    if let Some(id)=id{
        elt.add_attribute("id", &id);
    }
    return elt;
}