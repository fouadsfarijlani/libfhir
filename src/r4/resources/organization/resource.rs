use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        Address, BackboneElement, CodeableConcept, ContactPoint, GetResourceReferences, HumanName,
        Identifier, Reference, ReferenceTypes,
    },
    resources::{DomainResource, Endpoint, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct OrganizationContact {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HumanName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

impl ResourceType for OrganizationContact {
    const TYPE: &'static str = "OrganizationContact";
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Organization {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<OrganizationContact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    #[serde(default = "Organization::get_resource_type")]
    pub resource_type: String,
}

impl Default for Organization {
    fn default() -> Self {
        Organization {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            address: None,
            alias: None,
            endpoint: None,
            part_of: None,
            contact: None,
            r#type: None,
            name: None,
            telecom: None,
        }
    }
}

impl Organization {
    pub fn from_json(data: &str) -> Self {
        let results = serde_json::from_str::<Organization>(data);
        match results {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap_or_else(|e| panic!("{e:?}"))
    }

    pub fn to_json_string(&self) -> String {
        let result = serde_json::to_string_pretty(&self);
        match result {
            Ok(data) => data,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl ResourceType for Organization {
    const TYPE: &'static str = "Organization";
}

impl GetResourceReferences for Organization {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(eps) = &self.endpoint {
            references = eps.into_iter().map(|e| ReferenceTypes::from(e)).collect();
        }

        if let Some(part_of) = &self.part_of {
            references.push(ReferenceTypes::from(part_of));
        }

        references
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::{
        elements::{Coding, ReferenceBuilder},
        resources::{OrganizationBuilder, Resource},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_suceed() {
        let data = include_str!("../../../../fixtures/r4/resources/organization.json");
        let expected = Organization {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("org-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![Identifier {
                r#use: Some("official".to_string()),
                system: Some("http://example.org/orgs".to_string()),
                value: Some("ORG-001".to_string()),
                ..Default::default()
            }]),
            active: Some(true),
            r#type: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/organization-type".to_string(),
                    ),
                    code: Some("prov".to_string()),
                    display: Some("Healthcare Provider".to_string()),
                    ..Default::default()
                }]),
                text: Some("Healthcare Provider".to_string()),
                ..Default::default()
            }]),
            name: Some("Burgers University Medical Center".to_string()),
            alias: Some(vec!["BUMC".to_string(), "Burgers UMC".to_string()]),
            telecom: Some(vec![
                ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-123-4567".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
                ContactPoint {
                    system: Some("email".to_string()),
                    value: Some("info@bumc.example.org".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
            ]),
            address: Some(vec![Address {
                r#use: Some("work".to_string()),
                line: Some(vec!["123 Medical Way".to_string()]),
                city: Some("PleasantVille".to_string()),
                state: Some("Vic".to_string()),
                postal_code: Some("3999".to_string()),
                country: Some("Australia".to_string()),
                ..Default::default()
            }]),
            part_of: Some(Reference::<Organization> {
                reference: Some("Organization/parent-org".to_string()),
                display: Some("Burgers Health Network".to_string()),
                ..Default::default()
            }),
            contact: Some(vec![OrganizationContact {
                purpose: Some(CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some(
                            "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(),
                        ),
                        code: Some("ADMIN".to_string()),
                        display: Some("Administrative".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("Administrative".to_string()),
                    ..Default::default()
                }),
                name: Some(HumanName {
                    family: Some("Smith".to_string()),
                    given: Some(vec!["Jane".to_string()]),
                    ..Default::default()
                }),
                telecom: Some(vec![ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-987-6543".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                }]),
                address: Some(Address {
                    line: Some(vec!["123 Medical Way".to_string()]),
                    city: Some("PleasantVille".to_string()),
                    state: Some("Vic".to_string()),
                    postal_code: Some("3999".to_string()),
                    country: Some("Australia".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            endpoint: Some(vec![
                Reference::<Endpoint> {
                    reference: Some("Endpoint/endpoint-1".to_string()),
                    ..Default::default()
                },
                Reference::<Endpoint> {
                    reference: Some("Endpoint/endpoint-2".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let actual = Organization::from_json(data);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        );

        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let ep_1 = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let ep_2 = ReferenceBuilder::default()
            .with_reference("Endpoint/2")
            .build::<Endpoint>();

        let endpoint = vec![ep_1, ep_2];
        let org = OrganizationBuilder::new(String::from("some-id"))
            .active(true)
            .part_of(part_of)
            .endpoint(endpoint)
            .build();

        let actual = org.to_json_value();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        );

        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let ep_1 = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let ep_2 = ReferenceBuilder::default()
            .with_reference("Endpoint/2")
            .build::<Endpoint>();

        let endpoint = vec![ep_1, ep_2];
        let org = OrganizationBuilder::new(String::from("some-id"))
            .active(true)
            .part_of(part_of)
            .endpoint(endpoint)
            .build();

        let value = org.to_json_string();
        let actual: serde_json::Value = serde_json::from_str(value.as_str()).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_get_all_referenecs_should_succeed() {
        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let endpoints = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();
        let expected = vec![
            ReferenceTypes::from(&endpoints),
            ReferenceTypes::from(&part_of),
        ];
        let org = OrganizationBuilder::default()
            .part_of(part_of.clone())
            .add_endpoint(endpoints.clone())
            .build();

        let actual = org.get_references();

        assert_eq!(expected, actual)
    }
}
