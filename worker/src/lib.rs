mod cell_grid;

use std::{
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

use abi_stable::{library::RootModule, std_types::RSlice};
use cell_grid::CellGridBuffer;
use core_extensions::SelfOps;
use itertools::{iproduct, Itertools};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use shared::{FractalLib_Ref, RCell, RFractalCellFuncBox, Tuple2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}

// #[derive(Debug)]
pub struct FractalWorker {
    width: u32,
    height: u32,
    epoch: u32,
    pixel_receiver: Option<Receiver<(RCell, u32)>>,
    grid_buf: CellGridBuffer,
    cell_func: RFractalCellFuncBox,
}

const CHUNK_SIZE: usize = 128;

fn start_worker(cell_func: RFractalCellFuncBox, epoch: u32, sender: Sender<(RCell, u32)>) {
    rayon::spawn(move || {
        let (width, height) = cell_func.get_size().into();
        let pixel_positions = iproduct!(0..width, 0..height)
            .map(Tuple2::from)
            .collect_vec();
        let res = pixel_positions
            .par_chunks(CHUNK_SIZE)
            .map_with(cell_func, |cell_func, positions| {
                cell_func.compute_cells(RSlice::from(positions)).into_vec()
            })
            .flatten()
            .try_for_each_with(sender, |tx, cell| tx.send((cell, epoch)));
        match res {
            Ok(_) => println!("render complete"),
            Err(_) => println!("render interrupted"),
        }
    });
}

impl FractalWorker {
    pub fn new(width: u32, height: u32, lib_path: &str) -> Self {
        let fractal_lib: FractalLib_Ref =
            FractalLib_Ref::load_from_file(Path::new(lib_path)).expect("failed to load library");
        Self {
            width,
            height,
            epoch: 0,
            pixel_receiver: None,
            cell_func: fractal_lib.default_cell_func_for_size()(width, height),
            grid_buf: CellGridBuffer::new(width, height),
        }
        .mutated(|s| s.start_new_worker(s.cell_func.clone()))
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn reload_library(&mut self, _lib_path: &str) {
        println!("TODO: reload the library")
    }

    pub fn receive_into_buf(&mut self) {
        if let Some(receiver) = &self.pixel_receiver {
            // for pixel in receiver.try_iter() {
            //     self.buf.push(pixel);
            // }
            for (rcell, epoch) in receiver.try_iter() {
                if epoch != self.epoch {
                    continue;
                }
                self.grid_buf.put_rcell(rcell);
            }
        }
    }

    fn stop_worker(&mut self) {
        self.pixel_receiver = None;
        self.epoch += 1;
    }
    fn start_new_worker(&mut self, cell_func: RFractalCellFuncBox) {
        self.stop_worker();
        let (tx, rx) = channel();
        self.pixel_receiver = Some(rx);
        self.cell_func = cell_func;
        start_worker(self.cell_func.clone(), self.epoch, tx);
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        self.stop_worker();
        self.grid_buf.apply_resize(size);
        self.start_new_worker(self.cell_func.with_size(size.into()));
    }

    pub fn apply_zoom(&mut self, mouse_wheel: f32) {
        self.stop_worker();
        let zoom_factor = if mouse_wheel > 0.0 { 1.1 } else { 1.0 / 1.1 };
        self.grid_buf.apply_zoom(zoom_factor);
        self.start_new_worker((&self.cell_func).add_zoom(zoom_factor));
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        self.stop_worker();
        let offset = offset.mutated(|p| {
            p.0 *= -1;
            p.1 *= -1;
        });
        self.grid_buf.apply_offset(offset);
        self.start_new_worker(self.cell_func.with_offset(offset.into()));
    }

    pub fn draw_with_offset(&self, offset: (i32, i32), screen: &mut [u8], screen_size: (u32, u32)) {
        self.grid_buf.draw_with_offset(offset, screen, screen_size);
    }
}
