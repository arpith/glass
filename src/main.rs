extern crate hyper;
extern crate html5ever;
extern crate tendril;
extern crate cssparser;
extern crate piston_window;

#[macro_use]
extern crate string_cache;

#[macro_use] extern crate conrod;

use std::env;
use std::default::Default;
use std::string::String;

use tendril::*;

use html5ever::parse_document;
use html5ever::rcdom::{Element, RcDom, Handle, Text};

mod layout;

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn get_title(handle: Handle) -> String {
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    let mut is_title_child = false;
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle.borrow();
        match node.node {
            Element(ref name, _, _) => {
                if !is_title_child {
                    if name.local == string_cache::Atom::from("title") {
                        queue = Vec::new();
                        is_title_child = true;
                    }
                }
            },
            Text(ref text) => {
                if is_title_child {
                    return String::from(text.clone());
                }
            },
             _ => {
                //don't do anything
            }
        }
        for child in node.children.iter() {
            queue.push(child.clone());
        }
    }
    return String::from("");
}

fn get_css_links(handle: Handle) -> Vec<String> {
    let mut csslinks: Vec<String> = Vec::new();
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle.borrow();
        match node.node {
            Element(ref name, _, ref attrs) => {
                assert!(name.ns == ns!(html));
                let mut is_css = false;
                for attr in attrs.iter() {
                    assert!(attr.name.ns == ns!());
                    if name.local == string_cache::Atom::from("link") && 
                        attr.name.local == string_cache::Atom::from("type") && 
                        attr.value == Tendril::from("text/css") {
                        is_css = true;
                    }
                    if is_css && attr.name.local == string_cache::Atom::from("href") {
                        csslinks.push(String::from(attr.value.clone()));
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
    return csslinks;
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
        let csslinks = get_css_links(dom.document.clone());
        println!("CSS links: {:?}", csslinks);
/*        for link in csslinks.iter() {
            let res = client.get(link).send().unwrap();
            println!("Status for {}: {}", link, res.status);
        }
        */
        let title = get_title(dom.document.clone());
        layout::render(title, dom.document);
    }
}
