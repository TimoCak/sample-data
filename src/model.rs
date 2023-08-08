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
pub struct GlucoseMeasure {
    DeviceDtTm: String,
    Glucose: u16,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Patient {
    PtID: u32,
    Gender: char,
    Age: u8,
    Race: String,
    Ethnicity: String,
    Height: f32,
    Weight: f32,
    DavDiabetes: f32,
    InsulinModality: String,
    NumSevHypo: String,
    HGMReadAvg: i16,
    EduCareGvrP: String,
    EduCareGvrPEdu: String,
    Date: String,
    TxGroup: String,
    SubStudyGrp: String,
    Measurements: Vec<GlucoseMeasure>,
}
