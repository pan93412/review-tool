use std::ops::{Deref, DerefMut};

use egui::{Widget, DragValue};

use crate::{ui::ReviewToolApp, types::rank::{Item, StandardChoice, sitcon_gdsc, ItemGroup}};

#[derive(Default)]
pub struct Rank {
    pub(crate) group: sitcon_gdsc::Group,
}

impl Rank {
    pub fn show(&mut self, ui: &mut eframe::egui::Ui) {
        render_item_group(&mut self.group.subject, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.student_related));
            ui.add(&mut ChoiceWidget::new(&mut group.community_related));
            ui.add(&mut ChoiceWidget::new(&mut group.coding_related));
            ui.add(&mut ChoiceWidget::new(&mut group.floss_related));
        });
    }
}

impl ReviewToolApp {
    pub(crate) fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        let selected = self
            .manuscripts
            .get(self.current_selected)
            .unwrap_or_else(|| &self.manuscripts[0]);

        let selected_rank = match self.rank.get_mut(selected) {
            Some(entry) => entry,
            None => {
                self.rank.insert(selected.clone(), Rank::default());
                self.rank.get_mut(selected).expect("must be inserted")
            }
        };

        selected_rank.show(ui);
    }
}

fn render_item_group<G: ItemGroup>(group: &mut G, ui: &mut eframe::egui::Ui, add_choice_widget: impl FnOnce(&mut eframe::egui::Ui, &mut G)) {
    ui.heading(group.name());
    group.description().map(|d| ui.label(d));

    egui::Grid::new(group.name())
        .min_col_width(ui.available_width() / 3.0)
        .max_col_width(ui.available_width() / 1.5)
        .min_row_height(ui.available_height())
        .spacing([24.0, 12.0])
        .show(ui, |ui| {
            ui.vertical(|ui| {
                add_choice_widget(ui, group);
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("總分");
                    ui.add(DragValue::new(&mut group.score()));
                });

                ui.label("分數描述");
                ui.text_edit_multiline(&mut group.score_description().unwrap_or_default())
            });
            ui.end_row();
        });
}

/// The choice widget.
pub struct ChoiceWidget<'a, I: Item>(&'a mut I);

impl<'a, I: Item> ChoiceWidget<'a, I> {
    pub fn new(item: &'a mut I) -> Self {
        Self(item)
    }
}

impl<'a, I: Item> Deref for ChoiceWidget<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, I: Item> DerefMut for ChoiceWidget<'a, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, I: Item> Widget for &mut ChoiceWidget<'a, I> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ComboBox::from_label(self.0.name())
            .selected_text(self.0.choice().as_ref())
            .show_ui(ui, |ui| {
                for choice in [StandardChoice::Full, StandardChoice::Partial, StandardChoice::Maybe, StandardChoice::No].iter() {
                    ui.selectable_value(self.0.choice_mut(), *choice, choice.as_ref());
                }
            })
            .response
    }
}
