use serde::{Deserialize, Serialize};

//Data-Struct of Summary:
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