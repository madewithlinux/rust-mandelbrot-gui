use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use core_extensions::{SelfOps, ToTime};
use itertools::Itertools;
use rust_mandelbrot_gui::fractal::mandelbrot::MandelbrotCellFunc;
use rust_mandelbrot_gui::fractal::{Cell, FractalCellFunc};

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
    pixel_receiver: Receiver<Pixel>,
    // worker: JoinHandle<()>,
    buf: Vec<Pixel>,
    cell_func: MandelbrotCellFunc,
}

fn start_worker(cell_func: MandelbrotCellFunc, tx: Sender<Pixel>) {
    // let tx = tx.clone();
    // let cell_func = cell_func.clone();
    let worker = thread::spawn(move || loop {
        let (width, height) = cell_func.get_size();
        for x in 0..width {
            for y in 0..height {
                let cell = cell_func.compute_cell((x, y));
                let pix = Pixel {
                    x: cell.pos.0,
                    y: cell.pos.1,
                    r: cell.rgb.0,
                    g: cell.rgb.1,
                    b: cell.rgb.2,
                };
                if tx.send(pix).is_err() {
                    println!("worker thread exiting");
                    return;
                }
            }
            thread::sleep(1.milliseconds());
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
            pixel_receiver: rx,
            cell_func,
            buf: Default::default(),
        }
    }

    pub fn receive_into_buf(&mut self) {
        for pixel in self.pixel_receiver.try_iter() {
            // if pixel.x == 0 && pixel.y == 0 {
            //     self.buf.clear();
            // }
            self.buf.push(pixel);
        }
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
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

        self.cell_func = self.cell_func.with_offset(offset);
        start_worker(self.cell_func, tx);
        self.pixel_receiver = rx;
    }

    pub fn draw_full_buffer_with_offset(&self, dx: i32, dy: i32, screen: &mut [u8]) {
        for pixel in screen.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
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
