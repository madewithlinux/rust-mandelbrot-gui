use std::collections::HashMap;

// use arrow::{
//     array::{Array, ArrayRef, StructArray, UInt32Array, UInt8Array},
//     record_batch::RecordBatch,
// };

use itertools::{izip, Itertools};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub pos: (u32, u32),
    pub iter: f32,
    pub rgb: (u8, u8, u8),
    // TODO: data?
    // pub data: Vec<u8>,
}

pub trait FractalCellFunc: Clone {
    fn default_for_size(width: u32, height: u32) -> Self;
    fn get_size(&self) -> (u32, u32);

    fn compute_cell(&self, pos: (u32, u32)) -> Cell;

    // fn compute_cells_arrow(&self, xs: UInt32Array, ys: UInt32Array) -> RecordBatch {
    //     let cells = izip!(xs.iter().flatten(), ys.iter().flatten())
    //         .map(|pos| self.compute_cell(pos))
    //         .collect_vec();
    //     // let r = UInt8Array::from_iter_values(cells.iter().map(|c| c.rgb.0));
    //     // let g = UInt8Array::from_iter_values(cells.iter().map(|c| c.rgb.1));
    //     // let b = UInt8Array::from_iter_values(cells.iter().map(|c| c.rgb.2));
    //     let x = UInt32Array::from_iter_values(cells.iter().map(|c| c.pos.0));
    //     let y = UInt32Array::from_iter_values(cells.iter().map(|c| c.pos.1));
    //     let rgb = UInt32Array::from_iter_values(
    //         cells
    //             .iter()
    //             .map(|&Cell { rgb: (r, g, b), .. }| u32::from_ne_bytes([r, g, b, 0xff])),
    //     );

    //     RecordBatch::from(
    //         &StructArray::try_from(vec![
    //             ("x", x.slice(0, x.len())),
    //             ("y", y.slice(0, y.len())),
    //             ("rgb", rgb.slice(0, rgb.len())),
    //         ])
    //         .expect("bad columns"),
    //     )
    // }

    fn compute_cells(&self, positions: &[(u32, u32)]) -> Vec<Cell> {
        positions
            .iter()
            .map(|&pos| self.compute_cell(pos))
            .collect_vec()
    }

    fn with_size(&self, size: (u32, u32)) -> Self;
    fn with_offset(&self, offset: (i32, i32)) -> Self;
    fn add_zoom(&self, zoom_factor: f64) -> Self;

    fn with_option(&self, name: &str, value: &str) -> Self;
    fn get_options(&self) -> HashMap<String, String>;
}
