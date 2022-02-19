use std::{
    cmp::max,
    mem::{swap, take},
    ops::{Deref, Index, IndexMut},
    slice::ChunksExactMut,
};

use abi_stable::rtuple;
use abi_stable::std_types::{RVec, Tuple2, Tuple3};
use core_extensions::collections::IntoArray;
use image::{ImageBuffer, Pixel, Rgb, Rgba};
use itertools::Itertools;
use color_func::RColor;
use fractal_func::RCell;

use ultraviolet::{DVec2, IVec2, UVec2};

// type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Cell {
    state: CellState,
    data: Option<RVec<u8>>,
    // TODO: refactor this to [u8; 3]
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

impl Cell {
    pub fn from_rcell_rcolor(rcell: RCell, rcolor: RColor) -> Self {
        Self {
            state: CellState::FreshRgb,
            data: Some(rcell.data),
            rgb: rtuple!(rcolor.rgb[0], rcolor.rgb[1], rcolor.rgb[2]),
        }
    }

    pub fn stale_from(other: &Self) -> Self {
        Cell {
            state: CellState::Stale,
            data: None,
            rgb: other.rgb,
        }
    }
    pub fn stale_from_rgb(rgb: &Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;
        Cell {
            state: CellState::Stale,
            data: None,
            rgb: Tuple3(r, g, b),
        }
    }

    pub fn get_rgba(&self) -> Rgba<u8> {
        match self.state {
            CellState::Uninitialized => Rgba([0, 0, 0, 0]),
            _ => {
                let (r, g, b) = self.rgb.into_tuple();
                Rgba([r, g, b, 0xff])
            }
        }
    }
    pub fn get_rgb(&self) -> Rgb<u8> {
        match self.state {
            CellState::Uninitialized => Rgb([0, 0, 0]),
            _ => Rgb(self.rgb.into_tuple().into_array()),
        }
    }
}

#[derive(Debug)]
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

    pub fn mark_all_positions_stale(&mut self) {
        set_stale(&mut self.front);
    }
    pub fn get_stale_positions(&self) -> Vec<[u32; 2]> {
        self.front
            .iter_points_i32()
            .filter(|p| self.front[p].state != CellState::FreshRgb)
            .map(|IVec2 { x, y }| [x as u32, y as u32])
            .collect_vec()
    }

    pub fn put_value(&mut self, rcell: RCell, rcolor: RColor) -> bool {
        assert_eq!(rcell.pos, rcolor.pos);
        let pos: UVec2 = rcell.pos.into();
        if self.front.contains_uvec2(pos) {
            self.front[pos] = Cell::from_rcell_rcolor(rcell, rcolor);
            true
        } else {
            false
        }
    }

    pub fn draw_with_offset(&self, offset: (i32, i32), screen: &mut [u8], screen_size: (u32, u32)) {
        fill_checkerboard(screen, screen_size);

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

            let x = screen_pos.x as u32;
            let y = screen_pos.y as u32;
            let cell = &self.front[grid_pos];
            match cell.state {
                CellState::Uninitialized => {}
                CellState::Stale | CellState::FreshData => screen_buf
                    .get_pixel_mut(x, y)
                    .apply2(&cell.get_rgba(), |a, b| a / 2 + b / 2),
                CellState::FreshRgb => screen_buf.put_pixel(x, y, cell.get_rgba()),
            };
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
        // self.back.reset();
        // let center_width = (self.width() as f64 / zoom_factor) as u32;
        // let center_height = (self.height() as f64 / zoom_factor) as u32;
        // let x_offset = (self.width() - center_width) / 2;
        // let y_offset = (self.width() - center_width) / 2;
        // let mut center_image = Image::<Rgb<u8>>::new(center_width, center_height);
        // for (x, y, p) in center_image.enumerate_pixels_mut() {
        //     let front_pos: UVec2 = (x + x_offset, y + y_offset).into();
        //     if self.front.contains_uvec2(front_pos) {
        //         *p = self.front[front_pos].get_rgb();
        //     }
        // }
        // let zoomed_image = imageops::resize(
        //     &center_image,
        //     self.width(),
        //     self.height(),
        //     FilterType::Nearest,
        // );
        // for (x, y, p) in zoomed_image.enumerate_pixels() {
        //     self.back[(x, y)] = Cell::stale_from_rgb(p);
        // }
        // swap(&mut self.front, &mut self.back);
        // self.back.reset();
        // set_stale(&mut self.front);

        let middle = self.front.middle_f64();
        self.back.reset();
        for backpos in self.back.iter_points_i32() {
            let dpos = DVec2::from(backpos);
            let frontpos = (middle + (dpos - middle) / zoom_factor).try_into().unwrap();
            if self.front.contains_ivec2(frontpos) {
                self.back[backpos] = Cell::stale_from(&self.front[frontpos]);
            }
        }
        // let middle = self.front.middle_f64();
        // self.back.reset();
        // for pos in self.front.iter_points_i32() {
        //     let dpos = DVec2::from(pos);
        //     let newpos = (middle + (dpos - middle) * zoom_factor).try_into().unwrap();
        //     if self.back.contains_ivec2(newpos) {
        //         self.back[newpos] = take(&mut self.front[pos]);
        //     }
        // }
        swap(&mut self.front, &mut self.back);
        self.back.reset();
        set_stale(&mut self.front);
    }

    pub fn apply_offset(&mut self, offset: (i32, i32)) {
        apply_offset(offset, &mut self.front, &mut self.back);
        swap(&mut self.front, &mut self.back);
        self.back.reset();
        set_stale(&mut self.front);
    }
}

