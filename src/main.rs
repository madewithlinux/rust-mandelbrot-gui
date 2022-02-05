mod fractal_worker;
mod gui;

use crate::gui::Framework;
use anyhow::{Context, Result};
use fractal_worker::FractalWorker;
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

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let framework =
            Framework::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, framework)
    };

    let mut frame_number: usize = 0;
    let mut drag_start = (0, 0);
    let mut offset = (0, 0);
    let mut worker = FractalWorker::new(WIDTH, HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            if input.mouse_pressed(0) {
                if let Some(mouse_pos) = input.mouse().map(|pos| {
                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel(pos)
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    (mx_i as i32, my_i as i32)
                }) {
                    drag_start = mouse_pos;
                }
            }
            if input.mouse_held(0) {
                if let Some(mouse_pos) = input.mouse().map(|pos| {
                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel(pos)
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    (mx_i as i32, my_i as i32)
                }) {
                    offset = (mouse_pos.0 - drag_start.0, mouse_pos.1 - drag_start.1);
                }
            }
            if input.mouse_released(0) {
                drag_start = (0, 0);
                offset = (0, 0);
            }

            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // // Draw the world
                // draw(frame_number, pixels.get_frame());
                // // frame_number += 1;
                // frame_number += framework.gui.speed;
                // worker.draw_pending_pixels(pixels.get_frame());
                worker.receive_into_buf();
                worker.draw_full_buffer_with_offset(offset.0, offset.1, pixels.get_frame());

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context)?;

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

fn draw(frame_number: usize, screen: &mut [u8]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let idx = ((x + y * WIDTH) * 4) as usize;
            let color = [
                ((frame_number as u32) + x + y) as u8, //
                ((frame_number as u32) + x * y) as u8,
                // ((frame_number as u32) + x / (y + 1)) as u8,
                ((frame_number as u32) + x * x * y * y) as u8,
                0xff,
            ];
            let pix = &mut screen[idx..idx + 4];
            pix.copy_from_slice(&color);
        }
    }
}
