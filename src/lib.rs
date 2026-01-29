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
            pub mod address {
                pub mod builder;
                pub mod element;
            }
            pub use address::builder::*;
            pub use address::element::*;

            pub mod attachment {
                pub mod builder;
                pub mod element;
            }
            pub use attachment::builder::*;
            pub use attachment::element::*;

            // pub mod attachment;
            // pub use attachment::*;

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
        pub mod healthcare_service {
            pub mod builder;
            pub mod resource;
        }
        pub use healthcare_service::builder::*;
        pub use healthcare_service::resource::*;

        pub mod endpoint {
            pub mod builder;
            pub mod resource;
        }
        pub use endpoint::builder::*;
        pub use endpoint::resource::*;

        pub mod organization {
            pub mod builder;
            pub mod resource;
        }
        pub use organization::builder::*;
        pub use organization::resource::*;

        pub mod organization_affiliation {
            pub mod builder;
            pub mod resource;
        }
        pub use organization_affiliation::builder::*;
        pub use organization_affiliation::resource::*;

        pub mod location {
            pub mod builder;
            pub mod resource;
        }
        pub use location::builder::*;
        pub use location::resource::*;

        pub mod resource;
        pub use resource::*;

        pub mod practitioner {
            pub mod builder;
            pub mod resource;
        }
        pub use practitioner::builder::*;
        pub use practitioner::resource::*;

        pub mod practitioner_role {
            pub mod builder;
            pub mod resource;
        }
        pub use practitioner_role::builder::*;
        pub use practitioner_role::resource::*;
    }
}

pub mod error;
pub use error::*;
