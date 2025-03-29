use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
// use serde_json::Result;
use chrono::NaiveDateTime;

use crate::
{
    types::{Identifier, List},
    open_vocab::{ThreatActorTypeOv, ThreatActorRoleOv, ThreatActorSophisticationOv,
    AttackResourceLevelOv, AttackMotivationOv},
    opt_com_props::OptComProps,
    serialization_functions::{
        serialize_ndt, deserialize_ndt, serialize_ondt, deserialize_ondt
    },
};


// TODO: Optional Common Properties - not yet included
// NOTE: TRA will serialize only output data. The rest of the ingress data that is needed only to be read,
//       TRA will only Deserialize them.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreatActor {
    #[serde(alias = "type")] // type is a rust keyword
    _type: String,
    pub spec_version: String,
    pub id: String,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "serialize_ndt", deserialize_with = "deserialize_ndt")]
    pub modified: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub threat_actor_types: Option<List<ThreatActorTypeOv>>,
    pub aliases: Option<List<String>>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub first_seen: Option<NaiveDateTime>,
    #[serde(default, serialize_with = "serialize_ondt", deserialize_with = "deserialize_ondt")]
    pub last_seen: Option<NaiveDateTime>,
    pub roles: Option<List<ThreatActorRoleOv>>,
    pub goals: Option<List<String>>,
    pub sophistication: Option<ThreatActorSophisticationOv>,
    pub resource_level: Option<AttackResourceLevelOv>,
    pub primary_motivation: Option<AttackMotivationOv>,
    pub secondary_motivations: Option<List<AttackMotivationOv>>,
    pub personal_motivations: Option<List<AttackMotivationOv>>,
    pub sample_refs: Option<List<Identifier>>,
    #[serde(flatten)]
    pub opt_com_props: Option<OptComProps>
}
