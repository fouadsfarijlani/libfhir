use crate::elements::{element::Element, reference::Reference};

#[derive(Debug)]
pub struct Attachement {
    element: Element,
    connection_type: Option<String>,
    language: Option<Vec<String>>,
    data: Option<String>, // to be resolved later
    url: Option<String>,  // to be resolved later
    size: Option<u32>,
    hash: Option<String>,
    title: Option<String>,
    creation: Option<String>, // to be resolved later
}

#[derive(Debug)]
pub struct Coding {
    element: Element,
    system: Option<String>, // to be resolved later
    version: Option<String>,
    code: Option<String>, // to be resolved later
    user_selected: Option<bool>,
}

#[derive(Debug)]
pub struct CodeableConcept {
    elements: Element,
    coding: Option<Vec<Coding>>,
    test: Option<String>,
}

#[derive(Debug)]
pub struct Quantity {
    element: Element,
    value: Option<f32>,
    comperator: Option<String>, // to be resolved later
    unit: Option<String>,
    system: Option<String>,
    code: Option<String>,
}

#[derive(Debug)]
pub struct Money {
    element: Element,
    value: Option<f32>,
    currenty: Option<String>, // toe be resolved later
}

#[derive(Debug)]
pub struct Range {
    element: Element,
    low: Option<Quantity>,
    high: Option<Quantity>,
}

#[derive(Debug)]
pub struct Ratio {
    element: Element,
    numerator: Option<Quantity>,
    denomenator: Option<Quantity>,
}

#[derive(Debug)]
pub struct Period {
    element: Element,
    start: Option<String>, // to br resolved later,
    end: Option<String>,   // to be resolved later
}

#[derive(Debug)]
pub struct SampledData {
    element: Element,
    origin: Option<Quantity>,
    period: Option<f32>, // this should be positive I think
    factor: Option<f32>,
    lower_limit: Option<f32>,
    upper_limit: Option<f32>,
    dimentions: Option<u32>,
    data: Option<String>,
}

#[derive(Debug)]
pub struct Identifier {
    element: Element,
    r#use: Option<String>,
    r#type: Option<CodeableConcept>,
    system: Option<String>,
    value: Option<String>,
    period: Option<Period>, // to be resolved
    assigner: Option<Reference>,
}

#[derive(Debug)]
pub struct HumanName {
    element: Element,
    r#use: Option<String>, // to be resolved
    text: Option<String>,
    family: Option<String>,
    given: Option<Vec<String>>,
    prefix: Option<Vec<String>>,
    suffix: Option<Vec<String>>,
    period: Option<Period>,
}

#[derive(Debug)]
pub struct Address {
    element: Element,
    r#use: Option<String>, // to be resolved
    r#type: Option<CodeableConcept>,
    text: Option<String>,
    line: Option<Vec<String>>,
    city: Option<String>,
    district: Option<String>,
    statte: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    period: Option<Period>,
}

#[derive(Debug)]
pub struct ContactPoint {
    element: Element,
    system: Option<String>, // to be resolved,
    value: Option<String>,
    r#use: Option<String>,
    rank: Option<u32>,
    period: Option<Period>,
}
