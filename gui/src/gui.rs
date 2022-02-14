use egui::CtxRef;
use worker::FractalWorker;

use crate::mouse_drag::MouseDragState;

pub fn gui(
    ctx: &CtxRef,
    worker: &mut FractalWorker,
    mouse_drag: &mut MouseDragState,
    lib_path: &str,
) {
    egui::Window::new("mandelbrot gui").show(&ctx, |ui| {
        ui.label("Hello world!");

        ui.separator();

        let (width, height) = worker.get_size();
        ui.label(format!("canvas size: {}x{}", width, height));
        ui.label(format!("lib path: {}", lib_path));
        ui.label(format!("mouse drag state: {:?}", mouse_drag));
        
        ui.separator();

        if ui.button("Click me").clicked() {
            println!("clicked")
        }
    });
}
