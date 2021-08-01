use smol_rs::core::*;
use smol_rs::input::{InputState, get_mouse_pos, update, query_ui_input, query_world_input};
use smol_rs::map::{MapChunk, get_map};
use smol_rs::math::*;
use smol_rs::pathfinding::{a_star};
use smol_rs::queries::get_entity_grid_position;
use smol_rs::render::*;
use smol_rs::components::{SpriteRenderer, Transform};
use smol_rs::ui::*;
use smol_rs::world_setup::setup_world;
use smol_rs::texture_packer::{TexturePacker};
use rand::{self, Rng};




fn main() {
    init();
    let mut input_state = InputState::default();

    
    let mut items_window = ItemsWindow::new(Rectangle { x: 240., y: 110., w: 500., h: 300. });

    let (mut world, player) = setup_world();
    // let mut batch = RenderBatch::default();

    let texture_packer = TexturePacker::new();
    let mut context_menu = ContextMenu::default();
    let log_info_box = LogInfoBox {};
    let sidebar = SideBar {};

    while is_running() {
        query_ui_input(&mut input_state);
        
        context_menu.update(&mut input_state, &world, player);
        items_window.update(&mut input_state, &world, player);

        query_world_input(&mut input_state, &world, player);

        update(&mut input_state, &mut world, player);
        
        clear(Color (28, 33, 43, 1.));

        capture_framebuffer();
        clear(Color (28, 33, 43, 1.));
        get_map().get_current_chunk().tiles.iter().for_each(|(p, t)| {
            render_texture_partial(&texture_packer.get_texture(&t.texture_name), Vector2::new(p.x as f32 * 16., p.y as f32 * 16.));
        });

        world.query::<(&Transform, &SpriteRenderer)>().iter().for_each(|(_, (t, s))| {
            render_rect(t.screen_positon.x, t.screen_positon.y, 16., 16., Color (3, 3, 3, 1.));
            render_texture_partial(&texture_packer.get_texture(&s.name), t.screen_positon);
        });
        
        let scale = get_window_scale().x;

        if input_state.start_grid_path.is_some() && input_state.end_grid_path.is_some() {
            test_pathfinding(
                input_state.start_grid_path.unwrap(),
                input_state.end_grid_path.unwrap(),
                get_map().get_current_chunk()
            );
        }

        stop_capture_framebuffer();
        render_framebuffer(Vector2 { x: 0., y: 0. }, get_window_scale().x);

        if let Some(pos) = input_state.context_menu_position {
            let grid_pos = to_world_position(pos);
            render_rect(grid_pos.x as _, grid_pos.y as _, 16. * scale, 16. * scale, Color (255, 51, 50, 0.2));
            context_menu.render();
        }

        log_info_box.render();
        sidebar.render();
        // queue_text(&format!("FPS: {}", fps()), Vector2::default(), 20., Color(150, 50, 50, 1.));

        items_window.render(); 
        
        end_render();
    }
}

fn to_world_position(screen_pos: Vector2) -> Vector2Int {
    let scale = get_window_scale().x;
    let position: Vector2Int = screen_pos.into();
    let grid_scale = 16 * scale as i32;
    let grid_pos = (position / grid_scale) * grid_scale;

    grid_pos
}

fn test_pathfinding(start: Vector2Int, end: Vector2Int, chunk: &MapChunk) {
    let path = a_star(chunk.tiles.clone(), start, end);

    if path.is_some() {
        for Vector2Int { x, y } in path.unwrap() {
            let position = Vector2Int{ x, y};
            let grid_pos = position * 16;
            render_rect(grid_pos.x as _, grid_pos.y as _, 16., 16., Color (255, 31, 30, 0.5));
        }
    }
    
}