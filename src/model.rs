use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct PatientSummary {
    pub PtID: u32,
    pub Gender: char,
    pub AgeAsOfRandDt: u8,
    pub Race: String,
    pub Ethnicity: String,
    pub Height: f32,
    pub Weight: f32,
    pub DavDiabetes: f32,
    pub InsulinModality: String,
    pub NumSevHypo: String,
    pub HGMReadAvg: Option<i16>,
    pub EduCareGvrP: String,
    pub EduCareGvrPEdu: String,
    pub RandDt: String,
    pub TxGroup: String,
    pub SubStudyGrp: String,
}

#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct GlucoseMeasureEntry {
    pub PtID: u32,
    pub DeviceDtTm: String,
    pub Glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct GlucoseMeasure {
    pub deviceDtTm: String,
    pub glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Patient {
    pub ptID: u32,
    pub gender: char,
    pub age: u8,
    pub race: String,
    pub ethnicity: String,
    pub height: f32,
    pub weight: f32,
    pub davDiabetes: f32,
    pub insulinModality: String,
    pub numSevHypo: String,
    pub hgmReadAvg: i16,
    pub eduCareGvrP: String,
    pub eduCareGvrPEdu: String,
    pub date: String,
    pub txGroup: String,
    pub subStudyGrp: String,
    pub measurements: Vec<GlucoseMeasure>,
}
