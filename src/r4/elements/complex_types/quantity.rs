use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::Element,
        resources::ResourceType,
    },
};
// TODO: Consider adding Comperator Set
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Quantity {
    #[serde(flatten)]
    pub element: Element,
    pub value: Option<f32>,
    pub comparator: Option<String>, // to be resolved later
    pub unit: Option<String>,
    pub system: Option<String>,
    pub code: Option<String>,
}

impl ResourceType for Quantity {
    const TYPE: &'static str = "Quantity";
}

impl Quantity {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct QuantityBuilder {
    element: Element,
    value: Option<f32>,
    comparator: Option<String>, // to be resolved later
    unit: Option<String>,
    system: Option<String>,
    code: Option<String>,
}

impl QuantityBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut quantity_builder = QuantityBuilder::default();
        quantity_builder.element.id = Some(id.into());
        quantity_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_comparator(mut self, comparator: impl Into<String>) -> Self {
        self.comparator = Some(comparator.into());
        self
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn build(self) -> Quantity {
        Quantity {
            element: self.element,
            value: self.value,
            comparator: self.comparator,
            unit: self.unit,
            system: self.system,
            code: self.code,
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
            "id": "quantity-1",
            "value": 20.05,
            "comparator": "the comparator",
            "unit": "C",
            "system": "any system",
            "code": "exact code"
        }
        "#;
        let expected = QuantityBuilder::default()
            .with_id("quantity-1")
            .with_value(20.05)
            .with_comparator("the comparator")
            .with_unit("C")
            .with_system("any system")
            .with_code("exact code")
            .build();

        let actual = Quantity::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Quantity {
            element: Element {
                id: Some("quantity-1".to_string()),
                extention: None,
            },
            value: Some(20.01),
            comparator: Some("a comparator".to_string()),
            unit: Some("C".to_string()),
            system: Some("the system".to_string()),
            code: Some("exact code".to_string()),
        };

        let actual = QuantityBuilder::new("quantity-1")
            .with_value(20.01)
            .with_comparator("a comparator")
            .with_unit("C")
            .with_system("the system")
            .with_code("exact code")
            .build();

        assert_eq!(expected, actual)
    }
}
