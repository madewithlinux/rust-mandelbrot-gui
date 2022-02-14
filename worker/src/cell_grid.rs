use std::ops::{Deref, Index, IndexMut};

use abi_stable::rtuple;
use array2d::Array2D;
use image::{ImageBuffer, Rgba};
use itertools::Itertools;
use shared::{RCell, RVec, Tuple2, Tuple3};
use ultraviolet::{DVec2, IVec2, UVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Uninitialized,
    Stale,
    FreshData,
    FreshRgb,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Uninitialized
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    state: CellState,
    data: RVec<u8>,
    rgb: Tuple3<u8, u8, u8>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: Default::default(),
            data: Default::default(),
            rgb: rtuple!(0, 0, 0),
        }
    }
}

impl From<&RCell> for Cell {
    fn from(rcell: &RCell) -> Self {
        Cell {
            state: CellState::FreshRgb,
            data: rcell.data.clone(),
            rgb: rcell.rgb,
        }
    }
}

impl Cell {
    pub fn get_rgba(&self) -> Rgba<u8> {
        match self.state {
            CellState::Uninitialized => Rgba([0, 0, 0, 0]),
            _ => {
                let (r, g, b) = self.rgb.into_tuple();
                Rgba([r, g, b, 0xff])
            }
        }
    }
}

pub struct CellGridBuffer(Array2D<Cell>);

impl Deref for CellGridBuffer {
    type Target = Array2D<Cell>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn iter_points_i32(width: i32, height: i32) -> impl Iterator<Item = IVec2> {
    (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| IVec2 { x, y })
}
fn iter_points_u32(width: u32, height: u32) -> impl Iterator<Item = UVec2> {
    (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| UVec2 { x, y })
}

impl CellGridBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self(Self::build_inner(width, height))
    }

    pub fn empty_clone(&self) -> Self {
        Self::new(self.width() as u32, self.height() as u32)
    }

    pub fn width(&self) -> usize {
        self.row_len()
    }
    pub fn height(&self) -> usize {
        self.column_len()
    }

    pub fn put_rcell(&mut self, rcell: &RCell) -> bool {
        let pos = UVec2::from(rcell.pos.into_tuple());
        if self.contains_uvec2(pos) {
            self[pos] = rcell.into();
            true
        } else {
            false
        }
    }

    pub fn draw_with_offset(&self, offset: (i32, i32), screen: &mut [u8], screen_size: (u32, u32)) {
        // this is much faster than GenericImage::copy_from()
        for pixel in screen.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x50, 0x00, 0x00, 0xff]);
        }

        // https://docs.rs/image/0.24.0/image/flat/enum.NormalForm.html
        // > The ImageBuffer uses row major form with packed samples.
        let mut screen_buf =
            ImageBuffer::<Rgba<u8>, _>::from_raw(screen_size.0, screen_size.1, screen)
                .expect("pixel buffer layout bad");

        let offset: IVec2 = offset.into();
        for grid_pos in self.iter_points_i32() {
            let screen_pos = grid_pos + offset;
            if screen_pos.x < 0
                || screen_pos.y < 0
                || screen_pos.x >= screen_size.0 as i32
                || screen_pos.y >= screen_size.1 as i32
            {
                continue;
            }
            screen_buf.put_pixel(
                screen_pos.x as u32,
                screen_pos.y as u32,
                self[grid_pos].get_rgba(),
            );
        }
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        let old_middle = IVec2::new(self.width() as i32, self.height() as i32) / 2;
        let (new_width, new_height) = size;
        let mut new_buf = Self::new(new_width, new_height);
        let new_middle = new_buf.middle_i32();

        for pos in self.iter_points_i32() {
            let newpos = (new_middle + (pos - old_middle)).try_into().unwrap();
            if new_buf.contains_ivec2(newpos) {
                new_buf[newpos] = self[pos].clone();
            }
        }
        *self = new_buf;
    }

    pub fn apply_zoom(&mut self, zoom_factor: f64) {
        let middle = self.middle_f64();
        let mut new_buf = self.empty_clone();

        for pos in self.iter_points_i32() {
            let dpos = DVec2::from(pos);
            let newpos = (middle + (dpos - middle) * zoom_factor).try_into().unwrap();
            if new_buf.contains_ivec2(newpos) {
                new_buf[newpos] = self[pos].clone();
            }
        }
        *self = new_buf;
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        let offset = IVec2::from(offset);

        let mut new_buf = self.empty_clone();

        for pos in self.iter_points_i32() {
            let newpos = pos - offset;
            if new_buf.contains_ivec2(newpos) {
                new_buf[newpos] = self[pos].clone();
            }
        }
        *self = new_buf;
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    fn iter_points_i32(&self) -> impl Iterator<Item = IVec2> {
        iter_points_i32(self.width() as i32, self.height() as i32)
    }
    fn iter_points_u32(&self) -> impl Iterator<Item = UVec2> {
        iter_points_u32(self.width() as u32, self.height() as u32)
    }

    fn size_u32(&self) -> UVec2 {
        (self.width() as u32, self.height() as u32).into()
    }
    fn size_i32(&self) -> IVec2 {
        (self.width() as i32, self.height() as i32).into()
    }

    fn contains_i32(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < (self.width() as i32) && y < (self.height() as i32)
    }
    fn contains_ivec2(&self, IVec2 { x, y }: IVec2) -> bool {
        self.contains_i32(x, y)
    }
    fn contains_u32(&self, x: u32, y: u32) -> bool {
        x < (self.width() as u32) && y < (self.height() as u32)
    }
    fn contains_uvec2(&self, UVec2 { x, y }: UVec2) -> bool {
        self.contains_u32(x, y)
    }

    fn middle_i32(&self) -> IVec2 {
        self.size_i32() / 2
    }
    fn middle_f64(&self) -> DVec2 {
        DVec2::from(self.size_i32()) / 2.0
    }

    fn build_inner(width: u32, height: u32) -> Array2D<Cell> {
        Array2D::filled_with(Default::default(), height as usize, width as usize)
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
}

macro_rules! impl_index {
    ($($indexor:ty, $x:tt, $y:tt),+) => {
        $(
        impl Index<$indexor> for CellGridBuffer {
            type Output = Cell;
            #[inline]
            fn index(&self, index: $indexor) -> &Self::Output {
                self.get(index.$y as usize, index.$x as usize).unwrap()
            }
        }
        impl IndexMut<$indexor> for CellGridBuffer {
            #[inline]
            fn index_mut(&mut self, index: $indexor) -> &mut Self::Output {
                self.0.get_mut(index.$y as usize, index.$x as usize).unwrap()
            }
        }
        impl Index<&$indexor> for CellGridBuffer {
            type Output = Cell;
            #[inline]
            fn index(&self, index: &$indexor) -> &Self::Output {
                self.get(index.$y as usize, index.$x as usize).unwrap()
            }
        }
        impl IndexMut<&$indexor> for CellGridBuffer {
            #[inline]
            fn index_mut(&mut self, index: &$indexor) -> &mut Self::Output {
                self.0.get_mut(index.$y as usize, index.$x as usize).unwrap()
            }
        }
        )+
    };
}

impl_index!(IVec2, x, y);
impl_index!(UVec2, x, y);
impl_index!((u32, u32), 0, 1);
impl_index!((i32, i32), 0, 1);
impl_index!((usize, usize), 0, 1);
impl_index!(Tuple2<u32, u32>, 0, 1);
impl_index!(Tuple2<i32, i32>, 0, 1);
impl_index!(Tuple2<usize, usize>, 0, 1);
