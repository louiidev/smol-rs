use smol_rs::errors::SmolError;
use smol_rs::math::Vector3;

use smol_rs::{import_file, App, AppSettings, Color, Keycode, Transform};

extern crate smol_rs;

fn main() -> Result<(), SmolError> {
    let mut app = App::new(AppSettings {
        target_fps: 144.,
        ..Default::default()
    });

    let t = app
        .load_texture(import_file!("../assets/test.png"))
        .unwrap();

    while app.is_running() {
        let mut position = Vector3::default();
        let mut zoom = app.renderer.camera.zoom;
        for key in app.input.get_pressed_keys().iter() {
            match key {
                Keycode::Up => position.y -= 1.,
                Keycode::Down => position.y += 1.,
                Keycode::Right => position.x += 1.,
                Keycode::Left => position.x -= 1.,
                _ => {}
            }
        }

        if app.input.mouse_scroll_direction > 0 {
            zoom += 0.5;
        } else if app.input.mouse_scroll_direction < 0 {
            zoom -= 0.5;
        }

        app.renderer.camera.zoom = zoom;

        if position.magnitude() > 0.1 {
            app.renderer.camera.position += position.normalize() * 200. * app.delta;
        }

        app.renderer.clear(Color::BLACK);

        app.renderer.texture(Transform::default(), &t);

        app.end_scene();
    }

    Ok(())
}
