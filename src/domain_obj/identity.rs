use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::types::{Identifier, List};
use crate::opt_com_props::OptComProps;
use crate::serialization_functions::{
    serialize_ndt, deserialize_ndt
};
use crate::open_vocab::OpenVocab;


// TODO: Optional Common Properties - not yet included
// NOTE: TRA will serialize only output data. The rest of the ingress data that is needed only to be read,
//       TRA will only Deserialize them.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    pub spec_version: String,
    pub id: Identifier,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    pub roles: Option<List<String>>,
    #[serde(flatten)]
    pub identity_class: Option<OpenVocab>,
    #[serde(flatten)]
    pub sectors: Option<List<OpenVocab>>,
    pub contact_information : Option<String>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_deserialization() {

        let json_str = r#"
             {
                 "type": "identity",
                 "spec_version": "2.1",
                 "id": "identity--12345678-1234-1234-1234-123456789012",
                 "created": "2024-07-29T12:34:56.123Z",
                 "modified": "2024-07-29T12:34:56.123Z",
                 "name": "John Doe",
                 "description": "Chief Information Security Officer at Example Corp.",
                 "roles": ["CISO"],
                 "identity_class": "individual",
                 "sectors": ["technology", "finance"],
                 "contact_information": "johndoe@example.com, +1-555-555-5555",
                 "labels": ["executive", "security"],
                 "external_references": [
                     {
                         "source_name": "LinkedIn",
                         "description": "John Doe's LinkedIn profile",
                         "url": "https://www.linkedin.com/in/johndoe",
                         "external_id": "john-doe-linkedin"
                     }
                 ],
                 "object_marking_refs": [
                     "marking-definition--abcdef12-1234-5678-abcd-1234567890ab"
                 ],
                 "granular_markings": [
                     {
                         "marking_ref": "marking-definition--abcdef12-1234-5678-abcd-1234567890ab",
                         "selectors": ["description", "contact_information"]
                     }
                 ],
                 "created_by_ref": "identity--abcd1234-abcd-1234-abcd-1234abcd5678",
                 "extensions": {
                     "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                         "extension_type": "new-sdo",
                         "name": "Extended Identity",
                         "description": "Extension for capturing additional information specific to the identity.",
                         "data": {
                             "employment_history": [
                                 {
                                     "company": "Example Corp.",
                                     "position": "CISO",
                                     "start_date": "2018-01-01",
                                     "end_date": "present"
                                 },
                                 {
                                     "company": "Tech Solutions",
                                     "position": "Security Analyst",
                                     "start_date": "2015-06-01",
                                     "end_date": "2017-12-31"
                                 }
                             ],
                             "education": [
                                 {
                                     "institution": "University of Example",
                                     "degree": "M.S. in Information Security",
                                     "graduation_year": 2014
                                 }
                             ],
                             "certifications": [
                                 "CISSP",
                                 "CISM"
                             ]
                         }
                     }
                 }
             }
        "#;

        let _parsed_json: Identity = serde_json::from_str(&json_str).unwrap();
    }
}
