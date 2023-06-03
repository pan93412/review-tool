//! The eframe GUI of this review tool.
//!
//! Note that it is pretty dirty and need to be refactored.

mod components;
mod fonts;

use std::rc::Rc;

use eframe::egui;

use crate::types;

use self::fonts::create_font_def;

pub struct ReviewToolApp {
    manuscripts: Vec<Rc<types::Manuscript>>,

    current_selected: usize,
}

impl ReviewToolApp {
    pub fn new(cc: &eframe::CreationContext<'_>, manuscripts: Vec<Rc<types::Manuscript>>) -> Self {
        cc.egui_ctx.set_fonts(create_font_def());
        Self {
            manuscripts,
            current_selected: 0,
        }
    }
}

impl eframe::App for ReviewToolApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Review tool");
            ui.horizontal_top(|ui| {
                ui.label("Current manuscripts:");
                ui.label(self.manuscripts.len().to_string());
            });

            ui.separator();

            egui::Grid::new("review-ui")
                .num_columns(2)
                .min_col_width(256.0)
                .min_row_height(ui.available_height())
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.push_id("manuscript-list", |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                self.manuscript_list(ui);
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.push_id("manuscript-info", |ui| {
                            egui::ScrollArea::vertical().max_height(ui.available_height() / 2.0 - 2.0).show(ui, |ui| {
                                self.manuscript(ui);
                            });
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("這裡可以根據評分標準進行評分。我們會幫你計算出最終的分數。")
                            });
                        });
                    });

                    ui.end_row();
                });
        });
    }
}
