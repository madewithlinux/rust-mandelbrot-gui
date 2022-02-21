use std::time::Instant;

use pixels::Pixels;
use ultraviolet::{IVec2, Mat4, Similarity3, Vec3};
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Clone, Copy)]
pub struct PanZoomDebounce {
    // state
    pub width: u32,
    pub height: u32,
    pub is_dirty: bool,
    pub is_mouse_down: bool,
    pub prev_mouse_pos: Option<(f32, f32)>,
    pub transform: Similarity3,
    pub last_update: Instant,
    // config
    pub debounce_seconds: f64,
    pub min_move_size: f32,
}

impl PanZoomDebounce {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            is_dirty: false,
            is_mouse_down: false,
            prev_mouse_pos: None,
            transform: Similarity3::identity(),
            last_update: Instant::now(),
            debounce_seconds: 0.8,
            min_move_size: 4.0,
        }
    }
    pub fn handle_input(
        &mut self,
        width: u32,
        height: u32,
        input: &WinitInputHelper,
        pixels: &Pixels,
    ) {
        self.width = width;
        self.height = height;
        self.is_mouse_down = input.mouse_held(0);

        let mouse_pos = input.mouse().map(|pos| {
            // allow mouse drag to extend outside window
            match pixels.window_pos_to_pixel(pos) {
                Ok((x, y)) => (x as f32, y as f32),
                Err((x, y)) => (x as f32, y as f32),
            }
        });
        let mouse_pos = match mouse_pos {
            Some(x) => x,
            None => return,
        };

        if self.is_mouse_down {
            self.handle_input_mouse_held(mouse_pos);
        } else {
            self.prev_mouse_pos = None;
        }

        if input.scroll_diff() != 0.0 {
            self.handle_scroll_diff(mouse_pos, input.scroll_diff())
        }
    }

    fn handle_input_mouse_held(&mut self, mouse_pos: (f32, f32)) {
        let prev_mouse_pos = match self.prev_mouse_pos {
            None => {
                self.prev_mouse_pos = Some(mouse_pos);
                return;
            }
            Some(x) => x,
        };
        let dx = mouse_pos.0 - prev_mouse_pos.0;
        let dy = mouse_pos.1 - prev_mouse_pos.1;

        if (dx, dy) == (0.0, 0.0) {
            return;
        }

        let translation = Vec3::new(dx, dy, 0.0) * self.transform.scale;
        self.transform.append_translation(translation);
        self.last_update = Instant::now();
        self.is_dirty = true;
        self.prev_mouse_pos = Some(mouse_pos);
    }

    fn handle_scroll_diff(&mut self, _mouse_pos: (f32, f32), scroll_diff: f32) {
        assert!(scroll_diff != 0.0);
        let zoom_factor = if scroll_diff > 0.0 { 1.0 / 1.1 } else { 1.1 };

        // // TODO: get it to work with like zooming into where the mouse is
        // let mouse_pos: Vec2 = mouse_pos.into();
        // let mouse_pos: Vec3 = mouse_pos.into();
        // let width = self.width as f32;
        // let height = self.height as f32;
        // let zoom_middle: Vec3 = mouse_pos - Vec3::new(width / 2.0, height / 2.0, 0.0);
        // let mut s = Similarity3::identity();
        // s.append_translation(zoom_middle);
        // s.append_scaling(zoom_factor);
        // s.append_translation(-zoom_middle);
        // self.transform.append_similarity(s);

        self.transform.append_scaling(zoom_factor);
        self.last_update = Instant::now();
        self.is_dirty = true;
    }

    pub fn get_render_matrix(&self) -> Mat4 {
        let width = self.width as f32;
        let height = self.height as f32;
        Mat4::from_translation(Vec3::new(0.5, 0.5, 1.0))
            * Mat4::from_nonuniform_scale(Vec3::new(-1.0 / width, -1.0 / height, 1.0))
            * self.transform.into_homogeneous_matrix()
            * Mat4::from_nonuniform_scale(Vec3::new(-width, -height, 1.0))
            * Mat4::from_translation(Vec3::new(-0.5, -0.5, 1.0))
    }

    /// return value is (dx, dy, zoom_factor), if the input is done
    pub fn get_completed_input(&mut self) -> Option<(i32, i32, f64)> {
        if self.is_dirty
            && !self.is_mouse_down
            && self.last_update.elapsed().as_secs_f64() > self.debounce_seconds
            && ((self.transform.scale - 1.0).abs() > 0.001
                || self.transform.translation.xy().mag() > self.min_move_size)
        {
            let IVec2 { x, y } = self.transform.translation.xy().try_into().unwrap();
            let zoom_factor = 1.0 / (self.transform.scale as f64);
            if (x, y, zoom_factor) == (0, 0, 1.0) {
                return None;
            }
            self.transform = Similarity3::identity();
            self.is_dirty = false;
            Some((x, y, zoom_factor))
        } else {
            None
        }
    }
}
