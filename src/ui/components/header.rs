//! Component: Header

use eframe::{App, Storage};
use serde::Serialize;

use crate::{types::rank::MetaGroup, ui::ReviewToolApp};

trait SaveExt {
    fn save_btn(&mut self, ui: &mut eframe::egui::Ui, storage: &mut dyn Storage);
}

impl<M: MetaGroup> SaveExt for ReviewToolApp<M> {
    default fn save_btn(&mut self, _ui: &mut eframe::egui::Ui, _storage: &mut dyn Storage) {}
}

impl<M: MetaGroup + Serialize> SaveExt for ReviewToolApp<M> {
    fn save_btn(&mut self, ui: &mut eframe::egui::Ui, storage: &mut dyn Storage) {
        if ui.button("Save").clicked() {
            self.save(storage);
            self.state.saved();
        }
    }
}

impl<M: MetaGroup> ReviewToolApp<M> {
    pub(crate) fn header(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Review tool");
                ui.horizontal_top(|ui| {
                    ui.label("Current manuscripts:");
                    ui.label(self.manuscripts.len().to_string());
                });
            });

            ui.separator();
            if let Some(s) = frame.storage_mut() {
                self.save_btn(ui, s);
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status: ");
                ui.label(self.state.get_human_text());
            });
        });
    }
}