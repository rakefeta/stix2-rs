use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use crate::types::List;
use crate::objects::Objects;

// TODO: Optional Common Properties - not yet included
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    #[serde(alias = "type", rename="type")] // type is a rust keyword
    pub _type: String,
    // pub spec_version: String,
    pub id: String,
    pub objects: List<Objects>
}
