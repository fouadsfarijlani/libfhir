use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        CodeableConcept, Coding, ContactPoint, GetResourceReferences, Identifier, Period,
        Reference, ReferenceTypes,
    },
    resources::{DomainResource, Organization, Resource, ResourceType},
};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Endpoint {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub status: String, // to be resolved later
    pub connection_type: Coding,
    pub managing_organization: Option<Reference<Organization>>,
    pub contact: Option<Vec<ContactPoint>>,
    pub period: Option<Period>,
    pub payload_type: Vec<CodeableConcept>,
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later
    pub address: String,                        // to be resolved later
    pub header: Option<String>,
}

impl Endpoint {
    pub fn new(
        status: String,
        connection_type: Coding,
        address: String,
        payload_type: Vec<CodeableConcept>,
    ) -> Self {
        Endpoint {
            domain_resource: DomainResource {
                resource: Resource {
                    id: None,
                    meta: None,
                    implicit_rules: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            identifier: None,
            status: status,
            connection_type: connection_type,
            managing_organization: None,
            contact: None,
            period: None,
            payload_type: payload_type,
            payload_mime_type: None,
            address: address,
            header: None,
        }
    }

    pub fn from_json(data: &str) -> Self {
        let result = serde_json::from_str(data);
        match result {
            Ok(ep) => ep,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl ResourceType for Endpoint {
    const TYPE: &'static str = "Endpoint";
}

impl GetResourceReferences for Endpoint {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();
        if let Some(managing_organization) = &self.managing_organization {
            references.push(ReferenceTypes::from(managing_organization));
        }

        references
    }
}

pub struct EndpointBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: String, // to be resolved later
    connection_type: Coding,
    managing_organization: Option<Reference<Organization>>,
    contact: Option<Vec<ContactPoint>>,
    period: Option<Period>,
    payload_type: Vec<CodeableConcept>,
    payload_mime_type: Option<Vec<String>>, // to be resolved later
    address: String,                        // to be resolved later
    header: Option<String>,
}

impl EndpointBuilder {
    pub fn new(
        status: String,
        connection_type: Coding,
        address: String,
        payload_type: Vec<CodeableConcept>,
    ) -> Self {
        Self {
            domain_resource: DomainResource {
                resource: Resource {
                    id: None,
                    meta: None,
                    implicit_rules: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            identifier: None,
            status: status,
            connection_type: connection_type,
            managing_organization: None,
            contact: None,
            period: None,
            payload_type: payload_type,
            payload_mime_type: None,
            address: address,
            header: None,
        }
    }

    pub fn with_identifier(mut self, indentifier: Vec<Identifier>) -> Self {
        self.identifier = Some(indentifier);
        self
    }

    pub fn with_managing_organization(
        mut self,
        managing_organization: Reference<Organization>,
    ) -> Self {
        self.managing_organization = Some(managing_organization);
        self
    }

    pub fn with_contact(mut self, contact: Vec<ContactPoint>) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn with_payload_mime_type(mut self, payload_mime_type: Vec<String>) -> Self {
        self.payload_mime_type = Some(payload_mime_type);
        self
    }

    pub fn with_header(mut self, header: String) -> Self {
        self.header = Some(header);
        self
    }

    pub fn build(self) -> Endpoint {
        Endpoint {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            status: self.status,
            connection_type: self.connection_type,
            managing_organization: self.managing_organization,
            contact: self.contact,
            period: self.period,
            payload_type: self.payload_type,
            payload_mime_type: self.payload_mime_type,
            address: self.address,
            header: self.header,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = Endpoint::new(
            "test".to_string(),
            Coding::default(),
            "http://example.org".to_string(),
            vec![CodeableConcept::default()],
        );

        let actual = EndpointBuilder::new(
            "test".to_string(),
            Coding::default(),
            "http://example.org".to_string(),
            vec![CodeableConcept::default()],
        )
        .build();
        assert_eq!(expected, actual);
    }
    #[test]
    fn from_json_should_succeed() {
        let data = r#"
        {
            "resourceType":"Endpoint",
            "status": "error",
            "connectionType": {
                "system": "some-system",
                "code": "some-code"
            },
            "payloadType": [
                {
                    "coding":[
                        {
                            "system": "some-system"
                        }
                    ],
                    "test": "some text"
                }
            ],
            "address": "http://example.com"
        }
        "#;

        // TODO: this syntax is stupid
        let mut connection_type = Coding::default();
        connection_type.system = Some("some-system".to_string());
        connection_type.code = Some("code".to_string());
        let mut payload_entry = CodeableConcept::default();
        let mut coding = Coding::default();
        coding.system = Some("some-system".to_string());
        payload_entry.coding = Some(vec![coding]);
        payload_entry.test = Some("some text".to_string());
        let payload_type = vec![payload_entry];
        let expected = EndpointBuilder::new(
            "error".to_string(),
            connection_type,
            "http://example.org".to_string(),
            payload_type,
        )
        .build();

        let actual = Endpoint::from_json(data);

        assert_eq!(expected, actual);
    }
}
