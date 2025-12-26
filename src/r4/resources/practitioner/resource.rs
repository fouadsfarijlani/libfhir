use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{
        Address, Attachement, BackboneElement, CodeableConcept, ContactPoint,
        GetResourceReferences, HumanName, Identifier, Period, Reference, ReferenceTypes,
    },
    resources::{self, DomainResource, Organization, ResourceType},
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct PractitionerQualification {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub identifier: Option<Vec<Identifier>>,
    pub code: CodeableConcept,
    pub period: Option<Vec<Period>>,
    pub issuer: Option<Reference<Organization>>,
}

impl ResourceType for PractitionerQualification {
    const TYPE: &'static str = "PractitionerQualification";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum Gender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Practitioner {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub name: Option<Vec<HumanName>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Vec<Address>>,
    pub gender: Option<Gender>,
    pub birth_date: Option<String>, // Date for later
    pub photo: Option<Vec<Attachement>>,
    pub qualification: Option<Vec<PractitionerQualification>>,
    pub communication: Option<Vec<CodeableConcept>>,
}

impl ResourceType for Practitioner {
    const TYPE: &'static str = "Practitioner";
}

impl Practitioner {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

impl GetResourceReferences for Practitioner {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(qualifications) = &self.qualification {
            for q in qualifications {
                if let Some(issuer) = &q.issuer {
                    references.push(ReferenceTypes::from(issuer));
                }
            }
        }

        references
    }
}

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{
            AddressBuilder, AttachmentBuilder, CodeableConceptBuilder, ContactPointBuilder,
            HumanNameBuilder, IdentifierBuilder, ReferenceBuilder,
        },
        resources::{PractitionerBuilder, PractitionerQualificationBuilder},
    };

    use super::*;

    #[test]
    pub fn test_practitioner() {
        let data = r#"
{
  "resourceType": "Practitioner",
  "id": "pr-1",
  "identifier": [
    {
      "system": "http://example.com",
      "value": "12345"
    }
  ],
  "active": true,
  "name": [
    {
      "use": "official",
      "family": "Doe",
      "given": ["John"]
    }
  ],
  "telecom": [
    {
      "system": "phone",
      "value": "12345"
    }
  ],
  "address": [
    {
      "line": ["123 Main Street"],
      "city": "Exampleville",
      "state": "EX",
      "postalCode": "12345",
      "country": "Exampleland"
    }
  ],
  "gender": "male",
  "birthDate": "1980-01-01",
  "photo": [
    {
      "contentType": "image/png",
      "url": "http://example.com"
    }
  ],
  "qualification": [
    {
      "code": {
        "text": "General Medical Practitioner"
      }
    }
  ],
  "communication": [
    {
      "text": "English"
    }
  ]
}
"#;
        let identifier = IdentifierBuilder::default()
            .with_system("http://example.com")
            .with_value("12345")
            .build();
        let name = HumanNameBuilder::default()
            .with_use("official")
            .with_family("Doe")
            .add_given("John")
            .build();
        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("12345")
            .build();
        let address = AddressBuilder::default()
            .add_line("123 Main Street")
            .with_city("Exampleville")
            .with_state("EX")
            .with_postal_code("12345")
            .with_country("Exampleland")
            .build();
        let photo = AttachmentBuilder::default()
            .with_content_type("image/png")
            .with_url("http://example.com")
            .build();
        let qualification = PractitionerQualificationBuilder::default()
            .with_code(
                CodeableConceptBuilder::default()
                    .with_text("General Medical Practitioner")
                    .build(),
            )
            .build();
        let communication = CodeableConceptBuilder::default()
            .with_text("English")
            .build();
        let expected = PractitionerBuilder::new("pr-1")
            .add_identifier(identifier)
            .with_active(true)
            .add_name(name)
            .add_telecom(telecom)
            .add_address(address)
            .with_gender(Gender::Male)
            .with_birth_date("1980-01-01")
            .add_photo(photo)
            .add_qualification(qualification)
            .add_communication(communication)
            .build();

        let actual = Practitioner::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let issuer_1 = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let issuer_2 = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .build::<Organization>();
        let qualification_1 = PractitionerQualificationBuilder::default()
            .with_issuer(issuer_1.clone())
            .build();
        let qualification_2 = PractitionerQualificationBuilder::default()
            .with_issuer(issuer_2.clone())
            .build();
        let practitioner = PractitionerBuilder::default()
            .with_qualification(vec![qualification_1, qualification_2])
            .build();
        let expected = vec![
            ReferenceTypes::from(&issuer_1),
            ReferenceTypes::from(&issuer_2),
        ];

        let actual = practitioner.get_references();

        assert_eq!(expected, actual)
    }
}
