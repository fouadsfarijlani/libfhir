use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        AvailableTime, CodeableConcept, ContactPoint, GetResourceReferences, Identifier,
        NotAvailable, Period, Reference, ReferenceTypes,
    },
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, Practitioner,
        ResourceType,
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct PractitionerRole {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub period: Option<Period>,
    pub practitioner: Option<Reference<Practitioner>>,
    pub organization: Option<Reference<Organization>>,
    pub code: Option<Vec<CodeableConcept>>,
    pub speciality: Option<Vec<CodeableConcept>>,
    pub location: Option<Vec<Reference<Location>>>,
    pub healthcare_service: Option<Vec<Reference<HealthcareService>>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub available_time: Option<Vec<AvailableTime>>,
    pub not_available: Option<Vec<NotAvailable>>,
    pub availability_exceptions: Option<String>,
    pub endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl ResourceType for PractitionerRole {
    const TYPE: &'static str = "PractitionerRole";
}

impl PractitionerRole {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
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
    use crate::r4::{
        elements::{Coding, DaysOfWeek},
        resources::Resource,
    };

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"{
        "resourceType": "PractitionerRole",
        "id": "pr1",
        "active": true,
        "practitioner": {
            "reference": "Practitioner/prac1"
        },
        "organization": {
            "reference": "Organization/org1"
        },
        "code": [
            {
                "coding": [
                    {
                        "system": "http://example.com",
                        "code": "doctor",
                        "display": "Doctor"
                    }
                ]
            }
        ],
        "speciality": [
            {
                "coding": [
                    {
                        "system": "http://example.com",
                        "code": "cardiology",
                        "display": "Cardiology"
                    }
                ]
            }
        ],
        "location": [
            {
                "reference": "Location/location1"
            }
        ],
        "healthcareService": [
            {
                "reference": "HealthcareService/hs1"
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "123456789",
                "use": "work"
            }
        ],
        "availableTime": [
            {
                "daysOfWeek": ["mon", "tue", "wed"],
                "availableStartTime": "08:00:00",
                "availableEndTime": "16:00:00"
            }
        ],
        "notAvailable": [
            {
                "description": "Closed during holidays",
                "during": {
                    "start": "2025-12-24",
                    "end": "2025-12-26"
                }
            }
        ],
        "availabilityExceptions": "Reduced hours during summer",
        "endpoint": [
            {
                "reference": "Endpoint/ep1"
            }
        ]
    }"#;

        let expected = PractitionerRole {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("pr1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            active: Some(true),
            practitioner: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/prac1".to_string()),
                ..Default::default()
            }),
            organization: Some(Reference::<Organization> {
                reference: Some("Organization/org1".to_string()),
                ..Default::default()
            }),
            code: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.com".to_string()),
                    code: Some("doctor".to_string()),
                    display: Some("Doctor".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            speciality: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.com".to_string()),
                    code: Some("cardiology".to_string()),
                    display: Some("Cardiology".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/location1".to_string()),
                ..Default::default()
            }]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                reference: Some("HealthcareService/hs1".to_string()),
                ..Default::default()
            }]),
            telecom: Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: Some("123456789".to_string()),
                r#use: Some("work".to_string()),
                ..Default::default()
            }]),
            available_time: Some(vec![AvailableTime {
                days_of_week: Some(vec![DaysOfWeek::Mon, DaysOfWeek::Tue, DaysOfWeek::Wed]),
                available_start_time: Some("08:00:00".to_string()),
                available_end_time: Some("16:00:00".to_string()),
                ..Default::default()
            }]),
            not_available: Some(vec![NotAvailable {
                description: "Closed during holidays".to_string(),
                during: Some(Period {
                    start: Some("2025-12-24".to_string()),
                    end: Some("2025-12-26".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            availability_exceptions: Some("Reduced hours during summer".to_string()),
            endpoint: Some(vec![Reference::<Endpoint> {
                reference: Some("Endpoint/ep1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = PractitionerRole::from_json(data);

        assert_eq!(expected, actual)
    }
}
