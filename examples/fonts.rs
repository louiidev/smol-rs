use nalgebra::Vector;
use smol_rs::errors::SmolError;
use smol_rs::renderer::text::{TextAlignment, TextSettings};
use smol_rs::{import_file, App, AppSettings, Color};

extern crate smol_rs;

fn main() -> Result<(), SmolError> {
    let mut app = App::new(AppSettings::default());

    let open_sans_bold = app
        .load_font(import_file!("../assets/OpenSans-SemiBold.ttf"))
        .unwrap();

    let open_sans_light = app
        .load_font(import_file!("../assets/OpenSans-Light.ttf"))
        .unwrap();

    while app.is_running() {
        app.renderer.clear(Color::BLACK);

        app.renderer.text_ex(
            &open_sans_bold,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus rutrum tempor dolor",
            &TextSettings {
                color: Color::WHITE,
                alignment: TextAlignment::Center,
                ..Default::default()
            },
        );

        app.renderer.text_ex(
            &open_sans_light,
            "Nullam ac luctus erat. Curabitur dignissim consequat sapien sed cursus.",
            &TextSettings {
                color: Color::WHITE,
                alignment: TextAlignment::Center,
                position: Vector::from([25., 25.]),
                ..Default::default()
            },
        );

        app.renderer.text_ex(
            &open_sans_bold,
            " Mauris et odio id libero dignissim pretium vitae vitae est.",
            &TextSettings {
                color: Color::WHITE,
                alignment: TextAlignment::Center,
                position: Vector::from([50., 50.]),
                ..Default::default()
            },
        );

        app.end_scene();
    }

    Ok(())
}
