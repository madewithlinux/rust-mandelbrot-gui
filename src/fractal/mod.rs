use std::collections::HashMap;

use itertools::Itertools;
pub mod mandelbrot;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub pos: (u32, u32),
    pub iter: f32,
    pub rgb: (u8, u8, u8),
    // TODO: data?
    // pub data: Vec<u8>,
}

pub trait FractalCellFunc {
    fn default_for_size(width: u32, height: u32) -> Self;

    fn compute_cell(&self, pos: (u32, u32)) -> Cell;
    fn compute_cells(&self, positions: &[(u32, u32)]) -> Vec<Cell> {
        positions
            .iter()
            .map(|&pos| self.compute_cell(pos))
            .collect_vec()
    }

    fn with_offset(&self, offset: (i32, i32)) -> Self;
    fn with_zoom(&self, zoom: f32) -> Self;

    fn with_option(&self, name: &str, value: &str) -> Self;
    fn get_options(&self) -> HashMap<String, String>;
}
