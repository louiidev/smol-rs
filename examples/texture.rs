use nalgebra::Vector;
use smol_rs::errors::SmolError;

use smol_rs::{import_file, App, AppSettings, Color, Transform};

extern crate smol_rs;

fn main() -> Result<(), SmolError> {
    let mut app = App::new(AppSettings::default());

    app.load_texture(import_file!("../assets/test.png"))
        .unwrap();

    while app.is_running() {
        app.renderer.clear(Color::BLACK);

        app.renderer.texture(
            Transform::from(Vector::from([0. * 150., 0. * 150.])),
            &app.get_texture("test").unwrap(),
        );

        app.end_scene();
    }

    Ok(())
}
