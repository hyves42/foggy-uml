use crate::render::*;


const BOX_LIGHT_HORIZONTAL:char = '\u{2500}'; // ─
const BOX_HEAVY_HORIZONTAL:char = '\u{2501}'; // ━
const BOX_LIGHT_VERTICAL:char = '\u{2502}'; // │
const BOX_HEAVY_VERTICAL:char = '\u{2503}'; // ┃
const BOX_LIGHT_TRIPLE_DASH_HORIZONTAL:char = '\u{2504}'; // ┄
const BOX_HEAVY_TRIPLE_DASH_HORIZONTAL:char = '\u{2505}'; // ┅
const BOX_LIGHT_TRIPLE_DASH_VERTICAL:char = '\u{2506}'; //   ┆
const BOX_HEAVY_TRIPLE_DASH_VERTICAL:char = '\u{2507}'; //   ┇
const BOX_LIGHT_QUADRUPLE_DASH_HORIZONTAL:char = '\u{2508}'; //  ┈
const BOX_HEAVY_QUADRUPLE_DASH_HORIZONTAL:char = '\u{2509}'; //  ┉
const BOX_LIGHT_QUADRUPLE_DASH_VERTICAL:char = '\u{250A}'; //    ┊
const BOX_HEAVY_QUADRUPLE_DASH_VERTICAL:char = '\u{250B}'; //    ┋
const BOX_LIGHT_DOWN_AND_RIGHT:char = '\u{250C}'; // ┌  
const BOX_HEAVY_DOWN_AND_RIGHT:char = '\u{250F}'; // ┏   
const BOX_LIGHT_DOWN_AND_LEFT:char = '\u{2510}'; // ┐   
const BOX_HEAVY_DOWN_AND_LEFT:char = '\u{2513}'; // ┓   
const BOX_LIGHT_UP_AND_RIGHT:char = '\u{2514}'; // └   
const BOX_HEAVY_UP_AND_RIGHT:char = '\u{2517}'; // ┗   
const BOX_LIGHT_UP_AND_LEFT:char = '\u{2518}'; // ┘   
const BOX_HEAVY_UP_AND_LEFT:char = '\u{251B}'; // ┛   
const BOX_LIGHT_VERTICAL_AND_RIGHT:char = '\u{251C}'; // ├   
const BOX_HEAVY_VERTICAL_AND_RIGHT:char = '\u{2523}'; // ┣   
const BOX_LIGHT_VERTICAL_AND_LEFT:char = '\u{2524}'; // ┤   
const BOX_HEAVY_VERTICAL_AND_LEFT:char = '\u{252B}'; // ┫   
const BOX_LIGHT_DOWN_AND_HORIZONTAL:char = '\u{252C}'; // ┬   
const BOX_HEAVY_DOWN_AND_HORIZONTAL:char = '\u{2533}'; // ┳   
const BOX_LIGHT_UP_AND_HORIZONTAL:char = '\u{2534}'; // ┴   
const BOX_HEAVY_UP_AND_HORIZONTAL:char = '\u{253B}'; // ┻   
const BOX_LIGHT_VERTICAL_AND_HORIZONTAL:char = '\u{253C}'; // ┼
const BOX_HEAVY_VERTICAL_AND_HORIZONTAL:char = '\u{254B}'; // ╋   
const BOX_LIGHT_DOUBLE_DASH_HORIZONTAL:char = '\u{254C}'; // ╌   
const BOX_HEAVY_DOUBLE_DASH_HORIZONTAL:char = '\u{254D}'; // ╍   
const BOX_LIGHT_DOUBLE_DASH_VERTICAL:char = '\u{254E}'; // ╎   
const BOX_HEAVY_DOUBLE_DASH_VERTICAL:char = '\u{254F}'; // ╏   
const BOX_LIGHT_ARC_DOWN_AND_RIGHT:char = '\u{256D}'; // ╭   
const BOX_LIGHT_ARC_DOWN_AND_LEFT:char = '\u{256E}'; // ╮   
const BOX_LIGHT_ARC_UP_AND_LEFT:char = '\u{256F}'; // ╯   
const BOX_LIGHT_ARC_UP_AND_RIGHT:char = '\u{2570}'; // ╰   



pub struct AsciiRenderer {
    scene: Vec<Vec<char>>,
    width: usize,
    height: usize,
}


