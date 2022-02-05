mod gui;

use crate::gui::Framework;
use anyhow::{Context, Result};
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

#[derive(structopt::StructOpt)]
struct Args {
    // TODO
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("hello world")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut frame_number: usize = 0;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            // TODO: render something
            draw(frame_number, pixels.get_frame());
            frame_number += 1;
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // if input.key_pressed(VirtualKeyCode::P) {
            //     paused = !paused;
            // }
            // if input.key_pressed(VirtualKeyCode::Space) {
            //     // Space is frame-step, so ensure we're paused
            //     paused = true;
            // }
            if input.key_pressed(VirtualKeyCode::Q) {
                *control_flow = ControlFlow::Exit;
                return;
                // life.randomize();
            }
            // Handle mouse. This is a bit involved since support some simple
            // line drawing (mostly because it makes nice looking patterns).
            let (mouse_cell, mouse_prev_cell) = input
                .mouse()
                .map(|(mx, my)| {
                    let (dx, dy) = input.mouse_diff();
                    let prev_x = mx - dx;
                    let prev_y = my - dy;

                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    let (px_i, py_i) = pixels
                        .window_pos_to_pixel((prev_x, prev_y))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    (
                        (mx_i as isize, my_i as isize),
                        (px_i as isize, py_i as isize),
                    )
                })
                .unwrap_or_default();

            // if input.mouse_pressed(0) {
            //     debug!("Mouse click at {:?}", mouse_cell);
            //     draw_state = Some(life.toggle(mouse_cell.0, mouse_cell.1));
            // } else if let Some(draw_alive) = draw_state {
            //     let release = input.mouse_released(0);
            //     let held = input.mouse_held(0);
            //     debug!("Draw at {:?} => {:?}", mouse_prev_cell, mouse_cell);
            //     debug!("Mouse held {:?}, release {:?}", held, release);
            //     // If they either released (finishing the drawing) or are still
            //     // in the middle of drawing, keep going.
            //     if release || held {
            //         debug!("Draw line of {:?}", draw_alive);
            //         life.set_line(
            //             mouse_prev_cell.0,
            //             mouse_prev_cell.1,
            //             mouse_cell.0,
            //             mouse_cell.1,
            //             draw_alive,
            //         );
            //     }
            //     // If they let go or are otherwise not clicking anymore, stop drawing.
            //     if release || !held {
            //         debug!("Draw end");
            //         draw_state = None;
            //     }
            // }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            // if !paused || input.key_pressed(VirtualKeyCode::Space) {
            //     life.update();
            // }
            window.request_redraw();
        }
    });

    Ok(())
}

fn draw(frame_number: usize, screen: &mut [u8]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let idx = ((x + y * WIDTH) * 4) as usize;
            let color = [
                ((frame_number as u32) + x + y) as u8, //
                ((frame_number as u32) + x * y) as u8,
                ((frame_number as u32) + x / (y + 1)) as u8,
                0xff,
            ];
            let pix = &mut screen[idx..idx + 4];
            pix.copy_from_slice(&color);
        }
    }

    // for (c, pix) in cells.iter().zip(screen.chunks_exact_mut(4)) {
    //     let color = if c.alive {
    //         [0, 0xff, 0xff, 0xff]
    //     } else {
    //         [0, 0, c.heat, 0xff]
    //     };
    //     pix.copy_from_slice(&color);
    // }
}
