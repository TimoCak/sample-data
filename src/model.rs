use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct PatientSummary {
    PtID: u32,
    Gender: char,
    AgeAsOfRandDt: u8,
    Race: String,
    Ethnicity: String,
    Height: f32,
    Weight: f32,
    DavDiabetes: f32,
    InsulinModality: String,
    NumSevHypo: String,
    HGMReadAvg: Option<i16>,
    EduCareGvrP: String,
    EduCareGvrPEdu: String,
    RandDt: String,
    TxGroup: String,
    SubStudyGrp: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GlucoseMeasureEntry {
    PtID: u32,
    DeviceDtTm: String,
    Glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GlucoseMeasure {
    deviceDtTm: String,
    glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Patient {
    ptID: u32,
    gender: char,
    age: u8,
    race: String,
    ethnicity: String,
    height: f32,
    weight: f32,
    davDiabetes: f32,
    insulinModality: String,
    numSevHypo: String,
    hGMReadAvg: i16,
    eduCareGvrP: String,
    eduCareGvrPEdu: String,
    date: String,
    txGroup: String,
    subStudyGrp: String,
    measurements: Vec<GlucoseMeasure>,
}
