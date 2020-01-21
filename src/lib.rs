extern crate js_sys;

mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)] // ensures it's a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn live_nn_count(&self, row: u32, col: u32) -> u8 {
        let mut alive: u8 = 0;
        for i in [self.height - 1, 0, 1].iter() {
            for j in [self.width - 1, 0, 1].iter() {
                if *i == 0 && *j == 0 {
                    continue;
                }
                let nn_row = (row + i) % self.height;
                let nn_col = (col + j) % self.width;
                let idx = self.get_index(nn_row, nn_col);
                alive += self.cells[idx] as u8;
            }
        }
        alive
    }
}

/// Public methods, exported to js
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for i in 0..self.width {
            for j in 0..self.height {
                let idx = self.get_index(i, j);
                let cell = self.cells[idx];
                let live_count = self.live_nn_count(i, j);
                let next_cell_state = match (cell, live_count) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (current_state, _) => current_state,
                };
                next_cells[idx] = next_cell_state;
            }
        }
        self.cells = next_cells;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