fn merge_char(new:char, old:char) -> char{
    match new{
        BOX_LIGHT_HORIZONTAL=> match old{ //─
            BOX_LIGHT_VERTICAL               => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_RIGHT         => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_LEFT          => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_RIGHT           => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_LEFT            => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_HORIZONTAL    => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL      => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT     => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_LEFT      => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT        => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_RIGHT       => BOX_LIGHT_UP_AND_HORIZONTAL,
            _ => new,
        }, 
        BOX_LIGHT_VERTICAL=> match old{ //│
            BOX_LIGHT_HORIZONTAL             => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_RIGHT         => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_LEFT          => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_UP_AND_RIGHT           => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_UP_AND_LEFT            => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_VERTICAL_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_VERTICAL_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_HORIZONTAL    => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_ARC_DOWN_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_LEFT        => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_RIGHT       => BOX_LIGHT_VERTICAL_AND_RIGHT,
            _ => new,
        },
        BOX_LIGHT_DOWN_AND_RIGHT=> match old { //┌
            BOX_LIGHT_HORIZONTAL             => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL               => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_LEFT          => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_RIGHT           => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_UP_AND_LEFT            => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_VERTICAL_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_HORIZONTAL    => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_LEFT      => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT        => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_RIGHT       => BOX_LIGHT_VERTICAL_AND_RIGHT,
            _ => new,
        },
        BOX_LIGHT_DOWN_AND_LEFT=> match old{ //┐
            BOX_LIGHT_HORIZONTAL          => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL            => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_RIGHT      => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_RIGHT        => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_LEFT         => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_VERTICAL_AND_RIGHT  => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_LEFT   => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_HORIZONTAL => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL   => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT  => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT     => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_RIGHT    => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            _ => new,
        },  
        BOX_LIGHT_UP_AND_RIGHT=> match old{ //└
            BOX_LIGHT_HORIZONTAL          => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL            => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_LEFT         => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT  => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_VERTICAL_AND_LEFT   => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_HORIZONTAL => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL   => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT  => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_ARC_DOWN_AND_LEFT   => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT     => BOX_LIGHT_UP_AND_HORIZONTAL,
            _ => new,
        },
        BOX_LIGHT_UP_AND_LEFT=> match old{ //┘
            BOX_LIGHT_HORIZONTAL             => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL               => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_RIGHT         => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_LEFT          => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_UP_AND_RIGHT           => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_HORIZONTAL    => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL      => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT        => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_RIGHT       => BOX_LIGHT_UP_AND_HORIZONTAL,
            _ => new,
        }
        BOX_LIGHT_ARC_DOWN_AND_RIGHT=> match old{ //╭
            BOX_LIGHT_HORIZONTAL              => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL                => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_LEFT           => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_RIGHT            => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_UP_AND_LEFT             => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_VERTICAL_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_HORIZONTAL     => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_LEFT       => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT         => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_RIGHT        => BOX_LIGHT_VERTICAL_AND_RIGHT,
            _ => new,
        },
        BOX_LIGHT_ARC_DOWN_AND_LEFT=> match old{ //╮
            BOX_LIGHT_HORIZONTAL             => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL               => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_RIGHT         => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_RIGHT           => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_LEFT            => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_VERTICAL_AND_RIGHT     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_LEFT      => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_HORIZONTAL    => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL=> BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT     => BOX_LIGHT_DOWN_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT        => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_RIGHT       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            _ => new,
        },
        BOX_LIGHT_ARC_UP_AND_LEFT=> match old{ //╯
            BOX_LIGHT_HORIZONTAL              => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL                => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_RIGHT          => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_LEFT           => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_UP_AND_RIGHT            => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_DOWN_AND_HORIZONTAL     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL       => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_LEFT,
            BOX_LIGHT_ARC_UP_AND_RIGHT        => BOX_LIGHT_UP_AND_HORIZONTAL,
            _ => new,
        },
        BOX_LIGHT_ARC_UP_AND_RIGHT=> match old{ //╰
            BOX_LIGHT_HORIZONTAL              => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL                => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_RIGHT          => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_DOWN_AND_LEFT           => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_LEFT             => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_VERTICAL_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_DOWN_AND_HORIZONTAL     => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_UP_AND_HORIZONTAL       => BOX_LIGHT_UP_AND_HORIZONTAL,
            BOX_LIGHT_VERTICAL_AND_HORIZONTAL => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_DOWN_AND_RIGHT      => BOX_LIGHT_VERTICAL_AND_RIGHT,
            BOX_LIGHT_ARC_DOWN_AND_LEFT       => BOX_LIGHT_VERTICAL_AND_HORIZONTAL,
            BOX_LIGHT_ARC_UP_AND_LEFT         => BOX_LIGHT_UP_AND_HORIZONTAL,
            _ => new,
        },
        _ => new
    }
}


