use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type Identifier = String;
pub type List<T> = Vec<T>;
pub type Hash = HashMap<String, String>;
// pub type OpenVocab = _OpenVocab; // TODO
pub type Timestamp = String; // TODO
pub type Float = f64;
pub type Integer = i64;
pub type Binary = String;
pub type Boolean = bool;
// TODO: Validation example: A dictionary captures an arbitrary set of key/value pairs. Dictionary keys MUST
// be unique in each dictionary, MUST be in ASCII, and are limited to the characters a-z (lowercase ASCII),
// A-Z (uppercase ASCII), numerals 0-9, hyphen (-), and underscore (_). Dictionary keys MUST be no longer 
// than 250 ASCII characters in length and SHOULD be lowercase.A dictionary captures an arbitrary set of
// key/value pairs. Dictionary keys MUST be unique in each dictionary, MUST be in ASCII, and are limited to
// the characters a-z (lowercase ASCII), A-Z (uppercase ASCII), numerals 0-9, hyphen (-), and underscore (_).
// Dictionary keys MUST be no longer than 250 ASCII characters in length and SHOULD be lowercase.
// Empty dictionaries are prohibited in STIX and MUST NOT be used as a substitute for omitting the property
// if it is optional. If the property is required, the dictionary MUST be present and MUST have at least one
// key-value pair. dictionary values MUST be valid property base types.
pub type Dictionary = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hashes {
    pub kill_chain_name: String,
    pub phase_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KillChainPhase {
    kill_chain_name: String,
    phase_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalReference {
    source_name: String,
    description: Option<String>,
    url: Option<String>,
    hashes: Option<Hashes>,
    external_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GranularMarking {
    lang: Option<String>,
    marking_ref: Option<Identifier>,
    selectors: List<String>
}



    