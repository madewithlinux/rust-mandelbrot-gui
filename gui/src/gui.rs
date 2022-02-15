use std::collections::HashMap;

use egui::CtxRef;
use itertools::Itertools;
use shared::{RHashMap, ROptionsMap, RString, Tuple2};
use worker::FractalWorker;

use crate::mouse_drag::MouseDragState;
#[derive(Default, Debug)]

pub struct GuiState {
    pub edited_fractal_options: HashMap<RString, String>,
}

impl GuiState {
    pub fn draw_gui(
        &mut self,
        ctx: &CtxRef,
        worker: &mut FractalWorker,
        mouse_drag: &mut MouseDragState,
        lib_path: &str,
    ) {
        egui::Window::new("fractal app").show(&ctx, |ui| {
            let (width, height) = worker.get_size();
            ui.label("fractal app");

            ui.separator();

            ui.label("render progress");
            ui.add(egui::ProgressBar::new(0.5).show_percentage().animate(true));

            ui.separator();

            ui.collapsing("general info", |ui| {
                egui::Grid::new("info")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("canvas size:");
                        ui.label(format!("{}x{}", width, height));
                        ui.end_row();

                        ui.label("lib path:");
                        ui.label(lib_path);
                        ui.end_row();

                        ui.label("mouse drag state:");
                        ui.label(format!("{:?}", mouse_drag));
                        ui.end_row();

                        ui.label("worker state:");
                        ui.label(format!("{:?}", worker.get_state()));
                        ui.end_row();
                    });
            });

            ui.separator();

            ui.collapsing("fractal options", |ui| {
                egui::Grid::new("fractal options")
                    .num_columns(3)
                    .striped(true)
                    .show(ui, |ui| {
                        for Tuple2(key, value) in worker.get_fractal_options().into_iter().sorted()
                        {
                            ui.label(key.as_str());
                            ui.label(value.as_str());
                            let edited_value = self.edited_fractal_options.entry(key).or_default();
                            ui.text_edit_singleline(edited_value);
                            ui.end_row();
                        }

                        if ui.button("apply").clicked() {
                            println!("TODO: apply grid config")
                        }
                        ui.end_row();
                    });
            });

            ui.separator();

            if ui.button("reload lib").clicked() {
                println!("TODO: reload library")
            }
        });
    }
}
