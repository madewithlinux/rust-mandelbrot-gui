use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use abi_stable::std_types::{RString, Tuple2};
use egui::{CtxRef, Ui};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use worker::fractal_worker2::FractalWorker;

use crate::pan_zoom_debounce::PanZoomDebounce;

const FRAME_TIMES_COUNT: usize = 60;

#[derive(Debug)]
pub struct GuiState {
    pub edited_fractal_options: HashMap<RString, String>,
    pub match_window_size: bool,
    pub window_visible: bool,
    //
    frame_times: VecDeque<OrderedFloat<f64>>,
    last_frame_time: Instant,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            edited_fractal_options: Default::default(),
            match_window_size: true,
            window_visible: true,
            frame_times: Default::default(),
            last_frame_time: Instant::now(),
        }
    }
}

impl GuiState {
    fn update_frame_time(&mut self) {
        if self.frame_times.len() >= FRAME_TIMES_COUNT {
            self.frame_times.pop_front();
        }
        self.frame_times
            .push_back(self.last_frame_time.elapsed().as_secs_f64().into());
        self.last_frame_time = Instant::now();
    }

    fn avg_frame_time(&self) -> f64 {
        (self.frame_times.iter().sum::<OrderedFloat<f64>>() / (self.frame_times.len() as f64))
            .into()
    }
    fn max_frame_time(&self) -> f64 {
        self.frame_times.iter().max().unwrap().clone().into()
    }

    pub fn draw_gui(
        &mut self,
        ctx: &CtxRef,
        window_width: u32,
        window_height: u32,
        worker: &mut FractalWorker,
        pan_zoom: &PanZoomDebounce,
        color_lib_path: &str,
        fractal_lib_path: &str,
    ) {
        self.update_frame_time();

        let mut open = self.window_visible;
        egui::Window::new("fractal app")
            .open(&mut open)
            .show(&ctx, |ui| {
                ui.label("fractal app");

                ui.separator();

                ui.label("render progress");
                ui.add(
                    egui::ProgressBar::new(worker.get_progress())
                        .show_percentage()
                        .animate(true),
                );

                ui.checkbox(&mut self.match_window_size, "match window size");

                egui::CollapsingHeader::new("general info")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.general_info_grid(
                            ui,
                            window_width,
                            window_height,
                            worker,
                            pan_zoom,
                            color_lib_path,
                            fractal_lib_path,
                        );
                    });

                egui::CollapsingHeader::new("fractal options")
                    .default_open(false)
                    .show(ui, |ui| {
                        self.fractal_options_grid(ui, worker);
                    });

                ui.horizontal(|ui| {
                    if ui.button("reload lib").clicked() {
                        println!("TODO: reload library")
                    }
                });
            });
        self.window_visible = open;
    }

    // fn gui_options_grid(&mut self, ui: &mut Ui) {
    //     egui::Grid::new("GUI options")
    //         .num_columns(2)
    //         .striped(true)
    //         .show(ui, |ui| {
    //             ui.label("match window size");
    //             ui.checkbox(&mut self.match_window_size, "match window size");
    //         });
    // }

    fn general_info_grid(
        &mut self,
        ui: &mut Ui,
        window_width: u32,
        window_height: u32,
        worker: &mut FractalWorker,
        pan_zoom: &PanZoomDebounce,
        color_lib_path: &str,
        fractal_lib_path: &str,
    ) {
        let (canvas_width, canvas_height) = worker.get_size();
        egui::Grid::new("info")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                // ui.style_mut().wrap = Some(true);
                // ui.set_max_width(400.0);

                ui.label("frame time avg (max):");
                ui.label(format!(
                    "{:.1} ms ({:.1} ms)",
                    self.avg_frame_time() * 1000.0,
                    self.max_frame_time() * 1000.0
                ));
                ui.end_row();

                ui.label("window size:");
                ui.label(format!("{}x{}", window_width, window_height));
                ui.end_row();
                ui.label("canvas size:");
                ui.label(format!("{}x{}", canvas_width, canvas_height));
                ui.end_row();

                ui.label("color lib path:");
                ui.label(color_lib_path);
                ui.end_row();
                ui.label("fractal lib path:");
                ui.label(fractal_lib_path);
                ui.end_row();

                // ui.label("pan zoom state");
                // ui.label(format!("{:?}", pan_zoom));
                // ui.end_row();
                ui.label("x offset");
                ui.label(format!("{}", pan_zoom.transform.translation.x));
                ui.end_row();
                ui.label("y offset");
                ui.label(format!("{}", pan_zoom.transform.translation.y));
                ui.end_row();
                ui.label("zoom factor");
                ui.label(format!("{}", pan_zoom.transform.scale));
                ui.end_row();

                ui.label("worker state:");
                ui.label(format!("{:?}", worker.get_state()));
                ui.end_row();
            });
    }

    fn fractal_options_grid(&mut self, ui: &mut Ui, worker: &mut FractalWorker) {
        egui::Grid::new("fractal options")
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                for Tuple2(key, value) in worker.get_fractal_options().into_iter().sorted() {
                    ui.label(key.as_str());
                    ui.label(value.as_str());
                    let value_to_edit = self.edited_fractal_options.entry(key).or_default();
                    let response = ui.text_edit_singleline(value_to_edit);
                    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                        self.set_fractal_options(worker);
                    }
                    ui.end_row();
                }

                if ui.button("reset").clicked() {
                    worker.reset_fractal_options();
                }
                if ui.button("clear").clicked() {
                    self.edited_fractal_options.clear();
                }
                if ui.button("apply").clicked() {
                    self.set_fractal_options(worker);
                }
                ui.end_row();
            });
    }

    fn set_fractal_options(&mut self, worker: &mut FractalWorker) {
        // remove empty values
        self.edited_fractal_options.retain(|_, v| !v.is_empty());
        if !self.edited_fractal_options.is_empty() {
            worker.set_fractal_options(&self.edited_fractal_options);
        }
    }
}
