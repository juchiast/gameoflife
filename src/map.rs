use ::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[derive(Deserialize, Serialize)]
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
        origin: *origin,
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

impl Map {
    /// Acorn methuselah
    pub fn acorn() -> Map {
        let list = [pos(3, 2), pos(5, 3), pos(2, 4), pos(3, 4), pos(6, 4), pos(7, 4), pos(8, 4)];
        Map::new_from_alive_list(&list)
    }

    /// Blom methuselah
    pub fn blom() -> Map {
        let list = [pos(1, 1), pos(12, 1),
        pos(2, 2), pos(3, 2), pos(4, 2), pos(5, 2), pos(12, 2),
        pos(3, 3), pos(4, 3), pos(12, 3),
        pos(11, 4), pos(11, 5), pos(9, 5)];
        Map::new_from_alive_list(&list)
    }
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        use std::fs;
        use std::io::Write;
        use std::io::BufWriter;
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for x in &self.alive_cells {
            if let Ok(mut s) = serde_json::to_string(x) {
                s.push('\n');
                writer.write_all(s.as_bytes())?;
            }
        }
        Ok(())
    }
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Map> {
        use std::fs;
        use std::io::Read;
        let mut contents = String::new();
        let mut file = fs::File::open(path)?;
        file.read_to_string(&mut contents)?;
        let mut map = Map::new();
        for line in contents.lines() {
            if let Ok(pos) = serde_json::from_str::<Pos>(line) {
                map.set_cell_alive(&pos);
            }
        }
        Ok(map)
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
            for (nei, i) in neighbors(pos) {
                if self.cell_is_alive(&nei) {
                    new |= 1<<i;
                }
            }
            assert_eq!(new, *state);
        }

        for pos in &self.alive_cells {
            for (pos, _) in neighbors(pos) {
                let state = self.neighbors_state[&pos];
                let mut new = 0;
                for (nei, i) in neighbors(&pos) {
                    if self.cell_is_alive(&nei) {
                        new |= 1<<i;
                    }
                }
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
    pub fn new_from_alive_list<'a, I>(list: I) -> Map
    where I: IntoIterator<Item=&'a Pos> {
        let mut result = Map::new();
        for pos in list {
            result.set_cell_alive(pos);
        }
        result
    }

    /// Generate next generation
    pub fn next_generation(&mut self) {
        let new_alive = self.neighbors_state.iter().filter_map(|(pos, state)| {
            if self.eval_state(*state) == 2 {
                Some(*pos)
            } else { None }
        }).collect::<Vec<_>>();
        let new_dead = self.neighbors_state.iter().filter_map(|(pos, state)| {
            if self.eval_state(*state) == 0 {
                Some(*pos)
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
    pub fn get_alive_cells_in(&self, top_left: Pos, size: Pos) -> Vec<Pos> {
        let bottom_right = pos(top_left.x + size.x, top_left.y + size.y);
        self.alive_cells.iter().filter(|&pos| {
            top_left.x <= pos.x && pos.x <= bottom_right.x &&
            top_left.y <= pos.y && pos.y <= bottom_right.y
        }).cloned().collect()
    }

    /// Force a cell to be alive
    pub fn set_cell_alive(&mut self, pos: &Pos) {
        if self.alive_cells.insert(*pos) {
            let mut new_state = 0;
            for (nei, i) in neighbors(pos) {
                if self.cell_is_alive(&nei) {
                    new_state |= 1<<i;
                }
                let state = self.neighbors_state.entry(nei).or_insert(0);
                *state |= 1<<(7^i);
            }
            let mut state = self.neighbors_state.entry(*pos).or_insert(0);
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
    for (i, x) in result.iter_mut().enumerate() {
        let count = usize::count_ones(i);
        if count == 3 { *x = 2; }
        else if count == 2 { *x = 1; }
    }
    result
}

#[test]
#[ignore]
fn test_with_acorn() {
    let mut map = Map::acorn();
    let mut i = 0;
    let mut max_population = 0;
    let mut max_generation = 0;
    while i<=6000 {
        assert!(i<5206 || map.count_alive_cells()==633);
        if map.count_alive_cells() > max_population {
            max_population = map.count_alive_cells();
            max_generation = i;
        }
        i+=1;
        map.next_generation();
        map.check();
    }
    assert_eq!(max_population, 1057);
    assert_eq!(max_generation, 4408);
}
