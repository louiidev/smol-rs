use nalgebra::Vector;
use smol_rs::errors::SmolError;
use smol_rs::renderer::text::TextSettings;
use smol_rs::{import_file, App, AppSettings, Color, Rect, Transform};

extern crate smol_rs;

fn main() -> Result<(), SmolError> {
    let mut app = App::new(AppSettings::default());

    let open_sans_bold = app
        .load_font(import_file!("../assets/OpenSans-SemiBold.ttf"))
        .unwrap();

    let open_sans_light = app
        .load_font(import_file!("../assets/OpenSans-Light.ttf"))
        .unwrap();

    let t = app
        .load_texture(import_file!("../assets/test.png"))
        .unwrap();

    while app.is_running() {
        app.renderer.clear(Color::BLACK);
        app.renderer.rect(
            Transform::from(Rect {
                x: 0.,
                y: 0.,
                width: 50.,
                height: 50.,
            }),
            Color::BLUE,
        );

        app.renderer.text_ex(
            &open_sans_bold,
            "Heeeeello",
            &TextSettings {
                color: Color::WHITE,
                ..Default::default()
            },
        );

        app.renderer.text_ex(
            &open_sans_light,
            "my name is",
            &TextSettings {
                color: Color::WHITE,
                position: Vector::from([25., 25.]),
                ..Default::default()
            },
        );

        app.renderer.text_ex(
            &open_sans_bold,
            "Slim shady",
            &TextSettings {
                color: Color::WHITE,
                position: Vector::from([50., 50.]),
                ..Default::default()
            },
        );

        app.renderer.texture(Transform::default(), &t);

        app.end_scene();
    }

    Ok(())
}
