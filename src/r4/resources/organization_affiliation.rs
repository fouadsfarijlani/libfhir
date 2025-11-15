use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        CodeableConcept, ContactPoint, GetResourceReferences, Identifier, Period, Reference,
        ReferenceTypes,
    },
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, ResourceType,
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct OrganizationAffiliation {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub period: Option<Period>,
    pub organization: Option<Reference<Organization>>,
    pub participating_organization: Option<Reference<Organization>>,
    pub network: Option<Vec<Reference<Organization>>>,
    pub code: Option<Vec<CodeableConcept>>,
    pub speciality: Option<Vec<CodeableConcept>>,
    pub location: Option<Vec<Reference<Location>>>,
    pub healthcare_service: Option<Vec<Reference<HealthcareService>>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl ResourceType for OrganizationAffiliation {
    const TYPE: &'static str = "OrganizationAffiliation";
}

impl OrganizationAffiliation {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
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
        }
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{
        elements::{
            CodeableConceptBuilder, Coding, CodingBuilder, ContactPointBuilder, Element,
            IdentifierBuilder, PeriodBuilder, ReferenceBuilder,
        },
        resources::Resource,
    };

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
{
  "resourceType": "OrganizationAffiliation",
  "id": "org-aff-1",
  "identifier": [
    {
      "system": "http://example.org/affiliations",
      "value": "AFF-12345"
    }
  ],
  "active": true,
  "period": {
    "start": "2020-01-01",
    "end": "2030-01-01"
    },
  "organization": {
    "reference": "Organization/1",
    "display": "Primary Health Org"
  },
  "participatingOrganization": {
    "reference": "Organization/2",
    "display": "Partner Healthcare Org"
  },
  "network": [
    { "reference": "Organization/3" }
  ],
  "code": [
    {
      "coding": [
        {
          "system": "http://example.org/codes",
          "code": "provider",
          "display": "Provider"
        }
      ],
      "text": "Provider Role"
    }
  ],
  "speciality": [
    {
      "coding": [
        {
          "system": "http://example.com/sct",
          "code": "408443003",
          "display": "General medical practice"
        }
      ],
      "text": "General Practice"
    }
  ],
  "location": [
    { "reference": "Location/loc1" }
  ],
  "healthcareService": [
    { "reference": "HealthcareService/hs1" }
  ],
  "telecom": [
    {
      "system": "phone",
      "value": "12345678"
    }
  ],
  "endpoint": [
    { "reference": "Endpoint/ep1" }
  ]
}
"#;
        let identifier = IdentifierBuilder::default()
            .with_system("http://example.org/affiliations")
            .with_value("AFF-12345")
            .build();
        let org = ReferenceBuilder::default()
            .with_refernece("Organization/1")
            .with_display("Primary Health Org")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_refernece("Organization/2")
            .with_display("Partner Healthcare Org")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_refernece("Organization/3")
            .build::<Organization>();

        let coding = CodingBuilder::default()
            .with_system("http://example.org/codes")
            .with_code("provider")
            .with_display("Provider")
            .build();
        let code = CodeableConceptBuilder::default()
            .add_coding(coding)
            .with_text("Provider Role")
            .build();
        let code_speciality = CodingBuilder::default()
            .with_system("http://example.com/sct")
            .with_code("408443003")
            .with_display("General medical practice")
            .build();
        let speciality = CodeableConceptBuilder::default()
            .add_coding(code_speciality)
            .with_text("General Practice")
            .build();
        let location = ReferenceBuilder::default()
            .with_refernece("Location/loc1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_refernece("HealthcareService/hs1")
            .build::<HealthcareService>();
        let endpoint = ReferenceBuilder::default()
            .with_refernece("Endpoint/ep1")
            .build::<Endpoint>();
        let period = PeriodBuilder::default()
            .with_start("2020-01-01")
            .with_end("2030-01-01")
            .build();
        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("12345678")
            .build();

        let expected = OrganizationAffiliationBuilder::new("org-aff-1")
            .add_identifier(identifier)
            .with_active(true)
            .with_organization(org)
            .with_participating_organization(participating_org)
            .add_network(network)
            .add_code(code)
            .add_speciality(speciality)
            .add_location(location)
            .add_healthcare_service(healthcare_service)
            .add_endpoint(endpoint)
            .with_period(period)
            .add_telecom(telecom)
            .build();

        let actual = OrganizationAffiliation::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let org = ReferenceBuilder::default()
            .with_refernece("Organization/1")
            .with_display("Primary Health Org")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_refernece("Organization/2")
            .with_display("Partner Healthcare Org")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_refernece("Organization/3")
            .build::<Organization>();
        let location = ReferenceBuilder::default()
            .with_refernece("Location/loc1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_refernece("HealthcareService/hs1")
            .build::<HealthcareService>();
        let endpoint = ReferenceBuilder::default()
            .with_refernece("Endpoint/1")
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

    #[test]
    fn test_build_should_succeed() {
        let expected = OrganizationAffiliation {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("org-aff-1".to_string()),
                    meta: None,
                    implicit_rules: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            active: Some(true),
            period: Some(Period {
                element: Element {
                    id: None,
                    extention: None,
                },
                start: Some("2025-01-01".to_string()),
                end: Some("2025-12-31".to_string()),
            }),
            organization: Some(Reference::<Organization> {
                element: Element {
                    id: None,
                    extention: None,
                },
                reference: Some("Organization/1".to_string()),
                identifier: None,
                r#type: None,
                display: None,
                _m: PhantomData,
            }),
            participating_organization: Some(Reference::<Organization> {
                element: Element {
                    id: None,
                    extention: None,
                },
                reference: Some("Organization/2".to_string()),
                identifier: None,
                r#type: None,
                display: None,
                _m: PhantomData,
            }),
            network: Some(vec![Reference::<Organization> {
                element: Element {
                    id: None,
                    extention: None,
                },
                reference: Some("Organization/3".to_string()),
                identifier: None,
                r#type: None,
                display: None,
                _m: PhantomData,
            }]),
            code: None,
            speciality: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    system: Some("http://example.com".to_string()),
                    display: None,
                    code: None,
                    version: None,
                    user_selected: None,
                }]),
                element: Element {
                    id: None,
                    extention: None,
                },
                text: None,
            }]),
            location: Some(vec![Reference::<Location> {
                element: Element {
                    id: None,
                    extention: None,
                },
                reference: Some("Location/1".to_string()),
                identifier: None,
                r#type: None,
                display: None,
                _m: PhantomData,
            }]),
            healthcare_service: Some(vec![Reference::<HealthcareService> {
                element: Element {
                    id: None,
                    extention: None,
                },
                reference: Some("HealthcareService/1".to_string()),
                identifier: None,
                r#type: None,
                display: None,
                _m: PhantomData,
            }]),
            telecom: None,
            endpoint: None,
            identifier: None,
        };
        let org = ReferenceBuilder::default()
            .with_refernece("Organization/1")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_refernece("Organization/2")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_refernece("Organization/3")
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
            .with_refernece("Location/1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_refernece("HealthcareService/1")
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
