use color_func::{prelude::*, RChunk};
use impl_util::compute_colors_rmp;
use mandelbrot_f64::MandelbrotData;

#[derive(Debug, Clone, Copy)]
pub struct BasicLumaColorFunc {}

impl BasicLumaColorFunc {
    fn compute_color_impl(&self, data: &MandelbrotData) -> [u8; 3] {
        let MandelbrotData { iter, outside } = *data;
        let luma = if outside {
            ((iter as f32).sqrt().sin().powi(2) * 255.0) as u8
        } else {
            0
        };
        [luma, luma, luma]
        // [luma, 0, 0]
    }
}

impl RColorFunc for BasicLumaColorFunc {
    fn compute_colors(&self, chunk: &RChunk) -> RVec<RColor> {
        compute_colors_rmp(chunk, |d| self.compute_color_impl(d))
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
