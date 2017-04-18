mod map;

use map::*;

fn main() {
    let list = [pos(3, 2), pos(5, 3), pos(2, 4), pos(3, 4), pos(6, 4), pos(7, 4), pos(8, 4)];
    let mut map = Map::new_from_alive_list(&list);
    let mut i = 0;
    while !map.is_empty() {
        i+=1;
        map.next_generation();
        println!("{} {} {}", i, map.count_alive_cells(), map.neighbors_state.len());
    }
}
