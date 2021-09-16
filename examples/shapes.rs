use nalgebra::Vector;

use smol_rs::{App, AppSettings, Color, Rectangle};

extern crate smol_rs;

fn main() {
    let mut app = App::new(AppSettings::default());

    while app.is_running() {
        app.renderer.clear(Color::BLACK);

        app.renderer
            .circle(Vector::from([150., 150.]), 50., 360, Color::BLUE);

        app.renderer.rectangle(
            Rectangle {
                x: -150.,
                y: -150.,
                width: 50.,
                height: 50.,
            },
            Color::GREEN,
        );

        app.renderer.line_rect(
            Rectangle {
                x: -50.,
                y: -70.,
                width: 30.,
                height: 30.,
            },
            1.0,
            Color::GREEN,
        );

        app.renderer.line(
            Vector::from([0., 90.]),
            Vector::from([100., 76.]),
            1.,
            Color::WHITE,
        );

        app.end_scene();
    }
}
