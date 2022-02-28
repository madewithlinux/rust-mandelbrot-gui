use impl_util::{compute_cells_rmp, config_helper::OptionSetter};
use num::complex::Complex64;
use num::Zero;
use serde::{Deserialize, Serialize};

use fractal_func::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct {{to_class_case package_name}}Data {
    // TODO
}

#[derive(Debug, Clone)]
pub struct {{to_class_case package_name}}CellFunc {
    width: u32,
    height: u32,
    // TODO
}

impl {{to_class_case package_name}}CellFunc {
    fn default_for_size(width: u32, height: u32) -> Self {
        todo!();
    }

    #[inline]
    fn compute_cell_impl(&self, pos: [u32; 2]) -> {{to_class_case package_name}}Data {
        todo!();
    }
}

impl From<{{to_class_case package_name}}CellFunc> for RFractalFuncBox {
    fn from(inner: {{to_class_case package_name}}CellFunc) -> Self {
        RFractalFuncBox::from_value(inner, TD_Opaque)
    }
}

impl RFractalFunc for {{to_class_case package_name}}CellFunc {
    fn get_size(&self) -> Tuple2<u32, u32> {
        Tuple2(self.width, self.height)
    }

    fn compute_cells(&self, positions: RSlice<[u32; 2]>) -> RChunk {
        compute_cells_rmp(positions, |pos| self.compute_cell_impl(pos))
    }

    fn with_size(&self, width: u32, height: u32) -> RFractalFuncBox {
        todo!();
    }

    fn with_offset(&self, dx: i32, dy: i32) -> RFractalFuncBox {
        todo!()
    }
    
    fn add_zoom(&self, zoom_factor: f64) -> RFractalFuncBox {
        todo!()
    }
    
    fn with_option(&self, name: RStr, value: RStr) -> RResult<RFractalFuncBox, RString> {
        todo!()
    }
    
    fn get_options(&self) -> ROptionsMap {
        todo!()
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
        {{to_class_case package_name}}CellFunc::default_for_size(width, height),
        TD_Opaque,
    )
}
