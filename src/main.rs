extern crate hyper;
extern crate html5ever;

#[macro_use]
extern crate string_cache;
extern crate tendril;

use std::env;
use std::io::{self, Read};
use std::iter::repeat;
use std::default::Default;
use std::string::String;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(indent: usize, handle: Handle) {
    let node = handle.borrow();
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node {
        Document
            => println!("#Document"),

        Doctype(ref name, ref public, ref system)
            => println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system),

        Text(ref text)
            => println!("#text: {:?}", escape_default(text)),

        Comment(ref text)
            => println!("<!-- {:?} -->", escape_default(text)),

        Element(ref name, _, ref attrs) => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone());
    }
}

fn main() {
    println!("Going to make get request");
    let client = hyper::Client::new();
    if let Some(arg1) = env::args().nth(1) {
        let hostname = arg1;
        let mut res = client.get(&hostname).send().unwrap();
        println!("Status: {}", res.status);
        let dom = parse_document(RcDom::default(), Default::default()).from_utf8().read_from(&mut res).unwrap();
        println!("Parsed dom!");
        walk(0, dom.document);
    }
}
