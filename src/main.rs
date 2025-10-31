use libfhir::{
    r4::resources::Organization,
    resources::Endpoint,
};

mod fhir;

fn main() {
    let example = r#"
    {
    "resourceType": "Organization",
    "id": "organization-1",
    "name": "example organization",
    "part_of": {"reference": "Organization/big-corp-1"},
    "endpoint": [{"reference": "Endpoint/ep-default-1"}]
    }
    "#;
    let org = Organization::from(example);
    let org_json = org.to_json();
    match &org_json {
        Ok(org_json) => println!("{:?}", org_json),
        Err(err) => panic!("{err:?}"),
    }

    let example_2 = r#"
    {
    "resourceType": "Endpoint",
  "status": "test",
  "connectionType": {
    "system": "http://terminology.hl7.org/CodeSystem/endpoint-connection-type",
    "code": "hl7-fhir-rest"
  },
     "payloadType": [
    {
      "coding": [
        {
          "system": "http://hl7.org/fhir/resource-types",
          "code": "Patient"
        }
      ],
      "text": "Patient"
    }
  ],
  "address": "https://api.acmehealth.org/fhir/R4"
}
"#;

    let ep = Endpoint::from(example_2);
    println!("{:?}", ep.to_json())
}
