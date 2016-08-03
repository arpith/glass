extern crate hyper;
extern crate html5ever;

#[macro_use]
extern crate string_cache;
extern crate tendril;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::{self, Read};
use std::str::FromStr;
use std::iter::repeat;
use std::default::Default;
use std::string::String;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

use tendril::*;
use tendril::fmt::{UTF8};

use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};
use html5ever::tokenizer::{TokenSink, Token, TokenizerOpts, ParseError};
use html5ever::tokenizer::{TagToken, StartTag, Tag};

   /* {
lazy_static! {
    static ref CSSlinks: Vec<String> = Vec::new();
        let mut init = Vec::new();
        init
    };
}
    */
static mut CSSlinks: Vec<String> = vec!();

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
            let mut isCSS = false;
            print!("<{}", name.local);
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                if name.local == string_cache::Atom::from("link") && attr.name.local == string_cache::Atom::from("type") && attr.value == Tendril::from("text/css") {
                   isCSS = true;
                }
                if isCSS && attr.name.local == string_cache::Atom::from("href") {
                    unsafe {
                       CSSlinks.push(String::from(attr.value.clone()));
                    }
                }
                print!(" {}=\"{}\"", attr.name.local, attr.value);
                println!(">");
            }
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
