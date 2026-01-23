use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            AvailableTime, CodeableConcept, ContactPoint, GetResourceReferences, Identifier,
            NotAvailable, Period, Reference, ReferenceTypes,
        },
        resources::{
            DomainResource, Endpoint, HealthcareService, Location, Organization,
            Practitioner, ResourceType,
        },
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct PractitionerRole {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub practitioner: Option<Reference<Practitioner>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Reference<Organization>>,

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
    pub available_time: Option<Vec<AvailableTime>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_available: Option<Vec<NotAvailable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_exceptions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    pub resource_type: String,
}

impl Default for PractitionerRole {
    fn default() -> Self {
        PractitionerRole {
            resource_type: Self::get_resource_type(),
            domain_resource: DomainResource {
                ..Default::default()
            },
            identifier: None,
            active: None,
            period: None,
            practitioner: None,
            organization: None,
            code: None,
            speciality: None,
            location: None,
            healthcare_service: None,
            telecom: None,
            available_time: None,
            not_available: None,
            availability_exceptions: None,
            endpoint: None,
        }
    }
}

impl ResourceType for PractitionerRole {
    const TYPE: &'static str = "PractitionerRole";
}

impl PractitionerRole {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }
}

impl GetResourceReferences for PractitionerRole {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(pract) = &self.practitioner {
            references.push(ReferenceTypes::from(pract));
        }

        if let Some(org) = &self.organization {
            references.push(ReferenceTypes::from(org));
        }

        if let Some(locs) = &self.location {
            for loc in locs {
                references.push(ReferenceTypes::from(loc));
            }
        }

        if let Some(h_sevices) = &self.healthcare_service {
            for h in h_sevices {
                references.push(ReferenceTypes::from(h));
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

    use serde_json::json;

    use crate::r4::{
        elements::{Coding, DaysOfWeek},
        resources::Resource,
    };

    use super::*;

    #[ignore]
    #[test]
    pub fn get_practitioner_role_from_json() {
        let data = include_str!("../../../../fixtures/r4/resources/practitioner_role.json");
        let resource = PractitionerRole::from_json(data);
        println!("{:#?}", resource)
    }

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/practitioner_role.json");

        let expected = PractitionerRole {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("practitioner-role-1".to_string()),
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
            practitioner: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/practitioner-1".to_string()),
                display: Some("Dr John Doe".to_string()),
                ..Default::default()
            }),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/org-1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            code: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/practitioner-role".to_string(),
                    ),
                    code: Some("doctor".to_string()),
                    display: Some("Doctor".to_string()),
                    ..Default::default()
                }]),
                text: Some("Attending Physician".to_string()),
                ..Default::default()
            }]),
            speciality: None,
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/location-1".to_string()),
                display: Some("Emergency Department".to_string()),
                ..Default::default()
            }]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                reference: Some("HealthcareService/healthcare-service-1".to_string()),
                display: Some("Emergency Services".to_string()),
                ..Default::default()
            }]),
            telecom: Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: Some("+1-555-999-0000".to_string()),
                r#use: Some("work".to_string()),
                ..Default::default()
            }]),
            available_time: Some(vec![AvailableTime {
                days_of_week: Some(vec![
                    DaysOfWeek::Mon,
                    DaysOfWeek::Tue,
                    DaysOfWeek::Wed,
                    DaysOfWeek::Thu,
                    DaysOfWeek::Fri,
                ]),
                available_start_time: Some("08:00".to_string()),
                available_end_time: Some("17:00".to_string()),
                ..Default::default()
            }]),
            not_available: Some(vec![NotAvailable {
                description: "On leave".to_string(),
                during: Some(Period {
                    start: Some("2025-07-01".to_string()),
                    end: Some("2025-07-15".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            availability_exceptions: Some("Unavailable on public holidays".to_string()),
            endpoint: Some(vec![Reference::<Endpoint> {
                reference: Some("Endpoint/endpoint-1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = PractitionerRole::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!({
            "resourceType": "PractitionerRole",
            "active": true,
            "practitioner": {
                "reference": "Practitioner/1"
            }
        });

        let data = PractitionerRole {
            active: Some(true),
            practitioner: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/1".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let parsed_data = data.to_json_string().unwrap_or_else(|e| panic!("{e:?}"));
        let acutal: serde_json::Value = serde_json::from_str(&parsed_data).unwrap();

        assert_eq!(expected, acutal)
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!({
            "resourceType": "PractitionerRole",
            "active": true,
            "practitioner": {
                "reference": "Practitioner/1"
            }
        });

        let data = PractitionerRole {
            active: Some(true),
            practitioner: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/1".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let actual = data.to_json_value().unwrap_or_else(|e| panic!("{e:?}"));

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_get_references_should_succeed() {
        let practitioner_role = PractitionerRole {
            practitioner: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/2".to_string()),
                ..Default::default()
            }),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/1".to_string()),
                ..Default::default()
            }),
            location: Some(vec![
                Reference::<Location> {
                    reference: Some("Location/3".to_string()),
                    ..Default::default()
                },
                Reference::<Location> {
                    reference: Some("Location/4".to_string()),
                    ..Default::default()
                },
            ]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                reference: Some("HealthcareService/5".to_string()),
                ..Default::default()
            }]),
            endpoint: Some(vec![Reference::<Endpoint> {
                reference: Some("Endpoint/6".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let prac = Reference::<Practitioner> {
            reference: Some("Practitioner/2".to_string()),
            ..Default::default()
        };
        let org = Reference::<Organization> {
            reference: Some("Organization/1".to_string()),
            ..Default::default()
        };
        let loc_1 = Reference::<Location> {
            reference: Some("Location/3".to_string()),
            ..Default::default()
        };
        let loc_2 = Reference::<Location> {
            reference: Some("Location/4".to_string()),
            ..Default::default()
        };
        let hcs = Reference::<HealthcareService> {
            reference: Some("HealthcareService/5".to_string()),
            ..Default::default()
        };
        let ep = Reference::<Endpoint> {
            reference: Some("Endpoint/6".to_string()),
            ..Default::default()
        };
        let expected = vec![
            ReferenceTypes::from(&prac),
            ReferenceTypes::from(&org),
            ReferenceTypes::from(&loc_1),
            ReferenceTypes::from(&loc_2),
            ReferenceTypes::from(&hcs),
            ReferenceTypes::from(&ep),
        ];

        let actual = practitioner_role.get_references();

        assert_eq!(expected, actual)
    }
}
