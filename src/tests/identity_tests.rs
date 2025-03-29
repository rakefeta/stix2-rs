use crate::identity::Identity;
use serde_json;

// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn identity_deserialization() {

    let json_str = r#"
        {
            "type": "identity",
            "spec_version": "2.1",
            "id": "identity--e5f1b90a-d9b6-40ab-81a9-8a29df4b6b65",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:00.000Z",
            "modified": "2016-04-06T20:03:00.000Z",
            "name": "ACME Widget, Inc.",
            "identity_class": "organization",
            "sectors": ["agriculture","aerospace","automotive"]
        }"#;

    let _parsed_indicator: Identity = serde_json::from_str(&json_str).unwrap();
}

// #[test]
// fn identity_deserialization_without_optional_field() {

//     let json_str: &str = r#"
//         {

//         }"#;

//     let _parsed_identity: Identity = serde_json::from_str(&json_str).unwrap();
// }

// #[test]
// #[should_panic]
// fn identity_deserialization_without_required_field() {
    
//     // "pattern_type" required field is missing
//     let json_str: &str = r#"
//         {

//         }"#;

//     let _parsed_identity: Identity = serde_json::from_str(&json_str).unwrap();
// }


// #[test]
// fn identity_deserialization_with_optional_common_properies_field() {
    
//     // "pattern_type" required field is missing
//     let json_str: &str = r#"
//         {

//         }"#;

//     let _parsed_identity: Identity = serde_json::from_str(&json_str).unwrap();
// }