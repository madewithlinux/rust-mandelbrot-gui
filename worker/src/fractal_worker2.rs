use std::{
    cmp::min,
    collections::HashSet,
    path::{Path, PathBuf},
    sync::mpsc::{channel, Receiver},
};

use abi_stable::{library::RootModule, std_types::RSlice};
use abi_stable::{
    library::{lib_header_from_raw_library, RawLibrary},
    std_types::{
        RResult::{RErr, ROk},
        RString,
    },
    utils::leak_value,
};
use anyhow::Context;
use core_extensions::SelfOps;
use itertools::Itertools;
use log::info;
use rand::{prelude::SliceRandom, thread_rng};
use rayon::{
    current_num_threads,
    iter::{IntoParallelIterator, ParallelIterator},
};

use color_func::{
    prelude::{ColorLib_Ref, RColorFuncBox},
    RColor,
};
use fractal_func::prelude::*;

///////////////////////////////////////////////////////////////////////////////

/// this is similar to RootModule::load_from_file(), except that it supports reloading the module.
/// (It still leaks the memory each time the module is loaded, though...)
fn load_from_file<Lib: RootModule>(path: &Path) -> anyhow::Result<Lib> {
    // copy the library to a unique path, to make sure it gets reloaded even if it was already loaded
    let unique_name = {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        path.with_file_name(format!(
            "{}-{}",
            path.file_name().unwrap().to_string_lossy(),
            timestamp
        ))
    };
    std::fs::copy(&path, &unique_name).expect("Failed to copy lib to unique path");
    let unique_lib_path = Path::new(&unique_name).canonicalize().unwrap();
    let raw_library = RawLibrary::load_at(&unique_lib_path)?;
    // if the library isn't leaked
    // it would cause any use of the module to be a use after free.
    //
    // By leaking the library
    // this allows the root module loader to do anything that'd prevent
    // sound library unloading.
    let lib = leak_value(raw_library);

    let items = unsafe { lib_header_from_raw_library(lib)? };

    items.ensure_layout::<Lib>()?;

    // safety: the layout was checked in the code above,
    let ret = unsafe {
        items
            .init_root_module_with_unchecked_layout::<Lib>()?
            .initialization()
    }
    .context("loading library")?;

    // remove the file after we load it
    std::fs::remove_file(&unique_lib_path)?;

    Ok(ret)
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Init,
    Started,
    Working { total: usize, completed: usize },
    Interrupted,
    Finished,
}

#[derive(Debug)]
enum WorkerMessage {
    Init,
    Finished,
    Chunk(RChunk, RVec<RColor>, u32),
}

pub struct FractalWorker {
    // config
    width: u32,
    height: u32,
    chunk_size: usize,
    // state
    epoch: u32,
    state: WorkerState,
    receiver: Option<Receiver<WorkerMessage>>,
    chunks: Vec<RChunk>,
    should_clear_screen: bool,
    // FFI
    fractal_lib_path: PathBuf,
    fractal_lib: FractalLib_Ref,
    fractal_func: RFractalFuncBox,
    color_lib_path: PathBuf,
    color_lib: ColorLib_Ref,
    color_func: RColorFuncBox,
}

impl FractalWorker {
    pub fn new(width: u32, height: u32, fractal_lib_path: &str, color_lib_path: &str) -> Self {
        let fractal_lib_path: PathBuf = PathBuf::from(fractal_lib_path);
        let color_lib_path: PathBuf = PathBuf::from(color_lib_path);

        let fractal_lib: FractalLib_Ref = FractalLib_Ref::load_from_file(&fractal_lib_path)
            .expect("failed to load fractal library");

        // let color_lib: ColorLib_Ref = ColorLib_Ref::load_from_file(&color_lib_path)
        //     .expect("failed to load color library");
        let color_lib: ColorLib_Ref =
            load_from_file(&color_lib_path).expect("failed to load color library");
        Self {
            width,
            height,
            //
            epoch: 0,
            state: WorkerState::Init,
            receiver: None,
            chunk_size: 32,
            chunks: vec![],
            should_clear_screen: true,
            //
            fractal_lib_path,
            fractal_lib,
            fractal_func: fractal_lib.default_fractal_func_for_size()(width, height),
            color_lib_path,
            color_lib,
            color_func: color_lib.default_color_func()(),
        }
        .mutated(|s| s.start_worker(None, None, None))
    }

