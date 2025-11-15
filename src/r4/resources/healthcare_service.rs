use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct HealthcareService {}

impl ResourceType for HealthcareService {
    const TYPE: &'static str = "HealthcareService";
}
