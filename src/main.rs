use smol_rs::core::*;
use smol_rs::input::{get_mouse_pos, is_mouse_down, query_player_input};
use smol_rs::math::*;
use smol_rs::render::*;
use smol_rs::components::{SpriteRenderer, Transform};
use smol_rs::ui::{context_menu};
use smol_rs::world_setup::setup_world;
use std::collections::HashMap;
use smol_rs::texture_packer::{TexturePacker};
use rand::{self, Rng};


const TILE_SIZE_X: i32 = 16;
const TILE_SIZE_Y: i32 = 16;


fn main() {
    
    init();
    let (mut world, player) = setup_world();
    // let mut batch = RenderBatch::default();

    let texture_packer = TexturePacker::new();

    let dot_texture = texture_packer.get_texture("dot");
    let grass_texture = texture_packer.get_texture("grass");
    let mut rng = rand::thread_rng();


    let tiles = {
        let mut v: HashMap<Vector2, PartialTexture> = HashMap::new();
        for x in 0..(RENDER_RES_W/TILE_SIZE_X) {
            for y in 0..(RENDER_RES_W/TILE_SIZE_Y) {
                let value = rng.gen_range(0..5);
                if value == 0 {
                    v.insert(Vector2::new(x as f32, y as f32), grass_texture.clone());
                } else {
                    v.insert(Vector2::new(x as f32, y as f32), dot_texture.clone());
                }
                    
            }
        }
        v
    };

    let mut context_pos: Option<Vector2Int> = None;

    while is_running() {

        if is_mouse_down(MouseButton::Right) {
            context_pos = Some(get_mouse_pos())
        } else if is_mouse_down(MouseButton::Left) {
            // check in bounds to see if they clicked an item

            context_pos = None;
        }


        query_player_input(&mut world, player);
        clear(Color (3, 31, 30, 1.));

        // batch.add_sprite(&Sprite {
        //     color: Color(100., 100., 100., 255.),
        //     transform: Transform {
        //         grid_position: Vector2Int::new(0, 0),
        //         screen_positon: Vector2::new(0., 0.),
        //         scale: Vector2::new(1., 1.)
        //     }
        // });

        //batch.render();
        capture_framebuffer();
        clear(Color (3, 31, 30, 1.));
        tiles.iter().for_each(|t| {
            render_texture_partial(t.1, Vector2::new(t.0.x * 16., t.0.y * 16.));    
        });

        world.query::<(&Transform, &SpriteRenderer)>().iter().for_each(|(_, (t, s))| {
            render_rect(t.screen_positon.x, t.screen_positon.y, 16., 16., Color (3, 31, 30, 1.));
            render_texture_partial(&texture_packer.get_texture(&s.name), t.screen_positon);
        });

       
        stop_capture_framebuffer();
        let scale=  get_window_scale() as _;
        render_framebuffer(Vector2 { x: 0., y: 0. }, scale);

        

        if let Some(pos) = context_pos {
            let grid_scale = 16 * scale as i32;
            let grid_pos = (pos / grid_scale) * grid_scale;
            render_rect(grid_pos.x as _, grid_pos.y as _, 16. * scale, 16. * scale, Color (255, 51, 50, 0.2));
            context_menu(pos.into());
        }
        
        end_render();
    }
}
