use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::NaiveDateTime;
use crate::types::Float;


// ==============================================================================================
// STIX2 NaiveDateTime format serializer
pub fn serialize_ndt<S> (datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
S: Serializer,
{
    format!("{}", datetime).serialize(serializer)
}

pub fn serialize_ondt<S> (datetime: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
S: Serializer,
{
    format!("{}", datetime.unwrap()).serialize(serializer)
}

// TODO: To change it to DateTime::parse_from_rfc3339("2024-04-04T20:33:29.1Z").unwrap().with_timezone(&Utc),
//       for easier DateTime operation handling
// Decimal fraction of a second. Consumes the leading dot and needs at least one decimal
// digit after the dot. If not results in an error. Validates: 2.16.1 Requirements
const STIX2_DTFORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.fZ";
pub fn deserialize_ndt<'de, D>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, STIX2_DTFORMAT)
        .map_err(serde::de::Error::custom)?;
    Ok(dt)
}

pub fn deserialize_ondt<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, STIX2_DTFORMAT)
        .map_err(serde::de::Error::custom)?;
    Ok(Some(dt))
}

// // ==============================================================================================
// // Deserialize STIX2 fields that collide with rust language keywords
// pub fn deserialize_type<'de, D>(
//     deserializer: D,
// ) -> Result<NaiveDateTime, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     let dt = NaiveDateTime::parse_from_str(&s, STIX2_DTFORMAT)
//         .map_err(serde::de::Error::custom)?;
//     Ok(dt)
// }

// ==============================================================================================
// Deserialize STIX2 fields with arrays of numbers
// TODO: 1) The current version covers only the STIX2 Float type and not the Integer type.
//       If needed (in the future) this can be mitigated/refactored by using json::number::Number
//       or an Enum containing bot, Integer and Float.
//       2) Another thing to mitigate is the case of an empty list "[]" which can be created by
//       python generated json structures.
//       3) Make it from Vec to List
pub fn deserialize_vector<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<Float>>, D::Error>
where
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    if s.len() > 3 {
        let str_items: Vec<&str> = s.trim()
                                    .trim_matches(|p| p == '[' || p == ']')
                                    .split(",")
                                    .collect();
        let i32_items: Vec<Float> = str_items.iter().map(|x| x.trim().parse::<Float>().unwrap()).collect();
        return Ok(Some(i32_items))
    } else {
        Ok(None)
    }
    
}

// ==============================================================================================

