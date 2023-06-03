//! Component: Manuscript List

use egui::{Key, Modifiers};

use crate::ui::ReviewToolApp;

impl ReviewToolApp {
    pub(crate) fn manuscript_list(&mut self, ui: &mut eframe::egui::Ui) {
        for (i, manuscript) in self.manuscripts.iter() {
            ui.selectable_value(
                &mut self.current_selected,
                i.clone(),
                manuscript.title.as_str(),
            );
        }

        // ↓ key to select next item
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowDown)) {
            self.current_selected = match self.manuscripts.next(&self.current_selected) {
                Some(id) => id.clone(),
                None => {
                    tracing::warn!("manuscript is empty");
                    return;
                }
            };
        }

        // ↑ key to select previous item
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowUp)) {
            self.current_selected = match self.manuscripts.previous(&self.current_selected) {
                Some(id) => id.clone(),
                None => {
                    tracing::warn!("manuscript is empty");
                    return;
                }
            };
        }
    }
}
