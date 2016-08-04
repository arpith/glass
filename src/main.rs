extern crate hyper;
extern crate html5ever;

#[macro_use]
extern crate string_cache;
extern crate tendril;

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

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn getCSSlinks(handle: Handle) -> Vec<String> {
    let mut CSSlinks: Vec<String> = Vec::new();
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle.borrow();
        match node.node {
            Element(ref name, _, ref attrs) => {
                assert!(name.ns == ns!(html));
                let mut isCSS = false;
                for attr in attrs.iter() {
                    assert!(attr.name.ns == ns!());
                    if name.local == string_cache::Atom::from("link") && 
                        attr.name.local == string_cache::Atom::from("type") && 
                        attr.value == Tendril::from("text/css") {
                        isCSS = true;
                    }
                    if isCSS && attr.name.local == string_cache::Atom::from("href") {
                        CSSlinks.push(String::from(attr.value.clone()));
                    }
                }
            }
            _ => {
                //don't do anything
            }
        }
        for child in node.children.iter() {
            queue.push(child.clone());
        }
    }
    return CSSlinks;
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
        let CSSlinks = getCSSlinks(dom.document);
        println!("CSS links: {:?}", CSSlinks);
    }
}
