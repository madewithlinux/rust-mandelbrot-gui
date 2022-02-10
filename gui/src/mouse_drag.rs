use pixels::Pixels;
use winit_input_helper::WinitInputHelper;

// mouse_drag.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseDragState {
    Init,
    Dragging {
        drag_start: (i32, i32),
        offset: (i32, i32),
    },
    Released {
        offset: (i32, i32),
    },
}

use MouseDragState::*;

impl MouseDragState {
    pub fn new() -> Self {
        Self::Init
    }

    pub fn update(self, input: &WinitInputHelper, pixels: &Pixels) -> Self {
        let mouse_pos = input.mouse().map(|pos| {
            // allow mouse drag to extend outside window
            match pixels.window_pos_to_pixel(pos) {
                Ok((x, y)) => (x as i32, y as i32),
                Err((x, y)) => (x as i32, y as i32),
            }
        });
        let pressed = input.mouse_pressed(0);
        let held = input.mouse_held(0);
        let released = input.mouse_released(0);

        match (self, mouse_pos) {
            (_, Some((x, y))) if pressed => Dragging {
                drag_start: (x, y),
                offset: (0, 0),
            },
            (Dragging { drag_start, .. }, Some((x, y))) if held => Dragging {
                drag_start,
                offset: (x - drag_start.0, y - drag_start.1),
            },
            (Dragging { drag_start, .. }, Some((x, y))) if released => Released {
                offset: (x - drag_start.0, y - drag_start.1),
            },
            // if mouse leaves window, that counts as released
            (Dragging { offset, .. }, None) => Released { offset },

            // released state only stays around for one iteration
            (Released { .. }, _) => Init,
            // init can stay there if nothing interesting is happening
            (Init, _) => Init,

            // unknown what's happening here
            _ => {
                dbg!(self, mouse_pos, pressed, held, released);
                self
            }
        }
    }
}
