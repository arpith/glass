extern crate piston_window;
extern crate find_folder;

use piston_window::*;

pub fn render(hostname: String) {
    let hostname_str = &hostname[..];
    let mut window: PistonWindow = WindowSettings::new(
            hostname_str,
            [200, 200]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                hostname_str,
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }
}
