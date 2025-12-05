use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use crate::r4::{
    elements::{
        Address, BackboneElement, CodeableConcept, ContactPoint, GetResourceReferences, HumanName,
        Identifier, Reference, ReferenceTypes,
    },
    resources::{DomainResource, Endpoint, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct OrganizationContact {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HumanName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

impl ResourceType for OrganizationContact {
    const TYPE: &'static str = "OrganizationContact";
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
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

impl Serialize for Organization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Organization", 10)?;
        s.serialize_field("resourceType", Self::TYPE)?;

        if let Some(id) = &self.domain_resource.resource.id {
            s.serialize_field("id", id)?;
        }

        if let Some(meta) = &self.domain_resource.resource.meta {
            s.serialize_field("meta", meta)?;
        }

        if let Some(implicit_rules) = &self.domain_resource.resource.implicit_rules {
            s.serialize_field("implicitRules", implicit_rules)?;
        }

        if let Some(text) = &self.domain_resource.text {
            s.serialize_field("text", text)?;
        }

        if let Some(contained) = &self.domain_resource.contained {
            s.serialize_field("contained", contained)?;
        }

        if let Some(extensions) = &self.domain_resource.extensions {
            s.serialize_field("extensions", extensions)?;
        }

        if let Some(identifier) = &self.identifier {
            s.serialize_field("identifier", identifier)?;
        }

        if let Some(active) = &self.active {
            s.serialize_field("active", active)?;
        }

        if let Some(r#type) = &self.r#type {
            s.serialize_field("type", r#type)?;
        }

        if let Some(name) = &self.name {
            s.serialize_field("name", name)?;
        }

        if let Some(alias) = &self.alias {
            s.serialize_field("alias", alias)?;
        }

        if let Some(telecom) = &self.telecom {
            s.serialize_field("telecom", telecom)?;
        }

        if let Some(address) = &self.address {
            s.serialize_field("address", address)?;
        }

        if let Some(part_of) = &self.part_of {
            s.serialize_field("partOf", part_of)?;
        }

        if let Some(contact) = &self.contact {
            s.serialize_field("contact", contact)?;
        }

        if let Some(endpoint) = &self.endpoint {
            s.serialize_field("endpoint", endpoint)?;
        }

        s.end()
    }
}

impl Organization {
    pub fn from_json(data: &str) -> Self {
        let results = serde_json::from_str::<Organization>(data);
        match results {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }

    pub fn to_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|e| panic!("{e:?}"))
    }

    pub fn to_json_string(self) -> String {
        let result = serde_json::to_string_pretty(&self);
        match result {
            Ok(data) => data,
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::{elements::ReferenceBuilder, resources::OrganizationBuilder};

    use super::*;

    #[test]
    pub fn test_from_json_should_suceed() {
        let data = r#"
        {
            "resourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        "#;

        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let ep_1 = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let ep_2 = ReferenceBuilder::default()
            .with_reference("Endpoint/2")
            .build::<Endpoint>();

        let endpoint = vec![ep_1, ep_2];
        let expected = OrganizationBuilder::new(String::from("some-id"))
            .with_active(true)
            .with_part_of(part_of)
            .with_endpoint(endpoint)
            .build();

        let actual = Organization::from_json(data);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_to_json_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        );

        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let ep_1 = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let ep_2 = ReferenceBuilder::default()
            .with_reference("Endpoint/2")
            .build::<Endpoint>();

        let endpoint = vec![ep_1, ep_2];
        let org = OrganizationBuilder::new(String::from("some-id"))
            .with_active(true)
            .with_part_of(part_of)
            .with_endpoint(endpoint)
            .build();

        let actual = org.to_json();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Organization",
            "id": "some-id",
            "active": true,
            "partOf": {"reference": "Organization/1"},
            "endpoint": [
                {"reference": "Endpoint/1"},
                {"reference": "Endpoint/2"}
            ]
        }
        )
        .to_string();

        let data = r#"{"resourceType": "Organization", "id": "some-id", "active": true, "partOf": {"reference": "Organization/1"}, "endpoint": [{"reference": "Endpoint/1"},{"reference": "Endpoint/2"}]}"#;

        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let ep_1 = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let ep_2 = ReferenceBuilder::default()
            .with_reference("Endpoint/2")
            .build::<Endpoint>();

        let endpoint = vec![ep_1, ep_2];
        let org = OrganizationBuilder::new(String::from("some-id"))
            .with_active(true)
            .with_part_of(part_of)
            .with_endpoint(endpoint)
            .build();

        let actual = org.to_json_string();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_get_all_referenecs_should_succeed() {
        let part_of = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let endpoints = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();
        let expected = vec![
            ReferenceTypes::from(&endpoints),
            ReferenceTypes::from(&part_of),
        ];
        let org = OrganizationBuilder::default()
            .with_part_of(part_of.clone())
            .add_endpoint(endpoints.clone())
            .build();

        let actual = org.get_references();

        assert_eq!(expected, actual)
    }
}
