//! Component: Header

use eframe::{
    egui::{Key, Modifiers},
    App, Storage,
};
use serde::Serialize;

use crate::{
    types::rank::MetaGroup,
    ui::{ReviewToolApp, RANK_KEY},
};

trait SaveExt {
    fn save_btn(&mut self, ui: &mut eframe::egui::Ui, storage: &mut dyn Storage);
}

impl<M: MetaGroup> SaveExt for ReviewToolApp<M> {
    default fn save_btn(&mut self, _ui: &mut eframe::egui::Ui, _storage: &mut dyn Storage) {}
}

impl<M: MetaGroup + Serialize> SaveExt for ReviewToolApp<M> {
    fn save_btn(&mut self, ui: &mut eframe::egui::Ui, storage: &mut dyn Storage) {
        if ui.button("Save").clicked()
            || ui.input_mut(|s| s.consume_key(Modifiers::COMMAND, Key::S))
        {
            self.save(storage);
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
                if ui.button("Dump").clicked() {
                    #[cfg(not(target_family = "wasm"))]
                    {
                        println!("{}", s.get_string(RANK_KEY).unwrap_or_default());
                    }

                    #[cfg(target_family = "wasm")]
                    {
                        web_sys::console::log_1(&s.get_string(RANK_KEY).unwrap_or_default().into());
                    }

                    self.state.dump();
                }
            }
            if let Some(s) = frame.storage_mut() {
                self.save_btn(ui, s);
            }
            if ui
                .button("Reset")
                .on_hover_text("Double click to reset")
                .double_clicked()
            {
                // Clear the rank groups = Reset
                self.rank_groups.clear();
                self.state.reset();
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status: ");
                ui.label(self.state.get_human_text());
            });
        });
    }
}