impl AsciiRenderer {
    pub fn new() -> Self {
        AsciiRenderer {
            scene: vec![],
            width: 0,
            height: 0,
        }
    }

    fn set_char(&mut self, x: usize, y: usize, c: char) {
        if let Some(line) = self.scene.get_mut(y) {
            line[x] = c;
        }
        
    }


    fn get_char(&self, x: usize, y: usize) -> char {
        let mut out: char = ' ';
        if let Some(line) = self.scene.get(y) {
            if let Some(c) = line.get(x){
                out = *c;
            }
        }
        return out;
    }

    fn draw_rectangle(&mut self, x: usize, y: usize, w: usize, h: usize, round:bool){
        if w == 0  || h == 0{
            //Nothing to draw
            return;
        }
        // draw corners
        if round {
            {
                let c = self.get_char(x, y);
                self.set_char(x, y, merge_char(BOX_LIGHT_ARC_DOWN_AND_RIGHT, c));
            }
            {
                let c = self.get_char(x+w-1, y);
                self.set_char(x+w-1, y, merge_char(BOX_LIGHT_ARC_DOWN_AND_LEFT, c));
            }
            {
                let c = self.get_char(x, y+h-1);
                self.set_char(x, y+h-1, merge_char(BOX_LIGHT_ARC_UP_AND_RIGHT, c));
            }
            {
                let c = self.get_char(x+w-1, y+h-1);
                self.set_char(x+w-1, y+h-1, merge_char(BOX_LIGHT_ARC_UP_AND_LEFT, c));
            }
        }
        else{
            {
                let c = self.get_char(x, y);
                self.set_char(x, y, merge_char(BOX_LIGHT_DOWN_AND_RIGHT, c));
            }
            {
                let c = self.get_char(x+w-1, y);
                self.set_char(x+w-1, y, merge_char(BOX_LIGHT_DOWN_AND_LEFT, c));
            }
            {
                let c = self.get_char(x, y+h-1);
                self.set_char(x, y+h-1, merge_char(BOX_LIGHT_UP_AND_RIGHT, c));
            }
            {
                let c = self.get_char(x+w-1, y+h-1);
                self.set_char(x+w-1, y+h-1, merge_char(BOX_LIGHT_UP_AND_LEFT, c));
            }
        }

        // draw top+bottom line
        for i in x+1..x+w-1{
            // not the same rules as merge_char
            if self.get_char(i, y) == BOX_LIGHT_VERTICAL {
                self.set_char(i, y, BOX_LIGHT_UP_AND_HORIZONTAL);
            }
            else{
                self.set_char(i, y, BOX_LIGHT_HORIZONTAL);
            }

            if self.get_char(i, y+h-1) == BOX_LIGHT_VERTICAL {
                self.set_char(i, y+h-1, BOX_LIGHT_DOWN_AND_HORIZONTAL);
            }
            else{
                self.set_char(i, y+h-1, BOX_LIGHT_HORIZONTAL);
            }
        }

        // draw left+right line
        for i in y+1..y+h-1{
            // not the same rules as merge_char
            if self.get_char(x, i) == BOX_LIGHT_HORIZONTAL {
                self.set_char(x, i, BOX_LIGHT_VERTICAL_AND_LEFT)
            } else {
                self.set_char(x, i, BOX_LIGHT_VERTICAL)
            }

            if self.get_char(x+w-1, i) == BOX_LIGHT_HORIZONTAL {
                self.set_char(x+w-1, i, BOX_LIGHT_VERTICAL_AND_RIGHT);
            }
            else{
                self.set_char(x+w-1, i, BOX_LIGHT_VERTICAL);
            }
        }

        // Fill inside
        for i in x+1..x+w-1{
            for j in y+1..y+h-1{
                self.set_char(i, j, ' ');
            }
        }
    }


