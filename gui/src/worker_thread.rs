use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use core_extensions::{SelfOps, ToTime};
use itertools::{iproduct, Itertools};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use mandelbrot_f64::MandelbrotCellFunc;
use shared::FractalCellFunc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
pub struct FractalWorker {
    width: u32,
    height: u32,
    pixel_receiver: Option<Receiver<Pixel>>,
    buf: Vec<Pixel>,
    cell_func: MandelbrotCellFunc,
}

const CHUNK_SIZE: usize = 128;

fn start_worker(cell_func: MandelbrotCellFunc, sender: Sender<Pixel>) {
    rayon::spawn(move || {
        let (width, height) = cell_func.get_size();
        let pixel_positions = iproduct!(0..width, 0..height).collect_vec();
        let res = pixel_positions
            .par_chunks(CHUNK_SIZE)
            .map_with(cell_func, |cell_func, positions| {
                thread::sleep(4.milliseconds()); // TODO: remove
                cell_func.compute_cells(positions)
            })
            .flatten()
            .try_for_each_with(sender, |tx, cell| {
                tx.send(Pixel {
                    x: cell.pos.0,
                    y: cell.pos.1,
                    r: cell.rgb.0,
                    g: cell.rgb.1,
                    b: cell.rgb.2,
                })
            });
        match res {
            Ok(_) => println!("render complete"),
            Err(_) => println!("render interrupted"),
        }
    });
}

impl FractalWorker {
    pub fn new(width: u32, height: u32) -> Self {
        let (tx, rx) = channel();

        let cell_func = MandelbrotCellFunc::default_for_size(width, height);
        start_worker(cell_func, tx);

        Self {
            width,
            height,
            pixel_receiver: Some(rx),
            cell_func,
            buf: Default::default(),
        }
    }

    pub fn receive_into_buf(&mut self) {
        if let Some(receiver) = &self.pixel_receiver {
            for pixel in receiver.try_iter() {
                self.buf.push(pixel);
            }
        }
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        self.pixel_receiver = None;
        let offset = offset.mutated(|p| {
            p.0 *= -1;
            p.1 *= -1;
        });
        let (dx, dy) = offset;
        self.buf = self
            .buf
            .iter()
            .flat_map(|Pixel { x, y, r, g, b }| {
                let x = *x as i32 - dx;
                let y = *y as i32 - dy;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    None
                } else {
                    Some(Pixel {
                        x: x as u32,
                        y: y as u32,
                        r: *r,
                        g: *g,
                        b: *b,
                    })
                }
            })
            .collect_vec();

        let (tx, rx) = channel();
        self.pixel_receiver = Some(rx);
        self.cell_func = self.cell_func.with_offset(offset);
        start_worker(self.cell_func, tx);
    }

    pub fn draw_full_buffer_with_offset(&self, dx: i32, dy: i32, screen: &mut [u8]) {
        for pixel in screen.chunks_exact_mut(4) {
            pixel[0] = 0x50; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x00; // B
            pixel[3] = 0xff; // A
        }

        for Pixel { x, y, r, g, b } in self.buf.iter() {
            let x = *x as i32 + dx;
            let y = *y as i32 + dy;
            if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                continue;
            }
            let idx = ((x + y * self.width as i32) * 4) as usize;
            let color = [*r, *g, *b, 0xff];
            let pix = &mut screen[idx..idx + 4];
            pix.copy_from_slice(&color);
        }
    }
}
