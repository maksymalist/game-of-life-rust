use bevy::prelude::*;

// Structs and Implementations
#[derive(Clone, Debug)]
#[derive(Component)]
pub struct Cell {
    pub alive: bool,
    pub neighbors: i32,
}
#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Cell {
    pub fn new(alive: bool, neighbors: i32) -> Self {
        Self { alive, neighbors }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn set_state(&mut self, state: bool){
        self.alive = state;
    }

    pub fn set_neighbors(&mut self, neighbors: i32){
        self.neighbors = neighbors;
    }

    pub fn get_neighbors(&self) -> i32{
        self.neighbors
    }
}


impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::new(false, 0));
            }
            cells.push(row);
        }
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn get(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn get_cell_neighbours(&self, x: usize, y: usize) -> i32 {
        let mut neighbours: i32 = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x_ = x as i32 + i;
                let y_ = y as i32 + j;
                if x_ < 0 || x_ >= self.width as i32 || y_ < 0 || y_ >= self.height as i32 {
                    continue;
                }
                if self.cells[y_ as usize][x_ as usize].is_alive() {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    pub fn revive_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_state(true);
    }

    pub fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_state(false);
    }

}
