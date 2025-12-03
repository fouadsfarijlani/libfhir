use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        CodeableConcept, ContactPoint, GetResourceReferences, Identifier, Period, Reference,
        ReferenceTypes,
    },
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, ResourceType,
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
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

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{
            CodeableConceptBuilder, CodingBuilder, ContactPointBuilder, IdentifierBuilder,
            PeriodBuilder, ReferenceBuilder,
        },
        resources::OrganizationAffiliationBuilder,
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
            .with_reference("Organization/1")
            .with_display("Primary Health Org")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .with_display("Partner Healthcare Org")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_reference("Organization/3")
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
            .with_reference("Location/loc1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_reference("HealthcareService/hs1")
            .build::<HealthcareService>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/ep1")
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
            .with_reference("Organization/1")
            .with_display("Primary Health Org")
            .build::<Organization>();
        let participating_org = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .with_display("Partner Healthcare Org")
            .build::<Organization>();
        let network = ReferenceBuilder::default()
            .with_reference("Organization/3")
            .build::<Organization>();
        let location = ReferenceBuilder::default()
            .with_reference("Location/loc1")
            .build::<Location>();
        let healthcare_service = ReferenceBuilder::default()
            .with_reference("HealthcareService/hs1")
            .build::<HealthcareService>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
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
}
