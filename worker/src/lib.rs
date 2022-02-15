mod cell_grid;
pub mod util;

use std::{
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

use abi_stable::{library::RootModule, std_types::RSlice};
use cell_grid::CellGridBuffer;
use core_extensions::SelfOps;
use itertools::{iproduct, Itertools};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use shared::{FractalLib_Ref, ROptionsMap, RCell, RFractalCellFuncBox, Tuple2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Init,
    Working,
    Interrupted,
    Finished,
    /// implies that worker is not running (stopped or interrupted)
    InputDebounce,
}

#[derive(Debug)]
enum WorkerMessage {
    Init,
    Finished,
    // Interrupted, // not possible to send because interrupted only happens when the channel is closed
    Chunk(RCell, u32),
}

#[derive(Debug)]
pub struct FractalWorker {
    width: u32,
    height: u32,
    epoch: u32,
    state: WorkerState,
    pixel_receiver: Option<Receiver<WorkerMessage>>,
    grid_buf: CellGridBuffer,
    cell_func: RFractalCellFuncBox,
}

const CHUNK_SIZE: usize = 128;

fn start_worker(cell_func: RFractalCellFuncBox, epoch: u32, sender: Sender<WorkerMessage>) {
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
            .try_for_each_with(sender.clone(), |sender, cell| {
                sender.send(WorkerMessage::Chunk(cell, epoch))
            })
            .map(|_| sender.send(WorkerMessage::Finished));
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
            state: WorkerState::Init,
            pixel_receiver: None,
            cell_func: fractal_lib.default_cell_func_for_size()(width, height),
            grid_buf: CellGridBuffer::new(width, height),
        }
        .mutated(|s| s.start_new_worker(s.cell_func.clone()))
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_fractal_options(&self) -> ROptionsMap {
        self.cell_func.get_options()
    }

    pub fn get_state(&self) -> WorkerState {
        self.state
    }

    pub fn reload_library(&mut self, _lib_path: &str) {
        println!("TODO: reload the library")
    }

    pub fn receive_into_buf(&mut self) {
        if let Some(receiver) = &self.pixel_receiver {
            for message in receiver.try_iter() {
                match message {
                    WorkerMessage::Finished => self.state = WorkerState::Finished,
                    WorkerMessage::Chunk(rcell, epoch) if epoch == self.epoch => {
                        self.grid_buf.put_rcell(rcell);
                    }
                    _ => (),
                }
            }
        }
    }

    fn stop_worker(&mut self) {
        self.pixel_receiver = None;
        self.epoch += 1;
        self.state = WorkerState::Interrupted;
    }

    /// get ready to start a new worker once debounced events are handled
    fn debounce_start_new_worker(&mut self, cell_func: RFractalCellFuncBox) {
        self.cell_func = cell_func;
        self.state = WorkerState::InputDebounce;
    }

    fn start_or_restart_worker(&mut self) {
        self.stop_worker();
        let (tx, rx) = channel();
        self.pixel_receiver = Some(rx);
        start_worker(self.cell_func.clone(), self.epoch, tx);
        self.state = WorkerState::Working;
    }

    fn start_new_worker(&mut self, cell_func: RFractalCellFuncBox) {
        self.stop_worker();
        self.cell_func = cell_func;
        self.start_or_restart_worker();
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        self.stop_worker();
        self.grid_buf.apply_resize(size);
        self.debounce_start_new_worker(self.cell_func.with_size(size.into()));
    }

    pub fn apply_zoom(&mut self, mouse_wheel: f32) {
        self.stop_worker();
        let zoom_factor = if mouse_wheel > 0.0 { 1.1 } else { 1.0 / 1.1 };
        self.grid_buf.apply_zoom(zoom_factor);
        self.debounce_start_new_worker(self.cell_func.add_zoom(zoom_factor));
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        self.stop_worker();
        let offset = offset.mutated(|p| {
            p.0 *= -1;
            p.1 *= -1;
        });
        self.grid_buf.apply_offset(offset);
        self.debounce_start_new_worker(self.cell_func.with_offset(offset.into()));
    }

    pub fn draw_with_offset(&self, offset: (i32, i32), screen: &mut [u8], screen_size: (u32, u32)) {
        self.grid_buf.draw_with_offset(offset, screen, screen_size);
    }

    pub fn on_main_events_cleared(&mut self) {
        if self.state == WorkerState::InputDebounce {
            self.start_or_restart_worker();
        }
    }
}
