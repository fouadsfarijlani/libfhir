use crate::r4::{
    elements::{CodeableConcept, ContactPoint, Identifier, Period, Reference},
    resources::{
        DomainResource, Endpoint, HealthcareService, Location, Organization,
        OrganizationAffiliation, ResourceType,
    },
};

#[derive(Default)]
pub struct OrganizationAffiliationBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    period: Option<Period>,
    organization: Option<Reference<Organization>>,
    participating_organization: Option<Reference<Organization>>,
    network: Option<Vec<Reference<Organization>>>,
    code: Option<Vec<CodeableConcept>>,
    speciality: Option<Vec<CodeableConcept>>,
    location: Option<Vec<Reference<Location>>>,
    healthcare_service: Option<Vec<Reference<HealthcareService>>>,
    telecom: Option<Vec<ContactPoint>>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl OrganizationAffiliationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.domain_resource.resource.id = Some(id.into());
        builder
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

    pub fn with_organization(mut self, org: Reference<Organization>) -> Self {
        self.organization = Some(org);
        self
    }

    pub fn with_participating_organization(mut self, org: Reference<Organization>) -> Self {
        self.participating_organization = Some(org);
        self
    }

    pub fn with_network(mut self, networks: Vec<Reference<Organization>>) -> Self {
        self.network = Some(networks);
        self
    }

    pub fn add_network(mut self, network: Reference<Organization>) -> Self {
        match &mut self.network {
            Some(net) => net.push(network),
            None => self.network = Some(vec![network]),
        }
        self
    }

    pub fn with_code(mut self, codes: Vec<CodeableConcept>) -> Self {
        self.code = Some(codes);
        self
    }

    pub fn add_code(mut self, code: CodeableConcept) -> Self {
        match &mut self.code {
            Some(c) => c.push(code),
            None => self.code = Some(vec![code]),
        }
        self
    }

    pub fn with_speciality(mut self, specialies: Vec<CodeableConcept>) -> Self {
        self.speciality = Some(specialies);
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

    pub fn with_healthcare_service(
        mut self,
        healthcare_services: Vec<Reference<HealthcareService>>,
    ) -> Self {
        self.healthcare_service = Some(healthcare_services);
        self
    }

    pub fn add_healthcare_service(
        mut self,
        healthcare_service: Reference<HealthcareService>,
    ) -> Self {
        match &mut self.healthcare_service {
            Some(hs) => hs.push(healthcare_service),
            None => self.healthcare_service = Some(vec![healthcare_service]),
        }
        self
    }

    pub fn with_telecom(mut self, telecoms: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecoms);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(tc) => tc.push(telecom),
            None => self.telecom = Some(vec![telecom]),
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

    pub fn build(self) -> OrganizationAffiliation {
        OrganizationAffiliation {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            period: self.period,
            organization: self.organization,
            participating_organization: self.participating_organization,
            network: self.network,
            code: self.code,
            speciality: self.speciality,
            location: self.location,
            healthcare_service: self.healthcare_service,
            telecom: self.telecom,
            endpoint: self.endpoint,
            resource_type: OrganizationAffiliation::get_resource_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{
            CodeableConceptBuilder, Coding, CodingBuilder, PeriodBuilder, ReferenceBuilder,
        },
        resources::Resource,
    };

    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = OrganizationAffiliation {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("org-aff-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            active: Some(true),
            period: Some(Period {
                start: Some("2025-01-01".to_string()),
                end: Some("2025-12-31".to_string()),
                ..Default::default()
            }),
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
            code: None,
            speciality: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.com".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            location: Some(vec![Reference::<Location> {
                reference: Some("Location/1".to_string()),
                ..Default::default()
            }]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                reference: Some("HealthcareService/1".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_reference("Organization/3")
            .build::<Organization>();
        let period = PeriodBuilder::default()
            .with_start("2025-01-01")
            .with_end("2025-12-31")
            .build();
        let speciality_code = CodingBuilder::default()
            .with_system("http://example.com")
            .build();
        let speciality = CodeableConceptBuilder::default()
            .add_coding(speciality_code)
            .build();
        let location = ReferenceBuilder::default()
            .with_reference("Location/1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_reference("HealthcareService/1")
            .build::<HealthcareService>();
        let actual = OrganizationAffiliationBuilder::new("org-aff-1")
            .with_active(true)
            .with_organization(org)
            .with_participating_organization(participating_org)
            .add_network(network)
            .with_period(period)
            .add_speciality(speciality)
            .add_location(location)
            .add_healthcare_service(healthcare_service)
            .build();

        assert_eq!(expected, actual)
    }
}
