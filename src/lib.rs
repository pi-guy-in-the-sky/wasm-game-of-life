mod utils;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into())
//     };
// }

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

// not exposed to javascript
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    // this tile doesn't count as its own neighbor
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

// public functions for javascript
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);

        let mut universe = Universe {
            width,
            height,
            cells,
        };

        // "Hi! I'm Alex."
        universe.set_cells(&[(2,3),(2,7),(2,9),(2,10),(2,11),(2,13),(2,17),(2,18),(2,19),(2,21),(2,23),(2,24),(2,25),(2,26),(2,27),(3,3),(3,7),(3,10),(3,13),(3,18),(3,21),(3,23),(3,25),(3,27),(4,3),(4,7),(4,10),(4,13),(4,18),(4,23),(4,25),(4,27),(5,3),(5,4),(5,5),(5,6),(5,7),(5,10),(5,13),(5,18),(5,23),(5,25),(5,27),(6,3),(6,7),(6,10),(6,13),(6,18),(6,23),(6,27),(7,3),(7,7),(7,10),(7,18),(7,23),(7,27),(8,3),(8,7),(8,9),(8,10),(8,11),(8,13),(8,17),(8,18),(8,19),(8,23),(8,27),(11,3),(11,4),(11,5),(11,7),(11,11),(11,12),(11,13),(11,15),(11,17),(12,3),(12,5),(12,7),(12,11),(12,15),(12,17),(13,3),(13,5),(13,7),(13,11),(13,15),(13,17),(14,3),(14,4),(14,5),(14,7),(14,11),(14,12),(14,16),(15,3),(15,5),(15,7),(15,11),(15,15),(15,17),(16,3),(16,5),(16,7),(16,11),(16,15),(16,17),(17,3),(17,5),(17,7),(17,8),(17,9),(17,11),(17,12),(17,13),(17,15),(17,17),(17,19)]);

        universe
    }

    pub fn from_dimensions(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    // underpopulation
                    (true, x) if x < 2 => false,
                    // good
                    (true, 2) | (true, 3) => true,
                    // overpopulation
                    (true, x) if x > 3 => false,
                    // reproduction
                    (false, 3) => true,
                    // other cells same
                    (otherwise, _) => otherwise,
                };

                // log!("    it becomes {:?}", next_cell);

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }
}
