use serde_json;

use crate::course_of_action::CourseOfAction;
// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn course_of_action_deserialization() {

    let json_str = r#"
        {
            "type": "course-of-action",
            "spec_version": "2.1",
            "id": "course-of-action--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "name": "Add TCP port 80 Filter Rule to the existing Block UDP 1434 Filter",
            "description": "This is how to add a filter rule to block inbound access to TCP port 80 to the existing UDP 1434 filter ..."
        }"#;

    let _parsed_json: CourseOfAction = serde_json::from_str(&json_str).unwrap();
}
