pub mod r4 {
    pub mod elements {
        pub mod common {
            // this mod are backbone elemnets that do not have FHIR definition
            // but are are used across multiple resources.
            pub mod available_time;
            pub use available_time::*;

            pub mod not_available;
            pub use not_available::*;
        }
        pub use common::*;

        pub mod complex_types {
            pub mod address;
            pub use address::*;

            pub mod attachment;
            pub use attachment::*;

            pub mod codeable_concept;
            pub use codeable_concept::*;

            pub mod coding;
            pub use coding::*;

            pub mod contact_point;
            pub use contact_point::*;

            pub mod human_name;
            pub use human_name::*;

            pub mod identifier;
            pub use identifier::*;

            pub mod money;
            pub use money::*;

            pub mod period;
            pub use period::*;

            pub mod quantity;
            pub use quantity::*;

            pub mod range;
            pub use range::*;

            pub mod ratio;
            pub use ratio::*;

            pub mod sampled_data;
            pub use sampled_data::*;
        }
        pub use complex_types::*;

        pub mod element;
        pub use element::*;

        pub mod reference;
        pub use reference::*;
    }
    pub mod resources {
        pub mod healthcare_service;
        pub use healthcare_service::*;

        pub mod endpoint;
        pub use endpoint::*;

        pub mod organization;
        pub use organization::*;

        pub mod organization_affiliation;
        pub use organization_affiliation::*;

        pub mod location;
        pub use location::*;

        pub mod resource;
        pub use resource::*;

        pub mod practitioner;
        pub use practitioner::*;

        pub mod practitioner_role;
        pub use practitioner_role::*;
    }
}

pub use r4::*;