fn fill_checkerboard(screen: &mut [u8], (width, height): (u32, u32)) {
    let grid_size = 10;
    let c1 = &[0x99, 0x99, 0x99, 0xff];
    let c2 = &[0x66, 0x66, 0x66, 0xff];
    for (pix, (y, x)) in screen
        .chunks_exact_mut(4)
        .zip((0..height).cartesian_product(0..width))
    {
        pix.copy_from_slice(if (x / grid_size) % 2 == (y / grid_size) % 2 {
            c1
        } else {
            c2
        })
    }
}

fn set_stale(grid: &mut RowMajorGrid<Cell>) {
    for cell in grid.data.iter_mut() {
        if cell.state != CellState::Uninitialized {
            cell.state = CellState::Stale;
            cell.data = None;
        }
    }
}

// TODO: resize equivalent of this?
fn apply_offset<T: Default>(
    (dx, dy): (i32, i32),
    src: &mut RowMajorGrid<T>,
    dest: &mut RowMajorGrid<T>,
) {
    assert_eq!(src.width(), dest.width());
    assert_eq!(src.height(), dest.height());
    assert_eq!(src.data.len(), dest.data.len());
    dest.reset();

    let src_min_x = max(dx, 0) as usize;
    let dest_min_x = max(-dx, 0) as usize;
    let src_min_y = max(dy, 0) as usize;
    let dest_min_y = max(-dy, 0) as usize;
    let inner_width = src.width() as usize - dx.abs() as usize;
    let inner_height = src.height() as usize - dy.abs() as usize;

    let src_row_iter = src
        .row_chunks_exact_mut()
        .skip(src_min_y)
        .take(inner_height);
    let dest_row_iter = dest
        .row_chunks_exact_mut()
        .skip(dest_min_y)
        .take(inner_height);
    for (src_row, dest_row) in src_row_iter.zip(dest_row_iter) {
        // src_row[..src_min_x].fill_with(Default::default);
        dest_row[..dest_min_x].fill_with(Default::default);
        for (s, d) in src_row[src_min_x..src_min_x + inner_width]
            .iter_mut()
            .zip(dest_row[dest_min_x..dest_min_x + inner_width].iter_mut())
        {
            *d = take(s);
        }
        // src_row[src_min_x + inner_width..].fill_with(Default::default);
        dest_row[dest_min_x + inner_width..].fill_with(Default::default);
    }
}

///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
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

    pub fn row_chunks_exact_mut(&mut self) -> ChunksExactMut<'_, T> {
        self.data.chunks_exact_mut(self._width as usize)
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
