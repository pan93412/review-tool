//! Component: Manuscript List

use egui::{Modifiers, Key};

use crate::ui::ReviewToolApp;

impl ReviewToolApp {
    pub(crate) fn manuscript_list(&mut self, ui: &mut eframe::egui::Ui) {
        for (i, manuscript) in self.manuscripts.iter().enumerate() {
            ui.selectable_value(&mut self.current_selected, i, manuscript.title.as_str());
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
}
