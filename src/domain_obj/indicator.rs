// #[cfg(test)]
// #[path = "../../tests/indicator_tests.rs"]
// mod indicator_tests;

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::types::{Identifier, List, KillChainPhase};
use crate::open_vocab::{IndicatorTypeOv, PatternTypeOv};
use crate::opt_com_props::OptComProps;

use crate::serialization_functions::{
    serialize_ndt, deserialize_ndt, serialize_ondt, deserialize_ondt
};


// TODO: Optional Common Properties - not yet included
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Indicator {
    #[serde(alias = "type", rename="type")] // type is a rust keyword
    pub _type: String,
    pub spec_version: String,
    pub id: Identifier,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub pattern: String,
    pub pattern_type: PatternTypeOv,
    pub pattern_version: Option<String>,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub valid_from: NaiveDateTime,
    pub name: Option<String>,
    pub description: Option<String>,
    pub indicator_types: Option<List<IndicatorTypeOv>>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    // Default needed, since custom (de)serialization functions diregard the field's optionality
    pub valid_until: Option<NaiveDateTime>,
    pub flow_name: Option<String>,
    pub model_id: Option<String>,
    pub kill_chain_phases: Option<List<KillChainPhase>>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn indicator_deserialization() {

        let json_str: &str = r#"
             {
                 "type": "indicator",
                 "spec_version": "2.1",
                 "id": "indicator--12345678-1234-1234-1234-123456789012",
                 "created": "2024-07-29T12:34:56.123Z",
                 "modified": "2024-07-29T12:34:56.123Z",
                 "name": "Malicious IP Address",
                 "description": "Indicator of compromise for a known malicious IP address used in ransomware attacks.",
                 "indicator_types": ["malicious-activity"],
                 "pattern": "[ipv4-addr:value = '203.0.113.5']",
                 "pattern_type": "stix",
                 "pattern_version": "2.1",
                 "valid_from": "2024-07-29T12:34:56.123Z",
                 "valid_until": "2024-08-29T12:34:56.123Z",
                 "kill_chain_phases": [
                     {
                         "kill_chain_name": "mitre-attack",
                         "phase_name": "command-and-control"
                     }
                 ],
                 "labels": ["ransomware", "malicious-ip"],
                 "confidence": 90,
                 "lang": "en",
                 "external_references": [
                     {
                         "source_name": "Threat Intelligence Platform",
                         "description": "Details about the malicious IP address.",
                         "url": "https://example.com/threat-intel/malicious-ip",
                         "external_id": "TIP-123456"
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
                 "validity_id": "validity--12345678-1234-1234-1234-123456789012",
                 "detection": {
                     "detection_method": "Automated analysis",
                     "description": "Detected through automated network traffic analysis.",
                     "last_detection_time": "2024-07-29T12:34:56.123Z",
                     "frequency": "hourly"
                 },
                 "extensions": {
                     "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                         "extension_type": "new-sdo",
                         "name": "Extended Indicator",
                         "description": "Extension for capturing additional information specific to the indicator.",
                         "data": {
                             "related_malware": ["RansomwareXYZ"],
                             "threat_actor": ["APT Group 123"],
                             "attack_techniques": ["Phishing", "Exploit Public-Facing Application"]
                         }
                     }
                 }
             }
        "#;

        let _parsed_json: Indicator = serde_json::from_str(&json_str).unwrap();
    }
}
