use libfhir::{
    r4::resources::Organization,
    resources::{endpoint::Endpoint, resource::GetReferences},
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
    println!("{:?}", org.get_references());

    let example_2 = r#"
    {
    "resourceType": "Endpoint",
  "status": "test",
  "connection_type": {
    "system": "http://terminology.hl7.org/CodeSystem/endpoint-connection-type",
    "code": "hl7-fhir-rest"
  },
     "payload_type": [
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
    println!("{:?}", ep)
}
