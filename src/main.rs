use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, io::Result, vec};

//Data-Struct of Summary:
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PatientSummary {
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

//type PatientSummary = HashMap<String, String>;

fn main() -> Result<()> {
    read_csv_file(get_file_paths().get(0).unwrap())?;

    Ok(())
}

fn read_csv_file(path: &String) -> Result<()> {
    let f = File::open(path)?;

    let reader = BufReader::new(f);

    let mut rdr = csv::Reader::from_reader(reader);
    let mut patient_list: Vec<PatientSummary> = vec![];
    for result in rdr.deserialize() {
        let mut record: PatientSummary = result.expect("a csv record");

        patient_list.push(record.clone());
        //let mut patient: PatientSummary = PatientSummary::default();
        //let mut patient_record = StringRecord::default();
        //patient = record.deserialize(Some(&patient_record))?;
    }
    println!("{:?}", patient_list);
    Ok(())
}

fn get_file_paths() -> Vec<String> {
    vec!["./DataTables/tblAPtSummary.csv".to_string()]
}