    fn draw_text(&mut self, text: &str, x: usize, y: usize){
        let mut i:usize = 0;
        for c in text.chars(){
            self.set_char(x+i, y, c);
            i+=1;
        }
    }

    fn draw_line(&mut self, x1:usize, y1:usize, x2:usize, y2:usize){
        // horizontal line
        if y1 == y2{
            for i in x1..x2+1{
                self.set_char(i, y1, BOX_LIGHT_HORIZONTAL);
            }
        }

        // vertical line
        if x1 == x2{
            for i in y1..y2+1{
                self.set_char(x1, i, BOX_LIGHT_VERTICAL);
            }
        }

        // other not supported
    } 


    fn draw_arrow(&mut self, x1:usize, y1:usize, x2:usize, y2:usize, end1:bool, end2:bool){
        self.draw_line(x1, y1, x2, y2);

        // horizontal line
        if y1 == y2{
            if x1 > x2{
                if end1{
                    self.set_char(x1, y1, '▷');
                }
                if end2{
                    self.set_char(x2, y2, '◁');   
                }
            }
            else{
                if end1{
                    self.set_char(x1, y1, '◁');
                }
                if end2{
                    self.set_char(x2, y2, '▷');   
                }
            }
        }

        // vertical line
        if x1 == x2{
            if y1 > y2{
                if end1{
                    self.set_char(x1, y1, '▽');
                }
                if end2{
                    self.set_char(x2, y2, '△');   
                }
            }
            else{
                if end1{
                    self.set_char(x1, y1, '△');
                }
                if end2{
                    self.set_char(x2, y2, '▽');   
                }
            }
        }
        // other not supported
    }
}

impl Renderer for AsciiRenderer {
    fn init_scene(&mut self, width: usize, height: usize) {
        self.scene.clear();

        for _ in 0..height {
            let mut v:Vec<char> = Vec::with_capacity(width);
            for _ in 0..width{
                v.push(' ');
            } 
            self.scene.push(v);
        }
        self.width = width;
        self.height = height;
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for line in self.scene.iter() {
            for c in line.iter(){
                out.push(*c);
            }
            out.push('\n');
        }
        return out;
    }

    fn draw(&mut self, object: &RenderableItem) {
        match object{
            RenderableItem::Rectangle{x, y, w, h} => self.draw_rectangle(*x, *y, *w, *h, false),
            RenderableItem::RoundRectangle{x, y, w, h} => self.draw_rectangle(*x, *y, *w, *h, true),
            RenderableItem::Text{text, x, y} => self.draw_text(text, *x, *y),
            RenderableItem::Line{x1, y1, x2, y2} => self.draw_line(*x1, *y1, *x2, *y2),
            RenderableItem::Arrow {x1, y1, x2, y2, end1, end2} => self.draw_arrow(*x1, *y1, *x2, *y2, *end1, *end2),
        }
    }

    fn text_dimension(&self, text: &str, bold: bool, italic: bool) -> (u32, u32) {
        let len = text.chars().count();
        return (len as u32, 1);
    }

    fn box_min_dimensions(&self) -> (u32, u32) {
        return (3, 3);
    }

    fn line_keepout(&self) -> u32 {
        return 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(3, 3);
        let out = renderer.render();
        assert_eq!(out, "   \n   \n   \n");
        assert_eq!(' ', renderer.get_char(0, 0));
        assert_eq!(' ', renderer.get_char(2, 2));
        assert_eq!(' ', renderer.get_char(5, 2));
        assert_eq!(' ', renderer.get_char(2, 5));
    }

    #[test]
    fn test_set_char() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(3, 3);
        renderer.set_char(2, 2, '#');

        assert_eq!(renderer.scene[2].len(),3);
        assert_eq!(renderer.get_char(2, 2), '#');

        let out = renderer.render();
        assert_eq!(out, "   \n   \n  #\n");

