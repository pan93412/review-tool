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
    tracing::debug!("Constructing CSV reader…");
    let mut reader = csv::Reader::from_reader(reader);

    tracing::debug!("Deserializing with CSV…");
    reader
        .deserialize::<D>()
        .map(|item| {
            tracing::debug!("Deserialized. Converting data…");
            item.map(|item| item.into())
        })
        .collect()
}

pub fn deserialize(format: Format, reader: impl Read) -> Result<Vec<Manuscript>, csv::Error> {
    tracing::debug!("Deserializing {format:?}…");

    match format {
        Format::SitconGdsc => deserialize_internal::<SitconGdscFormat>(reader),
    }
}
