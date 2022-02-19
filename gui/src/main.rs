mod gui;
mod gui_framework;
mod mouse_drag;

use crate::gui_framework::Framework;
use anyhow::Result;
use gui::GuiState;
use log::error;
use mouse_drag::MouseDragState;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use worker::util::measure_execution_time;
use worker::FractalWorker;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    #[structopt(short, long, default_value = "1024")]
    width: u32,
    #[structopt(short, long, default_value = "1024")]
    height: u32,

    #[structopt(short, long)]
    fractal_lib: String,
    #[structopt(short, long)]
    color_lib: String,

    #[structopt(long, default_value = "1.25")]
    extra_scale_factor: f32,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let mut width = args.width;
    let mut height = args.height;
    let fractal_lib = args.fractal_lib;
    let color_lib = args.color_lib;
    let extra_scale_factor = args.extra_scale_factor;

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        // let scaled_size = LogicalSize::new(width as f64 * 3.0, height as f64 * 3.0);
        WindowBuilder::new()
            .with_title("hello world")
            // .with_inner_size(scaled_size)
            .with_inner_size(size)
            // .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        dbg!(scale_factor);
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(width, height, surface_texture)?;
        let framework = Framework::new(
            window_size.width,
            window_size.height,
            scale_factor * extra_scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    let mut mouse_drag = MouseDragState::new();
    let mut worker = FractalWorker::new(width, height, &fractal_lib, &color_lib);
    let mut gui_state = GuiState::default();

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
            // if input.key_pressed(VirtualKeyCode::F) {
            //     framework.gui.window_open = false;
            // }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                // dbg!(scale_factor);
                framework.scale_factor((scale_factor as f32) * extra_scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                dbg!(size);
                pixels.resize_surface(size.width, size.height);
                pixels.resize_buffer(size.width, size.height);
                framework.resize(size.width, size.height);
                width = size.width;
                height = size.height;
                measure_execution_time("worker.apply_resize", || {
                    worker.apply_resize((width, height));
                });
            }

            if !framework.wants_pointer_input() {
                mouse_drag = mouse_drag.update(&input, &pixels);
                match mouse_drag {
                    MouseDragState::Released { offset } => {
                        measure_execution_time("worker.apply_offset", || {
                            worker.apply_offset(offset);
                        });
                    }
                    _ => {}
                };
            }

            window.request_redraw();
        }

        match event {
            Event::MainEventsCleared => {
                worker.on_main_events_cleared();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(x, y),
                        phase: TouchPhase::Moved,
                        ..
                    },
                ..
            } if x.abs() < 0.1 => {
                // println!("scroll delta: {}", y);
                measure_execution_time("worker.apply_zoom", || {
                    worker.apply_zoom(y);
                });
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {
                worker.receive_into_buf();
                // measure_execution_time("worker.draw_with_offset", || {
                worker.draw_with_offset(
                    mouse_drag.drag_offset_or_zero(),
                    pixels.get_frame(),
                    (width, height),
                );
                // });

                // Prepare egui (including render UI)
                framework.prepare(&window, |ctx| {
                    gui_state.draw_gui(ctx, &mut worker, &mut mouse_drag, &fractal_lib);
                });

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
