use serde::{Deserialize, Serialize};

//Deserialize Data
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PatientSummary {
    RecID: u32,
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
    EduCareGvrPEdu: String,
    RandDt: String,
    TxGroup: String,
    SubStudyGrp: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GlucoseMeasureEntry {
    RecID: u32,
    PtID: u32,
    DeviceDtTim: String,
    Glucose: u16,
}

//serialize Data
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GlucoseMeasure {
    PtID: u32,
    DeviceDtTim: String,
    Glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Patient {
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
    HGMReadAvg: i16,
    EduCareGvrPEdu: String,
    RandDt: String,
    TxGroup: String,
    SubStudyGrp: String,
    Measurements: Vec<GlucoseMeasure>,
}
