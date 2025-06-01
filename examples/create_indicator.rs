use chrono::NaiveDateTime;
use stix2_rs::domain_obj::indicator::Indicator;
use stix2_rs::open_vocab::{IndicatorTypeOv, PatternTypeOv};

fn main() {
    let created = NaiveDateTime::parse_from_str("2024-06-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let modified = created;
    let valid_from = created;

    let indicator = Indicator {
        _type: "indicator".to_string(),
        spec_version: "2.1".to_string(),
        id: "indicator-b7830700-aa77-46d1-ac9c-c93fdb454785".to_string(),
        created: created,
        modified: modified,
        pattern: "[file:hashes.'SHA-256' = 'abcdef1234567890']".to_string(),
        pattern_type: PatternTypeOv::stix,
        pattern_version: Some("2.1".to_string()),
        valid_from,
        name: Some("Malicious file hash".to_string()),
        description: Some("Detects a malicious file by its SHA-256 hash.".to_string()),
        indicator_types: Some(vec![IndicatorTypeOv::malicious_activity].into()),
        valid_until: None,
        flow_name: Some("anomaly-flow".to_string()),
        model_id: Some("model-123".to_string()),
        kill_chain_phases: None,
        opt_com_props: None,
    };

    let json = serde_json::to_string_pretty(&indicator).unwrap();
    println!("{}", json);
}



