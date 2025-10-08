use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::fhir::r4b::resources::{Identifier, OrganizationKind, Reference};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EndpointResourceType {
    Endpoint,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EndpointStatus {
    Active,
    Suspended,
    Error,
    Off,
    EnteredInError,
    Test,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    #[serde(rename = "resourceType")]
    pub resource_type: EndpointResourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    pub status: EndpointStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The base URL/URI to connect to (e.g., https://api.example.org/fhir)
    pub address: String,

    /// Headers to include when connecting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,

    /// Organization that manages this endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<OrganizationKind>>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            resource_type: EndpointResourceType::Endpoint,
            id: None,
            identifier: None,
            status: EndpointStatus::Active,
            name: None,
            description: None,
            address: String::new(),
            header: None,
            managing_organization: None,
        }
    }
}

impl Endpoint {
    pub fn new(status: EndpointStatus, address: impl Into<String>) -> Self {
        Self {
            status,
            address: address.into(),
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
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

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn push_header(mut self, hdr: impl Into<String>) -> Self {
        let mut v = self.header.unwrap_or_default();
        v.push(hdr.into());
        self.header = Some(v);
        self
    }

    pub fn with_managing_org(mut self, org_ref: Reference<OrganizationKind>) -> Self {
        self.managing_organization = Some(org_ref);
        self
    }

    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        let ep: Self = serde_json::from_str(s)?;
        if ep.resource_type != EndpointResourceType::Endpoint {
            return Err(serde_json::Error::custom("resourceType must be 'Endpoint'"));
        }
        Ok(ep)
    }
}
