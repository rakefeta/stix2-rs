use crate::domain_obj::threat_actor::ThreatActor;

// Datetime format for STIX2 datetime (de)serialization
// const STIX2_DTFORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[test]
fn threat_actor_deserialization() {

    let json_str = r#"
        {
            "type": "threat-actor",
            "spec_version": "2.1",
            "id": "threat-actor--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
            "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
            "created": "2016-04-06T20:03:48.000Z",
            "modified": "2016-04-06T20:03:48.000Z",
            "threat_actor_types": [ "crime-syndicate"],
            "name": "Evil Org",
            "description": "The Evil Org threat actor group",
            "aliases": ["Syndicate 1", "Evil Syndicate 99"],
            "roles": ["director"],
            "goals": ["Steal bank money", "Steal credit cards"],
            "sophistication": "advanced",
            "resource_level": "team",
            "primary_motivation": "organizational-gain"
        }"#;

    let _parsed_indicator: ThreatActor = serde_json::from_str(&json_str).unwrap();

    // let sophistication = _parsed_indicator.sophistication.unwrap().score();

    // assert!(sophistication == 7);

    // let sc = _parsed_indicator.calculate_risk_score();

    // println!("{:?}", sc);

    assert!(true)
    
}


#[test]
fn threat_actor_deserialization2() {

    let json_str: &str = r#"
            {
                "type": "threat-actor",
                "spec_version": "2.1",
                "id": "threat-actor--12345678-1234-1234-1234-123456789012",
                "created": "2024-07-29T12:34:56.123Z",
                "modified": "2024-07-29T12:34:56.123Z",
                "name": "APT Group 123",
                "description": "APT Group 123 is a highly skilled and persistent threat actor group known for conducting sophisticated cyber espionage operations.",
                "aliases": ["Advanced Persistent Threat 123", "APT123"],
                "roles": ["agent", "malware-author"],
                "goals": [
                    "Exfiltrate sensitive information",
                    "Disrupt critical infrastructure"
                ],
                "sophistication": "advanced",
                "resource_level": "organization",
                "personal_motivations": ["notoriety"],
                "labels": ["espionage", "apt"],
                "first_seen": "2018-01-01T12:34:56.123Z",
                "last_seen": "2024-07-29T12:34:56.123Z",
                "primary_motivation": "ideology",
                "secondary_motivations": ["personal-gain", "personal-satisfaction"],
                "external_references": [
                    {
                        "source_name": "MITRE ATT&CK",
                        "description": "APT Group 123 details in MITRE ATT&CK.",
                        "url": "https://attack.mitre.org/groups/G0123/",
                        "external_id": "G0123"
                    },
                    {
                        "source_name": "Cyber Threat Intel",
                        "description": "Comprehensive analysis of APT Group 123.",
                        "url": "https://example.com/cyber-threat-intel/apt123",
                        "external_id": "CTI-890123"
                    }
                ],
                "object_marking_refs": [
                    "marking-definition--abcdef12-1234-5678-abcd-1234567890ab"
                ],
                "granular_markings": [
                    {
                        "marking_ref": "marking-definition--abcdef12-1234-5678-abcd-1234567890ab",
                        "selectors": ["description", "external_references[0].url"]
                    }
                ],
                "created_by_ref": "identity--abcd1234-abcd-1234-abcd-1234abcd5678",
                "extensions": {
                    "extension-definition--abcd1234-abcd-1234-abcd-1234abcd5678": {
                        "extension_type": "new-sdo",
                        "name": "Extended Threat Actor",
                        "description": "Extension for capturing additional information specific to the threat actor.",
                        "data": {
                            "operational_tactics": [
                                "Spear phishing",
                                "Exploitation of zero-day vulnerabilities"
                            ],
                            "known_tools": [
                                "Cobalt Strike",
                                "Mimikatz"
                            ],
                            "associated_groups": [
                                "threat-actor--abcd1234-abcd-1234-abcd-1234abcd5678"
                            ],
                            "observed_campaigns": [
                                "campaign--abcd1234-abcd-1234-abcd-1234abcd5678"
                            ]
                        }
                    }
                }
            }
    "#;

    let _parsed_json: ThreatActor = serde_json::from_str(&json_str).unwrap();

    // let sc = _parsed_json.calculate_risk_score();

    // println!("{:?}", sc);

    assert!(true)
}