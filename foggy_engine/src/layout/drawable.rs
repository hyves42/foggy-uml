use crate::layout::*;
use crate::utils::uid::*;




pub enum BoxPlacementStrategy {
    AtBox(LayoutGuid),     // Place relative to the BoxContainer of a layout container
    AtObject(DiagramGuid), // Place relative to an other drawable element
    AtLink(LinkGuid),      // eg:Place text on an arrow
    FromToBox(LayoutGuid, LayoutGuid), // Place over several layout elements
    FromToObject(LayoutGuid, LayoutGuid),
}

// How to align the drawable relative to the objects it references
pub enum BoxAlignStrategy {
    Origin, // top or left
    Center,
    End, //bottom or right
}

pub enum BoxGrowStrategy {
    Shrink, // Box is as small as its content requires
    Grow,   // Box occupies as much space as it can
}

pub struct DrawableBox {
    pub strategy: BoxPlacementStrategy,
    pub translate: (u32, u32),
    pub align_h: BoxAlignStrategy,
    pub align_v: BoxAlignStrategy,
    pub margin: (u32, u32, u32, u32),
}

impl DrawableBox {

    pub fn exterior_dim(&self, d: (u32, u32)) -> (u32, u32) {
        (
            d.0 + self.margin.0 + self.margin.2 + self.translate.0,
            d.1 + self.margin.1 + self.margin.3 + self.translate.1,
        )
    }

	// Position of the content if is is contained in a box starting at pos p=(x,y)
    pub fn item_pos(&self, p: (u32, u32)) -> (u32, u32) {
        (
        	p.0 + self.margin.0 + self.translate.0,
        	p.1 + self.margin.1 + self.translate.1
        )
    }
}



/***************************************************/

pub struct DiagramPlacement{
    pub drawables: UidStore<DrawableBox, DiagramGuid>
}

impl DiagramPlacement {
    pub fn new() -> Self {
        DiagramPlacement {
            drawables: UidStore::new(),
        }
    }

    // Returns dimensions x1,y1, x2,y2
    pub fn object_dimensions(&self, object_id: DiagramGuid, layout:&dyn LayoutDimensions) -> Option<(u32, u32, u32, u32)>{
        let mut cursor = object_id;
        let mut translate:(u32, u32) = (0, 0);
        let mut margin:(u32, u32, u32, u32) = (0, 0, 0, 0);
        let mut visited_objects:Vec<DiagramGuid> = vec![object_id];
        let mut outer = false;

        while let Some(d) = self.drawables.get(cursor) {
            translate.0 += d.translate.0;
            translate.1 += d.translate.1;
            if outer {
                margin.0 += d.margin.0;
                margin.1 += d.margin.1;
                margin.2 += d.margin.2;
                margin.3 += d.margin.3;
            }


            match d.strategy {
                BoxPlacementStrategy::AtBox(box_id) => {
                    if let Some((x1,y1,x2,y2)) = layout.box_dimensions(box_id){
                        println!("Box, dimensions {:?}", (x1,y1,x2,y2));
                        return Some((
                            x1 + translate.0 + margin.0,
                            y1 + translate.1 + margin.1,
                            x2 + translate.0 - margin.2,
                            y2 + translate.1 - margin.3
                        ));
                    }
                    else{
                        return None
                    };
                },
                BoxPlacementStrategy::AtObject(id) => {
                    if visited_objects.contains(&id){
                        return None;
                    }
                    cursor = id;
                    visited_objects.push(id);
                    outer = true;
                }
                _ => {
                    println!("Ignored drawable for rendering: {:?}", cursor);
                    return None;
                }
            }
        }
        return None;
    }


