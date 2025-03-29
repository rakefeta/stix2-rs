use std::fs::read_to_string;
use serde_json;
use crate::domain_obj::bundle::Bundle;


/// Test for the Bundle object using a json represented by a raw string literal.
#[test]
fn bundle_deserialization() {
    let json_str = r#"
        {
    "type": "bundle",
    "id": "bundle--97b40f76-c1b8-4407-b050-ff177f3d67ed",
    "objects": [
        {
            "type": "threat-actor",
            "spec_version": "2.1",
            "id": "threat-actor--56f3f0db-b5d5-431c-ae56-c18f02caf500",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Fake BPP (Branistan Peoples Party)",
            "threat_actor_types": [
                "nation-state"
            ],
            "roles": [
                "director"
            ],
            "goals": [
                "Influence the election in Branistan"
            ],
            "sophistication": "strategic",
            "resource_level": "government",
            "primary_motivation": "ideology",
            "secondary_motivations": [
                "dominance"
            ]
        },
        {
            "type": "identity",
            "spec_version": "2.1",
            "id": "identity--8c6af861-7b20-41ef-9b59-6344fd872a8f",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Franistan Intelligence",
            "identity_class": "organization"
        },
        {
            "type": "identity",
            "spec_version": "2.1",
            "id": "identity--ddfe7140-2ba4-48e4-b19a-df069432103b",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Branistan Peoples Party",
            "identity_class": "organization",
            "external_references": [
                {
                    "source_name": "website",
                    "url": "http://www.bpp.bn"
                }
            ]
        },
        {
            "type": "attack-pattern",
            "spec_version": "2.1",
            "id": "attack-pattern--19da6e1c-71ab-4c2f-886d-d620d09d3b5a",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2017-01-30T21:15:04.127Z",
            "name": "Content Spoofing",
            "external_references": [
                {
                    "source_name": "capec",
                    "url": "https://capec.mitre.org/data/definitions/148.html",
                    "external_id": "CAPEC-148"
                }
            ]
        },
        {
            "type": "attack-pattern",
            "spec_version": "2.1",
            "id": "attack-pattern--f6050ea6-a9a3-4524-93ed-c27858d6cb3c",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2017-01-30T21:15:04.127Z",
            "name": "HTTP Flood",
            "external_references": [
                {
                    "source_name": "capec",
                    "url": "https://capec.mitre.org/data/definitions/488.html",
                    "external_id": "CAPEC-488"
                }
            ]
        },
        {
            "type": "campaign",
            "spec_version": "2.1",
            "id": "campaign--e5268b6e-4931-42f1-b379-87f48eb41b1e",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Operation Bran Flakes",
            "description": "A concerted effort to insert false information into the BPP's web pages.",
            "aliases": [
                "OBF"
            ],
            "first_seen": "2016-01-08T12:50:40.123Z",
            "objective": "Hack www.bpp.bn"
        },
        {
            "type": "campaign",
            "spec_version": "2.1",
            "id": "campaign--1d8897a7-fdc2-4e59-afc9-becbe04df727",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "Operation Raisin Bran",
            "description": "A DDOS campaign to flood BPP web servers.",
            "aliases": [
                "ORB"
            ],
            "first_seen": "2016-02-07T19:45:32.126Z",
            "objective": "Flood www.bpp.bn"
        },
        {
            "type": "intrusion-set",
            "spec_version": "2.1",
            "id": "intrusion-set--ed69450a-f067-4b51-9ba2-c4616b9a6713",
            "created": "2016-08-08T15:50:10.983Z",
            "modified": "2016-08-08T15:50:10.983Z",
            "name": "APT BPP",
            "description": "An advanced persistent threat that seeks to disrupt Branistan's election with multiple attacks.",
            "aliases": [
                "Bran-teaser"
            ],
            "first_seen": "2016-01-08T12:50:40.123Z",
            "goals": [
                "Influence the Branistan election",
                "Disrupt the BPP"
            ],
            "resource_level": "government",
            "primary_motivation": "ideology",
            "secondary_motivations": [
                "dominance"
            ]
        }
    ]
}
        "#;

    let _parsed_json: Bundle = serde_json::from_str(&json_str).unwrap();
}



/// Test for the Bundle object using a json read from file.
/// Test for the Bundle object using a json represented by a raw string literal.
#[test]
fn bundle_deserialization_from_file() {

    let path = "./src/tests/test_data/defining-campaign-ta-is.json";
    let data = read_to_string(path).expect("Unable to read file");

    let _parsed_json: Bundle = serde_json::from_str(&data).unwrap();
    // _parsed_json
}
