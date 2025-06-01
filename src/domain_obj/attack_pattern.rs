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
pub struct AttackPattern {
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

