use std::{
    mem::{swap, take},
    ops::{Deref, Index, IndexMut},
};

use abi_stable::rtuple;
use image::{ImageBuffer, Rgba};
use itertools::Itertools;
use shared::{RCell, RVec, Tuple2, Tuple3};
use ultraviolet::{DVec2, IVec2, UVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Uninitialized,
    _Stale,
    _FreshData,
    FreshRgb,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Uninitialized
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl From<RCell> for Cell {
    fn from(rcell: RCell) -> Self {
        Cell {
            state: CellState::FreshRgb,
            data: rcell.data,
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

pub struct CellGridBuffer {
    front: RowMajorGrid<Cell>,
    back: RowMajorGrid<Cell>,
}

impl CellGridBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            front: RowMajorGrid::new(width, height),
            back: RowMajorGrid::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.front.width()
    }
    pub fn height(&self) -> u32 {
        self.front.height()
    }

    pub fn put_rcell(&mut self, rcell: RCell) -> bool {
        let pos = UVec2::from(rcell.pos.into_tuple());
        if self.front.contains_uvec2(pos) {
            self.front[pos] = rcell.into();
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
        for grid_pos in self.front.iter_points_i32() {
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
                self.front[grid_pos].get_rgba(),
            );
        }
    }

    pub fn apply_resize(&mut self, size: (u32, u32)) {
        let old_middle = IVec2::new(self.width() as i32, self.height() as i32) / 2;
        let (new_width, new_height) = size;
        // let mut new_buf = Self::new(new_width, new_height);
        self.back.reset_resize(new_width, new_height);
        let new_middle = self.back.middle_i32();

        for pos in self.front.iter_points_i32() {
            let newpos = (new_middle + (pos - old_middle)).try_into().unwrap();
            if self.back.contains_ivec2(newpos) {
                self.back[newpos] = take(&mut self.front[pos]);
            }
        }
        swap(&mut self.front, &mut self.back);
        self.back.reset_resize(new_width, new_height);
    }

    pub fn apply_zoom(&mut self, zoom_factor: f64) {
        let middle = self.front.middle_f64();

        self.back.reset();
        for pos in self.front.iter_points_i32() {
            let dpos = DVec2::from(pos);
            let newpos = (middle + (dpos - middle) * zoom_factor).try_into().unwrap();
            if self.back.contains_ivec2(newpos) {
                self.back[newpos] = take(&mut self.front[pos]);
            }
        }
        swap(&mut self.front, &mut self.back);
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        let offset = IVec2::from(offset);

        self.back.reset();
        for pos in self.front.iter_points_i32() {
            let newpos = pos - offset;
            if self.back.contains_ivec2(newpos) {
                self.back[newpos] = take(&mut self.front[pos]);
            }
        }
        swap(&mut self.front, &mut self.back);
    }
}

///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

struct RowMajorGrid<T> {
    _width: u32,
    _height: u32,
    data: Vec<T>,
}

impl<T: Default> RowMajorGrid<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            _width: width,
            _height: height,
            data: (0..(width * height))
                .map(|_| Default::default())
                .collect_vec(),
        }
    }

    pub fn reset(&mut self) {
        self.data.fill_with(Default::default);
    }

    pub fn reset_resize(&mut self, width: u32, height: u32) {
        self._width = width;
        self._height = height;
        self.data.fill_with(Default::default);
        self.data
            .resize_with((width * height) as usize, Default::default);
    }
}

impl<T> Deref for RowMajorGrid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<T> RowMajorGrid<T> {
    fn iter_points_i32(&self) -> impl Iterator<Item = IVec2> + 'static {
        let width = self._width as i32;
        let height = self._height as i32;
        (0..height)
            .cartesian_product(0..width)
            .map(|(y, x)| IVec2 { x, y })
    }

    #[inline]
    fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self._width && y < self._height {
            Some((y * self._width + x) as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self._width
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self._height
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

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.get_index(x, y).map(|index| &self.data[index])
    }
    #[inline]
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.get_index(x, y).map(move |index| &mut self.data[index])
    }
}

macro_rules! RowMajorGrid_index {
    ($($indexor:ty, $x:tt, $y:tt),+) => {
        $(
        impl<T> Index<$indexor> for RowMajorGrid<T> {
            type Output = T;
            #[inline]
            fn index(&self, index: $indexor) -> &Self::Output {
                self.get(index.$x as u32, index.$y as u32).unwrap()
            }
        }
        impl<T> IndexMut<$indexor> for RowMajorGrid<T> {
            #[inline]
            fn index_mut(&mut self, index: $indexor) -> &mut Self::Output {
                self.get_mut(index.$x as u32, index.$y as u32).unwrap()
            }
        }
        impl<T> Index<&$indexor> for RowMajorGrid<T> {
            type Output = T;
            #[inline]
            fn index(&self, index: &$indexor) -> &Self::Output {
                self.get(index.$x as u32, index.$y as u32).unwrap()
            }
        }
        impl<T> IndexMut<&$indexor> for RowMajorGrid<T> {
            #[inline]
            fn index_mut(&mut self, index: &$indexor) -> &mut Self::Output {
                self.get_mut(index.$x as u32, index.$y as u32).unwrap()
            }
        }
        )+
    };
}

RowMajorGrid_index!(IVec2, x, y);
RowMajorGrid_index!(UVec2, x, y);
RowMajorGrid_index!((u32, u32), 0, 1);
RowMajorGrid_index!((i32, i32), 0, 1);
RowMajorGrid_index!((usize, usize), 0, 1);
RowMajorGrid_index!(Tuple2<u32, u32>, 0, 1);
RowMajorGrid_index!(Tuple2<i32, i32>, 0, 1);
RowMajorGrid_index!(Tuple2<usize, usize>, 0, 1);
