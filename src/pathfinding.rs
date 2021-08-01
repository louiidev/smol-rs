use hashbrown::HashMap;
use crate::{map::Tile, math::{Vector2, Vector2Int}};

// G: Distance to current node
// H: Distance to end node
// F: the combined value of G & H


fn reconstruct_path(nodes: &HashMap<Vector2Int, Tile>, last: Vector2Int) -> Option<Vec<Vector2Int>> {
    let mut path = vec![last];
    let mut current = nodes.get(&last)?;
    while let Some(previous) = current.previous {
        path.push(previous);
        current = nodes.get(&previous)?;
    }

    Some(path)
}


pub fn a_star(mut nodes: HashMap<Vector2Int, Tile>, start: Vector2Int, mut end: Vector2Int) -> Option<Vec<Vector2Int>> {

    let mut open_nodes: Vec<Vector2Int> = Vec::default();
    let mut closed_nodes: Vec<Vector2Int> = Vec::default();
    

    open_nodes.push(start);

    let mut breaker_test = 0;

    if !nodes.contains_key(&end) || !nodes.contains_key(&start) {
        return None
    }
    
    if let Some(start) = nodes.get(&start) {
        if !start.walkable {
            return None
        }
    }
    
    if let Some(end_tile) = nodes.get(&end) {
        if !end_tile.walkable {
            let neighbours = get_neighbours_valid_positions(&nodes, end);
            if neighbours.is_empty() {
                return None
            }
            end = *neighbours.first()?;
        }
    }
    
    while !open_nodes.is_empty() {
        breaker_test+= 1;
        if breaker_test > 10000 {
            panic!("ERROR IN PATH FINDING");
        }
        let current_node_pos = {
            let mut current_position = *open_nodes.first()? ;
            let mut index = 0;
            for (n_index, node_pos) in open_nodes.iter().enumerate() {


                let (current_f, current_g) = (nodes.get(&current_position)?.f, nodes.get(&current_position)?.g);
                let (node_f, node_g) = (nodes.get(&node_pos)?.f, nodes.get(&node_pos)?.g);
                
                if node_f < current_f || node_f == current_f && node_g > current_g {
                    current_position = *node_pos;
                    index = n_index;
                }

                if node_g == current_g {
                    let vh_node = heuristic(*node_pos, end);
                    let vh_current = heuristic(current_position, end);
                    if vh_node < vh_current {
                        current_position = *node_pos;
                        index = n_index;
                    }
                }
            }
            open_nodes.remove(index);
            current_position
        };


        if current_node_pos == end {
            // return path
            return reconstruct_path(&nodes, current_node_pos);
        }

        let g = nodes.get(&current_node_pos)?.g;
        
        closed_nodes.push(current_node_pos);

        let neighbours_positions = get_neighbours_valid_positions(&nodes, current_node_pos);
        

        for neighbour_position in neighbours_positions {
            let neighbour = nodes.get_mut(&neighbour_position)?;

            if closed_nodes.contains(&neighbour_position) {
                continue;
            }
            
            let tentative_g = g + 1; // should use heuristic but the difference should always be 1 for manhattan distance

            if !open_nodes.contains(&neighbour_position) {
                open_nodes.push(neighbour_position);
            } else if tentative_g >= neighbour.g {
                continue;
            }

            neighbour.g = tentative_g;
            neighbour.f = tentative_g + manhattan_heuristic(neighbour_position, end);
            neighbour.previous = Some(current_node_pos);
        }

        
        
    }

    None
}


fn manhattan_heuristic(current: Vector2Int, target: Vector2Int) -> i32 {
    (current.x - target.x).abs() + (current.y - target.y).abs()
}

fn heuristic(current: Vector2Int, target: Vector2Int) -> f32 {
    let pos: Vector2 = current.into();
    pos.distance(target.into())
}


fn get_neighbours_valid_positions(nodes: &HashMap<Vector2Int, Tile>, base_position: Vector2Int) -> Vec<Vector2Int> {
    let mut neighbours: Vec<Vector2Int> = Vec::default();
    
    
    let positions = [
        // West
        Vector2Int::new(base_position.x - 1, base_position.y),
        // East
        Vector2Int::new(base_position.x + 1, base_position.y),
        // South
        Vector2Int::new(base_position.x, base_position.y - 1),
        // North
        Vector2Int::new(base_position.x, base_position.y + 1),
    ];


    for position in positions.iter() {
        if nodes.contains_key(position) && nodes.get(position).unwrap().walkable {
            neighbours.push(*position);
        }
    }

    neighbours
}




#[cfg(test)]
mod test {
    use super::*;    

    #[test]
    fn test_pathfinding() {

        let mut nodes = HashMap::default();

        for x in 0..6 {
            for y in 0..6 {
                nodes.insert(Vector2Int::new(x, y), Tile {
                    walkable: true,
                    ..Default::default()
                });
            } 
        }


        let path = a_star(nodes, Vector2Int::default(), Vector2Int::new(5, 5));
        assert_eq!(path.is_some(), true);
    }


    #[test]
    fn test_returns_none_if_no_valid_path() {
        let mut nodes = HashMap::default();

        for x in 0..6 {
            for y in 0..6 {
                nodes.insert(Vector2Int::new(x, y), Tile {
                    walkable: true,
                    ..Default::default()
                });
            } 
        }
    

        let path = a_star(nodes, Vector2Int::default(), Vector2Int::new(6, 6));
        assert_eq!(path.is_none(), true);
    }

}