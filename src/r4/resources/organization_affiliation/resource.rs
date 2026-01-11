use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::r4::{
    elements::{
        CodeableConcept, ContactPoint, GetResourceReferences, Identifier, Period, Reference,
        ReferenceTypes,
    },
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, ResourceType,
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct OrganizationAffiliation {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub participating_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<Reference<Organization>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speciality: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<Reference<Location>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcare_service: Option<Vec<Reference<HealthcareService>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    #[serde(default = "OrganizationAffiliation::get_resource_type")]
    pub resource_type: String,
}

impl Default for OrganizationAffiliation {
    fn default() -> Self {
        OrganizationAffiliation {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            period: None,
            organization: None,
            participating_organization: None,
            network: None,
            code: None,
            speciality: None,
            location: None,
            healthcare_service: None,
            telecom: None,
            endpoint: None,
        }
    }
}

impl ResourceType for OrganizationAffiliation {
    const TYPE: &'static str = "OrganizationAffiliation";
}

impl OrganizationAffiliation {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }

    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(&self).unwrap_or_else(|e| panic!("{e:?}"))
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|e| panic!("{e:?}"))
    }
}

impl GetResourceReferences for OrganizationAffiliation {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(org) = &self.organization {
            references.push(ReferenceTypes::from(org));
        }

        if let Some(participating_org) = &self.participating_organization {
            references.push(ReferenceTypes::from(participating_org));
        }

        if let Some(net) = &self.network {
            for n in net {
                references.push(ReferenceTypes::from(n));
            }
        }

        if let Some(loc) = &self.location {
            for l in loc {
                references.push(ReferenceTypes::from(l));
            }
        }

        if let Some(hs) = &self.healthcare_service {
            for h in hs {
                references.push(ReferenceTypes::from(h));
            }
        }

        if let Some(eps) = &self.endpoint {
            for ep in eps {
                references.push(ReferenceTypes::from(ep));
            }
        }

        references
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use serde_json::json;

    use crate::r4::{
        elements::{Coding, ReferenceBuilder},
        resources::{OrganizationAffiliationBuilder, Resource},
    };

    use super::*;

    #[ignore]
    #[test]
    pub fn get_org_affiliation_from_json() {
        let data = include_str!("../../../../fixtures/r4/resources/organization_affiliation.json");
        let org_aff = OrganizationAffiliation::from_json(data);
        println!("{:#?}", org_aff)
    }

    #[test]
    fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/organization_affiliation.json");

        let expected = OrganizationAffiliation {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("org-affiliation-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            active: Some(true),
            period: Some(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2026-01-01".to_string()),
                ..Default::default()
            }),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/org-1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            participating_organization: Some(Reference::<Organization> {
                reference: Some("Organization/org-2".to_string()),
                display: Some("Regional Specialty Clinic".to_string()),
                ..Default::default()
            }),
            network: Some(vec![Reference::<Organization> {
                reference: Some("Organization/org-network-1".to_string()),
                display: Some("Burgers Health Network".to_string()),
                ..Default::default()
            }]),
            code: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/organization-role".to_string(),
                    ),
                    code: Some("member".to_string()),
                    display: Some("Network Member".to_string()),
                    ..Default::default()
                }]),
                text: Some("Network membership".to_string()),
                ..Default::default()
            }]),
            speciality: None,
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/location-1".to_string()),
                display: Some("Main Campus".to_string()),
                ..Default::default()
            }]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                reference: Some("HealthcareService/healthcare-service-1".to_string()),
                display: Some("Emergency Department".to_string()),
                ..Default::default()
            }]),
            telecom: Some(vec![
                ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-333-4444".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
                ContactPoint {
                    system: Some("email".to_string()),
                    value: Some("affiliations@bumc.example.org".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
            ]),
            endpoint: Some(vec![Reference::<Endpoint> {
                reference: Some("Endpoint/endpoint-1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = OrganizationAffiliation::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_value_should_suceed() {
        let expected = json!(
        {
            "resourceType": "OrganizationAffiliation",
            "active": true,
            "organization": {
                "reference": "Organization/1",
            },
            "participatingOrganization" : {"reference":"Organization/2"},
            "network": [
                {
                    "reference": "Organization/3"
                }
            ]
        }
        );

        let data = OrganizationAffiliation {
            active: Some(true),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/1".to_string()),
                ..Default::default()
            }),
            participating_organization: Some(Reference::<Organization> {
                reference: Some("Organization/2".to_string()),
                ..Default::default()
            }),
            network: Some(vec![Reference::<Organization> {
                reference: Some("Organization/3".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = data.to_json_value();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
            "resourceType": "OrganizationAffiliation",
            "active": true,
            "organization": {
                "reference": "Organization/1",
            },
            "participatingOrganization" : {"reference":"Organization/2"},
            "network": [
                {
                    "reference": "Organization/3"
                }
            ]
        }
        );
        let data = OrganizationAffiliation {
            active: Some(true),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/1".to_string()),
                ..Default::default()
            }),
            participating_organization: Some(Reference::<Organization> {
                reference: Some("Organization/2".to_string()),
                ..Default::default()
            }),
            network: Some(vec![Reference::<Organization> {
                reference: Some("Organization/3".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let value = data.to_json_string();
        let actual: Value = serde_json::from_str(&value).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .with_display("Primary Health Org")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .with_display("Partner Healthcare Org")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_reference("Organization/3")
            .build::<Organization>();
        let location = ReferenceBuilder::default()
            .with_reference("Location/loc1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_reference("HealthcareService/hs1")
            .build::<HealthcareService>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let org_aff = OrganizationAffiliationBuilder::default()
            .with_organization(org.clone())
            .with_participating_organization(participating_org.clone())
            .add_network(network.clone())
            .add_location(location.clone())
            .add_healthcare_service(healthcare_service.clone())
            .add_endpoint(endpoint.clone())
            .build();

        let expeceted = vec![
            ReferenceTypes::from(&org),
            ReferenceTypes::from(&participating_org),
            ReferenceTypes::from(&network),
            ReferenceTypes::from(&location),
            ReferenceTypes::from(&healthcare_service),
            ReferenceTypes::from(&endpoint),
        ];

        let actual = org_aff.get_references();

        assert_eq!(expeceted, actual)
    }
}
