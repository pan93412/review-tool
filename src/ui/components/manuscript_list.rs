//! Component: Manuscript List

use std::{borrow::Cow, collections::hash_map::Entry};

use eframe::egui;
use egui::{Key, Modifiers};

use crate::{types::rank::MetaGroup, ui::ReviewToolApp};

impl<M: MetaGroup> ReviewToolApp<M> {
    pub(crate) fn manuscript_list(&mut self, ui: &mut eframe::egui::Ui) {
        for (id, manuscript) in self.manuscripts.iter() {
            let reviewed = match self.rank_groups.entry(*id) {
                Entry::Occupied(entry) => entry.get().reviewed(),
                Entry::Vacant(_) => false,
            };

            // the title string
            let title = if reviewed {
                Cow::Owned(format!("✔ {}", manuscript.title))
            } else {
                Cow::Borrowed(manuscript.title.as_str())
            };

            ui.selectable_value(&mut self.current_selected, *id, title);
        }

        // ↓ key to select next item
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowDown)) {
            self.current_selected = match self.manuscripts.next(&self.current_selected) {
                Some(id) => *id,
                None => {
                    tracing::warn!("manuscript is empty");
                    return;
                }
            };
        }

        // ↑ key to select previous item
        if ui.input_mut(|s| s.consume_key(Modifiers::NONE, Key::ArrowUp)) {
            self.current_selected = match self.manuscripts.previous(&self.current_selected) {
                Some(id) => *id,
                None => {
                    tracing::warn!("manuscript is empty");
                    return;
                }
            };
        }
    }
}
