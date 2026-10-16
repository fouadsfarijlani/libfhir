use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Address, Attachment, BackboneElement, ContactPoint, GetResourceReferences, HumanName,
            Identifier, Reference, ReferenceTypes,
        },
        resources::{DomainResource, Organization, ResourceType},
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PersonLink {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assurance: Option<String>,
}

impl ResourceType for PersonLink {
    const TYPE: &'static str = "PersonLink";
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Person {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Vec<PersonLink>>,

    #[serde(default = "Person::get_resource_type")]
    pub resource_type: String,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            name: None,
            telecom: None,
            gender: None,
            birth_date: None,
            address: None,
            photo: None,
            managing_organization: None,
            link: None,
        }
    }
}

impl Person {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}

impl ResourceType for Person {
    const TYPE: &'static str = "Person";
}

impl GetResourceReferences for Person {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(managing_org) = &self.managing_organization {
            references.push(ReferenceTypes::from(managing_org));
        }

        if let Some(links) = &self.link {
            for link in links {
                if let Some(target) = &link.target {
                    references.push(ReferenceTypes::from(target));
                }
            }
        }

        references
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::{
        elements::{Coding, ReferenceBuilder},
        resources::{Organization, PersonBuilder},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/person.json");
        let result = Person::from_json(data);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Person",
            "id": "person-1",
            "managingOrganization": {"reference": "Organization/org-1"}
        }
        );

        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/org-1")
            .build::<Organization>();

        let person = PersonBuilder::new("person-1")
            .managing_organization(managing_org)
            .build();

        let actual = person.to_json_value().unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/org-1")
            .build::<Organization>();

        let person = PersonBuilder::new("person-1")
            .managing_organization(managing_org)
            .build();

        let value = match person.to_json_string() {
            Ok(res) => res,
            Err(e) => panic!("{e:?}"),
        };
        let actual: serde_json::Value = serde_json::from_str(value.as_str()).unwrap();

        assert_eq!(actual["resourceType"], "Person");
        assert_eq!(actual["id"], "person-1");
    }

    #[test]
    pub fn test_get_all_references_should_succeed() {
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/org-1")
            .build::<Organization>();
        let expected = vec![ReferenceTypes::from(&managing_org)];
        let person = PersonBuilder::default()
            .managing_organization(managing_org.clone())
            .build();

        let actual = person.get_references();

        assert_eq!(expected, actual)
    }
}
