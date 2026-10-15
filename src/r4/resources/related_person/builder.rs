use std::vec;

use crate::r4::{
    elements::{
        Address, Attachment, BackboneElement, CodeableConcept, ContactPoint, HumanName, Identifier,
        Reference,
    },
    resources::{
        DomainResource, Patient, RelatedPerson, RelatedPersonCommunication, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct RelatedPersonCommunicationBuilder {
    backbone_element: BackboneElement,
    language: Option<CodeableConcept>,
    preferred: Option<bool>,
}

impl RelatedPersonCommunicationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn language(mut self, language: CodeableConcept) -> Self {
        self.language = Some(language);
        self
    }

    pub fn preferred(mut self, preferred: bool) -> Self {
        self.preferred = Some(preferred);
        self
    }

    pub fn build(self) -> RelatedPersonCommunication {
        RelatedPersonCommunication {
            backbone_element: self.backbone_element,
            language: self.language,
            preferred: self.preferred,
        }
    }
}

#[derive(Default)]
pub struct RelatedPersonBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    patient: Option<Reference<Patient>>,
    relationship: Option<Vec<CodeableConcept>>,
    name: Option<Vec<HumanName>>,
    telecom: Option<Vec<ContactPoint>>,
    gender: Option<String>,
    birth_date: Option<String>,
    address: Option<Vec<Address>>,
    photo: Option<Vec<Attachment>>,
    period: Option<crate::r4::elements::complex_types::Period>,
    communication: Option<Vec<RelatedPersonCommunication>>,
    resource_type: String,
}

impl RelatedPersonBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            resource_type: RelatedPerson::get_resource_type(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn identifier(mut self, identifier: Vec<Identifier>) -> Self {
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

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn patient(mut self, patient: Reference<Patient>) -> Self {
        self.patient = Some(patient);
        self
    }

    pub fn relationship(mut self, relationship: Vec<CodeableConcept>) -> Self {
        self.relationship = Some(relationship);
        self
    }

    pub fn add_relationship(mut self, relationship: CodeableConcept) -> Self {
        match &mut self.relationship {
            Some(rel) => rel.push(relationship),
            None => self.relationship = Some(vec![relationship]),
        }
        self
    }

    pub fn name(mut self, name: Vec<HumanName>) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_name(mut self, name: HumanName) -> Self {
        match &mut self.name {
            Some(n) => n.push(name),
            None => self.name = Some(vec![name]),
        }
        self
    }

    pub fn telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(tel) => tel.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = Some(gender.into());
        self
    }

    pub fn birth_date(mut self, birth_date: impl Into<String>) -> Self {
        self.birth_date = Some(birth_date.into());
        self
    }

    pub fn address(mut self, address: Vec<Address>) -> Self {
        self.address = Some(address);
        self
    }

    pub fn add_address(mut self, address: Address) -> Self {
        match &mut self.address {
            Some(add) => add.push(address),
            None => self.address = Some(vec![address]),
        }
        self
    }

    pub fn photo(mut self, photo: Vec<Attachment>) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn add_photo(mut self, photo: Attachment) -> Self {
        match &mut self.photo {
            Some(p) => p.push(photo),
            None => self.photo = Some(vec![photo]),
        }
        self
    }

    pub fn period(mut self, period: crate::r4::elements::complex_types::Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn communication(mut self, communication: Vec<RelatedPersonCommunication>) -> Self {
        self.communication = Some(communication);
        self
    }

    pub fn add_communication(mut self, communication: RelatedPersonCommunication) -> Self {
        match &mut self.communication {
            Some(comm) => comm.push(communication),
            None => self.communication = Some(vec![communication]),
        }
        self
    }

    pub fn build(self) -> RelatedPerson {
        RelatedPerson {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            patient: self.patient,
            relationship: self.relationship,
            name: self.name,
            telecom: self.telecom,
            gender: self.gender,
            birth_date: self.birth_date,
            address: self.address,
            photo: self.photo,
            period: self.period,
            communication: self.communication,
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::r4::elements::Coding;

    use super::*;

    #[test]
    pub fn test_build_related_person_should_succeed() {
        let mut expected = RelatedPerson::default();
        expected.domain_resource.resource.id = Some("related-person-1".to_string());
        expected.gender = Some("female".to_string());
        
        let actual = RelatedPersonBuilder::new("related-person-1")
            .gender("female")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_related_person_communication_should_succeed() {
        let expected = RelatedPersonCommunication {
            backbone_element: BackboneElement {
                element: crate::r4::elements::element::Element {
                    id: Some("comm-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            language: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://tools.ietf.org/html/bcp47".to_string()),
                    code: Some("en".to_string()),
                    display: Some("English".to_string()),
                    ..Default::default()
                }]),
                text: Some("English".to_string()),
                ..Default::default()
            }),
            preferred: Some(true),
        };

        let language = CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://tools.ietf.org/html/bcp47".to_string()),
                code: Some("en".to_string()),
                display: Some("English".to_string()),
                ..Default::default()
            }]),
            text: Some("English".to_string()),
            ..Default::default()
        };

        let actual = RelatedPersonCommunicationBuilder::new("comm-1")
            .language(language)
            .preferred(true)
            .build();

        assert_eq!(expected, actual)
    }
}
