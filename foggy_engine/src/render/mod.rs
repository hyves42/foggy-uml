pub mod asciirender;


pub enum TextSize {
    Normal,
}

pub enum RenderableItem {
    Rectangle { x: usize, y: usize, w: usize, h: usize },
    RoundRectangle { x: usize, y: usize, w: usize, h: usize },
    Text { text: String, x: usize, y: usize },
    Line { x1:usize, y1:usize, x2:usize, y2:usize },
    Arrow { x1:usize, y1:usize, x2:usize, y2:usize, end1:bool, end2:bool },
}

pub trait Renderer {
    fn init_scene(&mut self, width: usize, height: usize);

    fn draw(&mut self, object: &RenderableItem);

    fn text_dimension(&self, text: &str, bold: bool, italic: bool) -> (u32, u32);

    fn box_min_dimensions(&self) -> (u32, u32);

    fn line_keepout(&self) -> u32;

    fn render(&self) -> String;
}

pub struct Engine {}

impl Engine {
    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        let objects: Vec<RenderableItem> = vec![
            RenderableItem::Rectangle {
                x: 0,
                y: 0,
                w: 3,
                h: 3,
            },
            RenderableItem::Rectangle {
                x: 0,
                y: 0,
                w: 3,
                h: 3,
            },
            RenderableItem::Rectangle {
                x: 0,
                y: 0,
                w: 3,
                h: 3,
            },
        ];

        renderer.init_scene(100, 100);
        for obj in objects {
            renderer.draw(&obj);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct DummyRender {
        out: String,
    }

    impl Renderer for DummyRender {
        fn init_scene(&mut self, width: usize, height: usize) {}

        fn draw(&mut self, object: &RenderableItem) {}

        fn text_dimension(&self, text: &str, bold: bool, italic: bool) -> (u32, u32) {
            (0, 0)
        }

        fn box_min_dimensions(&self) -> (u32, u32) {
            (0, 0)
        }

        fn line_keepout(&self) -> u32 {
            0
        }

        fn render(&self) -> String{
            String::new()
        }

    }

    pub struct DummyRender2 {}

    impl Renderer for DummyRender2 {
        fn init_scene(&mut self, width: usize, height: usize) {}

        fn draw(&mut self, object: &RenderableItem) {}

        fn text_dimension(&self, text: &str, bold: bool, italic: bool) -> (u32, u32) {
            (0, 0)
        }

        fn box_min_dimensions(&self) -> (u32, u32) {
            (0, 0)
        }
        fn line_keepout(&self) -> u32 {
            0
        }

        fn render(&self) -> String{
            String::new()
        }
    }

    #[test]
    fn test() {
        // Just check that my lifetimes are OK
        let mut renderer: DummyRender = DummyRender { out: String::new() };
        let engine: Engine = Engine {};

        engine.render(&mut renderer);
        engine.render(&mut renderer);
        engine.render(&mut renderer);
    }
}
