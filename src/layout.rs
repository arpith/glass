extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use self::piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
use html5ever::rcdom::{Text as htmlText, Handle};

pub fn render(title: String, handle: Handle) {
    const WIDTH: u32 = 150;
    const HEIGHT: u32 = 600;
    let title_str = &title[..];
    let mut window: PistonWindow = WindowSettings::new(
            title_str, 
            [WIDTH, HEIGHT]
        )
        .fullscreen(true)
        .opengl(OpenGL::V3_2).exit_on_esc(true).samples(4).vsync(true).build().unwrap();
    window.set_ups(60);

    let mut ui = conrod::UiBuilder::new().build();

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("FiraSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);
    let image_map = conrod::image::Map::new();
    let list = get_text(handle.clone());

    while let Some(event) = window.next() {
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }
        
        event.update(|_| {
            set_ui(ui.set_widgets(), list.clone());
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }

}

fn get_text(handle: Handle) -> Vec<String> {
    let handle = handle.clone();
    let mut queue: Vec<Handle> = Vec::new();
    let mut text_strings: Vec<String> = Vec::new();
    queue.push(handle);
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle.borrow();
        match node.node {
            htmlText(ref text_ref) => {
                let text_string = String::from(text_ref);
                let text_str = &text_string.trim()[..];
                if text_str != "" {
                    text_strings.push(String::from(text_str));
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
    return text_strings;
}

fn set_ui(ref mut ui: conrod::UiCell, list: Vec<String>) {
    use conrod::{widget, Colorable, Positionable, Sizeable, Widget};
    widget_ids!{CANVAS, LIST};
    widget::Canvas::new().color(conrod::color::WHITE).set(CANVAS, ui);

    const ITEM_HEIGHT: conrod::Scalar = 50.0;
    let num_items = list.len();
    let (mut items, scrollbar) = widget::List::new(num_items, ITEM_HEIGHT)
        .scrollbar_on_top()
        .middle_of(CANVAS)
        .wh_of(CANVAS)
        .set(LIST, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let text_str = &list[i][..];
        let text = widget::Text::new(text_str)
            .color(conrod::color::DARK_CHARCOAL);
        item.set(text, ui);
    }

    if let Some(scrollbar) = scrollbar {
        scrollbar.set(ui);
    }
}
