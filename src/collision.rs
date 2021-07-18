use crate::math::Vector2Int;



pub fn is_point_inside_rect(point: Vector2Int, width: i32, height: i32, x: i32, y: i32) -> bool {
    let pointX = point.x;
    let pointY = point.y;

    
    pointX > x && pointX <= width + x && pointY > y && pointY <= height + y
}   