        renderer.set_char(1, 1, '\u{2603}');
        let out2 = renderer.render();
        assert_eq!(out2, "   \n \u{2603} \n  #\n");

    }

    #[test]
    fn test_draw_rectangle() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_rectangle(0, 0, 3, 3, false);

        let out = renderer.render();
        assert_eq!(out, "┌─┐ \n│ │ \n└─┘ \n    \n");
    }


    #[test]
    fn test_draw_round_rectangle() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_rectangle(0, 0, 3, 3, true);

        let out = renderer.render();
        assert_eq!(out, "╭─╮ \n│ │ \n╰─╯ \n    \n");
    }


    #[test]
    fn test_draw_text() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_text("abba", 0, 3);

        let out = renderer.render();
        assert_eq!(out, "    \n    \n    \nabba\n");
    }


    #[test]
    fn test_draw_line_vertical() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_line(0,0, 0, 3);

        let out = renderer.render();
        assert_eq!(out, "│   \n│   \n│   \n│   \n");
    }


    #[test]
    fn test_draw_line_horizontal() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_line(0,0, 3, 0);

        let out = renderer.render();
        assert_eq!(out, "────\n    \n    \n    \n");
    }


    #[test]
    fn test_draw_arrow_horizontal() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_arrow(0,0, 3, 0, true, true);
        let out = renderer.render();
        assert_eq!(out, "◁──▷\n    \n    \n    \n");
    }


    #[test]
    fn test_draw_arrow_vertical() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(4, 4);
        renderer.draw_arrow(0,0, 0, 3, true, true);

        let out = renderer.render();
        assert_eq!(out, "△   \n│   \n│   \n▽   \n");
    }


    #[test]
    fn test_draw_rectangle_on_top() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(5, 5);  
        renderer.draw_line(0,2, 4, 2);
        renderer.draw_line(2,0, 2, 4);        
        renderer.draw_rectangle(1, 1, 3, 3, false);

        let out = renderer.render();
        assert_eq!(out, "  │  \n ┌┴┐ \n─┤ ├─\n └┬┘ \n  │  \n");
    }

    #[test]
    fn test_draw_rectangle_on_top2() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(5, 5);
        renderer.draw_rectangle(0, 0, 3, 3, false);
        renderer.draw_rectangle(2, 2, 3, 3, false);

        let out = renderer.render();
        assert_eq!(out, "┌─┐  \n│ │  \n└─┼─┐\n  │ │\n  └─┘\n");
    }


    #[test]
    fn test_draw_rectangle_on_top3() {
        let mut renderer = AsciiRenderer::new();

        renderer.init_scene(5, 5);
        renderer.draw_line(1,0, 1, 4);
        renderer.draw_rectangle(1, 1, 3, 3, false);

        let out = renderer.render();
        assert_eq!(out, " │   \n ├─┐ \n │ │ \n ├─┘ \n │   \n");
    }

    // #[test]
    // fn test_draw_rectangle_on_top4() {
    //     let mut renderer = AsciiRenderer::new();

    //     renderer.init_scene(5, 5);
    //     renderer.draw_line(0,1, 4, 1);
    //     renderer.draw_rectangle(1, 1, 3, 3, false);

    //     let out = renderer.render();
    //     assert_eq!(out, "     \n─┬─┬─\n │ │ \n └─┘ \n     \n");
    // }

    #[test]
    fn test_merge_char() {
        let old ="─│┌┐└┘├┤┬┴┼╭╮╯╰";
        let new ="─│┌┐└┘╭╮╯╰";
        let matrix = [
            "─┼┬┬┴┴┬┬┴┴",
            "┼│├┤├┤├┤┤├",
            "┬├┌┬├┼╭┬┼├",
            "┬┤┬┐┼┤┬╮┤┼",
            "┴├├┼└┴├┼┴╰",
            "┴┤┼┤┴┘┼┤╯┴",
            "┼├├┼├┼├┼┼├",
            "┼┤┼┤┼┤┼┤┤┼",
            "┬┼┬┬┼┼┬┬┼┼",
            "┴┼┼┼┴┴┼┼┴┴",
            "┼┼┼┼┼┼┼┼┼┼",
            "┬├┌┬├┼╭┬┼├",
            "┬┤┬┐┼┘┬╮┤┼",
            "┴┤┼┤┴┤┼┤╯┴",
            "┴├├┼└┴├┼┴╰",
        ];


        let mut renderer = AsciiRenderer::new();

        for i in 0..old.chars().count() {
            for j in 0..new.chars().count(){
                println!("old :{}, new :{}, expected:{}", old.chars().nth(i).unwrap(), new.chars().nth(j).unwrap(), matrix[i].chars().nth(j).unwrap());
                assert_eq!(merge_char(new.chars().nth(j).unwrap(), old.chars().nth(i).unwrap()), matrix[i].chars().nth(j).unwrap());
            } 

        }
    }

}
