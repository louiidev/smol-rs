use crate::{math::Vector2Int, math::Rectangle};



pub fn is_point_inside_rect(point: Vector2Int, x: i32, y: i32, width: i32, height: i32) -> bool {
    let point_x = point.x;
    let point_y = point.y;

    
    point_x > x && point_x <= width + x && point_y > y && point_y <= height + y
}

pub fn is_point_inside_rectangle(point: Vector2Int, rect: &Rectangle) -> bool {
    let point_x = point.x as f32;
    let point_y = point.y as f32;
    
    point_x > rect.x && point_x <= rect.w + rect.x && point_y > rect.y && point_y <= rect.h + rect.y
}   