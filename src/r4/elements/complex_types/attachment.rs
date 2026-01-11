use serde::{Deserialize, Serialize};

use crate::r4::{
    elements::Element,
    resources::{self, ResourceType},
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Attachement {
    #[serde(flatten)]
    pub element: Element,
    pub content_type: Option<String>,
    pub language: Option<String>,
    pub data: Option<String>, // to be resolved later
    pub url: Option<String>,  // to be resolved later
    pub size: Option<u32>,
    pub hash: Option<String>, // TODO: consider hash implementation
    pub title: Option<String>,
    pub creation: Option<String>, // to be resolved later
}

impl ResourceType for Attachement {
    const TYPE: &'static str = "Attachement";
}

impl Attachement {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct AttachmentBuilder {
    element: Element,
    content_type: Option<String>,
    language: Option<String>,
    data: Option<String>, // to be resolved later
    url: Option<String>,  // to be resolved later
    size: Option<u32>,
    hash: Option<String>, // TODO: consider hash implementation
    title: Option<String>,
    creation: Option<String>, // to be resolved later
}

impl AttachmentBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut attachment_buider = AttachmentBuilder::default();
        attachment_buider.element.id = Some(id.into());
        attachment_buider
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_creation(mut self, creation: impl Into<String>) -> Self {
        self.creation = Some(creation.into());
        self
    }

    pub fn build(self) -> Attachement {
        Attachement {
            element: self.element,
            content_type: self.content_type,
            language: self.language,
            data: self.data,
            url: self.url,
            size: self.size,
            hash: self.hash,
            title: self.title,
            creation: self.creation,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_json_should_succed() {
        let data = r#"
        {
            "id": "attachment-1",
            "contentType": "image/png",
            "language": "en",
            "data": "some data", 
            "url": "https://example.org",
            "size": 58241,
            "hash": "some hash",
            "title": "Patient Photo",
            "creation": "2025-11-07T14:23:00Z"
        }"#;
        let expected = AttachmentBuilder::default()
            .with_id("attachment-1")
            .with_content_type("image/png")
            .with_language("en")
            .with_data("some data")
            .with_url("https://example.org")
            .with_size(58241)
            .with_hash("some hash")
            .with_title("Patient Photo")
            .with_creation("2025-11-07T14:23:00Z")
            .build();

        let actual = Attachement::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Attachement {
            element: Element {
                id: Some("attachment-1".to_string()),
                extention: None,
            },
            content_type: Some("image/png".to_string()),
            language: Some("nl".to_string()),
            data: Some("data".to_string()),
            url: Some("https://example.org".to_string()),
            size: Some(5432),
            hash: Some("hash".to_string()),
            title: Some("Patient Photo".to_string()),
            creation: Some("2025-11-07T14:23:00Z".to_string()),
        };

        let actual = AttachmentBuilder::new("attachment-1")
            .with_content_type("image/png")
            .with_language("nl")
            .with_data("data")
            .with_url("https://example.org")
            .with_size(5432)
            .with_hash("hash")
            .with_title("Patient Photo")
            .with_creation("2025-11-07T14:23:00Z")
            .build();

        assert_eq!(expected, actual)
    }
}
