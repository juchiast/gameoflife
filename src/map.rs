use ::std::collections::BTreeMap;
use ::std::collections::BTreeSet;

const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

#[derive(Eq, Ord, PartialEq, PartialOrd)]
#[derive(Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

/// Return new position from `x` and `y`
#[inline]
pub fn pos(x: i32, y: i32) -> Pos {
    Pos {
        x: x,
        y: y,
    }
}

pub struct Neighbors{
    origin: Pos,
    i: usize,
}

pub fn neighbors(origin: &Pos) -> Neighbors {
    Neighbors {
        origin: origin.clone(),
        i: 0,
    }
}

impl Iterator for Neighbors {
    type Item = (Pos, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 8 { None }
        else {
            self.i += 1;
            let j = self.i - 1;
            Some( (pos(self.origin.x+DX[j], self.origin.y+DY[j]), j) )
        }
    }
}

pub struct Map {
    neighbors_state: BTreeMap<Pos, u8>,
    pub alive_cells: BTreeSet<Pos>,
    state_eval_table: [u8; 256],
}
impl Clone for Map {
    fn clone(&self) -> Map {
        Map {
            neighbors_state: self.neighbors_state.clone(),
            alive_cells: self.alive_cells.clone(),
            state_eval_table: self.state_eval_table,
        }
    }
}

fn new_state_eval_table() -> [u8; 256] {
    let mut result = [0; 256];
    for i in 0..256 {
        let mut count = usize::count_ones(i);
        if count == 3 { result[i] = 2; }
        else if count == 2 { result[i] = 1; }
    }
    result
}

impl Map {
    pub fn print(&self) {
        let vec: Vec<_> = self.alive_cells.iter().map(|pos| (pos.x, pos.y)).collect();
        println!("{:?}", vec);
    }

    pub fn is_empty(&self) -> bool {
        self.alive_cells.is_empty()
    }

    pub fn count_alive_cells(&self) -> usize {
        self.alive_cells.len()
    }

    pub fn check(&self) {
        for (pos, state) in &self.neighbors_state {
            let mut new = 0;
            for (nei, i) in neighbors(&pos) {
                if self.cell_is_alive(&nei) {
                    new |= 1<<i;
                }
            }
            //assert!(new != 0);
            assert_eq!(new, *state);
        }

        for pos in &self.alive_cells {
            for (pos, _) in neighbors(&pos) {
                let state = self.neighbors_state[&pos];
                let mut new = 0;
                for (nei, i) in neighbors(&pos) {
                    if self.cell_is_alive(&nei) {
                        new |= 1<<i;
                    }
                }
                //assert!(new != 0);
                assert_eq!(new, state);
            }
        }
    }

    /// Return a new empty map
    pub fn new() -> Map {
        Map {
            neighbors_state: BTreeMap::new(),
            alive_cells: BTreeSet::new(),
            state_eval_table: new_state_eval_table(),
        }
    }

    /// Return a new map from list of alive cells
    pub fn new_from_alive_list(list: &[Pos]) -> Map {
        let mut result = Map::new();
        for pos in list {
            result.set_cell_alive(&pos);
        }
        result
    }

    /// Generate next generation
    pub fn next_generation(&mut self) {
        let new_alive = self.neighbors_state.iter().filter_map(|(pos, state)| {
            if self.eval_state(*state) == 2 {
                Some(pos.clone())
            } else { None }
        }).collect::<Vec<_>>();
        let new_dead = self.neighbors_state.iter().filter_map(|(pos, state)| {
            if self.eval_state(*state) == 0 {
                Some(pos.clone())
            } else { None }
        }).collect::<Vec<_>>();

        for pos in new_alive {
            self.set_cell_alive(&pos);
        }
        for pos in new_dead {
            self.set_cell_dead(&pos);
        }
    }

    /// Return list of alive cells within the rectangle from `top_left`
    /// to `bottom_right`
    pub fn get_alive_cells_in(&self, top_left: Pos, bottom_right: Pos) -> Vec<Pos> {
        self.alive_cells.iter().filter(|&pos| {
            top_left.x <= pos.x && pos.x <= bottom_right.x &&
            top_left.y <= pos.y && pos.y <= bottom_right.y
        }).cloned().collect()
    }

    /// Force a cell to be alive
    pub fn set_cell_alive(&mut self, pos: &Pos) {
        if self.alive_cells.insert(pos.clone()) {
            let mut new_state = 0;
            for (nei, i) in neighbors(pos) {
                if self.cell_is_alive(&nei) {
                    new_state |= 1<<i;
                }
                let state = self.neighbors_state.entry(nei).or_insert(0);
                *state |= 1<<(7^i);
            }
            let mut state = self.neighbors_state.entry(pos.clone()).or_insert(0);
            *state = new_state;
        }
    }

    /// Kill a cell
    pub fn set_cell_dead(&mut self, pos: &Pos) {
        if self.alive_cells.remove(pos) {
            for (nei, i) in neighbors(pos) {
                let mut rm = false;
                if let Some(mut state) = self.neighbors_state.get_mut(&nei) {
                    *state &= !(1<<(7^i));
                    if *state == 0 {
                        rm = true;
                    }
                }
                if rm && !self.cell_is_alive(&nei) {
                    self.neighbors_state.remove(&nei);
                }
            }
        }
    }

    /// Check if a cell is alive or not
    #[inline]
    pub fn cell_is_alive(&self, pos: &Pos) -> bool {
        self.alive_cells.contains(pos)
    }

    #[inline]
    fn eval_state(&self, state: u8) -> u8 {
        self.state_eval_table[state as usize]
    }
}

pub fn chaos() -> Map {
    let list = [pos(3, 2), pos(5, 3), pos(2, 4), pos(3, 4), pos(6, 4), pos(7, 4), pos(8, 4)];
    Map::new_from_alive_list(&list)
}

fn map_test() {
    let mut map = chaos();
    let mut i = 0;
    while !map.is_empty() {
        i+=1;
        map.next_generation();
        println!("{} {} {}", i, map.count_alive_cells(), map.neighbors_state.len());
    }
}
