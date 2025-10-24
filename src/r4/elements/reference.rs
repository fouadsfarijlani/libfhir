use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{
    elements::{complex_types::Identifier, element::Element},
    resources::{Endpoint, Organization, ResourceType},
};

#[derive(Debug, Serialize, PartialEq, Deserialize, Default)]
pub struct Reference<T: ResourceType> {
    #[serde(flatten)]
    pub element: Option<Element>,
    pub reference: Option<String>,
    pub r#type: Option<String>,
    pub display: Option<String>,
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip)]
    pub _m: PhantomData<T>,
}

#[derive(Debug)]
pub enum ReferenceTypes<'a> {
    ReferenceOrganization(&'a Reference<Organization>),
    ReferenceEndpoint(&'a Reference<Endpoint>),
}

impl<'a> From<&'a Reference<Endpoint>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Endpoint>) -> Self {
        Self::ReferenceEndpoint(value)
    }
}

impl<'a> From<&'a Reference<Organization>> for ReferenceTypes<'a> {
    fn from(value: &'a Reference<Organization>) -> Self {
        Self::ReferenceOrganization(value)
    }
}

pub trait GetResourceRefernces {
    fn get_references(&self) -> Vec<ReferenceTypes>;
}
