//! The eframe GUI of this review tool.
//!
//! Note that it is pretty dirty and need to be refactored.

mod components;
mod fonts;

use std::{rc::Rc, collections::{HashMap, hash_map::Entry}};

use eframe::egui;

use crate::types;

use self::{fonts::create_font_def, components::rank::Rank};

pub struct ReviewToolApp {
    manuscripts: Vec<Rc<types::Manuscript>>,
    rank: HashMap<Rc<types::Manuscript>, Rank>,

    current_selected: usize,
}

impl ReviewToolApp {
    pub fn new(cc: &eframe::CreationContext<'_>, manuscripts: Vec<Rc<types::Manuscript>>) -> Result<Self, Error> {
        if manuscripts.is_empty() {
            return Err(Error::NoManuscript)
        }

        cc.egui_ctx.set_fonts(create_font_def());
        Ok(Self {
            manuscripts,
            rank: HashMap::default(),
            current_selected: 0,
        })
    }

    pub(crate) fn get_current_manuscript(&self) -> &Rc<types::Manuscript> {
        self
            .manuscripts
            .get(self.current_selected)
            .unwrap_or_else(|| &self.manuscripts[0])
    }

    pub(crate) fn get_current_rank_or_set_default(&mut self) -> &mut Rank {
        let selected = self.get_current_manuscript().clone();

        match self.rank.entry(selected) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Rank::default()),
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
                            // leave space for rank-area and separator
                            egui::ScrollArea::vertical().max_height(ui.available_height() / 2.0 - 1.0).show(ui, |ui| {
                                self.manuscript(ui);
                            });
                        });
                        ui.separator();
                        ui.push_id("rank-area", |ui| {
                            egui::ScrollArea::vertical().max_height(ui.available_height()).show(ui, |ui| {
                                self.rank(ui);
                            });
                        });

                    });

                    ui.end_row();
                });
        });
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("expected at least 1 manuscript; nothing given.")]
    NoManuscript,
}
