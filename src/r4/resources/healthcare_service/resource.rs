use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        Attachement, AvailableTime, BackboneElement, CodeableConcept, ContactPoint,
        GetResourceReferences, Identifier, NotAvailable, Reference, ReferenceTypes,
    },
    resources::{self, DomainResource, Endpoint, Location, Organization, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Eligibility {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub code: Option<CodeableConcept>,
    pub comment: Option<String>,
}

impl ResourceType for Eligibility {
    const TYPE: &'static str = "Eligibility";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct HealthcareService {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub provided_by: Option<Reference<Organization>>,
    pub category: Option<Vec<CodeableConcept>>,
    pub r#type: Option<Vec<CodeableConcept>>,
    pub location: Option<Vec<Reference<Location>>>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub extra_details: Option<String>,
    pub photo: Option<Attachement>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub coverage_area: Option<Vec<Reference<Location>>>,
    pub service_provision_code: Option<Vec<CodeableConcept>>,
    pub eligibility: Option<Vec<Eligibility>>,
    pub program: Option<Vec<CodeableConcept>>,
    pub characteristic: Option<Vec<CodeableConcept>>,
    pub communication: Option<Vec<CodeableConcept>>,
    pub referral_method: Option<Vec<CodeableConcept>>,
    pub appointment_required: Option<bool>,
    pub available_time: Option<Vec<AvailableTime>>,
    pub not_available: Option<Vec<NotAvailable>>,
    pub availability_exceptions: Option<String>,
    pub endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl ResourceType for HealthcareService {
    const TYPE: &'static str = "HealthcareService";
}

impl HealthcareService {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
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
    use crate::r4::elements::{
        AvailableTimeBuilder, CodeableConceptBuilder, CodingBuilder, ContactPointBuilder,
        DaysOfWeek, Element, NotAvailableBuilder, Period, PeriodBuilder, ReferenceBuilder,
    };

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"{
        "resourceType": "HealthcareService",
        "id": "hcs-1",
        "active": true,
        "providedBy": {
            "reference": "Organization/1"
        },
        "category": [
            {
                "coding": [
                    {
                        "system": "http://example.com",
                        "code": "1",
                        "display": "Advisory"
                    }
                ],
                "text": "General Advisory Services"
            }
        ],
        "type": [
            {
                "coding": [
                    {
                        "system": "http://example.com",
                        "code": "gp",
                        "display": "General Practice"
                    }
                ],
                "text": "General Practitioner"
            }
        ],
        "location": [
            {
                "reference": "Location/1"
            }
        ],
        "name": "General Practice Service",
        "comment": "Open Monday to Friday",
        "extraDetails": "Appointments available on the same day",
        "telecom": [
            {
                "system": "phone",
                "value": "12345678"
            }
        ],
        "coverageArea": [
            {
                "reference": "Location/2"
            }
        ],
        "serviceProvisionCode": [
            {
                "coding": [
                    {
                        "system": "http://example.com",
                        "code": "home",
                        "display": "Home visit"
                    }
                ]
            }
        ],
        "eligibility": [
            {
                "code": {
                    "text": "Adults over 18"
                },
                "comment": "Must be registered with the clinic"
            }
        ],
        "program": [
            {
                "text": "Wellness Program"
            }
        ],
        "characteristic": [
            {
                "text": "Wheelchair accessible"
            }
        ],
        "communication": [
            {
                "text": "English"
            }
        ],
        "referralMethod": [
            {
                "text": "Phone"
            }
        ],
        "appointmentRequired": true,
        "availableTime": [
            {
                "daysOfWeek": ["mon", "tue", "wed", "thu", "fri"],
                "availableStartTime": "08:00",
                "availableEndTime": "17:00"
            }
        ],
        "notAvailable": [
            {
                "description": "Closed on public holidays",
                "during": {
                    "start": "2025-12-25",
                    "end": "2025-12-26"
                }
            }
        ],
        "availabilityExceptions": "Only during category 1 emergencies",
        "endpoint": [
            {
                "reference": "Endpoint/ep-1"
            }
        ]
    }"#;
        let provided_by = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let category = CodeableConceptBuilder::default()
            .add_coding(
                CodingBuilder::default()
                    .with_system("http://example.com")
                    .with_code("1")
                    .with_display("Advisory")
                    .build(),
            )
            .with_text("General Advisory Services")
            .build();
        let r#type = CodeableConceptBuilder::default()
            .add_coding(
                CodingBuilder::default()
                    .with_system("http://example.com")
                    .with_code("gp")
                    .with_display("General Practice")
                    .build(),
            )
            .with_text("General Practitioner")
            .build();
        let location = ReferenceBuilder::default()
            .with_reference("Location/1")
            .build::<Location>();
        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("12345678")
            .build();
        let coverage_area = ReferenceBuilder::default()
            .with_reference("Location/2")
            .build::<Location>();
        let eligibility = EligibilityBuilder::default()
            .with_code(
                CodeableConceptBuilder::default()
                    .with_text("Adults over 18")
                    .build(),
            )
            .with_comment("Must be registered with the clinic")
            .build();
        let program = CodeableConceptBuilder::default()
            .with_text("Wellness Program")
            .build();
        let characteristic = CodeableConceptBuilder::default()
            .with_text("Wheelchair accessible")
            .build();
        let communication = CodeableConceptBuilder::default()
            .with_text("English")
            .build();
        let referral_method = CodeableConceptBuilder::default().with_text("Phone").build();
        let available_time = AvailableTimeBuilder::default()
            .with_days_of_week(vec![
                DaysOfWeek::Mon,
                DaysOfWeek::Tue,
                DaysOfWeek::Wed,
                DaysOfWeek::Thu,
                DaysOfWeek::Fri,
            ])
            .with_available_start_time("08:00")
            .with_available_end_time("17:00")
            .build();
        let not_available = NotAvailableBuilder::default()
            .with_desscription("Closed on public holidays")
            .with_during(
                PeriodBuilder::default()
                    .with_start("2025-12-25")
                    .with_end("2025-12-26")
                    .build(),
            )
            .build();
        let service_provision = CodeableConceptBuilder::default()
            .add_coding(
                CodingBuilder::default()
                    .with_system("http://example.com")
                    .with_code("home")
                    .with_display("Home visit")
                    .build(),
            )
            .build();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/ep-1")
            .build::<Endpoint>();
        let expected = HealthcareServiceBuilder::new("hcs-1")
            .with_active(true)
            .with_name("General Practice Service")
            .with_comment("Open Monday to Friday")
            .with_extra_details("Appointments available on the same day")
            .add_category(category)
            .add_type(r#type)
            .with_provided_by(provided_by)
            .add_location(location)
            .add_telecom(telecom)
            .add_coverage_area(coverage_area)
            .add_eligibility(eligibility)
            .add_probram(program)
            .add_characteristic(characteristic)
            .add_communication(communication)
            .add_referral_method(referral_method)
            .add_available_time(available_time)
            .add_not_available(not_available)
            .add_endpoint(endpoint)
            .add_service_provision_code(service_provision)
            .with_appointment_required(true)
            .with_availability_exceptions("Only during category 1 emergencies")
            .build();

        let actual = HealthcareService::from_json(data);

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
