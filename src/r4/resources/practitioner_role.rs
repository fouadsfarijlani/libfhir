use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        AvailableTime, CodeableConcept, ContactPoint, GetResourceReferences, Identifier,
        NotAvailable, Period, Reference, ReferenceTypes,
    },
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, Practitioner, Resource, ResourceType,
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
    pub availability_exception: Option<String>,
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
    availability_exception: Option<String>,
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

    pub fn add_healthcare_service(
        mut self,
        healthcare_service: Reference<HealthcareService>,
    ) -> Self {
        match &mut self.healthcare_service {
            Some(services) => services.push(healthcare_service),
            None => self.healthcare_service = Some(vec![healthcare_service]),
        }
        self
    }

    pub fn with_telecom(mut self, telecoms: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecoms);
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
            availability_exception: self.availability_exception,
            endpoint: self.endpoint,
        }
    }
}
