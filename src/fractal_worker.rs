use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use core_extensions::ToTime;
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
    worker: JoinHandle<()>,
    buf: Vec<Pixel>,
}

impl FractalWorker {
    pub fn new(width: u32, height: u32) -> Self {
        let (tx, rx) = channel();

        let worker = thread::spawn(move || loop {
            let fractal_cell_func = MandelbrotCellFunc::default_for_size(width, height);

            for x in 0..width {
                for y in 0..height {
                    let cell = fractal_cell_func.compute_cell((x, y));
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
                    thread::sleep(1.microseconds());
                }
            }
        });

        Self {
            width,
            height,
            pixel_receiver: rx,
            worker,
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
