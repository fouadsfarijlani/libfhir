use crate::r4::{
    elements::{
        Attachment, AvailableTime, BackboneElement, CodeableConcept, ContactPoint, Identifier,
        NotAvailable, Reference,
    },
    resources::{
        DomainResource, Eligibility, Endpoint, HealthcareService, Location, Organization,
        ResourceType,
    },
};

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
    photo: Option<Attachment>,
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

    pub fn with_photo(mut self, photo: Attachment) -> Self {
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
            resource_type: HealthcareService::get_resource_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{
            AvailableTime, AvailableTimeBuilder, BackboneElement, CodeableConcept,
            CodeableConceptBuilder, DaysOfWeek, Element, NotAvailable, NotAvailableBuilder, Period,
            PeriodBuilder,
        },
        resources::{
            DomainResource, Eligibility, EligibilityBuilder, HealthcareService,
            HealthcareServiceBuilder,
        },
    };

    #[test]
    fn test_build_should_succeeed() {
        let expected = HealthcareService {
            domain_resource: DomainResource {
                ..Default::default()
            },
            eligibility: Some(vec![Eligibility {
                code: Some(CodeableConcept {
                    text: Some("some text".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            available_time: Some(vec![AvailableTime {
                backbone_element: BackboneElement {
                    ..Default::default()
                },
                days_of_week: Some(vec![DaysOfWeek::Mon, DaysOfWeek::Thu]),
                all_day: Some(false),
                available_start_time: Some("08:00".to_string()),
                available_end_time: Some("17:00".to_string()),
            }]),
            not_available: Some(vec![NotAvailable {
                backbone_element: BackboneElement {
                    ..Default::default()
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
            ..Default::default()
        };
        let eligibility = EligibilityBuilder::default()
            .with_code(
                CodeableConceptBuilder::default()
                    .with_text("some text")
                    .build(),
            )
            .build();
        let available_time = AvailableTimeBuilder::default()
            .add_day_of_week(DaysOfWeek::Mon)
            .add_day_of_week(DaysOfWeek::Thu)
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
