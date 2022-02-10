mod fractal_worker;
mod gui;
mod mouse_drag;

use crate::gui::Framework;
use anyhow::Result;
use fractal_worker::FractalWorker;
use log::error;
use mouse_drag::MouseDragState;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    // TODO
}

#[paw::main]
fn main(_args: Args) -> Result<()> {
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

    let mut mouse_drag = MouseDragState::new();
    let mut worker = FractalWorker::new(WIDTH, HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        if let Event::WindowEvent { event, .. } = &event {
            framework.handle_event(event);
        }

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

            if !framework.wants_pointer_input() {
                mouse_drag = mouse_drag.update(&input, &pixels);
                match mouse_drag {
                    MouseDragState::Released { offset } => {
                        worker.apply_offset(offset);
                    }
                    _ => {}
                };
            }

            window.request_redraw();
        }

        match event {
            // Draw the current frame
            Event::RedrawRequested(_) => {
                worker.receive_into_buf();
                match mouse_drag {
                    MouseDragState::Dragging { offset, .. } => {
                        worker.draw_full_buffer_with_offset(offset.0, offset.1, pixels.get_frame());
                    }
                    _ => {
                        worker.draw_full_buffer_with_offset(0, 0, pixels.get_frame());
                    }
                };

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
