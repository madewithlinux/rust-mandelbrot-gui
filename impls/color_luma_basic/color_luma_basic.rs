use color_func::prelude::*;
use mandelbrot_f64::MandelbrotData;

#[derive(Debug, Clone, Copy)]
pub struct BasicLumaColorFunc {}

impl BasicLumaColorFunc {}

impl RColorFunc for BasicLumaColorFunc {
    fn compute_color(&self, cell: &RCell) -> RColor {
        let &RCell { pos, data } = &cell;
        let MandelbrotData { iter, outside } =
            serde_json::from_slice(&data).expect("deserialization failed");
        let luma = if outside {
            ((iter as f32).sqrt().sin().powi(2) * 255.0) as u8
        } else {
            0
        };
        RColor {
            pos: *pos,
            rgb: [luma, luma, luma],
        }
    }
    fn compute_colors(&self, cells: RSlice<RCell>) -> RVec<RColor> {
        cells.iter().map(|cell| self.compute_color(cell)).collect()
    }
}

#[cfg(feature = "cdylib")]
#[export_root_module]
pub fn get_color_lib_ref() -> ColorLib_Ref {
    ColorLib { default_color_func }.leak_into_prefix()
}

#[cfg(feature = "cdylib")]
#[no_mangle]
pub extern "C" fn default_color_func() -> RColorFuncBox {
    RColorFuncBox::from_value(BasicLumaColorFunc {}, TD_Opaque)
}
