use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::opt_com_props::OptComProps;
use crate::serialization_functions::{
    serialize_ndt, deserialize_ndt
};


// TODO: Optional Common Properties - not yet included
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseOfAction {
    #[serde(alias = "type", rename="type")] // type is a rust keyword
    pub _type: String,
    pub name: String,
    pub spec_version: String,
    pub id: String,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn course_of_action_deserialization() {

        let json_str = r#"
             {
                 "type": "course-of-action",
                 "spec_version": "2.1",
                 "id": "course-of-action--12345678-1234-1234-1234-123456789012",
                 "created": "2024-07-29T12:34:56.123Z",
                 "modified": "2024-07-29T12:34:56.123Z",
                 "name": "Phishing Mitigation",
                 "description": "Implement measures to reduce the risk of phishing attacks.",
                 "action": "Deploy email filtering solutions and conduct regular user training on phishing awareness.",
                 "labels": ["mitigation", "phishing"],
                 "external_references": [
                     {
                         "source_name": "NIST SP 800-53",
                         "description": "NIST guidelines for phishing mitigation.",
                         "url": "https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf",
                         "external_id": "SI-8"
                     },
                     {
                         "source_name": "Cyber Threat Intel",
                         "description": "Recommendations on phishing mitigation strategies.",
                         "url": "https://example.com/cyber-threat-intel/phishing-mitigation",
                         "external_id": "CTI-789012"
                     }
                 ],
                 "object_marking_refs": [
                     "marking-definition--abcdef12-1234-5678-abcd-1234567890ab"
                 ],
                 "granular_markings": [
                     {
                         "marking_ref": "marking-definition--abcdef12-1234-5678-abcd-1234567890ab",
                         "selectors": ["description", "external_references[0].url"]
                     }
                 ],
                 "created_by_ref": "identity--abcd1234-abcd-1234-abcd-1234abcd5678",
                 "impact": "Reduces the likelihood of successful phishing attacks and minimizes the potential for credential compromise.",
                 "objective": "Increase the resilience of the organization against phishing attempts.",
                 "cost": "Medium",
                 "efficacy": "High",
                 "action_refs": [
                     "x-mitre-data-source--abcd1234-abcd-1234-abcd-1234abcd5678"
                 ],
                 "extensions": {
                     "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                         "extension_type": "new-sdo",
                         "name": "Extended Course of Action",
                         "description": "Extension for capturing additional information specific to the course of action.",
                         "data": {
                             "detailed_steps": [
                                 "Deploy a secure email gateway to filter out phishing emails.",
                                 "Conduct phishing simulation exercises regularly.",
                                 "Provide ongoing training to employees about identifying phishing emails."
                             ],
                             "tools": [
                                 "Secure Email Gateway",
                                 "Phishing Simulation Platform"
                             ]
                         }
                     }
                 }
             }
        "#;

        let _parsed_json: CourseOfAction = serde_json::from_str(&json_str).unwrap();
    }
}
