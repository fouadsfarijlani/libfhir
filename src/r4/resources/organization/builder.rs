use std::vec;

use crate::r4::{
    elements::{
        Address, BackboneElement, CodeableConcept, ContactPoint, HumanName, Identifier, Reference,
    },
    resources::{
        DomainResource, Endpoint, Organization, OrganizationContact, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct OrganizationContactBuilder {
    backbone_element: BackboneElement,
    purpose: Option<CodeableConcept>,
    name: Option<HumanName>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Address>,
}

impl OrganizationContactBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }
    pub fn purpose(mut self, purpose: CodeableConcept) -> Self {
        self.purpose = Some(purpose);
        self
    }

    pub fn name(mut self, name: HumanName) -> Self {
        self.name = Some(name);
        self
    }

    pub fn telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, point: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(tm) => tm.push(point),
            None => self.telecom = Some(vec![point]),
        }
        self
    }

    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn build(self) -> OrganizationContact {
        OrganizationContact {
            backbone_element: self.backbone_element,
            purpose: self.purpose,
            name: self.name,
            telecom: self.telecom,
            address: self.address,
        }
    }
}

pub struct OrganizationBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    r#type: Option<Vec<CodeableConcept>>,
    name: Option<String>,
    alias: Option<Vec<String>>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Vec<Address>>,
    part_of: Option<Reference<Organization>>,
    contact: Option<Vec<OrganizationContact>>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
    resource_type: String,
}

impl Default for OrganizationBuilder {
    fn default() -> Self {
        OrganizationBuilder {
            domain_resource: DomainResource {
                resource: Resource {
                    ..Default::default()
                },
                ..Default::default()
            },
            resource_type: Organization::get_resource_type(),
            identifier: None,
            active: None,
            address: None,
            alias: None,
            endpoint: None,
            part_of: None,
            contact: None,
            r#type: None,
            name: None,
            telecom: None,
        }
    }
}

impl OrganizationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
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

    pub fn r#type(mut self, org_type: Vec<CodeableConcept>) -> Self {
        self.r#type = Some(org_type);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn alias(mut self, alias: Vec<String>) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn add_alias(mut self, alias: impl Into<String>) -> Self {
        match &mut self.alias {
            Some(a) => a.push(alias.into()),
            None => self.alias = Some(vec![alias.into()]),
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

    pub fn part_of(mut self, part_of: Reference<Organization>) -> Self {
        self.part_of = Some(part_of);
        self
    }

    pub fn contact(mut self, organization_contact: Vec<OrganizationContact>) -> Self {
        self.contact = Some(organization_contact);
        self
    }

    pub fn add_contat(mut self, organization_contact: OrganizationContact) -> Self {
        match &mut self.contact {
            Some(c) => c.push(organization_contact),
            None => self.contact = Some(vec![organization_contact]),
        }
        self
    }

    pub fn endpoint(mut self, endpoint: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        match &mut self.endpoint {
            Some(ep) => ep.push(endpoint),
            None => self.endpoint = Some(vec![endpoint]),
        }
        self
    }

    pub fn build(self) -> Organization {
        Organization {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            r#type: self.r#type,
            name: self.name,
            alias: self.alias,
            telecom: self.telecom,
            address: self.address,
            part_of: self.part_of,
            contact: self.contact,
            endpoint: self.endpoint,
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::r4::elements::{Element, HumanNameBuilder};

    use super::*;
    #[test]
    pub fn test_build_org() {
        let mut expected = Organization::default();
        expected.domain_resource.resource.id = Some("some-id".to_string());
        expected.name = Some("some-name".to_string());
        let actual = OrganizationBuilder::new("some-id".to_string())
            .name("some-name".to_string())
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_organization_contact_should_succeed() {
        let expected = OrganizationContact {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("contact-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            purpose: None,
            name: Some(HumanName {
                r#use: Some("official".to_string()),

                family: Some("Doe".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let human_name = HumanNameBuilder::default()
            .with_use("official")
            .with_family("Doe")
            .build();
        let actual = OrganizationContactBuilder::new("contact-1")
            .name(human_name)
            .build();

        assert_eq!(expected, actual)
    }
}
