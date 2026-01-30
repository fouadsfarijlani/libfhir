use crate::{
    FhirError,
    r4::{elements::Element, resources::ResourceType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Attachment {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation: Option<String>,
}

impl ResourceType for Attachment {
    const TYPE: &'static str = "Attachement";
}

impl Attachment {
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
    use super::*;
    use crate::r4::elements::AttachmentBuilder;
    use serde_json::json;

    #[test]
    fn test_from_json_should_succeed() {
        let data = json!({
            "id": "attachment-1",
            "contentType": "image/png",
            "language": "en",
            "data": "some data",
            "url": "https://example.org",
            "size": 58241,
            "hash": "some hash",
            "title": "Patient Photo",
            "creation": "2025-11-07T14:23:00Z"
        })
        .to_string();

        let expected = AttachmentBuilder::new("attachment-1")
            .content_type("image/png")
            .language("en")
            .data("some data")
            .url("https://example.org")
            .size(58241)
            .hash("some hash")
            .title("Patient Photo")
            .creation("2025-11-07T14:23:00Z")
            .build();

        let actual = Attachment::from_json(&data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_json_value_should_succeed() {
        let attachment = AttachmentBuilder::new("attachment-1")
            .title("X-Ray Image")
            .content_type("image/jpeg")
            .build();

        let expected = json!({
            "id": "attachment-1",
            "contentType": "image/jpeg",
            "title": "X-Ray Image"
        });

        let actual = attachment.to_json_value().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_json_string_should_succeed() {
        let attachment = AttachmentBuilder::new("attachment-1")
            .title("MRI Scan")
            .content_type("image/dicom")
            .build();

        let expected = json!({
            "id": "attachment-1",
            "contentType": "image/dicom",
            "title": "MRI Scan"
        });

        let json_string = attachment.to_json_string().unwrap();
        let actual: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(expected, actual);
    }
}
