mod cell_grid;
pub mod matrix_util;
pub mod util;

use std::{
    cmp::min,
    collections::HashMap,
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

use abi_stable::std_types::{
    RResult::{RErr, ROk},
    RStr, RString,
};
use abi_stable::{library::RootModule, std_types::RSlice};
use cell_grid::CellGridBuffer;
use core_extensions::SelfOps;
use itertools::Itertools;
// use rand::{prelude::SliceRandom, thread_rng};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use color_func::{
    prelude::{ColorLib_Ref, RColorFuncBox},
    RColor,
};
use fractal_func::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Init,
    Working {
        total: usize,
        completed: usize,
    },
    Interrupted,
    Finished,
    FinishedRendered,
    /// implies that worker is not running (stopped or interrupted)
    InputDebounce,
}

#[derive(Debug)]
enum WorkerMessage {
    Init,
    Finished,
    // Interrupted, // not possible to send because interrupted only happens when the channel is closed
    Chunk(RCell, RColor, u32),
}

// #[derive(Debug)]
pub struct FractalWorker {
    width: u32,
    height: u32,
    epoch: u32,
    state: WorkerState,
    pixel_receiver: Option<Receiver<WorkerMessage>>,
    grid_buf: CellGridBuffer,
    //
    fractal_lib: FractalLib_Ref,
    color_lib: ColorLib_Ref,
    //
    fractal_func: RFractalFuncBox,
    color_func: RColorFuncBox,
}

const CHUNK_SIZE: usize = 128;

