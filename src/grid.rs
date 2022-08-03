use bevy::prelude::*;

// Structs and Implementations
#[derive(Clone, Debug)]
#[derive(Component)]
pub struct Cell {
    pub alive: bool,
    pub neighbors: i32,
}
#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    gen: i32,
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
            gen: 0
        }
    }

    pub fn get(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn get_cell_neighbours(&self, x: usize, y: usize) -> i32 {
        let mut neighbours: i32 = 0;

       if x <= 0 || y <= 0 || x >= self.width - 1 || y >= self.height - 1 {
           return 0;
         }

        //top left
        if self.cells[y-1][x-1].is_alive() {
            neighbours += 1;
        }
        //top
        if self.cells[y-1][x].is_alive() {
            neighbours += 1;
        }
        //top right
        if self.cells[y-1][x+1].is_alive() {
            neighbours += 1;
        }
        //left
        if self.cells[y][x-1].is_alive() {
            neighbours += 1;
        }
        //right
        if self.cells[y][x+1].is_alive() {
            neighbours += 1;
        }
        //bottom left
        if self.cells[y+1][x-1].is_alive() {
            neighbours += 1;
        }
        //bottom
        if self.cells[y+1][x].is_alive() {
            neighbours += 1;
        }
        //bottom right
        if self.cells[y+1][x+1].is_alive() {
            neighbours += 1;
        }

        neighbours
    }

    pub fn revive_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_state(true);
    }

    pub fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_state(false);
    }

    pub fn increment_gen(&mut self){
        self.gen += 1;
    }

    pub fn get_gen(&self) -> i32{
        self.gen
    }

    pub fn clear(&mut self){
        let mut cells = Vec::new();
        for _ in 0..self.height {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(Cell::new(false, 0));
            }
            cells.push(row);
        }
        self.cells = cells;
    }

}
