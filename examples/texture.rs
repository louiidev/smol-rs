use smol_rs::core::{App, AppSettings};
use smol_rs::render::Color;

extern crate smol_rs;

fn main() {
    let window_settings = AppSettings {
        ..Default::default()
    };
    let mut app = App::new(window_settings);
    app.asset_store
        .load_atlas_texture(
            include_bytes!("../assets/atlas.png"),
            include_str!("../assets/atlas.ron"),
        )
        .unwrap();

    while app.running {
        app.gfx.clear(Color::BLACK);

        app.end_render();
    }
}
