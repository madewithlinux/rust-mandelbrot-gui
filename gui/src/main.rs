mod gui;
mod gui_framework;

mod pan_zoom_debounce;
mod renderer;

use crate::gui_framework::Framework;
use anyhow::Result;
use gui::GuiState;
use log::error;
use pan_zoom_debounce::PanZoomDebounce;
use pixels::wgpu;
use pixels::{PixelsBuilder, SurfaceTexture};
use renderer::TransformRenderer;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
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
        WindowBuilder::new()
            .with_title("fractal app")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        dbg!(scale_factor);
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = PixelsBuilder::new(width, height, surface_texture)
            .clear_color(wgpu::Color::TRANSPARENT)
            .build()?;
        let framework = Framework::new(
            window_size.width,
            window_size.height,
            // scale_factor * extra_scale_factor,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    let mut pan_zoom = PanZoomDebounce::new(width, height);
    let mut worker = FractalWorker::new(width, height, &fractal_lib, &color_lib);
    let mut gui_state = GuiState::default();
    let mut transform_renderer = TransformRenderer::new(&pixels, width, height);

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        if let Event::WindowEvent { event, .. } = &event {
            framework.handle_event(event);
        }

        if input.update(&event) {
            // Close events
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                // dbg!(scale_factor);
                dbg!(scale_factor);
                framework.scale_factor((scale_factor as f32) * extra_scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                dbg!(size);
                pixels.resize_surface(size.width, size.height);
                pixels.resize_buffer(size.width, size.height);
                transform_renderer.resize(&pixels, size.width, size.height);
                framework.resize(size.width, size.height);
                width = size.width;
                height = size.height;
                measure_execution_time("worker.apply_resize", || {
                    worker.apply_resize((width, height));
                });
            }

            if !framework.wants_pointer_input() {
                if input.key_pressed(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                pan_zoom.handle_input(width, height, &input, &pixels);
            }

            window.request_redraw();
        }

        if let Some((dx, dy, zoom_factor)) = pan_zoom.get_completed_input() {
            measure_execution_time(
                format!(
                    "worker apply offset and zoom factor {:?}",
                    (dx, dy, zoom_factor)
                )
                .as_str(),
                || {
                    worker.apply_offset_and_zoom_factor(dx, dy, zoom_factor);
                },
            );
        }

        match event {
            Event::MainEventsCleared => {
                worker.on_main_events_cleared();
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {
                worker.receive_into_buf();
                worker.draw_with_offset((0, 0), pixels.get_frame(), (width, height));

                // Prepare egui (including render UI)
                framework.prepare(&window, |ctx| {
                    gui_state.draw_gui(ctx, &mut worker, &pan_zoom, &fractal_lib);
                });

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    if pan_zoom.did_input_just_finish() {
                        transform_renderer.copy_texture_back(encoder);
                    }

                    // Render the world texture
                    context
                        .scaling_renderer
                        .render(encoder, transform_renderer.get_texture_view());

                    transform_renderer.update(&context.queue, pan_zoom.get_render_matrix());

                    transform_renderer.render(
                        encoder,
                        render_target,
                        // context.scaling_renderer.clip_rect(),
                        context,
                    );
                    // // if pan_zoom.did_input_just_finish() {
                    // if pan_zoom.is_completed() {
                    //     transform_renderer.copy_texture_back(encoder);
                    // }

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
