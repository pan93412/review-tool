//! The eframe GUI of this review tool.
//!
//! Note that it is pretty dirty and need to be refactored.

mod components;
mod fonts;
mod state;

use std::collections::hash_map::Entry;

use eframe::egui;
use serde::{de::DeserializeOwned, Serialize};

use crate::types::{
    rank::{GroupMetaDatabase, MetaGroup},
    ManuscriptDatabase, ManuscriptId, RcManuscript,
};

use self::{components::rank::RankExt, fonts::create_font_def};

const RANK_KEY: &str = "rank";

/// The Review Tool application.
///
/// RG means a RankGroup such as [`crate::types::rank::sitcon_gdsc::Group`].
pub struct ReviewToolApp<M: MetaGroup> {
    manuscripts: ManuscriptDatabase,
    rank_groups: GroupMetaDatabase<M>,

    current_selected: ManuscriptId,
    state: state::State,
}

impl<M: MetaGroup> ReviewToolApp<M> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        manuscripts: ManuscriptDatabase,
    ) -> Result<Self, Error> {
        let first_manuscript = *manuscripts.first().ok_or(Error::NoManuscript)?;
        let rank = Self::retrieve_rank(cc, &manuscripts);

        cc.egui_ctx.set_fonts(create_font_def());

        Ok(Self {
            rank_groups: rank,
            manuscripts,
            current_selected: first_manuscript,
            state: state::State::default(),
        })
    }
}

trait RetrieveRankExt<M: MetaGroup> {
    fn retrieve_rank(
        cc: &eframe::CreationContext<'_>,
        manuscripts: &ManuscriptDatabase,
    ) -> GroupMetaDatabase<M>;
}

impl<M: MetaGroup> RetrieveRankExt<M> for ReviewToolApp<M> {
    default fn retrieve_rank(
        _cc: &eframe::CreationContext<'_>,
        _manuscripts: &ManuscriptDatabase,
    ) -> GroupMetaDatabase<M> {
        GroupMetaDatabase::with_capacity(0)
    }
}

impl<M: MetaGroup + DeserializeOwned> RetrieveRankExt<M> for ReviewToolApp<M> {
    fn retrieve_rank(
        cc: &eframe::CreationContext<'_>,
        manuscripts: &ManuscriptDatabase,
    ) -> GroupMetaDatabase<M> {
        cc.storage
            .and_then(|storage| storage.get_string(RANK_KEY))
            .and_then(|r| {
                serde_yaml::from_str::<'_, GroupMetaDatabase<M>>(&r).map_or_else(
                    |e| {
                        tracing::warn!("failed to deserialize rank: {}", e);
                        None
                    },
                    Some,
                )
            })
            .unwrap_or(GroupMetaDatabase::with_capacity(manuscripts.len()))
    }
}

impl<M: MetaGroup> ReviewToolApp<M> {
    pub(crate) fn get_current_manuscript(&self) -> &RcManuscript {
        self.manuscripts
            .get(&self.current_selected)
            .unwrap_or_else(|| self.manuscripts.values().next().expect("must have one"))
    }
}

impl<M: MetaGroup + Default> ReviewToolApp<M> {
    pub(crate) fn get_current_rank_or_set_default(&mut self) -> &mut M {
        match self.rank_groups.entry(self.current_selected) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(M::default()),
        }
    }
}

impl<M: MetaGroup + Serialize> eframe::App for ReviewToolApp<M> {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.rank_groups.is_empty() {
            storage.set_string(RANK_KEY, String::new());
        } else {
            let serialized_rank = serde_yaml::to_string(&self.rank_groups);

            match serialized_rank {
                Ok(serialized_rank) => {
                    storage.set_string(RANK_KEY, serialized_rank);
                }
                Err(e) => {
                    tracing::error!("failed to serialize rank: {e}");
                }
            }
        }

        storage.flush();
        tracing::info!("data has been stored");
    }
}

impl<M: MetaGroup> eframe::App for ReviewToolApp<M> {
    default fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.header(ui, frame);

            ui.separator();

            egui::Grid::new("review-ui")
                .num_columns(2)
                .min_col_width(256.0)
                .min_row_height(ui.available_height())
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.push_id("manuscript-list", |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                self.manuscript_list(ui);
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.push_id("manuscript-info", |ui| {
                            // leave space for rank-area and separator
                            egui::ScrollArea::vertical()
                                .max_height(ui.available_height() / 2.0 - 1.0)
                                .show(ui, |ui| {
                                    self.manuscript(ui);
                                });
                        });
                        ui.separator();
                        ui.push_id("rank-area", |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(ui.available_height())
                                .show(ui, |ui| {
                                    self.rank(ui);
                                });
                        });
                    });

                    ui.end_row();
                });
        });
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("expected at least 1 manuscript; nothing given.")]
    NoManuscript,
}
