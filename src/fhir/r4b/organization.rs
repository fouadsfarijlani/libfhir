use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::fhir::r4b::resources::{EndpointKind, Identifier, OrganizationKind, Reference, ResourceType};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "resourceType")]
    pub resource_type: ResourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Reference<OrganizationKind>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<Reference<EndpointKind>>,
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Organization,
            id: None,
            identifier: None,
            active: None,
            name: None,
            alias: None,
            description: None,
            part_of: None,
            endpoint: Vec::new(),
        }
    }
}

impl Organization {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn push_identifier(mut self, system: impl Into<String>, value: impl Into<String>) -> Self {
        let mut v = self.identifier.unwrap_or_default();
        v.push(Identifier {
            system: Some(system.into()),
            value: Some(value.into()),
        });
        self.identifier = Some(v);
        self
    }

    pub fn with_part_of(mut self, reference: impl Into<String>, display: impl Into<String>) -> Self {
        self.part_of = Some(Reference {
            reference: Some(reference.into()),
            r#type: None,
            identifier: None,
            display: Some(display.into()),
            _marker: Default::default(),
        });
        self
    }

    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        let mut v = self.alias.unwrap_or_default();
        v.push(alias.into());
        self.alias = Some(v);
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_endpoint(mut self, ep: Reference<EndpointKind>) -> Self {
        self.endpoint.push(ep);
        self
    }

    pub fn push_endpoint_id(mut self, id: impl Into<String>) -> Self {
        self.endpoint.push(Reference::<EndpointKind>::to_id(id));
        self
    }

    pub fn with_endpoints(mut self, eps: Vec<Reference<EndpointKind>>) -> Self {
        self.endpoint = eps;
        self
    }

    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        let org: Self = serde_json::from_str(s)?;
        if org.resource_type != ResourceType::Organization {
            return Err(serde_json::Error::custom("resourceType must be 'Organization'"));
        }
        Ok(org)
    }
}