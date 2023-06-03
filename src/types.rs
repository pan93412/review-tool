//! Parse the content in CSV file.

pub mod deserialize;
pub mod rank;

/// The difficulty of a manuscript.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(PartialEq, Eq, Debug)]
pub struct Author {
    /// The author of this manuscript.
    pub name: String,

    /// The self-introdcution of this manuscript.
    pub description: String,
}

/// The manuscript.
#[derive(PartialEq, Eq, Debug)]
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
