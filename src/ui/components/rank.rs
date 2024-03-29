use std::ops::{Deref, DerefMut};

use eframe::egui;
use egui::{Button, DragValue, Sense, TextEdit, Widget};

use crate::{
    types::rank::{
        sitcon_gdsc, CommentableItemGroup, Item, ItemGroup, MetaGroup, MutableMetaGroup,
        StandardChoice,
    },
    ui::ReviewToolApp,
};

pub struct RankComponent<'a, M: MetaGroup>(pub(crate) &'a mut M);

/// Add `rank()` to Review Tool App. This trait is for specialization.
pub(crate) trait RankExt {
    fn rank(&mut self, ui: &mut eframe::egui::Ui);
}

trait ReviewedExt {
    fn show_reviewed_button(&mut self, ui: &mut eframe::egui::Ui);
}

impl<'a> RankComponent<'a, sitcon_gdsc::Group> {
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        render_item_group_with_comment(&mut self.0.subject, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.student_related));
            ui.add(&mut ChoiceWidget::new(&mut group.community_related));
            ui.add(&mut ChoiceWidget::new(&mut group.coding_related));
            ui.add(&mut ChoiceWidget::new(&mut group.floss_related));
        });

        render_item_group_with_comment(&mut self.0.content, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.knowledges));
            ui.add(&mut ChoiceWidget::new(&mut group.experiences));
            ui.add(&mut ChoiceWidget::new(&mut group.uniqueness));
            ui.add(&mut ChoiceWidget::new(&mut group.structure));
            ui.add(&mut ChoiceWidget::new(&mut group.completeness));
        });

        render_item_group_with_comment(&mut self.0.expressive, ui, |ui, group| {
            ui.add(&mut ChoiceWidget::new(&mut group.organized));
            ui.add(&mut ChoiceWidget::new(&mut group.fluent));
            ui.add(&mut ChoiceWidget::new(&mut group.completeness));
        });
    }
}

impl<M: MetaGroup> RankComponent<'_, M> {
    fn reviewed_text(&self) -> &str {
        if self.0.reviewed() {
            "✅ Reviewed"
        } else {
            "❌ Reviewed"
        }
    }
}

impl<M: MetaGroup> ReviewedExt for RankComponent<'_, M> {
    default fn show_reviewed_button(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(Button::new(self.reviewed_text()).sense(Sense::hover()))
            .on_hover_text("Unable to switch this state");
    }
}

impl<M: MutableMetaGroup> ReviewedExt for RankComponent<'_, M> {
    fn show_reviewed_button(&mut self, ui: &mut eframe::egui::Ui) {
        let response = ui
            .button(self.reviewed_text())
            .on_hover_text("Click to switch this state");

        if response.clicked() {
            *self.0.reviewed_mut() = !self.0.reviewed();
        }
    }
}

impl<M: MetaGroup> RankExt for ReviewToolApp<M> {
    default fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("This meta group has not been supported, sorry :(");
    }
}

impl RankExt for ReviewToolApp<sitcon_gdsc::Group> {
    fn rank(&mut self, ui: &mut eframe::egui::Ui) {
        let mut c = RankComponent(self.get_current_rank_or_set_default());
        c.show_reviewed_button(ui);
        ui.separator();
        c.show(ui);
    }
}

fn render_item_group_with_comment<G: CommentableItemGroup>(
    group: &mut G,
    ui: &mut eframe::egui::Ui,
    add_choice_widget: impl FnOnce(&mut eframe::egui::Ui, &mut G),
) {
    render_item_group_advanced(group, ui, add_choice_widget, |ui, group| {
        ui.label("評論");
        ui.add(TextEdit::multiline(group.comment_mut()).desired_rows(1));
    })
}

fn render_item_group_advanced<G: ItemGroup>(
    group: &mut G,
    ui: &mut eframe::egui::Ui,
    add_choice_widget: impl FnOnce(&mut eframe::egui::Ui, &mut G),
    right_pane_bottom: impl FnOnce(&mut eframe::egui::Ui, &mut G),
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
                ui.text_edit_multiline(&mut group.score_description().unwrap_or_default());

                right_pane_bottom(ui, group);
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
