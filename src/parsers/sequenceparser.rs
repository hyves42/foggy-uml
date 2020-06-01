use std::rc::Rc;
use parsers::datatypes::{ElementType, Element, Document, Parser, ParserResult};
use datatypes::{SliceWithContext};
use parseutils::*;


//elements: 
// 1 array of actors, in their display order, eventually inside a box element.
// Even actors that are created during the sequence are in this array, in that case they haave an attribute
//
// 1 array of 'things that happen' during the sequence:
// - message passing/sending
// - activation/deactivation
// - alt
// - object creation/deletion
// - notes
// - state changes
// - vertical sepration

static reserved_tokens_header: [&'static str;12] = [
	//actors definitions in header
	"participant",
	"actor", 
	"boundary", 
	"control", 
	"entity", 
	"database", 
	"collections",
	"box",
	"title",
	"end",
	"hide",
	"show",
	];

static reserved_tokens_sequence: [&'static str;20] = [
	"alt",
	"else",
	"group",
	"loop",
	"end",
	"ref",
	"note",
	"rnote", 
	"hnote",
	"activate",
	"destroy",
	"deactivate",
	"deactivate",
	"return",
	"...",
	"|||",
	"||",
	"==",
	"[",
	"{"
];



struct SequenceDiagramParser {
    // a string buffer to collect text for the element being parsed
    collec: Option<String>,
    // full tree structure of document
    title: Option<Element>,
    header: Option<Element>,
    sequence: Option<Element>,
}

