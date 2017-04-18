use ::std::collections::BTreeMap;

pub type Pos = (i32, i32);

pub struct Map {
    cells: BTreeMap<Pos, u8>,
    changed: Vec<Pos>,
}

impl Map {
    /// Return a new empty map
    fn new() -> Map {
        Map{
            cells: BTreeMap::new(),
        }
    }
    /// Return a new map from list of alive cells
    fn new_from_alive_list(list: Vec<Pos>) -> Map {unimplemented!()}
    /// Generate next generation
    fn next_generation(&mut self) {}
    /// Return list of alive cells within the rectangle from `top_left`
    /// to `bottom_right`
    fn get_alive_cells_in(&self, top_left: Pos, bottom_right: Pos) -> Vec<Pos> {unimplemented!()}
    /// Force a cell to be alive
    fn set_cell_alive(&mut self, pos: Pos) {}
    /// Kill a cell
    fn set_cell_dead(&mut self, pos: Pos) {}
    /// Check if a cell is alive or not
    fn cell_is_alive(&self, pos: Pos) -> bool {unimplemented!()}
}
