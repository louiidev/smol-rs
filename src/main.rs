
use smol_rs::prelude::*;


fn main() {
    Smol::init();
    let texture = load_texture("src/test.png");
    while is_running() {
        clear();
        draw_rectangle();
        draw_sprite(&texture);
        end_render();
    }
}