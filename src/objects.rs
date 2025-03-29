use serde::{Serialize, Deserialize, Deserializer};
use serde_with::skip_serializing_none;

use crate::domain_obj::{attack_pattern::AttackPattern, course_of_action::CourseOfAction, identity::Identity, indicator::Indicator,
    intrusion_set::IntrusionSet, malware::Malware, threat_actor::ThreatActor, campaign::Campaign};
use crate::relationship::{Relationship, Sighting};


/// This enum serves the purpose of easing (de)serialization of the `Bundle` stix2 object. It allows the
/// creation of lists of heterogeneous/different stix2 obejct types.
#[derive(Debug, Serialize)]
#[skip_serializing_none]
#[serde(untagged)]
pub enum Objects {
    Indicator(Indicator),
    AttackPattern(AttackPattern),
    Campaign(Campaign),
    CourseOfAction(CourseOfAction),
    Identity(Identity),
    IntrusionSet(IntrusionSet),
    Malware(Malware),
    ThreatActor(ThreatActor),
    Relationship(Relationship),
    Sighting(Sighting)
}


/// This (de)serializer is implemented to mitigate the fact that different object types can have the
/// exact same signature in some json cases and the (de)serializer will chose the first type the type system
/// dispatches. To avoid this, before (de)serializing, the value of the `type` field is checked.
impl<'de> Deserialize<'de> for Objects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json_value = serde_json::Value::deserialize(deserializer)?;

        if let Some(_type) = json_value.get("type").and_then(|v| v.as_str()) {
            match _type {
                "attack-pattern" => {
                    let attack_pattern = AttackPattern::deserialize(json_value)
                        .map(Objects::AttackPattern)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(attack_pattern);
                },
                "course-of-action" => {
                    let course_of_action = CourseOfAction::deserialize(json_value)
                        .map(Objects::CourseOfAction)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(course_of_action);
                },
                "campaign" => {
                    let campaign = Campaign::deserialize(json_value)
                        .map(Objects::Campaign)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(campaign);
                },
                "identity" => {
                    let identity = Identity::deserialize(json_value)
                        .map(Objects::Identity)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(identity);
                },
                "indicator" => {
                    let indicator = Indicator::deserialize(json_value)
                        .map(Objects::Indicator)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(indicator);
                },
                "intrusion-set" => {
                    let intrusion_set = IntrusionSet::deserialize(json_value)
                        .map(Objects::IntrusionSet)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(intrusion_set);
                },
                "malware" => {
                    let malware = Malware::deserialize(json_value)
                        .map(Objects::Malware)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(malware);
                },
                "threat-actor" => {
                    let threat_actor = ThreatActor::deserialize(json_value)
                        .map(Objects::ThreatActor)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(threat_actor);
                },
                "relationship" => {
                    let relationship = Relationship::deserialize(json_value)
                        .map(Objects::Relationship)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(relationship);
                },
                "sighting" => {
                    let sighting = Sighting::deserialize(json_value)
                        .map(Objects::Sighting)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(sighting);
                },
                _ => panic!(),
            };
        }
        let course_of_action = CourseOfAction::deserialize(json_value)
            .map(Objects::CourseOfAction)
            .map_err(serde::de::Error::custom)?;
        return Ok(course_of_action);
    }
}
