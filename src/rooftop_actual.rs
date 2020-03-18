use crate::{FileKeyable, GetFromRawAemo, RawAemoFile, Result, mms_datetime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    actual: Vec<RooftopActual>,
}

impl crate::AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            actual: RooftopActual::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RooftopActual {
    #[serde(with = "mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    power: f64,
    qi: f64,
    #[serde(rename = "type")]
    type_: String,
    #[serde(with = "mms_datetime")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for RooftopActual {
    fn key() -> crate::FileKey {
        ("ROOFTOP".into(), "ACTUAL".into(), 2)
    }
}

impl GetFromRawAemo for RooftopActual {
    type Output = Self;
}
