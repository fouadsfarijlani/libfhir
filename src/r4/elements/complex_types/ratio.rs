use serde::{Deserialize, Serialize};

use crate::{
    elements::{Element, Quantity},
    resources::{self, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ratio {
    #[serde(flatten)]
    pub element: Element,
    pub numerator: Option<Quantity>,
    pub denomenator: Option<Quantity>,
}

impl ResourceType for Ratio {
    const TYPE: &'static str = "Ratio";
}

impl Ratio {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct RatioBuilder {
    element: Element,
    numerator: Option<Quantity>,
    denomenator: Option<Quantity>,
}

impl RatioBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut ratio_builder = RatioBuilder::default();
        ratio_builder.element.id = Some(id.into());
        ratio_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_nummerator(mut self, numerator: Quantity) -> Self {
        self.numerator = Some(numerator);
        self
    }

    pub fn with_denomenator(mut self, denomenator: Quantity) -> Self {
        self.denomenator = Some(denomenator);
        self
    }

    pub fn build(self) -> Ratio {
        Ratio {
            element: self.element,
            numerator: self.numerator,
            denomenator: self.denomenator,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::elements::{Element, Quantity, QuantityBuilder, Ratio, RatioBuilder};

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "Ratio",
                "id": "ratio-1",
                "numerator": {        
                    "id": "quantity-1",
                    "value": 10.00,
                    "comparator": "the comparator",
                    "unit": "C",
                    "system": "any system",
                    "code": "exact code"
                },
                "denomenator": {        
                    "id": "quantity-2",
                    "value": 20.00,
                    "comparator": "the comparator",
                    "unit": "C",
                    "system": "any system",
                    "code": "exact code"
                }
            }
        "#;
        let numerator = QuantityBuilder::default()
            .with_id("quantity-1")
            .with_value(10.00)
            .with_comparator("the comparator")
            .with_unit("C")
            .with_system("any system")
            .with_code("exact code")
            .build();
        let denomenator = QuantityBuilder::default()
            .with_id("quantity-2")
            .with_value(20.00)
            .with_comparator("the comparator")
            .with_unit("C")
            .with_system("any system")
            .with_code("exact code")
            .build();
        let expected = RatioBuilder::default()
            .with_id("ratio-1")
            .with_nummerator(numerator)
            .with_denomenator(denomenator)
            .build();

        let actual = Ratio::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Ratio {
            element: Element {
                id: Some("ratio-1".to_string()),
                extention: None,
            },
            numerator: Some(Quantity {
                element: Element {
                    id: Some("quantity-1".to_string()),
                    extention: None,
                },
                value: Some(10.00),
                code: Some("exact code".to_string()),
                comparator: Some("the comparator".to_string()),
                system: Some("any system".to_string()),
                unit: Some("kg".to_string()),
            }),
            denomenator: Some(Quantity {
                element: Element {
                    id: Some("quantity-2".to_string()),
                    extention: None,
                },
                value: Some(20.00),
                code: Some("exact code".to_string()),
                comparator: Some("the comparator".to_string()),
                system: Some("any system".to_string()),
                unit: Some("kg".to_string()),
            }),
        };
        let numerator = QuantityBuilder::new("quantity-1")
            .with_value(10.00)
            .with_code("exact code")
            .with_comparator("the comparator")
            .with_system("any system")
            .with_unit("kg")
            .build();
        let denomenator = QuantityBuilder::new("quantity-2")
            .with_value(20.00)
            .with_code("exact code")
            .with_comparator("the comparator")
            .with_system("any system")
            .with_unit("kg")
            .build();

        let actual = RatioBuilder::new("ratio-1")
            .with_nummerator(numerator)
            .with_denomenator(denomenator)
            .build();

        assert_eq!(expected, actual)
    }
}
