use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{CodeableConcept, Element, Period},
        resources::ResourceType,
    },
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Address {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
}

impl ResourceType for Address {
    const TYPE: &'static str = "Address";
}

impl Address {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(self)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::elements::{AddressBuilder, PeriodBuilder};

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = json!({
            "id": "address-1",
            "use": "official",
            "text": "Primary address",
            "line": ["123 Main St"],
            "city": "New York",
            "state": "NY",
            "postalCode": "10001",
            "country": "USA",
            "period": {
                "start": "2024-01-01",
                "end": "2025-01-01"
            }
        })
        .to_string();

        let period = PeriodBuilder::default()
            .with_start("2024-01-01")
            .with_end("2025-01-01")
            .build();

        let expected = AddressBuilder::new("address-1")
            .r#use("official")
            .text("Primary address")
            .add_line("123 Main St")
            .city("New York")
            .state("NY")
            .postal_code("10001")
            .country("USA")
            .period(period)
            .build();

        let actual = Address::from_json(&data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_json_value_should_succeed() {
        let address = AddressBuilder::new("address-1")
            .city("Paris")
            .country("France")
            .build();

        let expected = json!({
            "id": "address-1",
            "city": "Paris",
            "country": "France"
        });

        let actual = address.to_json_value().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_json_string_should_succeed() {
        let address = AddressBuilder::new("address-1")
            .city("Berlin")
            .country("Germany")
            .build();

        let expected = json!({
            "id": "address-1",
            "city": "Berlin",
            "country": "Germany"
        });

        let json_string = address.to_json_string().unwrap();
        let actual: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(expected, actual);
    }
}
