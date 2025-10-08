use crate::fhir::r4b::endpoint::{Endpoint, EndpointStatus};
use crate::fhir::r4b::organization;
use crate::fhir::r4b::resources::{EndpointKind, OrganizationKind, Reference};

mod fhir;

fn main() {
    println!("Fhir in a seaparks!?");

    // Create a organizatoin with some fields (sort of builder pattern)
    let mut org = organization::Organization::new("Regional Hospital")
        .with_active(true)
        .with_description("Teaching hospital")
        .with_alias("RH")
        .push_identifier("http://hospital.example.org/ids", "ORG-001")
        .push_identifier("http://hospital.example.org/ids", "ORG-002")
        .with_part_of("Organization/parent-1", "Parent Health Group");

    // Print as JSON
    let pretty = org.to_json_pretty().unwrap();
    println!("{}", pretty);


    /// Create a new organizaiton reference
    let org_ref: Reference<OrganizationKind> =
        Reference::to_id("parent-1").with_display("Parent Health Group");

    // Craete a new endpoint
    let ep = Endpoint::new(EndpointStatus::Active, "https://api.example.org/fhir")
        .with_id("endpoint-123")
        .with_name("FHIR R4B API")
        .push_identifier("urn:sys:endpoints", "ENDP-001")
        .push_header("Authorization: Bearer ${TOKEN}")
        .with_managing_org(org_ref);

    // Create a reference to the endpoint
    let ep_ref: Reference<EndpointKind> = Reference::from(&ep);

    // we could also do it this way:
    // let ep_ref: Reference<EndpointKind> = Reference::to_id("endpoint-123").with_display("FHIR R4B API");
    // but if we already have the endpoint (with id and display filled), we can extract that

    // Update the organization to include the endpoint reference
    let org = org.with_endpoint(ep_ref);

    // And display
    let pretty = org.to_json_pretty().unwrap();
    println!("{}", pretty);
}