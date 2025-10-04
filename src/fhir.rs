use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Resource {
    Organization {
        identifier: Vec<Identifier>,
        active: Option<bool>,
        // type: Vec<CodableConcepts>,
        name: Option<String>,
        alias: Vec<String>,
        description: Option<String>,
        partof: Reference,
    },
    Endpoint {
        identifier: Vec<Identifier>,
        status: EndpointCode,
        name: Option<String>,
        description: Option<String>,
        address: String,
        header: Vec<String>,
    },
    Facility {},
    Location {},
    Jurisdiction {},
    Practitioner {},
    PractitionerRole {},
    HealthcareService {},
    OrganizationAffiliation {},
}

struct Organization {
    identifier: Vec<Identifier>,
    active: Option<bool>,
    // type: Vec<CodableConcepts>,
    name: Option<String>,
    alias: Vec<String>,
    description: Option<String>,
    // partof: Reference,
}

// impl Organization {
//     pub fn identifier() -> &Vec<Identifier> {
//         &self.identifier
//     }
//
//     pub fn active() -> Option<bool> {
//         self.active
//     }
//
//     pub fn name() -> Option<String> {
//         self.name.clone()
//     }
//
//     pub fn alias() -> &Vec<String> {
//         &self.alias
//     }
//
//     pub fn description() -> Option<String> {
//         self.description.clone()
//     }
// }

// Organization::from_json(json_str: &str) -> anyhow::Result<Organization> {
//     let res = serde_json::from_str(json_str).map_err(|e| anyhow!(e));
//     match res.validate() {
//         Ok(_) => Ok(res),
//         Err(e) => Err(anyhow!("Validation error: {}", e)),
//     }
// }

// let res = OrganizationBuilder::new()
//     .withIdentifier(id1)
//     .withIdentifier(id2)
//     .withActive(true)
//     .withName("Health Org")
//     .withPartOf(ref)
//     .build();
//
//
// fn withPartOf(ref: Reference) {
//     if ref.resource_type != "Organization" {
//         panic!("partof must reference an Organization");
//     }
// }
//
//
// // res.active()
// res.identifier().len() > 4) { println!("Much iddentifiers here...") }
//
// for id in res.identifier() {
//     if id.system = "ura" && ura_allowed.contains(id.value) {
//         println!("Found a valid URA identifier");
//     }
// }
//
// }
//
//
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub _use: Option<IdentifierCode>,
    // type: Option<CodableConcept>,
    pub system: Option<String>,
    pub value: Option<String>, // pub period: Option<Period>,
                               // pub assigner: Option<Reference<Organization>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EndpointCode {
    Active,
    Limited,
    Suspended,
    Error,
    Off,
    EnteredInError,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdentifierCode {
    Usual,
    Official,
    Temp,
    Secondary,
    Old,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

