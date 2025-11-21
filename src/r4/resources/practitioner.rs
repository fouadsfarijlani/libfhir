use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        Address, Attachement, BackboneElement, CodeableConcept, ContactPoint, Element,
        GetResourceReferences, HumanName, Identifier, Period, Reference, ReferenceTypes,
    },
    resources::{self, DomainResource, Organization, Resource, ResourceType},
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

#[derive(Default)]
pub struct PractitionerQualificationBuilder {
    backbone_element: BackboneElement,
    identifier: Option<Vec<Identifier>>,
    code: CodeableConcept,
    period: Option<Vec<Period>>,
    issuer: Option<Reference<Organization>>,
}

impl PractitionerQualificationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PractitionerQualificationBuilder {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some(id.into()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            ..Default::default()
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn with_identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn with_code(mut self, code: CodeableConcept) -> Self {
        self.code = code;
        self
    }

    pub fn with_period(mut self, period: Vec<Period>) -> Self {
        self.period = Some(period);
        self
    }

    pub fn add_period(mut self, period: Period) -> Self {
        match &mut self.period {
            Some(p) => p.push(period),
            None => self.period = Some(vec![period]),
        }
        self
    }

    pub fn with_issuer(mut self, issuer: Reference<Organization>) -> Self {
        self.issuer = Some(issuer);
        self
    }

    pub fn build(self) -> PractitionerQualification {
        PractitionerQualification {
            backbone_element: self.backbone_element,
            identifier: self.identifier,
            code: self.code,
            period: self.period,
            issuer: self.issuer,
        }
    }
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

#[derive(Default)]
pub struct PractitionerBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    name: Option<Vec<HumanName>>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Vec<Address>>,
    gender: Option<Gender>,     // TODO: This should be a code
    birth_date: Option<String>, // Date for later
    photo: Option<Vec<Attachement>>,
    qualification: Option<Vec<PractitionerQualification>>,
    communication: Option<Vec<CodeableConcept>>,
}

impl PractitionerBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PractitionerBuilder {
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

    pub fn with_identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn with_active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn with_name(mut self, names: Vec<HumanName>) -> Self {
        self.name = Some(names);
        self
    }

    pub fn add_name(mut self, name: HumanName) -> Self {
        match &mut self.name {
            Some(n) => n.push(name),
            None => self.name = Some(vec![name]),
        }
        self
    }

    pub fn with_telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(t) => t.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn with_address(mut self, addresses: Vec<Address>) -> Self {
        self.address = Some(addresses);
        self
    }

    pub fn add_address(mut self, address: Address) -> Self {
        match &mut self.address {
            Some(a) => a.push(address),
            None => self.address = Some(vec![address]),
        }
        self
    }

    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn with_birth_date(mut self, date: impl Into<String>) -> Self {
        self.birth_date = Some(date.into());
        self
    }

    pub fn with_photo(mut self, photos: Vec<Attachement>) -> Self {
        self.photo = Some(photos);
        self
    }

    pub fn add_photo(mut self, photo: Attachement) -> Self {
        match &mut self.photo {
            Some(p) => p.push(photo),
            None => self.photo = Some(vec![photo]),
        }
        self
    }

    pub fn with_qualification(mut self, qualifications: Vec<PractitionerQualification>) -> Self {
        self.qualification = Some(qualifications);
        self
    }

    pub fn add_qualification(mut self, qualification: PractitionerQualification) -> Self {
        match &mut self.qualification {
            Some(q) => q.push(qualification),
            None => self.qualification = Some(vec![qualification]),
        }
        self
    }

    pub fn with_communication(mut self, communication: Vec<CodeableConcept>) -> Self {
        self.communication = Some(communication);
        self
    }

    pub fn add_communication(mut self, communication: CodeableConcept) -> Self {
        match &mut self.communication {
            Some(q) => q.push(communication),
            None => self.communication = Some(vec![communication]),
        }
        self
    }

    pub fn build(self) -> Practitioner {
        Practitioner {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            name: self.name,
            telecom: self.telecom,
            address: self.address,
            gender: self.gender,
            birth_date: self.birth_date,
            photo: self.photo,
            qualification: self.qualification,
            communication: self.communication,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::elements::{
        AddressBuilder, AttachmentBuilder, CodeableConceptBuilder, ContactPointBuilder,
        HumanNameBuilder, IdentifierBuilder, ReferenceBuilder,
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
    fn test_build_should_succeed() {
        let expected = Practitioner {
            qualification: Some(vec![PractitionerQualification {
                code: CodeableConcept {
                    text: Some("very qualified".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }]),
            ..Default::default()
        };
        let qualification = PractitionerQualificationBuilder::default()
            .with_code(
                CodeableConceptBuilder::default()
                    .with_text("very qualified")
                    .build(),
            )
            .build();

        let actual = PractitionerBuilder::default()
            .add_qualification(qualification)
            .build();

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
