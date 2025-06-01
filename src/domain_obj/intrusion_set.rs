use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::types::{Identifier, List};
use crate::opt_com_props::OptComProps;
use crate::serialization_functions::{
    serialize_ndt, deserialize_ndt, serialize_ondt, deserialize_ondt
};
use crate::open_vocab::OpenVocab;


// TODO: Optional Common Properties - not yet included
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct IntrusionSet {
    #[serde(alias = "type", rename="type")] // type is a rust keyword
    pub _type: String,
    spec_version: String,
    id: Identifier,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    modified: NaiveDateTime,
    name: String,
    description: Option<String>,
    aliases: Option<List<String>>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    first_seen: Option<NaiveDateTime>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    last_seen: Option<NaiveDateTime>,
    goals: Option<List<String>>,
    #[serde(flatten)]
    resource_level: Option<OpenVocab>,
    #[serde(flatten)]
    primary_motivation: Option<OpenVocab>,
    #[serde(flatten)]
    secondary_motivations: Option<List<OpenVocab>>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intrusion_set_deserialization() {

        let json_str = r#"
        {
            "type": "intrusion-set",
            "spec_version": "2.1",
            "id": "intrusion-set--ed69450a-f067-4b51-9ba2-c4616b9a6713",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "APT BPP",
            "description": "An advanced persistent threat that seeks to disrupt Branistan's election with multiple attacks.",
            "aliases": [
                "Bran-teaser"
            ],
            "first_seen": "2016-01-08T12:50:40.123Z",
            "goals": [
                "Influence the Branistan election",
                "Disrupt the BPP"
            ],
            "resource_level": "government",
            "primary_motivation": "ideology",
            "secondary_motivations": [
                "dominance"
            ]
        }
        "#;

        let _parsed_json: IntrusionSet = serde_json::from_str(&json_str).unwrap();
    }
}
