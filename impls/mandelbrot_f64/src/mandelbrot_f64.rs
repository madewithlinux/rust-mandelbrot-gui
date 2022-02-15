use std::collections::HashMap;

use num::complex::Complex64;
use num::Zero;

use shared::raw::{Cell, FractalCellFunc};

#[derive(Debug, Clone, Copy)]
pub struct MandelbrotCellFunc {
    width: u32,
    height: u32,
    max_iter: usize,
    center: Complex64,
    //
    top_left: Complex64,
    pixel_size: Complex64,
}

impl MandelbrotCellFunc {
    fn pixel_re(&self) -> Complex64 {
        Complex64::new(self.pixel_size.re, 0.0)
    }
    fn pixel_im(&self) -> Complex64 {
        Complex64::new(0.0, self.pixel_size.im)
    }

    fn pos_to_complex(&self, pos: (u32, u32)) -> Complex64 {
        self.top_left + self.pixel_re().scale(pos.0 as f64) + self.pixel_im().scale(pos.1 as f64)
    }
}

impl FractalCellFunc for MandelbrotCellFunc {
    fn default_for_size(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            max_iter: 1024,
            // max_iter: 8192,
            center: Complex64::new(0.0, 0.0),
            top_left: Complex64::new(-1.0, 1.0),
            pixel_size: Complex64::new(2.0 / (width as f64), -2.0 / (height as f64)),
        }
    }
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn compute_cell(&self, pos: (u32, u32)) -> Cell {
        let max_iterations = self.max_iter;
        let magnitude_threshold_sqr = 4.0;
        let mut z = Complex64::zero();
        let c = self.pos_to_complex(pos);

        let mut outside = false;
        let mut iter = 0;
        for i in 0..max_iterations {
            z = z.powu(2) + c;
            if z.norm_sqr() >= magnitude_threshold_sqr {
                iter = i;
                outside = true;
                break;
            }
            if z == Complex64::zero() {
                iter = i;
                outside = false;
                break;
            }
        }

        let color = if outside {
            ((iter as f32).sqrt().sin().powi(2) * 255.0) as u8
        } else {
            0
        };

        Cell {
            pos,
            iter: iter as f32,
            rgb: (color, color, color),
        }
    }

    fn with_size(&self, size: (u32, u32)) -> Self {
        let (width, height) = size;
        // middle doesn't change, just top-left
        let middle = self.pos_to_complex((self.width / 2, self.height / 2));
        let top_left = middle
            - self.pixel_re() * ((width / 2) as f64)
            - self.pixel_im() * ((height / 2) as f64);
        Self {
            width,
            height,
            top_left,
            ..*self
        }
    }

    fn with_offset(&self, offset: (i32, i32)) -> Self {
        let complex_offset =
            self.pixel_re().scale(offset.0 as f64) + self.pixel_im().scale(offset.1 as f64);
        Self {
            center: self.center + complex_offset,
            top_left: self.top_left + complex_offset,
            ..*self
        }
    }

    fn add_zoom(&self, zoom_factor: f64) -> Self {
        let middle = self.pos_to_complex((self.width / 2, self.height / 2));
        let pixel_size = self.pixel_size.scale(1.0 / zoom_factor);
        let top_left = middle
            + Complex64::new(
                -pixel_size.re * ((self.width / 2) as f64),
                -pixel_size.im * ((self.height / 2) as f64),
            );
        Self {
            top_left,
            pixel_size,
            ..*self
        }
    }

    fn with_option(&self, _name: &str, _value: &str) -> Self {
        todo!()
    }

    fn get_options(&self) -> std::collections::HashMap<String, String> {
        HashMap::from([
            ("width".to_owned(), format!("{}", self.width)),
            ("height".to_owned(), format!("{}", self.height)),
            ("max_iter".to_owned(), format!("{}", self.max_iter)),
            ("center".to_owned(), format!("{}", self.center)),
            ("top_left".to_owned(), format!("{}", self.top_left)),
            ("pixel_size".to_owned(), format!("{}", self.pixel_size)),
        ])
    }
}

