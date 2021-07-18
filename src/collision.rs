use crate::math::Vector2Int;



pub fn is_point_inside_rect(point: Vector2Int, x: i32, y: i32, width: i32, height: i32) -> bool {
    let point_x = point.x;
    let point_y = point.y;

    
    point_x > x && point_x <= width + x && point_y > y && point_y <= height + y
}   