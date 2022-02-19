pub mod mandelbrot_f64_impl;

#[cfg(feature = "cdylib")]
use shared::{raw::FractalCellFunc, FractalLib, FractalLib_Ref, RFractalCellFuncBox};

#[cfg(feature = "cdylib")]
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait};

#[cfg(feature = "cdylib")]
#[export_root_module]
pub fn get_library() -> FractalLib_Ref {
    FractalLib {
        default_cell_func_for_size,
    }
    .leak_into_prefix()
}

#[cfg(feature = "cdylib")]
#[no_mangle]
pub extern "C" fn default_cell_func_for_size(width: u32, height: u32) -> RFractalCellFuncBox {
    mandelbrot_f64_impl::MandelbrotCellFunc::default_for_size(width, height).into_box()
}
