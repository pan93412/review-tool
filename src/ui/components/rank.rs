use std::ops::{Deref, DerefMut};

use egui::{Widget, DragValue};

use crate::{ui::ReviewToolApp, types::rank::{Item, StandardChoice, ItemGroup, sitcon_gdsc::subject}};

#[derive(Default)]
pub struct Rank {
    pub(crate) subject: subject::Group,
}

impl Rank {
    pub fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("主題相關性");

        egui::Grid::new("主題相關性")
            .min_col_width(ui.available_width() / 3.0)
            .max_col_width(ui.available_width() / 1.5)
            .min_row_height(ui.available_height())
            .spacing([24.0, 12.0])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add(&mut ChoiceWidget::new(&mut self.subject.student_related));
                    ui.add(&mut ChoiceWidget::new(&mut self.subject.community_related));
                    ui.add(&mut ChoiceWidget::new(&mut self.subject.coding_related));
                    ui.add(&mut ChoiceWidget::new(&mut self.subject.floss_related));
                });

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("總分");
                        ui.add(DragValue::new(&mut self.subject.score()));
                    });

                    ui.label("分數描述");
                    ui.text_edit_multiline(&mut self.subject.score_description().unwrap_or_default())
                });
                ui.end_row();
            });
    }
}

impl ReviewToolApp {
    pub(crate) fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        self.rank.show(ui);
    }
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
