use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        CodeableConcept, Coding, ContactPoint, GetResourceReferences, Identifier, Period,
        Reference, ReferenceTypes,
    },
    resources::{DomainResource, Organization, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum EndpointStatus {
    Active,
    Suspended,
    Error,
    Off,
    EnteredInError,
    #[default]
    Test,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Endpoint {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    pub status: EndpointStatus,

    pub connection_type: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    pub payload_type: Vec<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later

    pub address: String, // to be resolved later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,

    #[serde(default = "Endpoint::get_resource_type")]
    pub resource_type: String,
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            domain_resource: DomainResource {
                ..Default::default()
            },
            identifier: None,
            status: EndpointStatus::Test,
            connection_type: Coding {
                ..Default::default()
            },
            managing_organization: None,
            contact: None,
            period: None,
            payload_type: vec![],
            payload_mime_type: None,
            address: "".to_string(),
            header: None,
            resource_type: Self::get_resource_type(),
        }
    }
}

impl Endpoint {
    pub fn from_json(data: &str) -> Self {
        let result = serde_json::from_str(data);
        match result {
            Ok(ep) => ep,
            Err(e) => panic!("{e:?}"),
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap_or_else(|e| panic!("{e:?}"))
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|e| panic!("{e:?}"))
    }
}

impl ResourceType for Endpoint {
    const TYPE: &'static str = "Endpoint";
}

impl GetResourceReferences for Endpoint {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();
        if let Some(managing_organization) = &self.managing_organization {
            references.push(ReferenceTypes::from(managing_organization));
        }

        references
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use crate::r4::resources::Resource;

    #[test]
    pub fn get_endpoint_from_json() {
        let data = include_str!("../../../../fixtures/r4/resources/endpoint.json");
        let ep = Endpoint::from_json(data);
        dbg!(ep);
        // println!("{:#?}", ep)
    }

    #[test]
    pub fn from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/endpoint.json");
        let expected = Endpoint {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("endpoint-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![Identifier {
                r#use: Some("official".to_string()),
                system: Some("http://example.org/endpoints".to_string()),
                value: Some("ENDPOINT-001".to_string()),
                ..Default::default()
            }]),
            status: EndpointStatus::Active,
            connection_type: Coding {
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/endpoint-connection-type".to_string(),
                ),
                code: Some("hl7-fhir-rest".to_string()),
                display: Some("FHIR REST".to_string()),
                ..Default::default()
            },
            managing_organization: Some(Reference::<Organization> {
                reference: Some("Organization/org-1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            contact: Some(vec![ContactPoint {
                system: Some("email".to_string()),
                value: Some("fhir-support@bumc.example.org".to_string()),
                r#use: Some("work".to_string()),
                ..Default::default()
            }]),
            period: Some(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2026-01-01".to_string()),
                ..Default::default()
            }),
            payload_type: vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://hl7.org/fhir/resource-types".to_string()),
                    code: Some("Bundle".to_string()),
                    display: Some("Bundle".to_string()),
                    ..Default::default()
                }]),
                text: Some("FHIR Bundle".to_string()),
                ..Default::default()
            }],
            payload_mime_type: Some(vec![
                "application/fhir+json".to_string(),
                "application/json".to_string(),
            ]),
            address: "https://fhir.bumc.example.org/r4".to_string(),
            header: Some(vec![
                "Authorization: Bearer {token}".to_string(),
                "Accept: application/fhir+json".to_string(),
            ]),
            resource_type: "Endpoint".to_string(),
        };

        let actual = Endpoint::from_json(data);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_get_resource_references_should_succeed() {
        let endpoint = Endpoint {
            managing_organization: Some(Reference::<Organization> {
                reference: Some("Organization/1".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let managing_org = Reference::<Organization> {
            reference: Some("Organization/1".to_string()),
            ..Default::default()
        };
        let expected = vec![ReferenceTypes::from(&managing_org)];

        let actual = endpoint.get_references();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
            "resourceType":"Endpoint",
            "id": "endpoint-1",
            "status": "error",
            "identifier": [
                {
                    "use": "official",
                    "system": "http://example.com",
                    "value": "ep-system-1"
                }
            ],
            "connectionType": {
                "system": "some-system",
                "code": "some-code"
            },
            "managingOrganization": {
                "reference": "Organization/1"
            },
            "contact": [
                {
                    "system": "http://example.com",
                    "value": "contact-value-1"
                }
            ],
            "period": {
                "start": "2025-01-01",
                "end": "2026-01-01"
            },
            "payloadType": [
                {
                    "coding":[
                        {
                            "system": "some-system"
                        }
                    ],
                    "text": "some text"
                }
            ],
            "address": "http://example.com",
            "header": ["Accept: application/json"]
        }
        );

        let endpoint = Endpoint {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("endpoint-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            status: EndpointStatus::Error,
            identifier: Some(vec![Identifier {
                r#use: Some("official".to_string()),
                system: Some("http://example.com".to_string()),
                value: Some("ep-system-1".to_string()),
                ..Default::default()
            }]),
            connection_type: Coding {
                system: Some("some-system".to_string()),
                code: Some("some-code".to_string()),
                ..Default::default()
            },
            managing_organization: Some(Reference::<Organization> {
                reference: Some("Organization/1".to_string()),
                ..Default::default()
            }),
            contact: Some(vec![ContactPoint {
                system: Some("http://example.com".to_string()),
                value: Some("contact-value-1".to_string()),
                ..Default::default()
            }]),
            period: Some(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2026-01-01".to_string()),
                ..Default::default()
            }),
            payload_type: vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("some-system".to_string()),
                    ..Default::default()
                }]),
                text: Some("some text".to_string()),
                ..Default::default()
            }],
            header: Some(vec!["Accept: application/json".to_string()]),
            address: "http://example.com".to_string(),
            ..Default::default()
        };

        let value = endpoint.to_json_string();
        let actual: serde_json::Value = serde_json::from_str(value.as_str()).unwrap();

        assert_eq!(expected, actual)
    }
}
