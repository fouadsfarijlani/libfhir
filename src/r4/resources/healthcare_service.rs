use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        Attachement, BackboneElement, CodeableConcept, ContactPoint, GetResourceReferences,
        Identifier, Period, Reference, ReferenceTypes,
    },
    resources::{self, DomainResource, Endpoint, Location, Organization, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Eligibility {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub code: Option<CodeableConcept>,
    pub comment: Option<String>,
}

impl ResourceType for Eligibility {
    const TYPE: &'static str = "Eligibility";
}

#[derive(Default)]
pub struct EligibilityBuilder {
    backbone_element: BackboneElement,
    code: Option<CodeableConcept>,
    comment: Option<String>,
}

impl EligibilityBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn with_code(mut self, code: CodeableConcept) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn build(self) -> Eligibility {
        Eligibility {
            backbone_element: self.backbone_element,
            code: self.code,
            comment: self.comment,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct AvailableTime {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub days_of_week: Option<Vec<String>>, // TODO: Enum instead
    pub all_day: Option<bool>,
    pub available_start_time: Option<String>, // to be addressed later
    pub available_end_time: Option<String>,   // to be addressed later
}

impl ResourceType for AvailableTime {
    const TYPE: &'static str = "AvailableTime";
}

#[derive(Default)]
pub struct AvailableTimeBuilder {
    backbone_element: BackboneElement,
    days_of_week: Option<Vec<String>>, // TODO: Enum instead
    all_day: Option<bool>,
    available_start_time: Option<String>, // to be addressed later
    available_end_time: Option<String>,   // to be addressed later
}

impl AvailableTimeBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_days_of_week(mut self, days_of_week: Vec<String>) -> Self {
        self.days_of_week = Some(days_of_week);
        self
    }

    pub fn add_day_of_week(mut self, day: impl Into<String>) -> Self {
        match &mut self.days_of_week {
            Some(dow) => dow.push(day.into()),
            None => self.days_of_week = Some(vec![day.into()]),
        }
        self
    }

    pub fn with_all_day(mut self, all_day: bool) -> Self {
        self.all_day = Some(all_day);
        self
    }

    pub fn with_available_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.available_start_time = Some(start_time.into());
        self
    }

    pub fn with_available_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.available_end_time = Some(end_time.into());
        self
    }

    pub fn build(self) -> AvailableTime {
        AvailableTime {
            backbone_element: self.backbone_element,
            days_of_week: self.days_of_week,
            all_day: self.all_day,
            available_start_time: self.available_start_time,
            available_end_time: self.available_end_time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NotAvailable {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub description: String,
    pub during: Option<Period>,
}

impl ResourceType for NotAvailable {
    const TYPE: &'static str = "NotAvailable";
}
#[derive(Default)]
pub struct NotAvailableBuilder {
    backbone_element: BackboneElement,
    description: String,
    during: Option<Period>,
}

impl NotAvailableBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_desscription(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_during(mut self, during: Period) -> Self {
        self.during = Some(during);
        self
    }

    pub fn build(self) -> NotAvailable {
        NotAvailable {
            backbone_element: self.backbone_element,
            description: self.description,
            during: self.during,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    fn get_references(&self) -> Vec<crate::elements::ReferenceTypes> {
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

#[derive(Default)]
pub struct HealthcareServiceBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    provided_by: Option<Reference<Organization>>,
    category: Option<Vec<CodeableConcept>>,
    r#type: Option<Vec<CodeableConcept>>,
    location: Option<Vec<Reference<Location>>>,
    name: Option<String>,
    comment: Option<String>,
    extra_details: Option<String>,
    photo: Option<Attachement>,
    telecom: Option<Vec<ContactPoint>>,
    coverage_area: Option<Vec<Reference<Location>>>,
    service_provision_code: Option<Vec<CodeableConcept>>,
    eligibility: Option<Vec<Eligibility>>,
    program: Option<Vec<CodeableConcept>>,
    characteristic: Option<Vec<CodeableConcept>>,
    communication: Option<Vec<CodeableConcept>>,
    referral_method: Option<Vec<CodeableConcept>>,
    appointment_required: Option<bool>,
    available_time: Option<Vec<AvailableTime>>,
    not_available: Option<Vec<NotAvailable>>,
    availability_exceptions: Option<String>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl HealthcareServiceBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.domain_resource.resource.id = Some(id.into());
        builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn with_identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(v) => v.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn with_provided_by(mut self, provided_by: Reference<Organization>) -> Self {
        self.provided_by = Some(provided_by);
        self
    }

    pub fn with_category(mut self, category: Vec<CodeableConcept>) -> Self {
        self.category = Some(category);
        self
    }

    pub fn add_category(mut self, category: CodeableConcept) -> Self {
        match &mut self.category {
            Some(v) => v.push(category),
            None => self.category = Some(vec![category]),
        }
        self
    }

    pub fn with_type(mut self, r#type: Vec<CodeableConcept>) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn add_type(mut self, r#type: CodeableConcept) -> Self {
        match &mut self.r#type {
            Some(v) => v.push(r#type),
            None => self.r#type = Some(vec![r#type]),
        }
        self
    }

    pub fn with_location(mut self, locations: Vec<Reference<Location>>) -> Self {
        self.location = Some(locations);
        self
    }

    pub fn add_location(mut self, location: Reference<Location>) -> Self {
        match &mut self.location {
            Some(v) => v.push(location),
            None => self.location = Some(vec![location]),
        }
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn with_extra_details(mut self, details: impl Into<String>) -> Self {
        self.extra_details = Some(details.into());
        self
    }

    pub fn with_photo(mut self, photo: Attachement) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn with_telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(v) => v.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn with_coverage_area(mut self, coverage_areas: Vec<Reference<Location>>) -> Self {
        self.coverage_area = Some(coverage_areas);
        self
    }

    pub fn add_coverage_area(mut self, coverage_area: Reference<Location>) -> Self {
        match &mut self.coverage_area {
            Some(v) => v.push(coverage_area),
            None => self.coverage_area = Some(vec![coverage_area]),
        }
        self
    }

    pub fn with_service_provision_code(mut self, codes: Vec<CodeableConcept>) -> Self {
        self.service_provision_code = Some(codes);
        self
    }

    pub fn add_service_provision_code(mut self, code: CodeableConcept) -> Self {
        match &mut self.service_provision_code {
            Some(v) => v.push(code),
            None => self.service_provision_code = Some(vec![code]),
        }
        self
    }

    pub fn with_eligibility(mut self, eligibilities: Vec<Eligibility>) -> Self {
        self.eligibility = Some(eligibilities);
        self
    }

    pub fn add_eligibility(mut self, eligibility: Eligibility) -> Self {
        match &mut self.eligibility {
            Some(el) => el.push(eligibility),
            None => self.eligibility = Some(vec![eligibility]),
        }
        self
    }

    pub fn with_program(mut self, pograms: Vec<CodeableConcept>) -> Self {
        self.program = Some(pograms);
        self
    }

    pub fn add_probram(mut self, program: CodeableConcept) -> Self {
        match &mut self.program {
            Some(p) => p.push(program),
            None => self.program = Some(vec![program]),
        }
        self
    }

    pub fn with_characteristic(mut self, c: Vec<CodeableConcept>) -> Self {
        self.characteristic = Some(c);
        self
    }

    pub fn add_characteristic(mut self, characteristic: CodeableConcept) -> Self {
        match &mut self.characteristic {
            Some(c) => c.push(characteristic),
            None => self.characteristic = Some(vec![characteristic]),
        }
        self
    }

    pub fn with_communication(mut self, communications: Vec<CodeableConcept>) -> Self {
        self.communication = Some(communications);
        self
    }

    pub fn add_communication(mut self, communication: CodeableConcept) -> Self {
        match &mut self.communication {
            Some(c) => c.push(communication),
            None => self.communication = Some(vec![communication]),
        }
        self
    }

    pub fn with_referral_method(mut self, referral_method: Vec<CodeableConcept>) -> Self {
        self.referral_method = Some(referral_method);
        self
    }

    pub fn add_referral_method(mut self, referral_method: CodeableConcept) -> Self {
        match &mut self.referral_method {
            Some(rm) => rm.push(referral_method),
            None => self.referral_method = Some(vec![referral_method]),
        }
        self
    }

    pub fn with_appointment_required(mut self, value: bool) -> Self {
        self.appointment_required = Some(value);
        self
    }

    pub fn with_available_time(mut self, available_times: Vec<AvailableTime>) -> Self {
        self.available_time = Some(available_times);
        self
    }

    pub fn add_available_time(mut self, available_time: AvailableTime) -> Self {
        match &mut self.available_time {
            Some(at) => at.push(available_time),
            None => self.available_time = Some(vec![available_time]),
        }
        self
    }

    pub fn with_not_available(mut self, not_available: Vec<NotAvailable>) -> Self {
        self.not_available = Some(not_available);
        self
    }

    pub fn add_not_available(mut self, not_available: NotAvailable) -> Self {
        match &mut self.not_available {
            Some(na) => na.push(not_available),
            None => self.not_available = Some(vec![not_available]),
        }
        self
    }

    pub fn with_availability_exceptions(mut self, exceptions: impl Into<String>) -> Self {
        self.availability_exceptions = Some(exceptions.into());
        self
    }

    pub fn with_endpoint(mut self, endpoints: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoints);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        match &mut self.endpoint {
            Some(ep) => ep.push(endpoint),
            None => self.endpoint = Some(vec![endpoint]),
        }
        self
    }

    pub fn build(self) -> HealthcareService {
        HealthcareService {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            provided_by: self.provided_by,
            category: self.category,
            r#type: self.r#type,
            location: self.location,
            name: self.name,
            comment: self.comment,
            extra_details: self.extra_details,
            photo: self.photo,
            telecom: self.telecom,
            coverage_area: self.coverage_area,
            service_provision_code: self.service_provision_code,
            eligibility: self.eligibility,
            program: self.program,
            characteristic: self.characteristic,
            communication: self.communication,
            referral_method: self.referral_method,
            appointment_required: self.appointment_required,
            available_time: self.available_time,
            not_available: self.not_available,
            availability_exceptions: self.availability_exceptions,
            endpoint: self.endpoint,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{
            CodeableConceptBuilder, CodingBuilder, ContactPointBuilder, Element, PeriodBuilder,
            ReferenceBuilder,
        },
        resources::Resource,
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
                "mon".to_string(),
                "tue".to_string(),
                "wed".to_string(),
                "thu".to_string(),
                "fri".to_string(),
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

    #[test]
    fn test_build_should_succeeed() {
        let expected = HealthcareService {
            domain_resource: DomainResource {
                resource: Resource {
                    id: None,
                    implicit_rules: None,
                    meta: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            identifier: None,
            active: None,
            provided_by: None,
            category: None,
            r#type: None,
            location: None,
            name: None,
            comment: None,
            extra_details: None,
            photo: None,
            telecom: None,
            coverage_area: None,
            service_provision_code: None,
            eligibility: Some(vec![Eligibility {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                code: Some(CodeableConcept {
                    coding: None,
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    text: Some("some text".to_string()),
                }),
                comment: None,
            }]),
            program: None,
            characteristic: None,
            communication: None,
            referral_method: None,
            appointment_required: None,
            available_time: Some(vec![AvailableTime {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                days_of_week: Some(vec!["mon".to_string(), "thu".to_string()]),
                all_day: Some(false),
                available_start_time: Some("08:00".to_string()),
                available_end_time: Some("17:00".to_string()),
            }]),
            not_available: Some(vec![NotAvailable {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                description: "closed on holidays".to_string(),
                during: Some(Period {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    start: Some("2025-01-01".to_string()),
                    end: Some("2030-01-01".to_string()),
                }),
            }]),
            availability_exceptions: None,
            endpoint: None,
        };
        let eligibility = EligibilityBuilder::default()
            .with_code(
                CodeableConceptBuilder::default()
                    .with_text("some text")
                    .build(),
            )
            .build();
        let available_time = AvailableTimeBuilder::default()
            .add_day_of_week("mon")
            .add_day_of_week("thu")
            .with_all_day(false)
            .with_available_start_time("08:00")
            .with_available_end_time("17:00")
            .build();
        let not_available = NotAvailableBuilder::default()
            .with_desscription("closed on holidays")
            .with_during(
                PeriodBuilder::default()
                    .with_start("2025-01-01")
                    .with_end("2030-01-01")
                    .build(),
            )
            .build();

        let actual = HealthcareServiceBuilder::default()
            .add_eligibility(eligibility)
            .add_available_time(available_time)
            .add_not_available(not_available)
            .build();

        assert_eq!(expected, actual)
    }
}
