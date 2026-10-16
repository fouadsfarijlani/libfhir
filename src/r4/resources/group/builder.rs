use std::vec;

use crate::r4::{
    elements::{
        BackboneElement, CodeableConcept, ContactPoint, HumanName, Identifier, Quantity, Range,
        Reference,
    },
    resources::{
        DomainResource, Group, GroupCharacteristic, GroupMember, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct GroupCharacteristicBuilder {
    backbone_element: BackboneElement,
    code: Option<CodeableConcept>,
    value_codeable_concept: Option<CodeableConcept>,
    value_boolean: Option<bool>,
    value_quantity: Option<Quantity>,
    value_range: Option<Range>,
    exclude: Option<bool>,
    period: Option<crate::r4::elements::complex_types::Period>,
}

impl GroupCharacteristicBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn code(mut self, code: CodeableConcept) -> Self {
        self.code = Some(code);
        self
    }

    pub fn value_codeable_concept(mut self, value: CodeableConcept) -> Self {
        self.value_codeable_concept = Some(value);
        self
    }

    pub fn value_boolean(mut self, value: bool) -> Self {
        self.value_boolean = Some(value);
        self
    }

    pub fn value_quantity(mut self, value: Quantity) -> Self {
        self.value_quantity = Some(value);
        self
    }

    pub fn value_range(mut self, value: Range) -> Self {
        self.value_range = Some(value);
        self
    }

    pub fn exclude(mut self, exclude: bool) -> Self {
        self.exclude = Some(exclude);
        self
    }

    pub fn period(mut self, period: crate::r4::elements::complex_types::Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> GroupCharacteristic {
        GroupCharacteristic {
            backbone_element: self.backbone_element,
            code: self.code,
            value_codeable_concept: self.value_codeable_concept,
            value_boolean: self.value_boolean,
            value_quantity: self.value_quantity,
            value_range: self.value_range,
            exclude: self.exclude,
            period: self.period,
        }
    }
}

#[derive(Default)]
pub struct GroupMemberBuilder {
    backbone_element: BackboneElement,
    entity: Option<Reference<Group>>,
    period: Option<crate::r4::elements::complex_types::Period>,
    inactive: Option<bool>,
}

impl GroupMemberBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn entity(mut self, entity: Reference<Group>) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn period(mut self, period: crate::r4::elements::complex_types::Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn inactive(mut self, inactive: bool) -> Self {
        self.inactive = Some(inactive);
        self
    }

    pub fn build(self) -> GroupMember {
        GroupMember {
            backbone_element: self.backbone_element,
            entity: self.entity,
            period: self.period,
            inactive: self.inactive,
        }
    }
}

#[derive(Default)]
pub struct GroupBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    r#type: Option<CodeableConcept>,
    actual: Option<bool>,
    characteristic: Option<Vec<GroupCharacteristic>>,
    code: Option<CodeableConcept>,
    name: Option<String>,
    description: Option<String>,
    quantity: Option<u32>,
    managing_entity: Option<Reference<Group>>,
    member: Option<Vec<GroupMember>>,
    resource_type: String,
}

impl GroupBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            resource_type: Group::get_resource_type(),
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

    pub fn r#type(mut self, r#type: CodeableConcept) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn actual(mut self, actual: bool) -> Self {
        self.actual = Some(actual);
        self
    }

    pub fn characteristic(mut self, characteristic: Vec<GroupCharacteristic>) -> Self {
        self.characteristic = Some(characteristic);
        self
    }

    pub fn add_characteristic(mut self, characteristic: GroupCharacteristic) -> Self {
        match &mut self.characteristic {
            Some(c) => c.push(characteristic),
            None => self.characteristic = Some(vec![characteristic]),
        }
        self
    }

    pub fn code(mut self, code: CodeableConcept) -> Self {
        self.code = Some(code);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn managing_entity(mut self, managing_entity: Reference<Group>) -> Self {
        self.managing_entity = Some(managing_entity);
        self
    }

    pub fn member(mut self, member: Vec<GroupMember>) -> Self {
        self.member = Some(member);
        self
    }

    pub fn add_member(mut self, member: GroupMember) -> Self {
        match &mut self.member {
            Some(m) => m.push(member),
            None => self.member = Some(vec![member]),
        }
        self
    }

    pub fn build(self) -> Group {
        Group {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            r#type: self.r#type,
            actual: self.actual,
            characteristic: self.characteristic,
            code: self.code,
            name: self.name,
            description: self.description,
            quantity: self.quantity,
            managing_entity: self.managing_entity,
            member: self.member,
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::r4::elements::{Coding, ReferenceBuilder};

    use super::*;

    #[test]
    pub fn test_build_group_should_succeed() {
        let mut expected = Group::default();
        expected.domain_resource.resource.id = Some("group-1".to_string());
        expected.name = Some("Test Group".to_string());
        
        let actual = GroupBuilder::new("group-1")
            .name("Test Group")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_group_characteristic_should_succeed() {
        let expected = GroupCharacteristic {
            backbone_element: BackboneElement {
                element: crate::r4::elements::element::Element {
                    id: Some("char-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            code: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://example.org".to_string()),
                    code: Some("test".to_string()),
                    display: Some("Test".to_string()),
                    ..Default::default()
                }]),
                text: Some("Test".to_string()),
                ..Default::default()
            }),
            value_codeable_concept: None,
            value_boolean: Some(true),
            value_quantity: None,
            value_range: None,
            exclude: Some(false),
            period: None,
        };

        let code = CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://example.org".to_string()),
                code: Some("test".to_string()),
                display: Some("Test".to_string()),
                ..Default::default()
            }]),
            text: Some("Test".to_string()),
            ..Default::default()
        };

        let actual = GroupCharacteristicBuilder::new("char-1")
            .code(code)
            .value_boolean(true)
            .exclude(false)
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_group_member_should_succeed() {
        let group_ref = ReferenceBuilder::default()
            .with_reference("Group/group-2")
            .build::<Group>();

        let expected = GroupMember {
            backbone_element: BackboneElement {
                element: crate::r4::elements::element::Element {
                    id: Some("member-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            entity: Some(group_ref.clone()),
            period: None,
            inactive: Some(false),
        };

        let actual = GroupMemberBuilder::new("member-1")
            .entity(group_ref)
            .inactive(false)
            .build();

        assert_eq!(expected, actual)
    }
}
