

enum BoxStrategy{
	AtBox(usize),   // Place relative to layout container
	AtObject(usize),  // Place relative to an other drawable element
	FromToBox(usize, usize),
	FromToObject (usize, usize),
}

enum AlignStrategy{
	Origin, // top or left
	Center,  
	End,   //bottom or right
}

struct DrawableBox{
	strategy: BoxStrategy,
	translate: (f32, f32),
	align_h: AlignStrategy,
	align_v: AlignStrategy,
	margin: (f32, f32, f32, f32)
}

impl DrawableBox{

	pub fn get_dimensions(&self) -> (f32,f32,f32,f32){
		
	}
}