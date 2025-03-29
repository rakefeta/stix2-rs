use serde_json;

use crate::domain_obj::attack_pattern::AttackPattern;
// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn attack_pattern_deserialization() {

    let json_str = r#"
        {
            "type": "attack-pattern",
            "spec_version": "2.1",
            "id": "attack-pattern--7e33a43e-e34b-40ec-89da-36c9bb2cacd5",
            "created": "2016-05-12T08:17:27.000Z",
            "modified": "2016-05-12T08:17:27.000Z",
            "name": "Spear Phishing as Practiced by Adversary X",
            "description": "A particular form of spear phishing where the attacker claims that the target had won a contest, including personal details, to get them to click on a link.",
            "external_references": [
                {
                    "source_name": "capec",
                    "external_id": "CAPEC-163"
                }
            ]
        }"#;

    let _parsed_json: AttackPattern = serde_json::from_str(&json_str).unwrap();
}

#[test]
fn attack_pattern_deserialization_without_optional_field() {

    let json_str: &str = r#"
        {
            "type": "attack-pattern",
            "spec_version": "2.1",
            "id": "attack-pattern--7e33a43e-e34b-40ec-89da-36c9bb2cacd5",
            "created": "2016-05-12T08:17:27.000Z",
            "modified": "2016-05-12T08:17:27.000Z",
            "name": "Spear Phishing as Practiced by Adversary X",
            "description": "A particular form of spear phishing where the attacker claims that the target had won a contest, including personal details, to get them to click on a link.",
            "external_references": [
                {
                    "source_name": "capec",
                    "external_id": "CAPEC-163"
                }
            ]
        }"#;

    let _parsed_json: AttackPattern = serde_json::from_str(&json_str).unwrap();
}

#[test]
#[should_panic]
fn attack_pattern_deserialization_without_required_field() {
    
    // "name" required field is missing
    let json_str: &str = r#"
        {
            "type": "attack-pattern",
            "spec_version": "2.1",
            "id": "attack-pattern--7e33a43e-e34b-40ec-89da-36c9bb2cacd5",
            "created": "2016-05-12T08:17:27.000Z",
            "modified": "2016-05-12T08:17:27.000Z",
            "description": "A particular form of spear phishing where the attacker claims that the target had won a contest, including personal details, to get them to click on a link.",
            "external_references": [
                {
                    "source_name": "capec",
                    "external_id": "CAPEC-163"
                }
            ]
        }"#;

    let _parsed_json: AttackPattern = serde_json::from_str(&json_str).unwrap();
}