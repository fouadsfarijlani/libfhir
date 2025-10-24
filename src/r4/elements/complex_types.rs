use serde::{Deserialize, Serialize};

use crate::{
    elements::{element::Element, reference::Reference},
    resources::Organization,
};

#[derive(Debug)]
pub struct Attachement {
    pub element: Element,
    pub connection_type: Option<String>,
    pub language: Option<Vec<String>>,
    pub data: Option<String>, // to be resolved later
    pub url: Option<String>,  // to be resolved later
    pub size: Option<u32>,
    pub hash: Option<String>,
    pub title: Option<String>,
    pub creation: Option<String>, // to be resolved later
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Coding {
    #[serde(flatten)]
    pub element: Element,
    pub system: Option<String>, // to be resolved later
    pub version: Option<String>,
    pub code: Option<String>, // to be resolved later
    pub user_selected: Option<bool>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct CodeableConcept {
    #[serde(flatten)]
    pub elements: Element,
    pub coding: Option<Vec<Coding>>,
    pub test: Option<String>,
}

#[derive(Debug)]
pub struct Quantity {
    pub element: Element,
    pub value: Option<f32>,
    pub comperator: Option<String>, // to be resolved later
    pub unit: Option<String>,
    pub system: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug)]
pub struct Money {
    pub element: Element,
    pub value: Option<f32>,
    pub currenty: Option<String>, // toe be resolved later
}

#[derive(Debug)]
pub struct Range {
    pub element: Element,
    pub low: Option<Quantity>,
    pub high: Option<Quantity>,
}

#[derive(Debug)]
pub struct Ratio {
    pub element: Element,
    pub numerator: Option<Quantity>,
    pub denomenator: Option<Quantity>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Period {
    #[serde(flatten)]
    pub element: Element,
    pub start: Option<String>, // to br resolved later,
    pub end: Option<String>,   // to be resolved later
}

#[derive(Debug)]
pub struct SampledData {
    pub element: Element,
    pub origin: Option<Quantity>,
    pub period: Option<f32>, // this should be positive I think
    pub factor: Option<f32>,
    pub lower_limit: Option<f32>,
    pub upper_limit: Option<f32>,
    pub dimentions: Option<u32>,
    pub data: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>,
    pub r#type: Option<CodeableConcept>,
    pub system: Option<String>,
    pub value: Option<String>,
    pub period: Option<Period>, // to be resolved
    pub assigner: Option<Reference<Organization>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HumanName {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>, // to be resolved
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
    pub period: Option<Period>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>, // to be resolved
    pub r#type: Option<CodeableConcept>,
    pub text: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub statte: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<Period>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactPoint {
    #[serde(flatten)]
    pub element: Element,
    pub system: Option<String>, // to be resolved,
    pub value: Option<String>,
    pub r#use: Option<String>,
    pub rank: Option<u32>,
    pub period: Option<Period>,
}
