use rand::Rng;

use crate::Cellule;

pub struct Game {
    pub active: bool,
    pub cellules: Vec<Cellule>,
    pub prev_cellules: Vec<Cellule>,
    pub cellules_width: usize,
    pub cellules_height: usize,
}

impl Game {
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.cellules_width = width;
        self.cellules_height = height;
        self.cellules = vec![Cellule::new_dead(); width * height];
    }

    pub fn random_mutate(&mut self) {
        self.prev_cellules = self.cellules.clone();
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    pub fn reset(&mut self) {
        self.prev_cellules = self.cellules.clone();
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    pub fn step(&mut self) {
        self.prev_cellules = self.cellules.clone();
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);
                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    pub fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    // fn view_cellule(&self) {
    //     let context = self.context_ref.as_ref().unwrap();
    //     let mut to_dead = Vec::new();
    //     let mut to_live = Vec::new();
    //     for row in 0..self.cellules_height {
    //         for col in 0..self.cellules_width {
    //             let neighbors = self.neighbors(row as isize, col as isize);

    //             let current_idx = self.row_col_as_idx(row as isize, col as isize);
    //             self.cellules[current_idx].set_render_flag(false);
    //             if self.cellules[current_idx].is_alive() {
    //                 if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
    //                     to_dead.push(current_idx);
    //                 }
    //             } else if Cellule::can_be_revived(&neighbors) {
    //                 to_live.push(current_idx);
    //             }
    //         }
    //        context.fill_rect(x, y, widht, height)
    //     }
    //     to_dead
    //         .iter()
    //         .for_each(|idx| self.cellules[*idx].set_dead());
    //     to_live
    //         .iter()
    //         .for_each(|idx| self.cellules[*idx].set_alive());
    // }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}
