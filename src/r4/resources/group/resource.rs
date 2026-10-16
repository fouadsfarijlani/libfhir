use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            BackboneElement, CodeableConcept, GetResourceReferences, Identifier, Quantity, Range,
            Reference, ReferenceTypes,
        },
        resources::{DomainResource, ResourceType},
    },
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Group {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<GroupCharacteristic>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_entity: Option<Reference<Group>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Vec<GroupMember>>,

    #[serde(default = "Group::get_resource_type")]
    pub resource_type: String,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            r#type: None,
            actual: None,
            characteristic: None,
            code: None,
            name: None,
            description: None,
            quantity: None,
            managing_entity: None,
            member: None,
        }
    }
}

impl Group {
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

impl ResourceType for Group {
    const TYPE: &'static str = "Group";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GroupCharacteristic {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_boolean: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<Quantity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<Range>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<crate::r4::elements::complex_types::Period>,
}

impl ResourceType for GroupCharacteristic {
    const TYPE: &'static str = "GroupCharacteristic";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GroupMember {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Reference<Group>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<crate::r4::elements::complex_types::Period>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
}

impl ResourceType for GroupMember {
    const TYPE: &'static str = "GroupMember";
}

impl GetResourceReferences for Group {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(managing_entity) = &self.managing_entity {
            references.push(ReferenceTypes::from(managing_entity));
        }

        if let Some(members) = &self.member {
            for member in members {
                if let Some(entity) = &member.entity {
                    references.push(ReferenceTypes::from(entity));
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
        resources::{GroupBuilder},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/group.json");
        let result = Group::from_json(data);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Group",
            "id": "group-1",
            "name": "Test Group",
            "type": {
                "coding": [
                    {
                        "system": "http://terminology.hl7.org/CodeSystem/group-type",
                        "code": "person",
                        "display": "Person"
                    }
                ],
                "text": "Person"
            }
        }
        );

        let group_type = CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/group-type".to_string()),
                code: Some("person".to_string()),
                display: Some("Person".to_string()),
                ..Default::default()
            }]),
            text: Some("Person".to_string()),
            ..Default::default()
        };

        let group = GroupBuilder::new("group-1")
            .name("Test Group")
            .r#type(group_type)
            .build();

        let actual = group.to_json_value().unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let group_type = CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/group-type".to_string()),
                code: Some("person".to_string()),
                display: Some("Person".to_string()),
                ..Default::default()
            }]),
            text: Some("Person".to_string()),
            ..Default::default()
        };

        let group = GroupBuilder::new("group-1")
            .name("Test Group")
            .r#type(group_type)
            .build();

        let value = match group.to_json_string() {
            Ok(res) => res,
            Err(e) => panic!("{e:?}"),
        };
        let actual: serde_json::Value = serde_json::from_str(value.as_str()).unwrap();

        assert_eq!(actual["resourceType"], "Group");
        assert_eq!(actual["id"], "group-1");
    }

    #[test]
    pub fn test_get_all_references_should_succeed() {
        let managing_entity = ReferenceBuilder::default()
            .with_reference("Group/group-2")
            .build::<Group>();
        let expected = vec![ReferenceTypes::from(&managing_entity)];
        let group = GroupBuilder::default()
            .managing_entity(managing_entity.clone())
            .build();

        let actual = group.get_references();

        assert_eq!(expected, actual)
    }
}
