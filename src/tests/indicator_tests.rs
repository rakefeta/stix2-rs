use crate::indicator::Indicator;
use serde_json;

// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn indicator_deserialization() {

    let json_str = r#"
        {
            "type": "indicator",
            "spec_version": "2.1",
            "id": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "indicator_types": ["malicious-activity"],
            "name": "Poison Ivy Malware",
            "description": "This file is part of Poison Ivy",
            "pattern": "[ file:hashes.'SHA-256' = '4bac27393bdd9777ce02453256c5577cd02275510b2227f473d03f533924f877' ]",
            "pattern_type": "stix",
            "valid_from": "2016-01-01T00:00:00Z"
        }"#;

    let _parsed_indicator: Indicator = serde_json::from_str(&json_str).unwrap();
}

#[test]
fn indicator_deserialization_without_optional_field() {

    let json_str: &str = r#"
        {
            "type": "indicator",
            "spec_version": "2.1",
            "id": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "indicator_types": ["malicious-activity"],
            "name": "Poison Ivy Malware",
            "description": "This file is part of Poison Ivy",
            "pattern": "[ file:hashes.'SHA-256' = '4bac27393bdd9777ce02453256c5577cd02275510b2227f473d03f533924f877' ]",
            "pattern_type": "stix",
            "valid_from": "2016-01-01T00:00:00Z"
        }"#;

    let _parsed_indicator: Indicator = serde_json::from_str(&json_str).unwrap();
}

#[test]
#[should_panic]
fn indicator_deserialization_without_required_field() {
    
    // "pattern_type" required field is missing
    let json_str: &str = r#"
        {
            "type": "indicator",
            "spec_version": "2.1",
            "id": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "indicator_types": ["malicious-activity"],
            "name": "Poison Ivy Malware",
            "description": "This file is part of Poison Ivy",
            "pattern": "[ file:hashes.'SHA-256' = '4bac27393bdd9777ce02453256c5577cd02275510b2227f473d03f533924f877' ]",
            "valid_from": "2016-01-01T00:00:00Z"
        }"#;

    let _parsed_indicator: Indicator = serde_json::from_str(&json_str).unwrap();
}


#[test]
fn indicator_deserialization_with_optional_common_properies_field() {
    
    // "pattern_type" required field is missing
    let json_str: &str = r#"
        {
            "type": "indicator",
            "spec_version": "2.1",
            "id": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "indicator_types": ["malicious-activity"],
            "name": "Poison Ivy Malware",
            "description": "This file is part of Poison Ivy",
            "pattern": "[ file:hashes.'SHA-256' = '4bac27393bdd9777ce02453256c5577cd02275510b2227f473d03f533924f877' ]",
            "valid_from": "2016-01-01T00:00:00Z",
            "lang": "English",
            "pattern_type": "stix"
        }"#;

    let _parsed_indicator: Indicator = serde_json::from_str(&json_str).unwrap();
}