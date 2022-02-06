use num::complex::Complex64;
use num::Zero;
use std::collections::HashMap;

use super::{Cell, FractalCellFunc};

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
    fn pos_to_complex(&self, pos: (u32, u32)) -> Complex64 {
        self.top_left
            + Complex64::new(
                self.pixel_size.re * (pos.0 as f64),
                self.pixel_size.im * (pos.1 as f64),
            )
    }
}

impl FractalCellFunc for MandelbrotCellFunc {
    fn default_for_size(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            max_iter: 1024,
            center: Complex64::new(0.0, 0.0),
            top_left: Complex64::new(-1.0, 1.0),
            pixel_size: Complex64::new(2.0 / (width as f64), -2.0 / (height as f64)),
        }
    }

    fn compute_cell(&self, pos: (u32, u32)) -> super::Cell {
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

    fn with_offset(&self, _offset: (i32, i32)) -> Self {
        todo!()
    }

    fn with_zoom(&self, _zoom: f32) -> Self {
        todo!()
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
