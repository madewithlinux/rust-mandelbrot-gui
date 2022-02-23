use std::iter::once;

use abi_stable::{
    std_types::{RSlice, RVec, Tuple2},
    StableAbi,
};
use color_func::RColor;
use fractal_func::{RCell, RChunk};
use rmp_serde::{self, Serializer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// #[repr(C)]
// #[derive(Debug, Clone, StableAbi)]
// pub struct RChunk {
//     // tuple of (pos, data_start_index)
//     // the last one goes to the end of the data array
//     pos_indexes: RVec<Tuple2<[u32; 2], usize>>,
//     data: RVec<u8>,
// }

#[inline]
pub fn compute_cells_rmp<F, C>(positions: RSlice<[u32; 2]>, func: F) -> RChunk
where
    F: Fn([u32; 2]) -> C,
    C: Serialize,
{
    let mut pos_indexes = RVec::with_capacity(positions.len());
    let mut data = RVec::with_capacity(positions.len());
    let mut serializer = Serializer::new(&mut data).with_struct_map();

    for &pos in positions {
        let cell = func(pos);
        let data_start_index = serializer.get_ref().len();
        cell.serialize(&mut serializer).unwrap();
        pos_indexes.push(Tuple2(pos, data_start_index));
    }

    RChunk { data, pos_indexes }
}

#[inline]
pub fn compute_colors_rmp<'de, F, C>(chunk: &'de RChunk, func: F) -> RVec<RColor>
where
    F: Fn(&C) -> [u8; 3],
    C: Deserialize<'de>,
{
    let mut colors = RVec::with_capacity(chunk.len());
    for (pos, data) in chunk.iter() {
        let cell = rmp_serde::from_slice(data).unwrap();
        let rgb = func(&cell);
        colors.push(RColor { pos, rgb });
    }
    colors
}
