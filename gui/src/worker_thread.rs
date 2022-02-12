use std::sync::mpsc::{channel, Receiver, Sender};

use core_extensions::SelfOps;
use image::{ImageBuffer, Rgba};
use itertools::{iproduct, Itertools};
use mandelbrot_f64::MandelbrotCellFunc;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use shared::FractalCellFunc;
use ultraviolet::{DVec2, IVec2};

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
                // thread::sleep(4.milliseconds()); // TODO: remove
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

    fn stop_worker(&mut self) {
        self.pixel_receiver = None;
    }
    fn start_new_worker(&mut self, cell_func: MandelbrotCellFunc) {
        self.stop_worker();
        let (tx, rx) = channel();
        self.pixel_receiver = Some(rx);
        self.cell_func = cell_func;
        start_worker(self.cell_func, tx);
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        self.stop_worker();

        let old_middle = IVec2::new(self.width as i32, self.height as i32) / 2;
        let (new_width, new_height) = size;
        let new_middle = IVec2::new(new_width as i32, new_height as i32) / 2;
        self.buf = self
            .buf
            .iter()
            .flat_map(|&p| {
                let pos = IVec2::new(p.x as i32, p.y as i32);
                let IVec2 { x, y } = (new_middle + (pos - old_middle)).try_into().unwrap();
                if x < 0 || x >= new_width as i32 || y < 0 || y >= new_height as i32 {
                    None
                } else {
                    Some(Pixel {
                        x: x as u32,
                        y: y as u32,
                        ..p
                    })
                }
            })
            .collect_vec();
        self.width = new_width;
        self.height = new_height;

        self.start_new_worker(self.cell_func.with_size(size));
    }

    pub fn apply_zoom(&mut self, mouse_wheel: f32) {
        self.stop_worker();

        let zoom_factor = if mouse_wheel > 0.0 { 1.1 } else { 1.0 / 1.1 };
        let middle = DVec2::new(self.width as f64, self.height as f64) / 2.0;
        self.buf = self
            .buf
            .iter()
            .flat_map(|&p| {
                let pos = DVec2::new(p.x as f64, p.y as f64);
                let IVec2 { x, y } = (middle + (pos - middle) * zoom_factor).try_into().unwrap();
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    None
                } else {
                    Some(Pixel {
                        x: x as u32,
                        y: y as u32,
                        ..p
                    })
                }
            })
            .collect_vec();

        self.start_new_worker(self.cell_func.add_zoom(zoom_factor));
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        self.stop_worker();

        let offset = offset.mutated(|p| {
            p.0 *= -1;
            p.1 *= -1;
        });
        let (dx, dy) = offset;
        self.buf = self
            .buf
            .iter()
            .flat_map(|&p| {
                let x = p.x as i32 - dx;
                let y = p.y as i32 - dy;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    None
                } else {
                    Some(Pixel {
                        x: x as u32,
                        y: y as u32,
                        ..p
                    })
                }
            })
            .collect_vec();

        self.start_new_worker(self.cell_func.with_offset(offset));
    }

    pub fn draw_full_buffer_with_offset(
        &self,
        dx: i32,
        dy: i32,
        screen: &mut [u8],
        screen_width: u32,
        screen_height: u32,
    ) {
        // this is much faster than GenericImage::copy_from()
        for pixel in screen.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x50, 0x00, 0x00, 0xff]);
        }

        let mut screen_buf =
            ImageBuffer::<Rgba<u8>, _>::from_raw(screen_width, screen_height, screen)
                .expect("pixel buffer layout bad");

        for Pixel { x, y, r, g, b } in self.buf.iter() {
            let x = *x as i32 + dx;
            let y = *y as i32 + dy;
            if x < 0 || x >= screen_width as i32 || y < 0 || y >= screen_height as i32 {
                continue;
            }
            screen_buf.put_pixel(x as u32, y as u32, Rgba([*r, *g, *b, 0xff]));
        }
    }
}
