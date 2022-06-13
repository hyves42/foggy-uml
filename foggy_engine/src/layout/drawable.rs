use crate::layout::*;

pub enum BoxPlacementStrategy{
	AtBox(LayoutGuid),   // Place relative to the BoxContainer of a layout container
	AtObject(DiagramGuid),  // Place relative to an other drawable element
	AtLink(LinkGuid),  // eg:Place text on an arrow
	FromToBox(LayoutGuid, LayoutGuid), // Place over several layout elements
	FromToObject (LayoutGuid, LayoutGuid),
}


// How to align the drawable relative to the objects it references
pub enum BoxAlignStrategy{
	Origin, // top or left
	Center,  
	End,   //bottom or right
}

pub enum BoxGrowStrategy{
	Shrink, // Box is as small as its content requires
	Grow,   // Box occupies as much space as it can
}


pub struct DrawableBox{
	pub strategy: BoxPlacementStrategy,
	pub translate: (u32, u32),
	pub align_h: BoxAlignStrategy,
	pub align_v: BoxAlignStrategy,
	pub margin: (u32, u32, u32, u32),
}


impl DrawableBox{
	pub fn exterior_dim(&self, d:(u32, u32))->(u32, u32){
		(d.0+self.margin.0+self.margin.2, d.1+self.margin.1+self.margin.3)
	}

	pub fn item_pos(&self, p:(u32, u32))->(u32, u32){
		(p.0+self.margin.0, p.1+self.margin.1)
	}
}



