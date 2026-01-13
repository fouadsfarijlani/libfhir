use crate::r4::{
    elements::{
        AvailableTime, CodeableConcept, ContactPoint, Identifier, NotAvailable, Period, Reference,
    },
    resources::{
        DomainResource, Endpoint, HealthcareService, Location, Organization, Practitioner,
        PractitionerRole, Resource,
    },
};

#[derive(Default)]
pub struct PractitionerRoleBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    period: Option<Period>,
    practitioner: Option<Reference<Practitioner>>,
    organization: Option<Reference<Organization>>,
    code: Option<Vec<CodeableConcept>>,
    speciality: Option<Vec<CodeableConcept>>,
    location: Option<Vec<Reference<Location>>>,
    healthcare_service: Option<Vec<Reference<HealthcareService>>>,
    telecom: Option<Vec<ContactPoint>>,
    available_time: Option<Vec<AvailableTime>>,
    not_available: Option<Vec<NotAvailable>>,
    availability_exceptions: Option<String>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl PractitionerRoleBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PractitionerRoleBuilder {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn with_identifier(mut self, identifiers: Vec<Identifier>) -> Self {
        self.identifier = Some(identifiers);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn with_practitioner(mut self, practitioner: Reference<Practitioner>) -> Self {
        self.practitioner = Some(practitioner);
        self
    }

    pub fn with_organization(mut self, organization: Reference<Organization>) -> Self {
        self.organization = Some(organization);
        self
    }

    pub fn with_code(mut self, code: Vec<CodeableConcept>) -> Self {
        self.code = Some(code);
        self
    }

    pub fn add_code(mut self, code: CodeableConcept) -> Self {
        match &mut self.code {
            Some(c) => c.push(code),
            None => self.code = Some(vec![code]),
        }
        self
    }

    pub fn with_speciality(mut self, specialities: Vec<CodeableConcept>) -> Self {
        self.speciality = Some(specialities);
        self
    }

    pub fn add_speciality(mut self, speciality: CodeableConcept) -> Self {
        match &mut self.speciality {
            Some(spec) => spec.push(speciality),
            None => self.speciality = Some(vec![speciality]),
        }
        self
    }

    pub fn with_location(mut self, locations: Vec<Reference<Location>>) -> Self {
        self.location = Some(locations);
        self
    }
    pub fn add_location(mut self, location: Reference<Location>) -> Self {
        match &mut self.location {
            Some(locs) => locs.push(location),
            None => self.location = Some(vec![location]),
        }
        self
    }

    pub fn with_healthcare_service(mut self, services: Vec<Reference<HealthcareService>>) -> Self {
        self.healthcare_service = Some(services);
        self
    }

    pub fn add_healthcare_service(mut self, service: Reference<HealthcareService>) -> Self {
        match &mut self.healthcare_service {
            Some(services) => services.push(service),
            None => self.healthcare_service = Some(vec![service]),
        }
        self
    }

    pub fn with_telecom(mut self, telecoms: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecoms);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(tels) => tels.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn with_available_time(mut self, available_times: Vec<AvailableTime>) -> Self {
        self.available_time = Some(available_times);
        self
    }

    pub fn add_available_time(mut self, available_time: AvailableTime) -> Self {
        match &mut self.available_time {
            Some(av) => av.push(available_time),
            None => self.available_time = Some(vec![available_time]),
        }
        self
    }

    pub fn with_not_available(mut self, not_available: Vec<NotAvailable>) -> Self {
        self.not_available = Some(not_available);
        self
    }

    pub fn add_not_available_time(mut self, not_available: NotAvailable) -> Self {
        match &mut self.not_available {
            Some(na) => na.push(not_available),
            None => self.not_available = Some(vec![not_available]),
        }
        self
    }

    pub fn with_availability_exceptions(mut self, exception: impl Into<String>) -> Self {
        self.availability_exceptions = Some(exception.into());
        self
    }

    pub fn with_endpoint(mut self, endpoints: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoints);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        match &mut self.endpoint {
            Some(eps) => eps.push(endpoint),
            None => self.endpoint = Some(vec![endpoint]),
        }
        self
    }

    pub fn build(self) -> PractitionerRole {
        PractitionerRole {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            period: self.period,
            practitioner: self.practitioner,
            organization: self.organization,
            code: self.code,
            speciality: self.speciality,
            location: self.location,
            healthcare_service: self.healthcare_service,
            telecom: self.telecom,
            available_time: self.available_time,
            not_available: self.not_available,
            availability_exceptions: self.availability_exceptions,
            endpoint: self.endpoint,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::{Coding, DaysOfWeek, ReferenceBuilder};

    use super::*;

    #[test]
    fn test_build_should_succeed() {
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
            identifier: Some(vec![Identifier {
                system: Some("fancy".to_string()),
                value: Some("ident-1".to_string()),
                ..Default::default()
            }]),
            period: Some(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2026-01-01".to_string()),
                ..Default::default()
            }),
        };

        let actual = PractitionerRoleBuilder::new("pr1")
            .with_active(true)
            .with_practitioner(
                ReferenceBuilder::default()
                    .with_reference("Practitioner/prac1")
                    .build::<Practitioner>(),
            )
            .with_organization(
                ReferenceBuilder::default()
                    .with_reference("Organization/org1")
                    .build::<Organization>(),
            )
            .with_code(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.com".to_string()),
                    code: Some("doctor".to_string()),
                    display: Some("Doctor".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }])
            .with_speciality(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.com".to_string()),
                    code: Some("cardiology".to_string()),
                    display: Some("Cardiology".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }])
            .add_location(Reference::<Location> {
                reference: Some("Location/location1".to_string()),
                ..Default::default()
            })
            .add_healthcare_service(Reference::<HealthcareService> {
                reference: Some("HealthcareService/hs1".to_string()),
                ..Default::default()
            })
            .add_telecom(ContactPoint {
                system: Some("phone".to_string()),
                value: Some("123456789".to_string()),
                r#use: Some("work".to_string()),
                ..Default::default()
            })
            .add_available_time(AvailableTime {
                days_of_week: Some(vec![DaysOfWeek::Mon, DaysOfWeek::Tue, DaysOfWeek::Wed]),
                available_start_time: Some("08:00:00".to_string()),
                available_end_time: Some("16:00:00".to_string()),
                ..Default::default()
            })
            .add_not_available_time(NotAvailable {
                description: "Closed during holidays".to_string(),
                during: Some(Period {
                    start: Some("2025-12-24".to_string()),
                    end: Some("2025-12-26".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with_availability_exceptions("Reduced hours during summer")
            .add_endpoint(Reference::<Endpoint> {
                reference: Some("Endpoint/ep1".to_string()),
                ..Default::default()
            })
            .add_identifier(Identifier {
                system: Some("fancy".to_string()),
                value: Some("ident-1".to_string()),
                ..Default::default()
            })
            .with_period(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2026-01-01".to_string()),
                ..Default::default()
            })
            .build();

        assert_eq!(expected, actual)
    }
}
