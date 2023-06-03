//! Deserialize the input CSV to a `Manuscript` struct.

use std::io::Read;

use serde::de::DeserializeOwned;

use super::Manuscript;

mod sitcon_gdsc;
pub use sitcon_gdsc::SitconGdscFormat;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    SitconGdsc,
}

fn deserialize_internal<D: DeserializeOwned + Into<Manuscript>>(
    reader: impl Read,
) -> Result<Vec<Manuscript>, csv::Error> {
    let mut reader = csv::Reader::from_reader(reader);

    reader
        .deserialize::<D>()
        .map(|item| item.map(|item| item.into()))
        .collect()
}

pub fn deserialize(format: Format, reader: impl Read) -> Result<Vec<Manuscript>, csv::Error> {
    tracing::debug!("Deserializing {format:?}…");

    match format {
        Format::SitconGdsc => deserialize_internal::<SitconGdscFormat>(reader),
    }
}
