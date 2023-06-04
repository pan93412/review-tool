use std::ops::{Deref, DerefMut};

use egui::{DragValue, Widget};

use crate::{
    types::rank::{sitcon_gdsc, Item, ItemGroup, MetaGroup, StandardChoice},
    ui::ReviewToolApp,
};

pub struct RankComponent<'a, M: MetaGroup>(pub(crate) &'a mut M);

/// Add `rank()` to Review Tool App. This trait is for specialization.
pub(crate) trait AppRankExtension {
    fn rank(&mut self, ui: &mut eframe::egui::Ui);
}

impl<'a> RankComponent<'a, sitcon_gdsc::Group> {
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        render_item_group(&mut self.0.subject, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.student_related));
            ui.add(&mut ChoiceWidget::new(&mut group.community_related));
            ui.add(&mut ChoiceWidget::new(&mut group.coding_related));
            ui.add(&mut ChoiceWidget::new(&mut group.floss_related));
        });

        render_item_group(&mut self.0.expressive, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.organized));
            ui.add(&mut ChoiceWidget::new(&mut group.fluent));
            ui.add(&mut ChoiceWidget::new(&mut group.completeness));
        });

        render_item_group(&mut self.0.content, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.knowledges));
            ui.add(&mut ChoiceWidget::new(&mut group.experiences));
            ui.add(&mut ChoiceWidget::new(&mut group.uniqueness));
            ui.add(&mut ChoiceWidget::new(&mut group.structure));
            ui.add(&mut ChoiceWidget::new(&mut group.completeness));
        })
    }
}

impl<M: MetaGroup> AppRankExtension for ReviewToolApp<M> {
    default fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("This meta group has not been supported, sorry :(");
    }
}

impl AppRankExtension for ReviewToolApp<sitcon_gdsc::Group> {
    fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        RankComponent(self.get_current_rank_or_set_default()).show(ui)
    }
}

fn render_item_group<G: ItemGroup>(
    group: &mut G,
    ui: &mut eframe::egui::Ui,
    add_choice_widget: impl FnOnce(&mut eframe::egui::Ui, &mut G),
) {
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
                for choice in [
                    StandardChoice::Full,
                    StandardChoice::Partial,
                    StandardChoice::Maybe,
                    StandardChoice::No,
                ]
                .iter()
                {
                    ui.selectable_value(self.0.choice_mut(), *choice, choice.as_ref());
                }
            })
            .response
    }
}