    // Size of the bounding box required to store this drawablebox
    // if the content has a size d= (w,h)
    // returns the size and the required size and the ID of tha layout box that contains the object
    pub fn object_exterior_size(&self, object_id: DiagramGuid, w:u32, h:u32) -> Option<(u32, u32, LayoutGuid)>{
        let mut cursor = object_id;
        let mut min_dim= (w,h);
        let mut visited_objects:Vec<DiagramGuid> = vec![object_id];
        let mut outer = false;



        while let Some(d) = self.drawables.get(cursor) {
            min_dim.0 += d.translate.0;
            min_dim.1 += d.translate.1;
            if outer {
                min_dim.0 += d.margin.0 + d.margin.2;
                min_dim.1 += d.margin.1 + d.margin.3;
            }

            match d.strategy {
                BoxPlacementStrategy::AtBox(id) => {
                    return Some((min_dim.0, min_dim.1, id));
                },
                BoxPlacementStrategy::AtObject(id) => {
                    if visited_objects.contains(&id){
                        return None;
                    }

                    cursor = id;
                    visited_objects.push(id);
                    outer = true;
                }
                _ => {
                    println!("Ignored drawable for exterior size computation: {:?}", cursor);
                    return None;
                }
            }
        }
        None
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use diagram::*;

    struct TestDimensionProvider{
        pub id: GuidManager<LayoutGuid>,
        pub boxes: UidStore<(u32,u32,u32,u32), LayoutGuid>,
    }
    impl TestDimensionProvider{
        pub fn new() -> Self {
            TestDimensionProvider {
                id: GuidManager::new(),
                boxes: UidStore::new(),
            }
        }
        pub fn insert(&mut self, x:u32, y:u32, w:u32, h:u32) -> LayoutGuid {
            let id = self.id.get();
            self.boxes.insert(id, (x,y,w,h)).unwrap();
            return id;
        }
    }

    impl LayoutDimensions for TestDimensionProvider{
        fn box_dimensions(&self, box_id: LayoutGuid) -> Option<(u32, u32, u32, u32)> {
            let b = self.boxes.get(box_id)?;
            Some((b.0, b.1, b.2, b.3))
        }
    }




    #[test]
    fn no_circular_dep1() {
        let layout = TestDimensionProvider::new();
        let mut diagram = Diagram::new();
        let diagram_root_id = diagram.node_ids.get();


        let mut placement = DiagramPlacement::new();
        placement.drawables
            .insert(
                diagram_root_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(diagram_root_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(diagram_root_id, 1,1), None);
        assert_eq!(placement.object_dimensions(diagram_root_id, &layout), None);
    }


    #[test]
    fn no_circular_dep2() {
        let layout = TestDimensionProvider::new();
        let mut diagram = Diagram::new();
        let diagram_root_id = diagram.node_ids.get();
        let block_id = diagram.node_ids.get();

        let mut placement = DiagramPlacement::new();
        placement.drawables
            .insert(
                diagram_root_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(block_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        placement.drawables
            .insert(
                block_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(diagram_root_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();


        assert_eq!(placement.object_exterior_size(block_id, 1,1), None);
        assert_eq!(placement.object_dimensions(block_id, &layout), None);
    }

    #[test]
    fn layout_normal() {
        let mut layout = TestDimensionProvider::new();
        let box_id = layout.insert(30, 40, 50, 60);
        let mut diagram = Diagram::new();
        let diagram_root_id = diagram.node_ids.get();
        let block_id = diagram.node_ids.get();

        let mut placement = DiagramPlacement::new();
        placement.drawables
            .insert(
                diagram_root_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtBox(box_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(diagram_root_id, 1,1), Some((1,1,box_id)));
        assert_eq!(placement.object_dimensions(diagram_root_id, &layout), Some((30, 40, 50, 60)));

        placement.drawables
            .insert(
                block_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(diagram_root_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(block_id, 1,1), Some((1,1,box_id)));
        assert_eq!(placement.object_dimensions(block_id, &layout), Some((30, 40, 50, 60)));
    }


    #[test]
    fn layout_translate() {
        let mut layout = TestDimensionProvider::new();
        let box_id = layout.insert(30, 40, 50, 60);
        let mut diagram = Diagram::new();
        let diagram_root_id = diagram.node_ids.get();
        let block_id1 = diagram.node_ids.get();
        let block_id2 = diagram.node_ids.get();

        let mut placement = DiagramPlacement::new();
        placement.drawables
            .insert(
                diagram_root_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtBox(box_id),
                    translate: (3, 4),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(diagram_root_id, 1,1), Some((4,5,box_id)));
        assert_eq!(placement.object_dimensions(diagram_root_id, &layout), Some((33,44,53,64)));

        placement.drawables
            .insert(
                block_id1,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(diagram_root_id),
                    translate: (3, 4),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(block_id1, 1,1), Some((7,9,box_id)));
        assert_eq!(placement.object_dimensions(block_id1, &layout), Some((36,48,56,68)));


        placement.drawables
            .insert(
                block_id2,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(block_id1),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(block_id2, 1,1), Some((7,9,box_id)));
        assert_eq!(placement.object_dimensions(block_id2, &layout), Some((36,48,56,68)));
    }


    #[test]
    fn layout_margin() {
        let mut layout = TestDimensionProvider::new();
        let box_id = layout.insert(30, 40, 50, 60);
        let mut diagram = Diagram::new();
        let diagram_root_id = diagram.node_ids.get();
        let block_id1 = diagram.node_ids.get();
        let block_id2 = diagram.node_ids.get();

        let mut placement = DiagramPlacement::new();
        placement.drawables
            .insert(
                diagram_root_id,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtBox(box_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (1, 2, 3, 4),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(diagram_root_id, 1,1), Some((1,1,box_id)));
        assert_eq!(placement.object_dimensions(diagram_root_id, &layout), Some((30, 40, 50, 60)));

        placement.drawables
            .insert(
                block_id1,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(diagram_root_id),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (1, 1, 1, 1),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(block_id1, 1,1), Some((5,7,box_id)));
        assert_eq!(placement.object_dimensions(block_id1, &layout), Some((31,42,47,56)));


        placement.drawables
            .insert(
                block_id2,
                DrawableBox {
                    strategy: BoxPlacementStrategy::AtObject(block_id1),
                    translate: (0, 0),
                    align_h: BoxAlignStrategy::Center,
                    align_v: BoxAlignStrategy::Center,
                    margin: (0, 0, 0, 0),
                },
            )
            .unwrap();

        assert_eq!(placement.object_exterior_size(block_id2, 1,1), Some((7,9,box_id)));
        assert_eq!(placement.object_dimensions(block_id2, &layout), Some((32,43,46,55)));
    }

}