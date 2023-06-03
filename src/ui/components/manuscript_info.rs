//! Component: Manuscript Info

use egui::{RichText, TextEdit};

use crate::ui::ReviewToolApp;

impl ReviewToolApp {
    pub(crate) fn manuscript(&mut self, ui: &mut eframe::egui::Ui) {
        let selected = self
            .manuscripts
            .get(self.current_selected)
            .unwrap_or_else(|| &self.manuscripts[0]);

        ui.heading(selected.title.as_str());
        ui.label(selected.type_.as_str());
        ui.label(selected.abstract_.as_str());

        ui.separator();

        egui::Grid::new("manuscript-details")
            .num_columns(2)
            .min_col_width(ui.available_width() / 2.0)
            .show(ui, |ui| {
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
                    ui.heading("Author");
                    let name = RichText::new(selected.author.name.as_str()).size(15.0);

                    ui.label(name);
                    ui.text_edit_multiline(&mut selected.author.description.as_str());
                });

                ui.end_row();

                ui.vertical(|ui| {
                    ui.heading("Description");
                    ui.text_edit_multiline(&mut selected.description.as_str());
                });

                ui.vertical(|ui| {
                    ui.heading("Extra");
                    ui.text_edit_multiline(&mut selected.extra.as_str());
                });

                ui.end_row();
            });
    }
}
