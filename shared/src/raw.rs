use std::{collections::HashMap, fmt::Debug};

use abi_stable::erased_types::TD_Opaque;
use abi_stable::rvec;
use abi_stable::std_types::RResult;
use abi_stable::std_types::RSlice;
use abi_stable::std_types::RStr;
use abi_stable::std_types::{RHashMap, RString, RVec, Tuple2};
use itertools::Itertools;

use crate::{RCell, RFractalCellFunc, RFractalCellFuncBox};

// TODO: probalby ought to just factor this all out anyway

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub pos: (u32, u32),
    pub iter: f32,
    pub rgb: (u8, u8, u8),
    // TODO: data?
    // pub data: Vec<u8>,
}

impl Into<RCell> for Cell {
    fn into(self) -> RCell {
        RCell {
            pos: self.pos.into(),
            iter: self.iter.into(),
            rgb: self.rgb.into(),
            data: rvec![],
        }
    }
}

pub trait FractalCellFunc: Clone + Debug + 'static + Sync + Send {
    fn default_for_size(width: u32, height: u32) -> Self;
    fn get_size(&self) -> (u32, u32);

    fn compute_cell(&self, pos: (u32, u32)) -> Cell;
    fn compute_cells(&self, positions: &[(u32, u32)]) -> Vec<Cell> {
        positions
            .iter()
            .map(|&pos| self.compute_cell(pos))
            .collect_vec()
    }

    fn with_size(&self, size: (u32, u32)) -> Self;
    fn with_offset(&self, offset: (i32, i32)) -> Self;
    fn add_zoom(&self, zoom_factor: f64) -> Self;

    fn with_option(&self, name: &str, value: &str) -> Result<Self, String>;
    fn get_options(&self) -> HashMap<String, String>;

    fn into_box(self) -> RFractalCellFuncBox {
        RFractalCellFuncBox::from_value(self, TD_Opaque)
    }
}

impl<T: FractalCellFunc> RFractalCellFunc for T {
    fn clone_self(&self) -> RFractalCellFuncBox {
        self.clone().into_box()
    }

    fn get_size(&self) -> Tuple2<u32, u32> {
        FractalCellFunc::get_size(self).into()
    }

    // fn compute_cell(&self, pos: Tuple2<u32, u32>) -> RCell {
    //     FractalCellFunc::compute_cell(self, pos.into()).into()
    // }
    fn compute_cells(&self, positions: RSlice<Tuple2<u32, u32>>) -> RVec<RCell> where {
        positions
            .iter()
            .map(|&pos| FractalCellFunc::compute_cell(self, pos.into()).into())
            .collect_vec()
            .into()
    }

    fn with_size(&self, size: Tuple2<u32, u32>) -> RFractalCellFuncBox {
        FractalCellFunc::with_size(self, size.into()).into_box()
    }

    fn with_offset(&self, offset: Tuple2<i32, i32>) -> RFractalCellFuncBox {
        FractalCellFunc::with_offset(self, offset.into()).into_box()
    }

    fn add_zoom(&self, zoom_factor: f64) -> RFractalCellFuncBox {
        FractalCellFunc::add_zoom(self, zoom_factor.into()).into_box()
    }

    fn with_option(&self, name: RStr, value: RStr) -> RResult<RFractalCellFuncBox, RString> {
        FractalCellFunc::with_option(self, name.into(), value.into())
            .map(|func| func.into_box())
            .map_err(|err_str| err_str.into())
            .into()
    }

    fn get_options(&self) -> RHashMap<RString, RString> {
        FractalCellFunc::get_options(self)
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}
