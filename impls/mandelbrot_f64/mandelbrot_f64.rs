use impl_util::{compute_cells_rmp, config_helper::OptionSetter};
use num::complex::Complex64;
use num::Zero;
use serde::{Deserialize, Serialize};

use fractal_func::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MandelbrotData {
    pub outside: bool,
    pub iter: usize,
}

#[derive(Debug, Clone)]
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

    fn pos_to_complex(&self, pos: [u32; 2]) -> Complex64 {
        self.top_left + self.pixel_re().scale(pos[0] as f64) + self.pixel_im().scale(pos[1] as f64)
    }

    #[cfg(feature = "cdylib")]
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

    #[inline]
    fn compute_cell_impl(&self, pos: [u32; 2]) -> MandelbrotData {
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
        MandelbrotData { outside, iter }
    }
}

impl From<MandelbrotCellFunc> for RFractalFuncBox {
    fn from(inner: MandelbrotCellFunc) -> Self {
        RFractalFuncBox::from_value(inner, TD_Opaque)
    }
}

impl RFractalFunc for MandelbrotCellFunc {
    fn get_size(&self) -> Tuple2<u32, u32> {
        Tuple2(self.width, self.height)
    }

    fn compute_cells(&self, positions: RSlice<[u32; 2]>) -> RChunk {
        compute_cells_rmp(positions, |pos| self.compute_cell_impl(pos))
    }

    fn with_size(&self, width: u32, height: u32) -> RFractalFuncBox {
        // middle doesn't change, just top-left
        let middle = self.pos_to_complex([self.width / 2, self.height / 2]);
        let top_left = middle
            - self.pixel_re() * ((width / 2) as f64)
            - self.pixel_im() * ((height / 2) as f64);
        Self {
            width,
            height,
            top_left,
            ..self.clone()
        }
        .into()
    }

    fn with_offset(&self, dx: i32, dy: i32) -> RFractalFuncBox {
        let complex_offset = self.pixel_re().scale(dx as f64) + self.pixel_im().scale(dy as f64);
        Self {
            center: self.center + complex_offset,
            top_left: self.top_left + complex_offset,
            ..self.clone()
        }
        .into()
    }

    fn add_zoom(&self, zoom_factor: f64) -> RFractalFuncBox {
        let middle = self.pos_to_complex([self.width / 2, self.height / 2]);
        let pixel_size = self.pixel_size.scale(1.0 / zoom_factor);
        let top_left = middle
            + Complex64::new(
                -pixel_size.re * ((self.width / 2) as f64),
                -pixel_size.im * ((self.height / 2) as f64),
            );
        Self {
            top_left,
            pixel_size,
            ..self.clone()
        }
        .into()
    }

    fn with_option(&self, name: RStr, value: RStr) -> RResult<RFractalFuncBox, RString> {
        OptionSetter::new(self, name, value)
            .option("center_re", |s, v| {
                let diag = s.top_left - s.center;
                s.center.re = v;
                s.top_left = s.center + diag;
            })
            .option("center_im", |s, v| {
                let diag = s.top_left - s.center;
                s.center.im = v;
                s.top_left = s.center + diag;
            })
            .option("max_iter", |s, v| s.max_iter = v)
            .finish()
    }

    fn get_options(&self) -> ROptionsMap {
        ROptionsMap::from_iter(
            [
                // ("width", format!("{}", self.width)),
                // ("height", format!("{}", self.height)),
                ("max_iter", format!("{}", self.max_iter)),
                // ("center", format!("{}", self.center)),
                ("center_re", format!("{}", self.center.re)),
                ("center_im", format!("{}", self.center.im)),
                // ("top_left", format!("{}", self.top_left)),
                // ("pixel_size", format!("{}", self.pixel_size)),
                ("pixel_size", format!("{}", self.pixel_size.re)),
            ]
            .map(|(k, v)| (RString::from(k), RString::from(v))),
        )
    }
}

#[cfg(feature = "cdylib")]
#[export_root_module]
pub fn get_fractal_lib_ref() -> FractalLib_Ref {
    FractalLib {
        default_fractal_func_for_size,
    }
    .leak_into_prefix()
}

#[cfg(feature = "cdylib")]
#[no_mangle]
pub extern "C" fn default_fractal_func_for_size(width: u32, height: u32) -> RFractalFuncBox {
    RFractalFuncBox::from_value(
        MandelbrotCellFunc::default_for_size(width, height),
        TD_Opaque,
    )
}