impl FractalWorker {
    pub fn new(width: u32, height: u32, fractal_lib_path: &str, color_lib_path: &str) -> Self {
        let fractal_lib: FractalLib_Ref =
            FractalLib_Ref::load_from_file(Path::new(fractal_lib_path))
                .expect("failed to load fractal library");
        let color_lib: ColorLib_Ref = ColorLib_Ref::load_from_file(Path::new(color_lib_path))
            .expect("failed to load color library");
        Self {
            width,
            height,
            epoch: 0,
            state: WorkerState::Init,
            pixel_receiver: None,
            grid_buf: CellGridBuffer::new(width, height),
            fractal_lib,
            color_lib,
            fractal_func: fractal_lib.default_fractal_func_for_size()(width, height),
            color_func: color_lib.default_color_func()(),
        }
        .mutated(|s| s.start_or_restart_worker())
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_fractal_options(&self) -> ROptionsMap {
        self.fractal_func.get_options()
    }
    pub fn reset_fractal_options(&mut self) {
        self.grid_buf.mark_all_positions_stale();
        self.color_func = self.color_lib.default_color_func()();
        self.start_new_worker(
            self.fractal_lib.default_fractal_func_for_size()(
                min(self.width, self.height),
                min(self.width, self.height),
            )
            .with_size(self.width, self.height),
        )
    }
    pub fn set_fractal_options(&mut self, options: &HashMap<RString, String>) {
        let mut cell_func = self.fractal_func.clone();
        for (name, value) in options.iter() {
            cell_func = match self
                .fractal_func
                .with_option(name.as_rstr(), value.as_str().into())
            {
                ROk(cell_func) => cell_func,
                RErr(msg) => {
                    println!("failed to set option {}={}: {}", name, value, msg);
                    return;
                }
            }
        }
        self.grid_buf.mark_all_positions_stale();
        self.start_new_worker(cell_func);
    }
    pub fn set_fractal_option(&mut self, name: RStr, value: &str) {
        match self.fractal_func.with_option(name, value.into()) {
            ROk(cell_func) => {
                self.grid_buf.mark_all_positions_stale();
                self.start_new_worker(cell_func);
            }
            RErr(msg) => println!("failed to set option {}={}: {}", name, value, msg),
        }
    }

    pub fn get_state(&self) -> WorkerState {
        self.state
    }

    /// range: 0 to 1
    pub fn get_progress(&self) -> f32 {
        match self.state {
            WorkerState::Init => 0.0,
            WorkerState::InputDebounce => 0.0,
            WorkerState::Interrupted => 0.0,
            WorkerState::Working { total, completed } => completed as f32 / total as f32,
            WorkerState::Finished | WorkerState::FinishedRendered => 1.0,
        }
    }

    pub fn reload_library(&mut self, _lib_path: &str) {
        println!("TODO: reload the library")
    }

    pub fn receive_into_buf(&mut self) {
        if let Some(receiver) = &self.pixel_receiver {
            for message in receiver.try_iter() {
                match message {
                    WorkerMessage::Finished => self.state = WorkerState::Finished,
                    WorkerMessage::Chunk(rcell, rcolor, epoch) if epoch == self.epoch => {
                        self.grid_buf.put_value(rcell, rcolor);
                        if let WorkerState::Working { total, completed } = self.state {
                            self.state = WorkerState::Working {
                                total,
                                completed: completed + 1,
                            };
                        }
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
    fn debounce_start_new_worker(&mut self, cell_func: RFractalFuncBox) {
        self.fractal_func = cell_func;
        self.state = WorkerState::InputDebounce;
    }

    fn start_or_restart_worker(&mut self) {
        self.stop_worker();
        let (tx, rx) = channel();
        self.pixel_receiver = Some(rx);
        let positions = self.grid_buf.get_stale_positions();
        let positions_len = positions.len();
        start_worker(
            self.fractal_func.clone(),
            self.color_func.clone(),
            self.epoch,
            positions,
            tx,
        );
        self.state = WorkerState::Working {
            total: positions_len,
            completed: 0,
        };
    }

    fn start_new_worker(&mut self, cell_func: RFractalFuncBox) {
        self.stop_worker();
        self.fractal_func = cell_func;
        self.start_or_restart_worker();
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        self.stop_worker();
        self.width = size.0;
        self.height = size.1;
        self.grid_buf.apply_resize(size);
        self.debounce_start_new_worker(self.fractal_func.with_size(size.0, size.1));
    }

    pub fn apply_zoom(&mut self, mouse_wheel: f32) {
        // self.stop_worker();
        let zoom_factor = if mouse_wheel > 0.0 { 1.1 } else { 1.0 / 1.1 };
        self.apply_zoom_factor(zoom_factor);
        // self.grid_buf.apply_zoom(zoom_factor);
        // self.debounce_start_new_worker(self.fractal_func.add_zoom(zoom_factor));
    }
    pub fn apply_zoom_factor(&mut self, zoom_factor: f64) {
        self.stop_worker();
        self.grid_buf.apply_zoom(zoom_factor);
        self.debounce_start_new_worker(self.fractal_func.add_zoom(zoom_factor));
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        self.stop_worker();
        let offset = offset.mutated(|p| {
            p.0 *= -1;
            p.1 *= -1;
        });
        self.grid_buf.apply_offset(offset);
        self.debounce_start_new_worker(self.fractal_func.with_offset(offset.0, offset.1));
    }

    pub fn apply_offset_and_zoom_factor(&mut self, dx: i32, dy: i32, zoom_factor: f64) {
        self.stop_worker();
        let dx = -dx;
        let dy = -dy;

        let mut new_func = self.fractal_func.clone();
        if dx != 0 || dy != 0 {
            new_func = new_func.with_offset(dx, dy);
            self.grid_buf.apply_offset((dx, dy));
        }
        if (zoom_factor - 1.0).abs() > 0.0001 {
            new_func = new_func.add_zoom(zoom_factor);
            self.grid_buf.apply_zoom(zoom_factor);
        }
        self.debounce_start_new_worker(new_func);
    }

    pub fn draw_with_offset(
        &mut self,
        offset: (i32, i32),
        screen: &mut [u8],
        screen_size: (u32, u32),
    ) {
        if self.state != WorkerState::FinishedRendered {
            self.grid_buf.draw_with_offset(offset, screen, screen_size);
        }
        if self.state == WorkerState::Finished {
            self.state = WorkerState::FinishedRendered
        }
    }

    pub fn on_main_events_cleared(&mut self) {
        if self.state == WorkerState::InputDebounce {
            self.start_or_restart_worker();
        }
    }
}

fn start_worker(
    fractal_func: RFractalFuncBox,
    color_func: RColorFuncBox,
    epoch: u32,
    pixel_positions: Vec<[u32; 2]>,
    sender: Sender<WorkerMessage>,
) {
    println!("starting worker");
    rayon::spawn(move || {
        sender
            .send(WorkerMessage::Init)
            .expect("interrupted before beginning render");

        // let mut rng = thread_rng();
        // pixel_positions.as_mut_slice().shuffle(&mut rng);

        let res = pixel_positions
            .par_chunks(CHUNK_SIZE)
            .map_with(fractal_func, |fractal_func, positions| {
                let cells = fractal_func.compute_cells(RSlice::from(positions));
                color_func
                    .compute_colors(cells.as_rslice())
                    .into_iter()
                    .zip(cells)
                    .collect_vec()
            })
            .flatten()
            .try_for_each_with(sender.clone(), |sender, (rcolor, rcell)| {
                sender.send(WorkerMessage::Chunk(rcell, rcolor, epoch))
            })
            .map(|_| sender.send(WorkerMessage::Finished));
        match res {
            Ok(_) => println!("render complete"),
            Err(_) => println!("render interrupted"),
        }
    });
}
