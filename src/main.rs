use sample_data::model::{GlucoseMeasure, GlucoseMeasureEntry, Patient, PatientSummary};
use std::{fs::create_dir_all, fs::write, fs::File, io::BufReader, io::Result, time::Instant, vec};

fn main() -> Result<()> {
    let time_stamp = Instant::now();
    println!("starting...");
    create_dir_all("./OutputJson")?;
    filter_data()?;
    println!(
        "script took {}min",
        time_stamp.elapsed().as_secs_f32() / 60.0
    );
    Ok(())
}

fn read_summary_csv_file(path: &String) -> Result<Vec<PatientSummary>> {
    let f = File::open(path)?;

    let reader = BufReader::new(f);

    let mut rdr = csv::Reader::from_reader(reader);
    let mut patient_list: Vec<PatientSummary> = vec![];
    for result in rdr.deserialize() {
        let record: PatientSummary = result.expect("a csv record");

        patient_list.push(record.clone());
    }
    println!("{:?}", patient_list.len());
    Ok(patient_list)
}

fn read_rtcgm_data() -> Result<Vec<GlucoseMeasureEntry>> {
    let mut measure_list: Vec<GlucoseMeasureEntry> = vec![];
    for i in 1..get_file_paths().len() {
        let f = File::open(get_file_paths().get(i).unwrap())?;

        let reader = BufReader::new(f);

        let mut rdr = csv::Reader::from_reader(reader);

        for result in rdr.deserialize() {
            let record: GlucoseMeasureEntry = result.expect("a csv record");

            measure_list.push(record.clone());
        }
    }

    Ok(measure_list)
}

fn filter_data() -> Result<()> {
    let time_stamp = Instant::now();
    let mut measurements: Vec<GlucoseMeasure> = vec![];

    let iter = read_rtcgm_data()?;

    for patient_summary in read_summary_csv_file(get_file_paths().get(0).unwrap())? {
        for glucose_measure_entry in &iter {
            if glucose_measure_entry.PtID == patient_summary.PtID {
                let glucose_measure = GlucoseMeasure {
                    deviceDtTm: glucose_measure_entry.DeviceDtTm.to_owned(),
                    glucose: glucose_measure_entry.Glucose,
                };
                measurements.push(glucose_measure);
            }
        }
        //patient definition and json output
        let patient = Patient {
            ptID: patient_summary.PtID,
            gender: patient_summary.Gender,
            age: patient_summary.AgeAsOfRandDt,
            race: patient_summary.Race,
            ethnicity: patient_summary.Ethnicity,
            height: patient_summary.Height,
            weight: patient_summary.Weight,
            davDiabetes: patient_summary.DavDiabetes,
            insulinModality: patient_summary.InsulinModality,
            numSevHypo: patient_summary.NumSevHypo,
            hgmReadAvg: match patient_summary.HGMReadAvg {
                Some(v) => v,
                None => -1,
            },
            eduCareGvrP: patient_summary.EduCareGvrP,
            eduCareGvrPEdu: patient_summary.EduCareGvrPEdu,
            date: patient_summary.RandDt,
            txGroup: patient_summary.TxGroup,
            subStudyGrp: patient_summary.SubStudyGrp,
            measurements: measurements,
        };
        write_json(patient.clone(), patient.ptID)?;
        println!(
            "wrote patient into file: ./OutputJson/patient{:?}.json",
            patient.ptID
        );
        measurements = vec![];
        println!(
            "script mean time: {}min\n",
            time_stamp.elapsed().as_secs_f32() / 60.0
        );
    }

    Ok(())
}

fn write_json(patient: Patient, pat_id: u32) -> Result<()> {
    let patient_json = serde_json::to_string_pretty(&patient).expect("patient serialize failed!");

    write(format!("./OutputJson/patient{pat_id}.json"), patient_json)?;
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
