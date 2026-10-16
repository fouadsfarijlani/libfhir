use std::vec;

use crate::r4::{
    elements::{
        Address, Attachment, BackboneElement, CodeableConcept, ContactPoint, HumanName, Identifier,
        Reference,
    },
    resources::{
        DomainResource, Organization, Person, PersonLink, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct PersonLinkBuilder {
    backbone_element: BackboneElement,
    target: Option<Reference<Organization>>,
    assurance: Option<String>,
}

impl PersonLinkBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn target(mut self, target: Reference<Organization>) -> Self {
        self.target = Some(target);
        self
    }

    pub fn assurance(mut self, assurance: impl Into<String>) -> Self {
        self.assurance = Some(assurance.into());
        self
    }

    pub fn build(self) -> PersonLink {
        PersonLink {
            backbone_element: self.backbone_element,
            target: self.target,
            assurance: self.assurance,
        }
    }
}

#[derive(Default)]
pub struct PersonBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    name: Option<Vec<HumanName>>,
    telecom: Option<Vec<ContactPoint>>,
    gender: Option<String>,
    birth_date: Option<String>,
    address: Option<Vec<Address>>,
    photo: Option<Vec<Attachment>>,
    managing_organization: Option<Reference<Organization>>,
    link: Option<Vec<PersonLink>>,
    resource_type: String,
}

impl PersonBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            resource_type: Person::get_resource_type(),
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

    pub fn managing_organization(mut self, managing_organization: Reference<Organization>) -> Self {
        self.managing_organization = Some(managing_organization);
        self
    }

    pub fn link(mut self, link: Vec<PersonLink>) -> Self {
        self.link = Some(link);
        self
    }

    pub fn add_link(mut self, link: PersonLink) -> Self {
        match &mut self.link {
            Some(l) => l.push(link),
            None => self.link = Some(vec![link]),
        }
        self
    }

    pub fn build(self) -> Person {
        Person {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            name: self.name,
            telecom: self.telecom,
            gender: self.gender,
            birth_date: self.birth_date,
            address: self.address,
            photo: self.photo,
            managing_organization: self.managing_organization,
            link: self.link,
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::r4::elements::{Coding, ReferenceBuilder};

    use super::*;

    #[test]
    pub fn test_build_person_should_succeed() {
        let mut expected = Person::default();
        expected.domain_resource.resource.id = Some("person-1".to_string());
        expected.gender = Some("male".to_string());
        
        let actual = PersonBuilder::new("person-1")
            .gender("male")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_person_link_should_succeed() {
        let org_ref = ReferenceBuilder::default()
            .with_reference("Organization/org-1")
            .build::<Organization>();

        let expected = PersonLink {
            backbone_element: BackboneElement {
                element: crate::r4::elements::element::Element {
                    id: Some("link-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            target: Some(org_ref.clone()),
            assurance: Some("level1".to_string()),
        };

        let actual = PersonLinkBuilder::new("link-1")
            .target(org_ref)
            .assurance("level1")
            .build();

        assert_eq!(expected, actual)
    }
}
