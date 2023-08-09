use sample_data::model::PatientSummary;
use std::{fs::File, io::BufReader, io::Result, vec};

fn main() -> Result<()> {
    read_summary_csv_file(get_file_paths().get(0).unwrap())?;

    Ok(())
}

fn read_summary_csv_file(path: &String) -> Result<()> {
    let f = File::open(path)?;

    let reader = BufReader::new(f);

    let mut rdr = csv::Reader::from_reader(reader);
    let mut patient_list: Vec<PatientSummary> = vec![];
    for result in rdr.deserialize() {
        let record: PatientSummary = result.expect("a csv record");

        patient_list.push(record.clone());
    }
    println!("{:?}", patient_list.len());
    Ok(())
}

fn get_file_paths() -> Vec<String> {
    vec![
        "./DataTables/tblAPtSummary.csv".to_string(),
        "./DataTables/tblADataRTCGM_Blind_Baseline.csv".to_string(),
        "./DataTables/tblADataRTCGM_Blind_ControlGroup.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_ControlGroup_1.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_ControlGroup_2.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_ControlGroup_3.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_ControlGroup_4.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_ControlGroup_5.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_1.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_2.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_3.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_4.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_5.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_6.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_7.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_8.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_9.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_10.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_11.csv".to_string(),
        "./DataTables/tblADataRTCGM_Unblinded_RTCGMGroup_12.csv".to_string(),
    ]
}
