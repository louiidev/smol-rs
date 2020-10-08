
use smol_rs::core::*;
use smol_rs::math::*;

struct GameState {
    current_chunks: Vec<Vector2Int>
}


struct MapChunk {

}


struct Player {

}

fn main() {
    init();
    let mut player_position = Vector2::default();
    
    
    let texture = load_texture("assets/tilemap_packed.png");
    while is_running() {

        let mut temp_pos = Vector2::default();
        
        if is_key_down(Keycode::W) {
            temp_pos.y-= 0.25 * delta_time();
        } else if is_key_down(Keycode::S) {
            temp_pos.y+= 0.25 * delta_time();
        }

        if is_key_down(Keycode::A) {
            temp_pos.x-= 0.25 * delta_time();
        } else if is_key_down(Keycode::D) {
            temp_pos.x+= 0.25 * delta_time();
        }

        if temp_pos != Vector2::default() {
            temp_pos.normalize();
        }

        // temp_pos.normalize();
        player_position+= temp_pos;
        clear();
        draw_sprite_from_atlas(&texture, player_position, Vector2Int { x: 24, y: 0}, 16);
        end_render();
    }
}