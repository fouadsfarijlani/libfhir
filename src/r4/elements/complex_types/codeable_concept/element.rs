use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{Coding, Element},
        resources::ResourceType,
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct CodeableConcept {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl ResourceType for CodeableConcept {
    const TYPE: &'static str = "CodeableConcept";
}

impl CodeableConcept {
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

    use crate::r4::elements::{CodeableConceptBuilder, CodingBuilder};

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "id": "some-id",
                "coding": [
                    {
                        "id": "coding-id",
                        "system": "http://example.org",
                        "version": "the version",
                        "code": "important code",
                        "userSelected": false
                    }
                ],
                "text": "nice text"
            }
        "#;

        let expected = CodeableConceptBuilder::new("some-id")
            .with_text("nice text")
            .add_coding(
                CodingBuilder::new("coding-id")
                    .with_system("http://example.org")
                    .with_version("the version")
                    .with_code("important code")
                    .with_user_selected(false)
                    .build(),
            )
            .build();

        let actual = CodeableConcept::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
                        "id": "some-id",
                        "coding": [
                            {
                                "id": "coding-id",
                                "system": "http://example.org",
                                "version": "the version",
                                "code": "important code",
                                "userSelected": false
                            }
                        ],
                        "text": "nice text"
                    }

                );

        let data = CodeableConceptBuilder::new("some-id")
            .with_text("nice text")
            .add_coding(
                CodingBuilder::new("coding-id")
                    .with_system("http://example.org")
                    .with_version("the version")
                    .with_code("important code")
                    .with_user_selected(false)
                    .build(),
            )
            .build()
            .to_json_string()
            .unwrap();

        let actual: serde_json::Value = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(expected, actual)
    }
}
