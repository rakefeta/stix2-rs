use crate::relationship::{Relationship, Sighting};
use serde_json;

// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn relationship_deserialization() {

    let json_str = r#"
        {
            "type": "relationship",
            "spec_version": "2.1",
            "id": "relationship--44298a74-ba52-4f0c-87a3-1824e67d7fad",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:07:10.000Z",
            "modified": "2016-04-06T20:07:10.000Z",
            "relationship_type": "mitigates",
            "source_ref": "course-of-action--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "target_ref": "malware--31b940d4-6f7f-459a-80ea-9c1f17b5891b"
        }"#;

    let _parsed_json: Relationship = serde_json::from_str(&json_str).unwrap();
}


#[test]
fn sighting_deserialization() {

    let json_str = r#"
        {
            "type": "sighting",
            "spec_version": "2.1",
            "id": "sighting--ee20065d-2555-424f-ad9e-0f8428623c75",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:08:31.000Z",
            "modified": "2016-04-06T20:08:31.000Z",
            "first_seen": "2015-12-21T19:00:00Z",
            "last_seen": "2015-12-21T19:00:00Z",
            "count": 50,
            "sighting_of_ref": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "observed_data_refs": ["observed-data--b67d30ff-02ac-498a-92f9-32f845f448cf"],
            "where_sighted_refs": ["identity--b67d30ff-02ac-498a-92f9-32f845f448ff"]
        }"#;

    let _parsed_json: Sighting = serde_json::from_str(&json_str).unwrap();
}

