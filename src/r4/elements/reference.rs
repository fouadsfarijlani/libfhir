use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{complex_types::Identifier, element::Element},
    resources::{
        self, Endpoint, HealthcareService, Location, Organization, Practitioner, ResourceType,
    },
};

#[derive(Debug, Serialize, PartialEq, Deserialize, Default, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Reference<T: ResourceType> {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip)]
    pub _m: PhantomData<T>,
}

impl<T: ResourceType> ResourceType for Reference<T> {
    const TYPE: &'static str = "Reference";
}

impl<'a, T> Reference<T>
where
    T: ResourceType,
    T: Deserialize<'a>,
{
    pub fn from_json(data: &'a str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Debug, PartialEq)]
pub enum ReferenceTypes<'a> {
    ReferenceOrganization(&'a Reference<Organization>),
    ReferenceEndpoint(&'a Reference<Endpoint>),
    ReferenceLocation(&'a Reference<Location>),
    ReferecenceHealthcareServce(&'a Reference<HealthcareService>),
    ReferencePractitioner(&'a Reference<Practitioner>),
}

impl<'a> From<&'a Reference<Endpoint>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Endpoint>) -> Self {
        Self::ReferenceEndpoint(value)
    }
}

impl<'a> From<&'a Reference<Organization>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Organization>) -> Self {
        Self::ReferenceOrganization(value)
    }
}

impl<'a> From<&'a Reference<Location>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Location>) -> Self {
        Self::ReferenceLocation(value)
    }
}

impl<'a> From<&'a Reference<HealthcareService>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<HealthcareService>) -> Self {
        Self::ReferecenceHealthcareServce(value)
    }
}

impl<'a> From<&'a Reference<Practitioner>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Practitioner>) -> Self {
        Self::ReferencePractitioner(value)
    }
}

pub trait GetResourceReferences {
    fn get_references(&self) -> Vec<ReferenceTypes>;
}

#[derive(Default)]
pub struct ReferenceBuilder {
    element: Element,
    reference: Option<String>,
    r#type: Option<String>,
    display: Option<String>,
    identifier: Option<Vec<Identifier>>,
}

impl ReferenceBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut reference_builder = Self::default();
        reference_builder.element.id = Some(id.into());
        reference_builder
    }

    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    pub fn with_type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    pub fn with_display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }

    pub fn with_identifier(mut self, identifiers: Vec<Identifier>) -> Self {
        self.identifier = Some(identifiers);
        self
    }

    pub fn add_identfier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn build<T: ResourceType>(self) -> Reference<T> {
        Reference {
            element: self.element,
            reference: self.reference,
            r#type: self.r#type,
            display: self.display,
            identifier: self.identifier,
            _m: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "Reference",
                "reference": "Organization/1",
                "type": "Organization",
                "display": "Org-1"
            }
        "#;

        let expected = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .with_type("Organization")
            .with_display("Org-1")
            .build::<Organization>();

        let actual = Reference::<Organization>::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Reference::<Endpoint> {
            element: Element {
                id: Some("ref-1".to_string()),
                extention: None,
            },
            reference: Some("Endpoint/1".to_string()),
            r#type: Some("Endpoint".to_string()),
            display: Some("ep-1".to_string()),
            identifier: None,
            _m: PhantomData,
        };

        let actual = ReferenceBuilder::new("ref-1")
            .with_reference("Endpoint/1")
            .with_type("Endpoint")
            .with_display("ep-1")
            .build::<Endpoint>();

        assert_eq!(expected, actual)
    }
}
