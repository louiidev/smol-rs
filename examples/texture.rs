use smol_rs::core::{App, AppSettings};
use smol_rs::errors::SmolError;
use smol_rs::render::Color;
use smol_rs::renderer::DrawOptions;

extern crate smol_rs;

fn main() -> Result<(), SmolError> {
    let mut app = App::new(AppSettings::default());

    app.asset_store.load_atlas_texture(
        include_bytes!("../assets/atlas.png"),
        include_str!("../assets/atlas.ron"),
    )?;

    while app.running {
        app.renderer.clear(Color::BLACK);

        app.renderer.texture(
            DrawOptions::default(),
            app.asset_store.get_texture("player").unwrap(),
        );

        app.end_scene();
    }

    Ok(())
}
