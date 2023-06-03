//! The structure related to a rank.

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use super::ManuscriptId;

pub mod sitcon_gdsc;

/// A rank item.
pub trait Item: Sized {
    /// The item name.
    fn name(&self) -> &str;

    /// The item description.
    fn description(&self) -> Option<&str>;

    /// The item choice.
    fn choice(&self) -> StandardChoice;

    /// The mutable item choice.
    fn choice_mut(&mut self) -> &mut StandardChoice;

    /// The item comment.
    fn comment(&self) -> Option<&str>;

    /// The mutable item comment.
    fn comment_mut(&mut self) -> &mut Option<String>;
}

/// A group of items.
pub trait ItemGroup {
    /// The name of this group.
    fn name(&self) -> &str;

    /// The description of this group.
    fn description(&self) -> Option<&str>;

    /// The score of this group.
    fn score(&self) -> f64;

    /// The description of the score.
    fn score_description(&self) -> Option<String>;
}

/// A group of [`ItemGroup`] – we called it *meta*.
pub trait MetaGroup {}

// No meta group.
impl MetaGroup for () {}

/// The database of manuscript id to meta group.
///
/// It contains a [`HashMap`] to store the mapping of
/// [`ManuscriptId`] to `T`.
#[derive(Serialize, Deserialize, Default)]
pub struct GroupMetaDatabase<M: MetaGroup>(HashMap<ManuscriptId, M>);

impl<M: MetaGroup> GroupMetaDatabase<M> {
    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }
}

impl<M: MetaGroup> Deref for GroupMetaDatabase<M> {
    type Target = HashMap<ManuscriptId, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: MetaGroup> DerefMut for GroupMetaDatabase<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The standard choice for ranking.
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum StandardChoice {
    /// 完全符合要求 (Full Match)
    #[default]
    Full,

    /// 部分符合要求 (Partial Match)
    Partial,

    /// 可能符合要求 (Maybe Match)
    Maybe,

    /// 完全不符合要求 (No Match)
    No,
}

impl AsRef<str> for StandardChoice {
    fn as_ref(&self) -> &str {
        match self {
            StandardChoice::Full => "完全符合要求",
            StandardChoice::Partial => "部分符合要求",
            StandardChoice::Maybe => "可能符合要求",
            StandardChoice::No => "完全不符合要求",
        }
    }
}

impl StandardChoice {
    /// Show choice as emoji.
    pub fn as_emoji(&self) -> &'static str {
        match self {
            StandardChoice::Full => "✅",
            StandardChoice::Partial => "👌",
            StandardChoice::Maybe => "🤔",
            StandardChoice::No => "⚠️",
        }
    }
}

impl std::fmt::Display for StandardChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
