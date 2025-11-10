use serde::{Deserialize, Serialize};

use crate::{
    elements::{CodeableConcept, Element, Period, Reference},
    resources::{self, Organization, ResourceType},
};
// TODO: add IdentifierUse Types
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Identifier {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>,
    pub r#type: Option<CodeableConcept>,
    pub system: Option<String>,
    pub value: Option<String>,
    pub period: Option<Period>, // to be resolved
    pub assigner: Option<Reference<Organization>>,
}

impl ResourceType for Identifier {
    const TYPE: &'static str = "Identifier";
}

impl Identifier {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct IdentifierBuilder {
    element: Element,
    r#use: Option<String>,
    r#type: Option<CodeableConcept>,
    system: Option<String>,
    value: Option<String>,
    period: Option<Period>, // to be resolved
    assigner: Option<Reference<Organization>>,
}

impl IdentifierBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut identifier_builder = IdentifierBuilder::default();
        identifier_builder.element.id = Some(id.into());
        identifier_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_use(mut self, r#use: impl Into<String>) -> Self {
        self.r#use = Some(r#use.into());
        self
    }

    pub fn with_type(mut self, r#type: CodeableConcept) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    // TODO: check if this can work with enum ReferenceType
    pub fn with_assigner(mut self, assigner: Reference<Organization>) -> Self {
        self.assigner = Some(assigner);
        self
    }

    pub fn build(self) -> Identifier {
        Identifier {
            element: self.element,
            r#use: self.r#use,
            r#type: self.r#type,
            system: self.system,
            value: self.value,
            period: self.period,
            assigner: self.assigner,
        }
    }
}
