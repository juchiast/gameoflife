use ::std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
#[derive(Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub struct Map {
    cells: BTreeMap<Pos, u8>,
    changed: Vec<Pos>,
    state_eval_table: [u8; 256],
}

/// Return new position from `x` and `y`
#[inline]
pub fn pos(x: i32, y: i32) -> Pos {
    Pos {
        x: x,
        y: y,
    }
}

fn new_state_eval_table() -> [u8; 256] {
    let mut result = [0; 256];
    for i in 0..256 {
        let mut count: usize = 0;
        for k in 0..8 {
            count += (i>>k)&1;
        }
        if count == 3 { result[i] = 2; }
        else if count == 2 {result[i] = 1; }
    }
    result
}

impl Map {
    /// Return a new empty map
    pub fn new() -> Map {
        Map {
            cells: BTreeMap::new(),
            changed: Vec::new(),
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
    pub fn next_generation(&mut self) {}

    /// Return list of alive cells within the rectangle from `top_left`
    /// to `bottom_right`
    pub fn get_alive_cells_in(&self, top_left: Pos, bottom_right: Pos) -> Vec<Pos> {
        self.cells.iter().filter(|&(pos, state)| {
            self.eval_state(*state) != 0 &&
            top_left.x <= pos.x && pos.x <= bottom_right.x &&
            top_left.y <= pos.y && pos.y <= bottom_right.y
        }).map(|(pos, _)| pos.clone()).collect()
    }

    /// Force a cell to be alive
    pub fn set_cell_alive(&mut self, pos: &Pos) {}

    /// Kill a cell
    pub fn set_cell_dead(&mut self, pos: &Pos) {}

    /// Check if a cell is alive or not
    #[inline]
    pub fn cell_is_alive(&self, pos: &Pos) -> bool {
        self.cells.contains_key(pos) && self.eval_state(self.cells[pos]) != 0
    }

    #[inline]
    fn eval_state(&self, state: u8) -> u8 {
        self.state_eval_table[state as usize]
    }
}
