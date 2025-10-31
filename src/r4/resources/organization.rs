use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        Address, BackboneElement, CodeableConcept, ContactPoint, GetResourceReferences, HumanName,
        Identifier, Reference, ReferenceTypes,
    },
    resources::{DomainResource, Endpoint, Resource, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganizationContact {
    #[serde(flatten)]
    pub element: BackboneElement,
    pub purpose: Option<CodeableConcept>,
    pub name: Option<HumanName>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Address>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Organization {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub r#type: Option<Vec<CodeableConcept>>,
    pub name: Option<String>,
    pub alias: Option<Vec<String>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Vec<Address>>,
    pub part_of: Option<Reference<Organization>>,
    pub contact: Option<Vec<OrganizationContact>>,
    pub endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl Organization {
    pub fn from_json(data: &str) -> Self {
        let results = serde_json::from_str::<Organization>(data);
        match results {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl ResourceType for Organization {
    const TYPE: &'static str = "Organization";
}

impl GetResourceReferences for Organization {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(eps) = &self.endpoint {
            references = eps.into_iter().map(|e| ReferenceTypes::from(e)).collect();
        }

        if let Some(part_of) = &self.part_of {
            references.push(ReferenceTypes::from(part_of));
        }

        references
    }
}

#[derive(Default)]
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
}

impl OrganizationBuilder {
    pub fn new(id: String) -> Self {
        Self {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id),
                    meta: None,
                    implicit_rules: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            identifier: None,
            active: None,
            r#type: None,
            name: None,
            alias: None,
            telecom: None,
            address: None,
            part_of: None,
            contact: None,
            endpoint: None,
        }
    }

    pub fn with_identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn with_type(mut self, org_type: Vec<CodeableConcept>) -> Self {
        self.r#type = Some(org_type);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_alias(mut self, alias: Vec<String>) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn with_telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn with_address(mut self, address: Vec<Address>) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_part_of(mut self, part_of: Reference<Organization>) -> Self {
        self.part_of = Some(part_of);
        self
    }

    pub fn with_contact(mut self, organization_contact: Vec<OrganizationContact>) -> Self {
        self.contact = Some(organization_contact);
        self
    }

    pub fn with_endpoint(mut self, endpoint: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoint);
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_build_org() {
        let mut expected = Organization::default();
        expected.domain_resource.resource.id = Some("some-id".to_string());
        expected.name = Some("some-name".to_string());
        let actual = OrganizationBuilder::new("some-id".to_string())
            .with_name("some-name".to_string())
            .build();

        assert_eq!(expected, actual)
    }

    // TODO: fix the annoying syntax below
    #[test]
    pub fn test_from_json_should_suceed() {
        let data = r#"
        {
            "reourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        "#;
        let mut part_of = Reference::<Organization>::default();
        part_of.reference = Some("Organization/1".to_string());

        let mut ep_1 = Reference::<Endpoint>::default();
        ep_1.reference = Some("Endpoint/1".to_string());

        let mut ep_2 = Reference::<Endpoint>::default();
        ep_2.reference = Some("Endpoint/2".to_string());

        let endpoint = vec![ep_1, ep_2];
        let expected = OrganizationBuilder::new(String::from("some-id"))
            .with_active(true)
            .with_part_of(part_of)
            .with_endpoint(endpoint)
            .build();

        let actual = Organization::from_json(data);

        assert_eq!(expected, actual);
    }
}
