use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::
{
    types::{Boolean, Identifier, Integer, List},
    opt_com_props::OptComProps,
    serialization_functions::{
    serialize_ndt, deserialize_ndt, serialize_ondt, deserialize_ondt
    }
};


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    pub spec_version: String,
    pub id: Identifier,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub relationship_type: String,
    pub description: Option<String>,
    pub source_ref: Identifier,
    pub target_ref: Identifier,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub start_time: Option<NaiveDateTime>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub stop_time: Option<NaiveDateTime>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Sighting {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    pub spec_version: String,
    pub id: Identifier,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub description: Option<String>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub first_seen: Option<NaiveDateTime>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub laast_seen: Option<NaiveDateTime>,
    pub count: Option<Integer>,
    pub sighting_of_ref: Option<Identifier>,
    pub observed_data_refs: Option<List<Identifier>>,
    pub where_sighted_refs: Option<List<Identifier>>,
    pub summary: Option<Boolean>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relationship_deserialization() {

        let json_str: &str = r#"
            {
                "type": "relationship",
                "spec_version": "2.1",
                "id": "relationship--12345678-1234-1234-1234-123456789012",
                "created": "2024-07-29T12:34:56.123Z",
                "modified": "2024-07-29T12:34:56.123Z",
                "relationship_type": "uses",
                "description": "APT Group 123 uses RansomwareXYZ to conduct attacks.",
                "source_ref": "threat-actor--12345678-1234-1234-1234-123456789012",
                "target_ref": "malware--12345678-1234-1234-1234-123456789012",
                "start_time": "2024-06-01T12:34:56.123Z",
                "stop_time": "2024-07-29T12:34:56.123Z",
                "labels": ["association", "malicious"],
                "confidence": 85,
                "lang": "en",
                "external_references": [
                    {
                        "source_name": "Threat Intel Report",
                        "description": "Report detailing the use of RansomwareXYZ by APT Group 123.",
                        "url": "https://example.com/threat-intel-report",
                        "external_id": "TIR-456789"
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
                "extensions": {
                    "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                        "extension_type": "new-sro",
                        "name": "Extended Relationship",
                        "description": "Extension for capturing additional information specific to the relationship.",
                        "data": {
                            "related_campaigns": [
                                "campaign--abcd1234-abcd-1234-abcd-1234abcd5678"
                            ],
                            "impact": "Facilitates ransomware attacks leading to data encryption and ransom demands."
                        }
                    }
                }
            }
        "#;

        let _parsed_json: Relationship = serde_json::from_str(&json_str).unwrap();
    }

    #[test]
    fn sighting_deserialization() {

        let json_str: &str = r#"
            {
                "type": "sighting",
                "spec_version": "2.1",
                "id": "sighting--12345678-1234-1234-1234-123456789012",
                "created": "2024-07-29T12:34:56.123Z",
                "modified": "2024-07-29T12:34:56.123Z",
                "first_seen": "2024-07-01T12:34:56.123Z",
                "last_seen": "2024-07-29T12:34:56.123Z",
                "count": 5,
                "sighting_of_ref": "indicator--12345678-1234-1234-1234-123456789012",
                "observed_data_refs": [
                    "observed-data--abcd1234-abcd-1234-abcd-1234abcd5678"
                ],
                "where_sighted_refs": [
                    "identity--abcd1234-abcd-1234-abcd-1234abcd5678"
                ],
                "summary": false,
                "description": "Multiple sightings of the malicious IP address associated with RansomwareXYZ.",
                "labels": ["sighting", "malicious-activity"],
                "confidence": 80,
                "lang": "en",
                "external_references": [
                    {
                        "source_name": "Threat Intel Report",
                        "description": "Report detailing sightings of the malicious IP address.",
                        "url": "https://example.com/threat-intel-report",
                        "external_id": "TIR-123456"
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
                "extensions": {
                    "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                        "extension_type": "new-sro",
                        "name": "Extended Sighting",
                        "description": "Extension for capturing additional information specific to the sighting.",
                        "data": {
                            "related_indicators": [
                                "indicator--abcd1234-abcd-1234-abcd-1234abcd5678"
                            ],
                            "related_observed_data": [
                                "observed-data--abcd1234-abcd-1234-abcd-1234abcd5678"
                            ],
                            "impact": "Indicates ongoing malicious activity related to the indicator."
                        }
                    }
                }
            }

        "#;

        let _parsed_json: Sighting = serde_json::from_str(&json_str).unwrap();
    }
}
