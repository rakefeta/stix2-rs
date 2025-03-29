use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use crate::types::{Boolean, Dictionary, ExternalReference, GranularMarking, Identifier, Integer, List};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptComProps {
    pub created_by_ref: Option<Identifier>,
    pub revoked: Option<Boolean>,
    pub labels: Option<List<String>>,
    pub confidence: Option<Integer>,
    pub lang: Option<String>,
    pub external_references: Option<List<ExternalReference>>,
    pub object_marking_refs: Option<List<Identifier>>,
    pub granular_markings: Option<List<GranularMarking>>,
    pub defanged: Option<Boolean>,
    pub extensions: Option<Dictionary>
}