    pub fn reload_libraries(&mut self) -> anyhow::Result<()> {
        // TODO: reload fractal lib also
        self.color_lib = load_from_file(&self.color_lib_path)?;
        self.color_func = self.color_lib.default_color_func()();

        self.reset();
        self.start_worker(None, None, None);

        Ok(())
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_fractal_options(&self) -> ROptionsMap {
        self.fractal_func.get_options()
    }
    pub fn get_color_options(&self) -> ROptionsMap {
        self.color_func.get_options()
    }

    pub fn reset_fractal_options(&mut self) {
        self.reset();
        self.start_worker(
            self.fractal_lib.default_fractal_func_for_size()(
                min(self.width, self.height),
                min(self.width, self.height),
            )
            .with_size(self.width, self.height),
            None,
            None,
        )
    }
    pub fn set_fractal_options(&mut self, new_options: RHashMap<RString, RString>) {
        let mut fractal_func = self.fractal_func.clone();
        for Tuple2(name, value) in new_options.into_iter() {
            fractal_func = match fractal_func.with_option(name.as_rstr(), value.as_rstr()) {
                ROk(cell_func) => cell_func,
                RErr(msg) => {
                    println!("failed to set option {}={}: {}", name, value, msg);
                    return;
                }
            }
        }
        self.reset();
        self.start_worker(fractal_func, None, None);
    }
    pub fn set_color_options(&mut self, new_options: RHashMap<RString, RString>) {
        let mut color_func = self.color_func.clone();
        for Tuple2(name, value) in new_options.into_iter() {
            color_func = match color_func.with_option(name.as_rstr(), value.as_rstr()) {
                ROk(cell_func) => cell_func,
                RErr(msg) => {
                    println!("failed to set option {}={}: {}", name, value, msg);
                    return;
                }
            }
        }
        self.reset();
        self.start_worker(None, color_func, None);
    }

    pub fn get_state(&self) -> WorkerState {
        self.state
    }

    /// range: 0 to 1
    pub fn get_progress(&self) -> f32 {
        match self.state {
            WorkerState::Init => 0.0,
            WorkerState::Interrupted => 0.0,
            WorkerState::Started => 0.0,
            WorkerState::Working { total, completed } => completed as f32 / total as f32,
            WorkerState::Finished => 1.0,
        }
    }

    pub fn draw_new_chunks(&mut self, width: u32, height: u32, screen: &mut [u8]) {
        if self.should_clear_screen {
            for rgba in screen.chunks_exact_mut(4) {
                rgba.copy_from_slice(&[0, 0, 0, 0]);
            }
            self.should_clear_screen = false;
        }

        self.width = width;
        self.height = height;
        if let Some(receiver) = &self.receiver {
            for message in receiver.try_iter() {
                match message {
                    WorkerMessage::Init => self.state = WorkerState::Init,
                    // TODO: what if it's finished but there's still messages in the buffer?
                    WorkerMessage::Finished => self.state = WorkerState::Finished,
                    WorkerMessage::Chunk(rchunk, rcolors, epoch) if epoch == self.epoch => {
                        draw_chunk_colors(rcolors, width, screen);
                        self.chunks.push(rchunk);
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

    fn reset(&mut self) {
        // self.receiver = None;
        self.chunks = vec![];
        self.should_clear_screen = true;
    }

    pub fn apply_offset_and_zoom_factor(&mut self, dx: i32, dy: i32, zoom_factor: f64) {
        info!("apply_offset_and_zoom_factor");
        self.reset();
        let dx = -dx;
        let dy = -dy;

        let mut new_func = self.fractal_func.clone();
        if dx != 0 || dy != 0 {
            new_func = new_func.with_offset(dx, dy);
        }
        if (zoom_factor - 1.0).abs() > 0.0001 {
            new_func = new_func.add_zoom(zoom_factor);
        }
        self.start_worker(new_func, None, None);
    }

    pub fn apply_resize(&mut self, new_size: (u32, u32)) {
        info!("apply_resize");
        self.start_worker(
            self.fractal_func.with_size(new_size.0, new_size.1),
            None,
            new_size,
        );
        // TODO: move around the stuff in pixels.frame so that it's in the right place, instead of clearing it
        self.reset();
    }

    fn start_worker(
        &mut self,
        fractal_func: impl Into<Option<RFractalFuncBox>>,
        color_func: impl Into<Option<RColorFuncBox>>,
        new_size: impl Into<Option<(u32, u32)>>,
    ) {
        if let Some(fractal_func) = fractal_func.into() {
            self.fractal_func = fractal_func;
        }
        if let Some(color_func) = color_func.into() {
            self.color_func = color_func;
        }
        let existing_chunks_offset = if let Some((width, height)) = new_size.into() {
            let dx = (width as i32 - self.width as i32) / 2;
            let dy = (height as i32 - self.height as i32) / 2;
            self.width = width;
            self.height = height;
            [dx, dy]
        } else {
            [0, 0]
        };
        self.receiver = start_worker(
            self.width,
            self.height,
            &self.fractal_func,
            &self.color_func,
            self.epoch,
            self.chunk_size,
            std::mem::take(&mut self.chunks),
            existing_chunks_offset,
        )
        .into();
        self.state = WorkerState::Started;
    }
}

fn draw_chunk_colors(rcolors: RVec<RColor>, width: u32, screen: &mut [u8]) {
    let width = width as usize;
    for rcolor in rcolors {
        let x = rcolor.pos[0] as usize;
        let y = rcolor.pos[1] as usize;
        let idx = (x + y * width) * 4;
        // if (idx + 3) >= screen.len() {
        //     dbg!(width, x, y, idx, screen.len());
        // }
        screen[idx..idx + 3].copy_from_slice(&rcolor.rgb);
        screen[idx + 3] = 0xff;
    }
}

fn get_all_pixel_positions(width: u32, height: u32, chunk_size: usize) -> Vec<Vec<[u32; 2]>> {
    let num_chunks = (0..width).step_by(chunk_size).len() * (0..height).step_by(chunk_size).len();
    info!(
        "chunk_size={chunk_size}, num_chunks={num_chunks}, per thread: {}",
        num_chunks / current_num_threads()
    );
    let mut chunks = Vec::with_capacity(num_chunks);
    for xmin in (0..width).step_by(chunk_size) {
        for ymin in (0..height).step_by(chunk_size) {
            chunks.push(
                (xmin..min(width, xmin + chunk_size as u32))
                    .cartesian_product(ymin..min(height, ymin + chunk_size as u32))
                    .map(|(x, y)| [x, y])
                    .collect_vec(),
            );
        }
    }
    chunks
}

fn _get_incomplete_pixel_positions(
    width: u32,
    height: u32,
    chunk_size: usize,
    existing_chunks: &[RChunk],
) -> Vec<Vec<[u32; 2]>> {
    let mut coordinates: HashSet<_> = (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| [x, y])
        .collect();

    for chunk in existing_chunks {
        for (pos, _) in chunk.iter() {
            coordinates.remove(&pos);
        }
    }

    coordinates
        .into_iter()
        .sorted()
        .chunks(chunk_size)
        .into_iter()
        .map(|vs| vs.collect_vec())
        .collect_vec()
}

fn start_worker(
    width: u32,
    height: u32,
    fractal_func: &RFractalFuncBox,
    color_func: &RColorFuncBox,
    epoch: u32,
    chunk_size: usize,
    _existing_chunks: Vec<RChunk>,
    existing_chunks_offset: [i32; 2],
) -> Receiver<WorkerMessage> {
    // println!("starting worker");
    info!("starting worker");
    let fractal_func = fractal_func.clone();
    let color_func = color_func.clone();
    let (sender, receiver) = channel();

    rayon::spawn(move || {
        info!("worker thread started");
        sender
            .send(WorkerMessage::Init)
            .expect("interrupted before beginning render");

        if existing_chunks_offset != [0, 0] {
            dbg!(existing_chunks_offset);
        }

        // TODO: fix the existing chunk recoloring thingy to work with new chunk type
        // let existing_chunks: Vec<RChunk> = existing_chunks
        //     .into_par_iter()
        //     .map(|chunk| {
        //         chunk
        //             .into_iter()
        //             .filter_map(|mut cell| {
        //                 let x = cell.pos[0] as i32 + existing_chunks_offset[0];
        //                 let y = cell.pos[1] as i32 + existing_chunks_offset[1];
        //                 if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
        //                     None
        //                 } else {
        //                     cell.pos[0] = x as u32;
        //                     cell.pos[1] = y as u32;
        //                     Some(cell)
        //                 }
        //             })
        //             .collect()
        //     })
        //     .collect();

        // for chunk in existing_chunks.iter_mut() {
        //     for cell in chunk.iter_mut() {
        //         cell.pos[0] += existing_chunks_offset[0];
        //         cell.pos[1] += existing_chunks_offset[1];
        //     }
        // }

        // let pixel_positions = if existing_chunks.len() > 0 {
        //     get_incomplete_pixel_positions(width, height, chunk_size, &existing_chunks)
        // } else {
        //     get_all_pixel_positions(width, height, chunk_size)
        // };

        // if existing_chunks.len() > 0 {
        //     info!("recolor existing chunks");
        //     // recolor existing chunks
        //     let res = existing_chunks
        //         .into_par_iter()
        //         .map(|rchunk| {
        //             let rcolors = color_func.compute_colors(&rchunk);
        //             (rchunk, rcolors)
        //         })
        //         .try_for_each_with(sender.clone(), |sender, (rchunk, rcolors)| {
        //             sender.send(WorkerMessage::Chunk(rchunk, rcolors, epoch))
        //         });
        //     match res {
        //         Ok(_) => info!("existing chunks complete"),
        //         Err(_) => info!("existing chunks interrupted"),
        //     }
        // } else {
        //     info!("existing chunks empty");
        // }

        let pixel_positions = {
            let mut pixel_positions = get_all_pixel_positions(width, height, chunk_size);
            let mut rng = thread_rng();
            pixel_positions.as_mut_slice().shuffle(&mut rng);
            pixel_positions
        };

        let res = pixel_positions
            .into_par_iter()
            .map(|positions| {
                let rchunk = fractal_func.compute_cells(RSlice::from(positions.as_slice()));
                let rcolors = color_func.compute_colors(&rchunk);
                (rchunk, rcolors)
            })
            .try_for_each_with(sender.clone(), |sender, (rchunk, rcolors)| {
                sender.send(WorkerMessage::Chunk(rchunk, rcolors, epoch))
            })
            .map(|_| sender.send(WorkerMessage::Finished));
        match res {
            Ok(_) => info!("render complete"),
            Err(_) => info!("render interrupted"),
        }
    });

    receiver
}
