//! The eframe GUI of this review tool.
//!
//! Note that it is pretty dirty and need to be refactored.

mod fonts;

use std::rc::Rc;

use eframe::egui;
use egui::{Key, Modifiers, RichText, TextEdit};

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

impl ReviewToolApp {
    fn render_manuscript_list(&mut self, ui: &mut eframe::egui::Ui) {
        for (i, manuscript) in self.manuscripts.iter().enumerate() {
            let resp =
                ui.selectable_value(&mut self.current_selected, i, manuscript.title.as_str());

            if resp.clicked() {
                self.current_selected = i;
            }
        }

        let final_item_idx = self.manuscripts.len() as isize - 1;
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowDown)) {
            let x = (self.current_selected as isize + 1)
                .min(final_item_idx)
                .max(0);
            self.current_selected = x as usize;
        }
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowUp)) {
            let x = (self.current_selected as isize - 1)
                .min(final_item_idx)
                .max(0);
            self.current_selected = x as usize;
        }
    }

    fn render_manuscript(&mut self, ui: &mut eframe::egui::Ui) {
        let selected = self
            .manuscripts
            .get(self.current_selected)
            .unwrap_or_else(|| &self.manuscripts[0]);

        ui.heading(selected.title.as_str());
        ui.label(selected.type_.as_str());
        ui.label(selected.description.as_str());

        ui.separator();

        egui::Grid::new("manuscript-details")
            .num_columns(2)
            .min_col_width(ui.available_width() / 2.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Abstract");
                    ui.text_edit_multiline(&mut selected.abstract_.as_str());
                });

                ui.vertical(|ui| {
                    ui.heading("Author");
                    let name = RichText::new(selected.author.name.as_str()).size(15.0);

                    ui.label(name);
                    ui.text_edit_multiline(&mut selected.author.description.as_str());
                });

                ui.end_row();

                ui.vertical(|ui| {
                    ui.heading("Audiences and Difficulty");

                    ui.horizontal_top(|ui| {
                        ui.label("Audience: ");

                        TextEdit::multiline(&mut selected.audience.as_str())
                            .desired_rows(1)
                            .show(ui);
                    });

                    ui.horizontal_top(|ui| {
                        ui.label("Difficulty: ");

                        TextEdit::multiline(&mut selected.difficulty.as_ref())
                            .desired_rows(1)
                            .show(ui);
                    });
                });

                ui.vertical(|ui| {
                    ui.heading("Extra");
                    ui.text_edit_multiline(&mut selected.extra.as_str());
                });

                ui.end_row();
            });
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
                                self.render_manuscript_list(ui);
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.push_id("manuscript-info", |ui| {
                            egui::ScrollArea::vertical().max_height(ui.available_height() / 2.0 - 2.0).show(ui, |ui| {
                                self.render_manuscript(ui);
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
