use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Attachement, AvailableTime, BackboneElement, CodeableConcept, ContactPoint,
            GetResourceReferences, Identifier, NotAvailable, Reference, ReferenceTypes,
        },
        resources::{self, DomainResource, Endpoint, Location, Organization, ResourceType},
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Eligibility {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ResourceType for Eligibility {
    const TYPE: &'static str = "Eligibility";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct HealthcareService {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<Reference<Location>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Attachement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_area: Option<Vec<Reference<Location>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provision_code: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility: Option<Vec<Eligibility>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_method: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_time: Option<Vec<AvailableTime>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_available: Option<Vec<NotAvailable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_exceptions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    #[serde(default = "HealthcareService::get_resource_type")]
    pub resource_type: String,
}

impl Default for HealthcareService {
    fn default() -> Self {
        HealthcareService {
            resource_type: Self::get_resource_type(),
            domain_resource: DomainResource {
                ..Default::default()
            },
            active: None,
            identifier: None,
            provided_by: None,
            coverage_area: None,
            category: None,
            comment: None,
            r#type: None,
            telecom: None,
            location: None,
            not_available: None,
            name: None,
            availability_exceptions: None,
            appointment_required: None,
            available_time: None,
            characteristic: None,
            communication: None,
            eligibility: None,
            endpoint: None,
            extra_details: None,
            photo: None,
            program: None,
            referral_method: None,
            service_provision_code: None,
        }
    }
}

impl ResourceType for HealthcareService {
    const TYPE: &'static str = "HealthcareService";
}

impl HealthcareService {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}

impl GetResourceReferences for HealthcareService {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(pb) = &self.provided_by {
            references.push(ReferenceTypes::from(pb));
        }

        if let Some(locs) = &self.location {
            for l in locs {
                references.push(ReferenceTypes::from(l));
            }
        }

        if let Some(ca) = &self.coverage_area {
            for c in ca {
                references.push(ReferenceTypes::from(c));
            }
        }

        if let Some(eps) = &self.endpoint {
            for e in eps {
                references.push(ReferenceTypes::from(e));
            }
        }

        references
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use serde_json::{Value, json};

    use crate::r4::{
        elements::{Coding, DaysOfWeek, Period, ReferenceBuilder},
        resources::{HealthcareServiceBuilder, Resource},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/healthcare_service.json");
        let expected = HealthcareService {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("healthcare-service-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![Identifier {
                r#use: Some("official".to_string()),
                system: Some("http://example.org/healthcare-services".to_string()),
                value: Some("HCS-001".to_string()),
                ..Default::default()
            }]),
            active: Some(true),
            provided_by: Some(Reference::<Organization> {
                reference: Some("Organization/org-1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            category: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/service-category".to_string(),
                    ),
                    code: Some("17".to_string()),
                    display: Some("Emergency".to_string()),
                    ..Default::default()
                }]),
                text: Some("Emergency Services".to_string()),
                ..Default::default()
            }]),
            r#type: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://terminology.hl7.org/CodeSystem/service-type".to_string()),
                    code: Some("57".to_string()),
                    display: Some("Emergency Medicine".to_string()),
                    ..Default::default()
                }]),
                text: Some("Emergency Medicine".to_string()),
                ..Default::default()
            }]),
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/location-1".to_string()),
                display: Some("Emergency Department".to_string()),
                ..Default::default()
            }]),
            name: Some("Emergency Department".to_string()),
            comment: Some("24/7 emergency care for all ages".to_string()),
            extra_details: Some(
                "Wheelchair accessible. Interpreter services available.".to_string(),
            ),
            photo: None,
            telecom: Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: Some("+1-555-111-2222".to_string()),
                r#use: Some("work".to_string()),
                ..Default::default()
            }]),
            coverage_area: Some(vec![Reference::<Location> {
                reference: Some("Location/region-1".to_string()),
                display: Some("Metropolitan Area".to_string()),
                ..Default::default()
            }]),
            service_provision_code: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/service-provision-conditions"
                            .to_string(),
                    ),
                    code: Some("free".to_string()),
                    display: Some("Free".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            eligibility: Some(vec![Eligibility {
                code: Some(CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some(
                            "http://terminology.hl7.org/CodeSystem/benefit-eligibility".to_string(),
                        ),
                        code: Some("eligible".to_string()),
                        display: Some("Eligible".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("Eligible patients".to_string()),
                    ..Default::default()
                }),
                comment: Some("All patients accepted".to_string()),
                ..Default::default()
            }]),
            program: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.org/programs".to_string()),
                    code: Some("ED-CARE".to_string()),
                    display: Some("Emergency Care Program".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            characteristic: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/service-characteristic".to_string(),
                    ),
                    code: Some("wheelchair".to_string()),
                    display: Some("Wheelchair accessible".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            appointment_required: Some(false),
            available_time: Some(vec![AvailableTime {
                days_of_week: Some(vec![
                    DaysOfWeek::Mon,
                    DaysOfWeek::Tue,
                    DaysOfWeek::Wed,
                    DaysOfWeek::Thu,
                    DaysOfWeek::Fri,
                    DaysOfWeek::Sat,
                    DaysOfWeek::Sun,
                ]),
                all_day: Some(true),
                ..Default::default()
            }]),
            not_available: Some(vec![NotAvailable {
                description: "Closed for annual maintenance".to_string(),
                during: Some(Period {
                    start: Some("2025-12-25".to_string()),
                    end: Some("2025-12-26".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            availability_exceptions: Some("Closed on major public holidays".to_string()),
            ..Default::default()
        };

        let actual = HealthcareService::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!({
            "resourceType": "HealthcareService",
            "type": [
                {
                    "coding": [{"system": "some-system", "code": "40" }
                    ]
                }
            ],
            "location": [
                {
                    "reference": "Location/1"
                }
            ]
        });
        let data = HealthcareService {
            r#type: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("some-system".to_string()),
                    code: Some("40".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = data.to_json_value().unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!({
            "resourceType": "HealthcareService",
            "type": [
                {
                    "coding": [{"system": "some-system", "code": "40" }
                    ]
                }
            ],
            "location": [
                {
                    "reference": "Location/1"
                }
            ]
        });
        let data = HealthcareService {
            r#type: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("some-system".to_string()),
                    code: Some("40".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let value = data.to_json_string().unwrap_or_else(|e| panic!("{e:?}"));
        let actual: Value = serde_json::from_str(&value).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let provided_by = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let location = ReferenceBuilder::default()
            .with_reference("Location/1")
            .build::<Location>();
        let coverage_area = ReferenceBuilder::default()
            .with_reference("Location/2")
            .build::<Location>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();
        let expected = vec![
            ReferenceTypes::from(&provided_by),
            ReferenceTypes::from(&location),
            ReferenceTypes::from(&coverage_area),
            ReferenceTypes::from(&endpoint),
        ];

        let healthcare_service = HealthcareServiceBuilder::default()
            .with_provided_by(provided_by.clone())
            .add_location(location.clone())
            .add_coverage_area(coverage_area.clone())
            .add_endpoint(endpoint.clone())
            .build();

        let actual = healthcare_service.get_references();

        assert_eq!(expected, actual)
    }
}
