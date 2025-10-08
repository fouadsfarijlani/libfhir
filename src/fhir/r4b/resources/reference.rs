use crate::fhir::r4b::resources::Identifier;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::fhir::r4b::resources::reference_types::ResourceKind;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference<T: ResourceKind> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

impl<T: ResourceKind> Reference<T> {
    pub fn to_id(id: impl Into<String>) -> Self {
        Self {
            reference: Some(format!("{}/{}", T::TYPE, id.into())),
            r#type: None,
            identifier: None,
            display: None,
            _marker: PhantomData,
        }
    }

    pub fn by_identifier(system: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            reference: None,
            r#type: None,
            identifier: Some(Identifier {
                system: Some(system.into()),
                value: Some(value.into()),
            }),
            display: None,
            _marker: PhantomData,
        }
    }

    pub fn with_display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }
}

impl<'de, T: ResourceKind> Deserialize<'de> for Reference<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawRef {
            #[serde(default)]
            reference: Option<String>,
            #[serde(default)]
            r#type: Option<String>,
            #[serde(default)]
            identifier: Option<Identifier>,
            #[serde(default)]
            display: Option<String>,
        }

        let raw = RawRef::deserialize(deserializer)?;
        if let Some(ref r) = raw.reference {
            if !r.starts_with(T::TYPE) || !r[T::TYPE.len()..].starts_with('/') {
                return Err(serde::de::Error::custom(format!(
                    "Reference must target {}",
                    T::TYPE
                )));
            }
        }

        Ok(Self {
            reference: raw.reference,
            r#type: raw.r#type,
            identifier: raw.identifier,
            display: raw.display,

            _marker: PhantomData,
        })
    }
}
