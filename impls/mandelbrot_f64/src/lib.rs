use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait};

use shared::{raw::FractalCellFunc, FractalLib, FractalLib_Ref, RFractalCellFuncBox};

mod mandelbrot_f64;

#[export_root_module]
pub fn get_library() -> FractalLib_Ref {
    FractalLib {
        default_cell_func_for_size,
    }
    .leak_into_prefix()
}

pub extern "C" fn default_cell_func_for_size(width: u32, height: u32) -> RFractalCellFuncBox {
    mandelbrot_f64::MandelbrotCellFunc::default_for_size(width, height).into_box()
}
