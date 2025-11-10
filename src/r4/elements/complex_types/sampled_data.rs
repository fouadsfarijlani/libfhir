use serde::{Deserialize, Serialize};

use crate::{
    elements::{Element, Quantity},
    resources::{self, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct SampledData {
    #[serde(flatten)]
    pub element: Element,
    pub origin: Option<Quantity>,
    pub period: Option<f32>, // this should be positive I think
    pub factor: Option<f32>,
    pub lower_limit: Option<f32>,
    pub upper_limit: Option<f32>,
    pub dimentions: Option<u32>,
    pub data: Option<String>,
}

impl ResourceType for SampledData {
    const TYPE: &'static str = "SampledData";
}

impl SampledData {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct SampledDataBuilder {
    element: Element,
    origin: Option<Quantity>,
    period: Option<f32>, // this should be positive I think
    factor: Option<f32>,
    lower_limit: Option<f32>,
    upper_limit: Option<f32>,
    dimentions: Option<u32>,
    data: Option<String>,
}

impl SampledDataBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut sampled_data_builder = Self::default();
        sampled_data_builder.element.id = Some(id.into());
        sampled_data_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_origin(mut self, origin: Quantity) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn with_period(mut self, period: f32) -> Self {
        self.period = Some(period);
        self
    }

    pub fn with_factor(mut self, factor: f32) -> Self {
        self.factor = Some(factor);
        self
    }

    pub fn with_lower_limit(mut self, lower_limit: f32) -> Self {
        self.lower_limit = Some(lower_limit);
        self
    }

    pub fn with_upper_limit(mut self, upper_limit: f32) -> Self {
        self.upper_limit = Some(upper_limit);
        self
    }

    pub fn with_dimentions(mut self, dimensions: u32) -> Self {
        self.dimentions = Some(dimensions);
        self
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn build(self) -> SampledData {
        SampledData {
            element: self.element,
            origin: self.origin,
            period: self.period,
            factor: self.factor,
            lower_limit: self.lower_limit,
            upper_limit: self.upper_limit,
            dimentions: self.dimentions,
            data: self.data,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::elements::QuantityBuilder;

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "SampledData",
                "id": "sample-1",
                "origin" : {
                   "value": 10.00,
                   "unit": "kg"
                },
                "period": 30.00,
                "factor": 10.00,
                "lowerLimit": 10.00,
                "upperLimit": 20.00,
                "dimentions": 50
            }
        "#;
        let origin = QuantityBuilder::default()
            .with_value(10.00)
            .with_unit("kg")
            .build();
        let expected = SampledDataBuilder::default()
            .with_id("sample-1")
            .with_origin(origin)
            .with_period(30.00)
            .with_factor(10.00)
            .with_lower_limit(10.00)
            .with_upper_limit(20.00)
            .with_dimentions(50)
            .build();

        let actual = SampledData::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = SampledData {
            element: Element {
                id: Some("sample-1".to_string()),
                extention: None,
            },
            origin: Some(Quantity {
                element: Element {
                    id: None,
                    extention: None,
                },
                value: Some(10.00),
                unit: Some("cm".to_string()),
                code: None,
                comparator: None,
                system: None,
            }),
            period: Some(10.00),
            factor: Some(10.00),
            lower_limit: Some(20.00),
            upper_limit: Some(30.00),
            dimentions: Some(50),
            data: Some("E | A".to_string()),
        };
        let origin = QuantityBuilder::default()
            .with_value(10.00)
            .with_unit("cm")
            .build();

        let actual = SampledDataBuilder::new("sample-1")
            .with_origin(origin)
            .with_period(10.00)
            .with_factor(10.00)
            .with_factor(10.00)
            .with_lower_limit(20.00)
            .with_upper_limit(30.00)
            .with_dimentions(50)
            .with_data("E | A")
            .build();

        assert_eq!(expected, actual)
    }
}
