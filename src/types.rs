//! Parse the content in CSV file.

use std::{collections::HashMap, ops::Deref, rc::Rc};

use serde::{Deserialize, Serialize};

pub mod deserialize;
pub mod rank;

/// The difficulty of a manuscript.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

impl AsRef<str> for Difficulty {
    fn as_ref(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// The author of a manuscript.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Author {
    /// The author of this manuscript.
    pub name: String,

    /// The self-introdcution of this manuscript.
    pub description: String,
}

/// The manuscript.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Manuscript {
    /// The title of this manuscript.
    pub title: String,

    /// The type of this manuscript.
    pub type_: String,

    /// The abstract of this manuscript.
    pub abstract_: String,

    /// The audience of this manuscript.
    pub audience: String,

    /// The difficulty of this manuscript.
    pub difficulty: Difficulty,

    /// The description of this manuscript.
    pub description: String,

    /// The extra information of this manuscript.
    pub extra: String,

    /// The author of this manuscript.
    pub author: Author,
}

/// The SHA-256 ID of the manuscript.
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
pub struct ManuscriptId(pub Vec<u8>);

impl ManuscriptId {
    pub fn hash(m: &Manuscript) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&m.title);
        hasher.update(&m.author.name);
        hasher.update(&m.type_);

        Self(hasher.finalize().to_vec())
    }
}

pub type RcManuscriptId = Rc<ManuscriptId>;
pub type RcManuscript = Rc<Manuscript>;

type ManuscriptDatabaseInner = HashMap<RcManuscriptId, RcManuscript>;

/// The database of manuscript.
///
/// It contains a [`HashMap`] to store the mapping of
/// [`RcManuscriptId`] to [`RcManuscript`], and a [`Vec`] to store
/// the order of the manuscripts.
///
/// Note that it is immutable and should not be changed.
pub struct ManuscriptDatabase {
    order: Vec<RcManuscriptId>,
    db: ManuscriptDatabaseInner,
}

pub struct ManuscriptDatabaseIter<'a> {
    idx: usize,
    db: &'a ManuscriptDatabase,
}

impl ManuscriptDatabase {
    /// Get the first item ID in the database.
    pub fn first(&self) -> Option<&RcManuscriptId> {
        self.order.first()
    }

    /// Get the last item ID in the database.
    pub fn last(&self) -> Option<&RcManuscriptId> {
        self.order.last()
    }

    /// Find the previous item to the given manuscript ID.
    ///
    /// If no such item, return the first item.
    /// If nothing in the database, return `None`.
    pub fn previous(&self, id: &ManuscriptId) -> Option<&RcManuscriptId> {
        self.order
            .split(|x| x.as_ref() == id)
            .next()
            .and_then(|x| x.last())
            .or_else(|| self.first())
    }

    /// Find the next item to the given manuscript ID.
    ///
    /// If no such item, return the last item.
    /// If nothing in the database, return `None`.
    pub fn next(&self, id: &ManuscriptId) -> Option<&RcManuscriptId> {
        self.order
            .split(|x| x.as_ref() == id)
            .nth(1)
            .and_then(|x| x.first())
            .or_else(|| self.last())
    }

    /// The iterator of the database.
    pub fn iter(&self) -> ManuscriptDatabaseIter<'_> {
        ManuscriptDatabaseIter { idx: 0, db: self }
    }
}

impl Deref for ManuscriptDatabase {
    type Target = ManuscriptDatabaseInner;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl From<Vec<Rc<Manuscript>>> for ManuscriptDatabase {
    fn from(v: Vec<Rc<Manuscript>>) -> Self {
        let mut db = HashMap::with_capacity(v.len());
        let mut order = Vec::with_capacity(v.len());

        for m in v {
            let id = Rc::new(ManuscriptId::hash(&m));
            db.insert(id.clone(), m);
            order.push(id);
        }

        Self { order, db }
    }
}

impl From<Vec<Manuscript>> for ManuscriptDatabase {
    fn from(v: Vec<Manuscript>) -> Self {
        let mut db = HashMap::with_capacity(v.len());
        let mut order = Vec::with_capacity(v.len());

        for m in v {
            let id = Rc::new(ManuscriptId::hash(&m));
            db.insert(id.clone(), Rc::new(m));
            order.push(id);
        }

        Self { order, db }
    }
}

impl<'a> Iterator for ManuscriptDatabaseIter<'a> {
    type Item = (&'a RcManuscriptId, &'a RcManuscript);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.db.order.get(self.idx)?;
        let item = self.db.db.get(id)?;

        self.idx += 1;
        Some((id, item))
    }
}
