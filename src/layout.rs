extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use std::string::String;

use html5ever::rcdom::{Text as htmlText, Handle};

pub fn render(hostname: String, handle: Handle) {
    let hostname_str = &hostname[..];
    let mut window: PistonWindow = WindowSettings::new(
            hostname_str,
            [200, 200]
        )
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {
        let handle = handle.clone();
        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            let mut queue: Vec<Handle> = Vec::new();
            let mut height = 0.0;
            queue.push(handle);
            while queue.len() != 0 {
                let handle = queue.remove(0);
                let node = handle.borrow();
                match node.node {
                    htmlText(ref text_ref) => {
                        let text_string = String::from(text_ref);
                        let text_str = &text_string.trim()[..];
                        if text_str != "" {
                            height = height + 50.0;
                            let transform = c.transform.trans(10.0, height);
                            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                                text_str,
                                &mut glyphs,
                                &c.draw_state,
                                transform,
                                g
                            );
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
        });
    }
}
