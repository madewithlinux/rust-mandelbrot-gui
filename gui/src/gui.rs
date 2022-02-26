use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use abi_stable::std_types::{RHashMap, RString, Tuple2};
use anyhow::{Context, Result};
use egui::{CtxRef, Ui};
use itertools::Itertools;
use native_dialog::FileDialog;
use ordered_float::OrderedFloat;
use worker::fractal_worker2::FractalWorker;

use crate::pan_zoom_debounce::PanZoomDebounce;

const FRAME_TIMES_COUNT: usize = 60;

#[derive(Debug)]
pub struct GuiState {
    // pub edited_fractal_options: HashMap<RString, String>,
    pub match_window_size: bool,
    pub window_visible: bool,
    //
    frame_times: VecDeque<OrderedFloat<f64>>,
    last_frame_time: Instant,
    //
    fractal_options: OptionsGrid,
    color_options: OptionsGrid,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            // edited_fractal_options: Default::default(),
            match_window_size: true,
            window_visible: true,
            frame_times: Default::default(),
            last_frame_time: Instant::now(),
            fractal_options: Default::default(),
            color_options: Default::default(),
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
        (*self.frame_times.iter().max().unwrap()).into()
    }

    pub fn draw_gui(
        &mut self,
        ctx: &CtxRef,
        window_width: u32,
        window_height: u32,
        frame_width: u32,
        frame_height: u32,
        frame: &[u8],
        worker: &mut FractalWorker,
        pan_zoom: &PanZoomDebounce,
        color_lib_path: &str,
        fractal_lib_path: &str,
    ) {
        self.update_frame_time();

        let mut open = self.window_visible;
        egui::Window::new("fractal app")
            .open(&mut open)
            .show(ctx, |ui| {
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
                    .show(ui, |ui| self.fractal_options_grid(ui, worker));
                egui::CollapsingHeader::new("color options")
                    .default_open(false)
                    .show(ui, |ui| self.color_options_grid(ui, worker));

                ui.horizontal(|ui| {
                    if ui.button("reload lib").clicked() {
                        // println!("TODO: reload library")
                        if let Err(e) = worker.reload_libraries() {
                            log::error!("error reloading libraries: {}", e);
                        }
                    }
                    if ui.button("save framme").clicked() {
                        if let Err(e) = self.save_frame(frame_width, frame_height, frame) {
                            log::error!("error saving image: {}", e);
                        }
                    }
                });
            });
        self.window_visible = open;
    }

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
        self.fractal_options
            .update_from_live_options(worker.get_fractal_options());
        if let Some(new_options) = self.fractal_options.options_grid(ui, "fractal options") {
            worker.set_fractal_options(new_options);
        }
    }

    fn color_options_grid(&mut self, ui: &mut Ui, worker: &mut FractalWorker) {
        self.color_options
            .update_from_live_options(worker.get_color_options());
        if let Some(new_options) = self.color_options.options_grid(ui, "color options") {
            worker.set_color_options(new_options);
        }
    }

    fn save_frame(&self, frame_width: u32, frame_height: u32, frame: &[u8]) -> Result<()> {
        let frame_buf =
            image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(frame_width, frame_height, frame)
                .expect("pixel buffer layout bad");

        let path = FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .set_filename(&format!(
                "fractal_{}.png",
                chrono::Local::now().format("%F_%H-%M-%S")
            ))
            .show_save_single_file()?;

        if let Some(path) = path {
            frame_buf.save(path).context("failed to write output image")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub struct OptionsGrid {
    live_options: RHashMap<RString, RString>,
    pending_options: HashMap<String, String>,
}

impl OptionsGrid {
    fn revert_from_live_options(&mut self) {
        self.pending_options = self
            .live_options
            .iter()
            .map(|Tuple2(k, v)| (k.to_string(), v.to_string()))
            .collect();
    }

    pub fn update_from_live_options(&mut self, live_options: RHashMap<RString, RString>) {
        if self.live_options != live_options {
            self.live_options = live_options;
            self.revert_from_live_options();
        }
    }

    fn get_edited_options(&self) -> Option<RHashMap<RString, RString>> {
        let edited_options: RHashMap<RString, RString> = self
            .pending_options
            .iter()
            .map(|(k, v)| (RString::from(k.as_str()), RString::from(v.as_str())))
            .filter(|(k, v)| self.live_options.get(k) != Some(v))
            .collect();

        if edited_options.is_empty() {
            None
        } else {
            Some(edited_options)
        }
    }

    pub fn options_grid(&mut self, ui: &mut Ui, id: &str) -> Option<RHashMap<RString, RString>> {
        egui::Grid::new(id)
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                let mut should_return_options = false;

                for (key, value) in self.pending_options.iter_mut().sorted() {
                    ui.label(key.as_str());
                    let response = ui.text_edit_singleline(value);
                    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                        should_return_options = true;
                    }
                    ui.end_row();
                }

                if ui.button("revert").clicked() {
                    self.revert_from_live_options();
                }
                if ui.button("apply").clicked() {
                    should_return_options = true;
                }
                ui.end_row();

                if should_return_options {
                    self.get_edited_options()
                } else {
                    None
                }
            })
            .inner
    }
}
