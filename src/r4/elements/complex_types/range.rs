use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::{Element, Quantity},
    resources::{self, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(flatten)]
    pub element: Element,
    pub low: Option<Quantity>,
    pub high: Option<Quantity>,
}

impl ResourceType for Range {
    const TYPE: &'static str = "Range";
}

impl Range {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct RangeBuilder {
    element: Element,
    low: Option<Quantity>,
    high: Option<Quantity>,
}

impl RangeBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut range_builder = Self::default();
        range_builder.element.id = Some(id.into());
        range_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_low(mut self, low: Quantity) -> Self {
        self.low = Some(low);
        self
    }

    pub fn with_high(mut self, high: Quantity) -> Self {
        self.high = Some(high);
        self
    }

    pub fn build(self) -> Range {
        Range {
            element: self.element,
            low: self.low,
            high: self.high,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::QuantityBuilder;

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "Range",
                "id": "range-1",
                "low": {
                    "id": "quantity-1",
                    "value": 20.05,
                    "comparator": "the comparator",
                    "unit": "C",
                    "system": "any system",
                    "code": "exact code"
                },
                "high": {
                    "id": "quantity-2",
                    "value": 30.05,
                    "comparator": "the comparator",
                    "unit": "C",
                    "system": "any system",
                    "code": "exact code"
                }
            }
        "#;

        let low = QuantityBuilder::default()
            .with_id("quantity-1")
            .with_value(20.05)
            .with_comparator("the comparator")
            .with_unit("C")
            .with_system("any system")
            .with_code("exact code")
            .build();
        let high = QuantityBuilder::default()
            .with_id("quantity-2")
            .with_value(30.05)
            .with_comparator("the comparator")
            .with_unit("C")
            .with_system("any system")
            .with_code("exact code")
            .build();
        let expected = RangeBuilder::default()
            .with_id("range-1")
            .with_low(low)
            .with_high(high)
            .build();

        let actual = Range::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Range {
            element: Element {
                id: Some("range-1".to_string()),
                extention: None,
            },
            low: Some(Quantity {
                element: Element {
                    id: Some("quantity-1".to_string()),
                    extention: None,
                },
                value: Some(10.00),
                comparator: None,
                unit: Some("C".to_string()),
                system: None,
                code: None,
            }),
            high: Some(Quantity {
                element: Element {
                    id: Some("quantity-2".to_string()),
                    extention: None,
                },
                value: Some(20.00),
                comparator: None,
                unit: Some("C".to_string()),
                system: None,
                code: None,
            }),
        };
        let low = QuantityBuilder::default()
            .with_id("quantity-1")
            .with_value(10.00)
            .with_unit("C")
            .build();
        let high = QuantityBuilder::default()
            .with_id("quantity-2")
            .with_value(20.00)
            .with_unit("C")
            .build();

        let actual = RangeBuilder::new("range-1")
            .with_low(low)
            .with_high(high)
            .build();

        assert_eq!(expected, actual)
    }
}
