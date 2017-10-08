// Conway's game of life
extern crate rand;
extern crate pancurses;

use rand::Rng;
use pancurses::{Window, initscr, endwin, Input};

enum GridInitType {Blank, RandPattern}

struct LifeGrid {
    width: i32,
    height: i32,
    grid: Vec<bool>,
}

impl LifeGrid {
    fn cell_index(&self, x: i32, y: i32) -> usize {
        (modulo(y, self.height) * self.width + modulo(x, self.width)) as usize
    }

    fn cell_val(&self, x: i32, y: i32) -> bool {
        self.grid[self.cell_index(x, y)]
    }

    fn set_cell(&mut self, x: i32, y: i32, val: bool) {
        let idx = self.cell_index(x, y);
        self.grid[idx] = val;
    }

    fn show(&self, window: &Window) {
        for y in 0..self.height {
            window.mv(y, 0);
            for x in 0..self.width {
                if self.cell_val(x, y) {
                    window.addch('O');
                }
                else {
                    window.addch(' ');
                }
            }
        }
        window.refresh();
    }

    fn neighbour_count(&self, x: i32, y: i32) -> i32 {
        (if self.cell_val(x - 1, y - 1) {1} else {0}) +
        (if self.cell_val(x, y - 1) {1} else {0}) +
        (if self.cell_val(x + 1, y - 1) {1} else {0}) +
        (if self.cell_val(x - 1, y) {1} else {0}) +
        (if self.cell_val(x + 1, y) {1} else {0}) +
        (if self.cell_val(x - 1, y + 1) {1} else {0}) +
        (if self.cell_val(x, y + 1) {1} else {0}) +
        (if self.cell_val(x + 1, y + 1) {1} else {0})
    }

    fn new(w: i32, h: i32, init_type: GridInitType) -> LifeGrid {
        match init_type {
            GridInitType::Blank => LifeGrid{width: w, height: h, grid: vec![false; (w * h) as usize]},
            GridInitType::RandPattern => {
                // must be a better way, not enough Rust yet..
                let mut lg = LifeGrid::new(w, h, GridInitType::Blank);
                for i in 0..(w * h) {
                    lg.grid[i as usize] = rand::thread_rng().gen_range(1, 4) == 1;
                }
                lg
            }
        }
    }
}

fn main() {
    let window = initscr();
    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

    // we're committed to these dimensions now, please don't resize the window!
    let mut life_grid1 = LifeGrid::new(max_x, max_y, GridInitType::RandPattern);
    let mut life_grid2 = LifeGrid::new(max_x, max_y, GridInitType::Blank);

    window.nodelay(true);
    loop {
        apply_life_rules(&life_grid1, &mut life_grid2);
        life_grid2.show(&window);
        apply_life_rules(&life_grid2, &mut life_grid1);
        life_grid1.show(&window);
        // pressing 'q' gets us out...
        match window.getch() {
            Some(Input::Character('q')) => break,
            _ => ()
        }
    }
    endwin();
}

fn modulo(a: i32, b: i32) -> i32 {
    let rem = a % b;
    if rem < 0 { rem + b } else { rem }
}

fn apply_life_rules(from_grid: &LifeGrid, to_grid: &mut LifeGrid) {
    for y in 0..from_grid.height {
        for x in 0..from_grid.width {
            let neighbours = from_grid.neighbour_count(x, y);
            if from_grid.cell_val(x, y) {
                to_grid.set_cell(x, y, neighbours > 1 && neighbours < 4);
            }
            else {
                to_grid.set_cell(x, y, neighbours  == 3);
            }
        }
    }
}
