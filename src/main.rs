use crate::fhir::{Identifier, IdentifierCode, Resource};

mod fhir;

fn main() {

    let res = Resource::Organization {
        identifier: vec![
            Identifier {
                _use: Some(IdentifierCode::Official),
                system: Some("http://example.org/identifiers".to_string()),
                value: Some("12345".to_string()),
            }
        ],
        active: Some(true),
        name: Some("Health Org".to_string()),
        alias: vec!["HO".to_string()],
        description: Some("A healthcare organization".to_string()),
    };

    let s = serde_json::to_string_pretty(&res).unwrap();
    println!("{}", s);

    let data = r#"
    {
  "Organization": {
    "identifier": [
      {
        "_use": "Official",
        "system": "http://example.org/identifiers",
        "value": "12345"
      }
    ],
    "active": true,
    "name": "Health Org",
    "alias": [
      "HO"
    ],
    "description": "A healthcare organization"
  }
}
"#;


    let res = Resource::Endpoint {
        identifier: vec![
            Identifier {
                _use: Some(IdentifierCode::Official),
                system: Some("http://example.org/identifiers".to_string()),
                value: Some("67890".to_string()),
            }
        ],
        status: fhir::EndpointCode::Active,
        name: Some("Health Endpoint".to_string()),
        description: Some("An endpoint for health services".to_string()),
        address: "https://health.example.org/endpoint".to_string(),
        header: vec!["Authorization: Bearer token".to_string()],
    };

    let data = serde_json::to_string_pretty(&res).unwrap();
    println!("{}", data);



    let deserialized: Resource = serde_json::from_str(&data).unwrap();
    println!("------------------------------------");
    println!("{:?}", deserialized);

    match deserialized {
        Resource::Organization { identifier, active, name, alias, description } => {
            println!("Identifier: {:?}", identifier);
            println!("Active: {:?}", active);
            println!("Name: {:?}", name);
            println!("Alias: {:?}", alias);
            println!("Description: {:?}", description);
        },
        Resource::Endpoint { identifier, status, name, description, address, header } => {
            println!("Identifier: {:?}", identifier);
            println!("Status: {:?}", status);
            println!("Name: {:?}", name);
            println!("Description: {:?}", description);
            println!("Address: {:?}", address);
            println!("Header: {:?}", header);
        },
        _ => println!("Not an Organization resource"),
    }
}

macro_rules! is_resource {
    ($res:expr, $res_type:expr) => {
        match $res {
            Resource::Organization { .. } if $res_type == "Organization" => true,
            Resource::Endpoint { .. } if $res_type == "Endpoint" => true,
            _ => false,
        }
    };
}

pub fn do_something_with_org(res: Resource) {
    if is_resource!(&res, "Organization") == false {
        println!("Resource is not an Organization");
        return;
    }

    if let Resource::Organization { identifier, active, name, alias, description } = res {
        println!("Identifier: {:?}", identifier);
        println!("Active: {:?}", active);
        println!("Name: {:?}", name);
        println!("Alias: {:?}", alias);
        println!("Description: {:?}", description);
    }

}