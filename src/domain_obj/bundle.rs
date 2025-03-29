use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use crate::types::List;
use crate::objects::Objects;

// TODO: Optional Common Properties - not yet included
// NOTE: TRA will serialize only output data. The rest of the ingress data that is needed only to be read,
//       TRA will only Deserialize them.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    // pub spec_version: String,
    pub id: String,
    pub objects: List<Objects>
}
