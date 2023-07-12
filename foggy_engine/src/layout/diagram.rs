use crate::layout::*;
use crate::utils::tree::*;
use crate::utils::uid::*;

// A logical representation of the diagram elements to draw:
// - nodes
// - edges/links
// - text and text format
// Basically a representation of all the things that need to be placed on the layout

// ids for linklabels
#[derive(Debug, PartialEq, From, Into, Copy, Clone)]
pub struct LinkLabelGuid(u64);

#[derive(Debug, PartialEq)]
pub enum TextFormatType {
    H1,
    H2,
    H3,
    H4,
    Bold,
    Italic,
    Strikethrough,
    Paragraph,
    List,
    Link(String),
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Basic,
    // To be expanded for specific type of blocks
}

#[derive(Debug, PartialEq)]
pub enum AnchorType {
    Invisible,
    // To be expanded
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Group,
    Text(String),
    TextFormat(TextFormatType),
    Block(BlockType),
    Lifeline, // For sequence diagrams
    Anchorpoint(AnchorType), // specific place to anchor a link to a node
              //Math ?
              //Picture ?
              //to be expanded
}

// A node in the diagram
#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
}

// A link between two nodes
// A link can also have floating ends (no origin/no destination node)
#[derive(Debug, PartialEq)]
pub struct Link {
    pub origin: Option<DiagramGuid>,      // ID of the origin node
    pub destination: Option<DiagramGuid>, // ID of the destination node
}

#[derive(Debug, PartialEq)]
pub enum LinkLabelPosition {
    AtOrigin,
    AtDestination,
    AtMiddle,
}

// Makes the logical link between a 'label' node and the link element
// Several labels can be attached to a link
#[derive(Debug, PartialEq)]
pub struct LinkLabel {
    pub link: LinkGuid,    // ID of the link to attach the label to
    pub node: DiagramGuid, // ID of the label node
    pub pos: LinkLabelPosition,
}

#[derive(Debug, PartialEq)]
pub struct Diagram {
    // All the nodes of the diagram
    pub nodes: TreeContainer<Node, DiagramGuid>,
    pub links: UidStore<Link, LinkGuid>,
    pub linklabels: UidStore<LinkLabel, LinkLabelGuid>,
    pub node_ids: GuidManager<DiagramGuid>,
    pub link_ids: GuidManager<LinkGuid>,
    pub linklabel_ids: GuidManager<LinkLabelGuid>,
    // + metadata, name, backlinks to source, etc.
}

impl Diagram {
    pub fn new() -> Self {
        Diagram {
            nodes: TreeContainer::new(),
            links: UidStore::new(),
            linklabels: UidStore::new(),
            node_ids: GuidManager::new(),
            link_ids: GuidManager::new(),
            linklabel_ids: GuidManager::new(),
        }
    }
}
