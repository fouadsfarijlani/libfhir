use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::Element,
    resources::{self, ResourceType},
};

// TODO: Consider including currency system ISO 4217
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Money {
    #[serde(flatten)]
    pub element: Element,
    pub value: Option<f32>,
    pub currency: Option<String>, // to be resolved later
}

impl ResourceType for Money {
    const TYPE: &'static str = "Money";
}

impl Money {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct MoneyBuilder {
    element: Element,
    value: Option<f32>,
    currency: Option<String>, // to be resolved later
}

impl MoneyBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut money_builder = MoneyBuilder::default();
        money_builder.element.id = Some(id.into());
        money_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn build(self) -> Money {
        Money {
            element: self.element,
            value: self.value,
            currency: self.currency,
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
                "resourceType": "Money",
                "value": 10.22,
                "currency": "Euro"
            }
        "#;
        let expected = MoneyBuilder::default()
            .with_value(10.22)
            .with_currency("Euro".to_string())
            .build();

        let actual = Money::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Money {
            element: Element {
                id: Some("money-1".to_string()),
                extention: None,
            },
            value: Some(100.543),
            currency: Some("USD".to_string()),
        };

        let actual = MoneyBuilder::new("money-1")
            .with_value(100.543)
            .with_currency("USD")
            .build();

        assert_eq!(expected, actual)
    }
}
