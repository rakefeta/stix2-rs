use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;


use crate::types::List;
use crate::opt_com_props::OptComProps;
use crate::serialization_functions::{
    serialize_ndt, deserialize_ndt, serialize_ondt, deserialize_ondt
};


// TODO: Optional Common Properties - not yet included
// NOTE: TRA will serialize only output data. The rest of the ingress data that is needed only to be read,
//       TRA will only Deserialize them.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Campaign {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    pub name: String,
    pub spec_version: String,
    pub id: String,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub description: String,
    pub aliases: Option<List<String>>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub first_seen: Option<NaiveDateTime>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub last_seen: Option<NaiveDateTime>,
    pub objective: Option<String>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn campaign_deserialization() {

        let json_str = r#"
             {
      "type": "campaign",
      "spec_version": "2.1",
      "id": "campaign--e6f5c290-376b-4c96-b86f-cf765983d145",
      "created": "2024-08-21T14:25:00.000Z",
      "modified": "2024-08-21T14:25:00.000Z",
      "name": "Operation Sunset",
      "description": "A targeted campaign aimed at government organizations in Southeast Asia.",
      "aliases": [
        "Sunset Attacks",
        "Golden Eye"
      ],
      "first_seen": "2024-08-01T00:00:00.000Z",
      "last_seen": "2024-08-20T00:00:00.000Z",
      "objective": "Exfiltrate sensitive information from government agencies.",
      "labels": [
        "espionage",
        "targeted"
      ],
      "confidence": 80,
      "lang": "en",
      "external_references": [
        {
          "source_name": "Threat Report",
          "description": "Detailed analysis of Operation Sunset",
          "url": "https://example.com/threat-report",
          "external_id": "TR-2024-001"
        }
      ],
      "object_marking_refs": [
        "marking-definition--34098fce-860f-48ae-8e50-ebd3cc5e41da"
      ],
      "granular_markings": [
        {
          "marking_ref": "marking-definition--34098fce-860f-48ae-8e50-ebd3cc5e41da",
          "selectors": [
            "name",
            "description"
          ]
        }
      ]
    }
        "#;

        let _parsed_json: Campaign = serde_json::from_str(&json_str).unwrap();
    }


    #[test]
    fn campaign_deserialization2() {

        let json_str = r#"
        {
            "type": "campaign",
            "spec_version": "2.1",
            "id": "campaign--e5268b6e-4931-42f1-b379-87f48eb41b1e",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Operation Bran Flakes",
            "description": "A concerted effort to insert false information into the BPP's web pages.",
            "aliases": [
                "OBF"
            ],
            "first_seen": "2016-01-08T12:50:40.123Z",
            "objective": "Hack www.bpp.bn"
        }
        "#;

        let _parsed_json: Campaign = serde_json::from_str(&json_str).unwrap();
    }
}
